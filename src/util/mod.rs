use async_channel;
use gio::prelude::{AppInfoExt, AppLaunchContextExt};
use glib::VariantDict;
use stable_eyre::{eyre::OptionExt, Result};
use std::path::{Path, PathBuf};
use zbus::zvariant::{OwnedObjectPath, Value};
pub mod session;
use crate::runtime;

// The mode needs to be one of "replace", "fail", "isolate",
// "ignore-dependencies", or "ignore-requirements". If "replace", the method
// will start the unit and its dependencies, possibly replacing already queued
// jobs that conflict with it. If "fail", the method will start the unit and its
// dependencies, but will fail if this would change an already queued job. If
// "isolate", the method will start the unit in question and terminate all units
// that are not dependencies of it. If "ignore-dependencies", it will start a
// unit but ignore all its dependencies. If "ignore-requirements", it will start
// a unit but only ignore the requirement dependencies. It is not recommended to
// make use of the latter two options. On reply, if successful, this method
// returns the newly created job object which has been enqueued for asynchronous
// activation. Callers that want to track the outcome of the actual start
// operation need to monitor the result of this job. This can be achieved in a
// race-free manner by first subscribing to the JobRemoved() signal, then
// calling StartUnit() and using the returned job object to filter out unrelated
// JobRemoved() signals, until the desired one is received, which will then
// carry the result of the start operation.

pub async fn adopt_scope(
    manager: &zbus_systemd::systemd1::ManagerProxy<'_>,
    pid: u32,
    unit_id: &str,
) -> Result<OwnedObjectPath> {
    let mut pid_array = zbus::zvariant::Array::new(&zbus::zvariant::Signature::U32);
    pid_array.append(Value::U32(pid))?;
    // manager.exit();
    let res = manager
        .start_transient_unit(
            unit_id.into(),
            "replace".into(),
            vec![
                ("PIDs".into(), pid_array.try_into()?),
                (
                    "CollectMode".into(),
                    Value::Str("inactive-or-failed".into()).try_into()?,
                ),
                ("Slice".into(), Value::Str("app.slice".into()).try_into()?),
            ],
            Vec::new(),
        )
        .await?;
    Ok(res)
}

pub async fn launch_transient_service(
    manager: &zbus_systemd::systemd1::ManagerProxy<'_>,
    unit_name: &str,
    // Optional slice
    slice: Option<&str>,
    extra_opts: Option<&[(&str, &str)]>,
) -> Result<OwnedObjectPath> {
    let mut properties = Vec::new();
    if let Some(slice) = slice {
        properties.push(("Slice".into(), Value::Str(slice.into()).try_into()?));
    }

    if let Some(extra_opts) = extra_opts {
        properties.extend(extra_opts.iter().map(|(key, value)| {
            (
                key.to_string().into(),
                Value::Str(value.to_string().into()).try_into().unwrap(),
            )
        }));
    }

    Ok(manager
        .start_transient_unit(
            unit_name.to_owned(),
            "replace".into(),
            properties,
            Vec::new(),
        )
        .await?)
}

pub fn appid_from_desktop(path: &str) -> Option<String> {
    let path = Path::new(path);
    path.file_stem().map(|s| s.to_string_lossy().to_string())
}

pub fn systemd_unit_name(appid: &str) -> String {
    let id = ulid::Ulid::new();
    format!("app-{appid}-{id}")
}

#[derive(Debug)]
struct AdoptionRequest {
    pid: i32,
    app_identifier: String,
}

// New common function for adopting launched PIDs to systemd scopes
// todo: use global session for this
async fn adopt_launched_pid_to_systemd_scope(pid: i32, app_identifier: String) -> Result<()> {
    let connection = zbus::Connection::session().await?;
    let manager = zbus_systemd::systemd1::ManagerProxy::new(&connection).await?;
    let unit = format!("{}.scope", systemd_unit_name(&app_identifier));

    let _objp = adopt_scope(&manager, pid.try_into()?, &unit).await?;
    println!("Successfully adopted PID {} to unit {}", pid, unit);

    Ok(())
}

