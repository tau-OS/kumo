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
    appinfo.launch_uris(&[], Some(&launch_ctx))?;

    // // // todo: fix above, comment below
    // std::process::Command::new("gio")
    //     .arg("launch")
    //     .arg("aaa")
    //     .output()?;

    // We don't wait for the process to finish, detach it

    // hdl.

    // wait infinitely
    Ok(())
}

#[cfg(test)]
#[test]
fn test_launch_desktop_file() {
    let file = PathBuf::from("/usr/share/applications/Alacritty.desktop");
    gio_launch_desktop_file(&file).unwrap();
}
