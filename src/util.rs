use async_channel;
use color_eyre::{eyre::OptionExt, Result};
use gio::prelude::{AppInfoExt, AppLaunchContextExt};
use glib::VariantDict;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use zbus::zvariant::{OwnedObjectPath, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemdRunResult {
    unit: String,
    invocation_id: Option<String>,
}

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

pub fn appid_from_desktop(path: &str) -> Option<String> {
    let path = Path::new(path);
    path.file_stem().map(|s| s.to_string_lossy().to_string())
}

pub fn systemd_unit_name(appid: &str) -> String {
    let id = ulid::Ulid::new();
    format!("app-{appid}-{id}")
}

/// `gio launch` a desktop file, which will open the selected application,
// todo: We should probably not rely on the `gio` CLI, but use AppInfo directly.
// help needed here
pub fn systemd_launch(file: &PathBuf) -> Result<SystemdRunResult> {
    let appinfo =
        gio::DesktopAppInfo::from_filename(file.to_str().ok_or_eyre("Invalid desktop file path")?)
            .ok_or_eyre("Invalid desktop file")?;

    use std::process::Command;
    let appid = appid_from_desktop(file.to_str().ok_or_eyre("Invalid desktop file path")?)
        .ok_or_eyre("Could not get appid from desktop file")?;
    let unit = systemd_unit_name(&appid);
    println!("Unit: {}", unit);

    let out = Command::new("systemd-run")
        .arg("--user")
        .arg("-G")
        .arg("--slice=app.slice")
        .arg("--json=pretty")
        .arg("--unit")
        .arg(unit)
        .arg(appinfo.executable())
        .output()?;

    if !out.status.success() {
        return Err(color_eyre::eyre::eyre!(
            "systemd-run failed with status: {}",
            out.status
        ));
    }

    Ok(serde_json::from_str(&String::from_utf8_lossy(&out.stdout))?)
}

#[derive(Debug)]
struct AdoptionRequest {
    pid: i32,
    app_identifier: String,
}

// New common function for adopting launched PIDs to systemd scopes
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
    glib::spawn_future_local(async move {
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

pub fn launch_desktop(app: &str) -> Result<()> {
    let launch_ctx = gio::AppLaunchContext::new();

    let app = {
        if !app.ends_with(".desktop") {
            tracing::warn!("Input {app} Doesn't seem to be a .desktop file, appending extension");
            format!("{app}.desktop")
        } else {
            app.to_string()
        }
    };

    let appinfo = gio::DesktopAppInfo::new(&app).ok_or_eyre("No such app")?;

    // Create channel for systemd adoption requests
    let (sender, receiver) = async_channel::unbounded::<AdoptionRequest>();

    // Set up the async handler for adoption requests
    glib::spawn_future_local(async move {
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