// todo: Figure out how to manage processes and stuff for this
// maybe also automatically move stuff to app.slice
pub fn gio_launch_desktop_file(file: &PathBuf) -> Result<()> {
    let appinfo =
        gio::DesktopAppInfo::from_filename(file.to_str().ok_or_eyre("Invalid desktop file path")?)
            .ok_or_eyre("Invalid desktop file")?;

    // Create channel for systemd adoption requests
    let (sender, receiver) = async_channel::unbounded::<AdoptionRequest>();

    // Set up the async handler for adoption requests
    runtime().spawn(async move {
        while let Ok(request) = receiver.recv().await {
            println!(
                "Processing adoption request for PID {} with app_id {}",
                request.pid, request.app_identifier
            );

            if let Err(e) =
                adopt_launched_pid_to_systemd_scope(request.pid, request.app_identifier).await
            {
                eprintln!("Failed to adopt PID to systemd scope: {}", e);
            }
        }
    });

    let file = file.clone();
    let launch_ctx = gio::AppLaunchContext::default();
    launch_ctx.connect_launched(move |ctx, _appinfo, v| {
        println!("Context: {:?}", ctx);
        println!("V: {:?}", v);
        let pid: Option<i32> = {
            let vdict: VariantDict = v.get().unwrap();
            vdict.lookup("pid").ok().flatten()
        };

        if let Some(pid) = pid {
            if let Some(appid) = appid_from_desktop(file.to_str().unwrap()) {
                let request = AdoptionRequest {
                    pid,
                    app_identifier: appid,
                };

                // Send adoption request through channel
                if let Err(e) = sender.try_send(request) {
                    eprintln!("Failed to send adoption request: {}", e);
                }
            } else {
                eprintln!("Could not get appid from desktop file");
            }
        }
    });

    appinfo.launch_uris(&[], Some(&launch_ctx))?;
    Ok(())
}

pub fn launch_desktop(appinfo: &gio::DesktopAppInfo) -> Result<()> {
    let launch_ctx = gio::AppLaunchContext::new();

    // let app = {
    //     if !app.ends_with(".desktop") {
    //         tracing::warn!("Input {app} Doesn't seem to be a .desktop file, appending extension");
    //         format!("{app}.desktop")
    //     } else {
    //         app.to_string()
    //     }
    // };

    // let appinfo = gio::DesktopAppInfo::new(&app).ok_or_eyre("No such app")?;

    // Create channel for systemd adoption requests
    let (sender, receiver) = async_channel::unbounded::<AdoptionRequest>();

    // Set up the async handler for adoption requests
    runtime().spawn(async move {
        tracing::info!("Spawning adoption thread");
        while let Ok(request) = receiver.recv().await {
            println!(
                "Processing adoption request for PID {} with app_id {}",
                request.pid, request.app_identifier
            );

            if let Err(e) =
                adopt_launched_pid_to_systemd_scope(request.pid, request.app_identifier).await
            {
                eprintln!("Failed to adopt PID to systemd scope: {}", e);
            }
        }
    });

    launch_ctx.connect_launched(move |ctx, appinfo, v| {
        // println!("App launched: {:?}", appinfo.name());
        println!("Context: {:?}", ctx);
        println!("V: {:?}", v);
        let pid: Option<i32> = {
            let vdict: VariantDict = v.get().unwrap();
            vdict.lookup("pid").ok().flatten()
        };

        if let Some(pid) = pid {
            let appid = appinfo.id().unwrap_or_default();
            if let Some(appid_clean) = appid_from_desktop(&appid) {
                let request = AdoptionRequest {
                    pid,
                    app_identifier: appid_clean,
                };

                // Send adoption request through channel
                if let Err(e) = sender.try_send(request) {
                    eprintln!("Failed to send adoption request: {}", e);
                }
            } else {
                eprintln!("Could not get appid from '{}'", appid);
            }
        }
    });

    appinfo.launch_uris(&[], Some(&launch_ctx))?;
    Ok(())
}

#[cfg(test)]
#[test]
fn test_launch_desktop_file() {
    let file = PathBuf::from("/usr/share/applications/Alacritty.desktop");
    gio_launch_desktop_file(&file).unwrap();
}
