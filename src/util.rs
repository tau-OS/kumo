use color_eyre::{eyre::OptionExt, Result};
use fork::{fork, Fork};
use gio::{
    prelude::{AppInfoExt, AppLaunchContextExt},
    DesktopAppInfo,
};
use glib::{variant::ToVariant, Pid, Variant, VariantDict};
use gtk::gio::AppInfo;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};
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

// todo: Figure out how to manage processes and stuff for this
// maybe also automatically move stuff to app.slice
pub fn gio_launch_desktop_file(file: &PathBuf) -> Result<()> {
    let appinfo =
        gio::DesktopAppInfo::from_filename(file.to_str().ok_or_eyre("Invalid desktop file path")?)
            .ok_or_eyre("Invalid desktop file")?;

    let file = file.clone();
    let launch_ctx = gio::AppLaunchContext::default();
    // let file = gio::File::for_path(file);
    // let pathstr = file.to_str().ok_or_eyre("Invalid desktop file path")?;
    launch_ctx.connect_launched(move |ctx, _appinfo, v| {
        // println!("App launched: {:?}", appinfo.name());
        println!("Context: {:?}", ctx);
        println!("V: {:?}", v);
        let pid: Option<i32> = {
            let vdict: VariantDict = v.get().unwrap();
            vdict.lookup("pid").ok().flatten()
        };
        // println!("pid: {:?}", pid);

        if let Some(pid) = pid {
            // todo: Don't actually do this lol
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    // Your async adoption function
                    let connection = zbus::Connection::session().await?;
                    let manager = zbus_systemd::systemd1::ManagerProxy::new(&connection).await?;
                    let appid = appid_from_desktop(file.to_str().unwrap())
                        .ok_or_eyre("Could not get appid")?;
                    let unit = format!("{}.scope", systemd_unit_name(&appid));

                    let _objp = adopt_scope(&manager, pid.try_into()?, &unit).await?;

                    Ok::<(), color_eyre::Report>(())
                })
                .expect("Adoption failed in child process");
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
    // let pathstr = file.to_str().ok_or_eyre("Invalid desktop file path")?;
    launch_ctx.connect_launched(move |ctx, appinfo, v| {
        // println!("App launched: {:?}", appinfo.name());
        println!("Context: {:?}", ctx);
        println!("V: {:?}", v);
        let pid: Option<i32> = {
            let vdict: VariantDict = v.get().unwrap();
            vdict.lookup("pid").ok().flatten()
        };

        let appid = appinfo.id().unwrap_or_default();
        // println!("pid: {:?}", pid);

        if let Some(pid) = pid {
            // todo: Don't actually do this lol
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    // Your async adoption function
                    let connection = zbus::Connection::session().await?;
                    let manager = zbus_systemd::systemd1::ManagerProxy::new(&connection).await?;
                    let appid_owned = appid.clone();
                    let appid = appid_from_desktop(&appid_owned).ok_or_eyre("Could not get appid")?;
                    let unit = format!("{}.scope", systemd_unit_name(&appid));

                    let _objp = adopt_scope(&manager, pid.try_into()?, &unit).await?;

                    Ok::<(), color_eyre::Report>(())
                })
                .expect("Adoption failed in child process");
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
