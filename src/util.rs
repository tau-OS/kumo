use color_eyre::{eyre::OptionExt, Result};
use gio::prelude::AppInfoExt;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemdRunResult {
    unit: String,
    invocation_id: Option<String>,
}

pub fn appid_from_desktop(path: &std::path::Path) -> Option<String> {
    path.file_stem().map(|s| s.to_string_lossy().to_string())
}

pub fn systemd_unit_name(appid: &str) -> String {
    let id = ulid::Ulid::new();
    format!("app-{appid}@{id}")
}

/// `gio launch` a desktop file, which will open the selected application,
// todo: We should probably not rely on the `gio` CLI, but use AppInfo directly.
// help needed here
pub fn systemd_launch(file: &PathBuf) -> Result<SystemdRunResult> {
    let appinfo =
        gio::DesktopAppInfo::from_filename(file.to_str().ok_or_eyre("Invalid desktop file path")?)
            .ok_or_eyre("Invalid desktop file")?;

    use std::process::Command;
    let appid = appid_from_desktop(file).ok_or_eyre("Could not get appid from desktop file")?;
    let unit = systemd_unit_name(&appid);
    println!("Unit: {}", unit);

    let out = Command::new("systemd-run")
        .arg("--user")
        .arg("-G")
        .arg("--slice=app.slice")
        // .arg("--no-block")
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

    let launch_ctx = gio::AppLaunchContext::new();

    // let file = gio::File::for_path(file);
    // let pathstr = file.to_str().ok_or_eyre("Invalid desktop file path")?;
    appinfo.launch_uris(&[], Some(&launch_ctx))?;

    Ok(())
}

#[cfg(test)]
#[test]
fn test_launch_desktop_file() {
    let file = PathBuf::from("/usr/share/applications/Alacritty.desktop");
    gio_launch_desktop_file(&file).unwrap();
}
