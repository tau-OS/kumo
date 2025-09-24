use color_eyre::{eyre::OptionExt, Result};
use gio::prelude::AppInfoExt;
use std::path::PathBuf;

/// `gio launch` a desktop file, which will open the selected application,
// todo: We should probably not rely on the `gio` CLI, but use AppInfo directly.
// help needed here
pub fn gio_launch_desktop_file(file: &PathBuf) -> Result<()> {
    let appinfo =
        gio::DesktopAppInfo::from_filename(file.to_str().ok_or_eyre("Invalid desktop file path")?)
            .ok_or_eyre("Invalid desktop file")?;

    let launch_ctx = gio::AppLaunchContext::new();

    // let file = gio::File::for_path(file);
    // let pathstr = file.to_str().ok_or_eyre("Invalid desktop file path")?;
    // appinfo.launch_uris(&[], Some(&launch_ctx))?;

    //todo: use systemd-run for this
    // https://systemd.io/DESKTOP_ENVIRONMENTS/

    use std::process::Command;

    let status = Command::new("systemd-run")
        .arg("--user")
        .arg("--slice=app.slice")
        .arg("--no-block")
        .arg(appinfo.executable())
        .status()?;

    // if !status.success() {
    //     return Err(color_eyre::eyre::eyre!("systemd-run failed with status: {}", status));
    // }

    Ok(())
}

#[cfg(test)]
#[test]
fn test_launch_desktop_file() {
    let file = PathBuf::from("/usr/share/applications/Alacritty.desktop");
    gio_launch_desktop_file(&file).unwrap();
}
