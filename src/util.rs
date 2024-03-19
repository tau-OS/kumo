use color_eyre::Result;
// use gtk::prelude::AppInfoExt;
use std::path::PathBuf;

/// `gio launch` a desktop file, which will open the selected application,
// todo: We should probably not rely on the `gio` CLI, but use AppInfo directly.
// help needed here
pub fn gio_launch_desktop_file(file: &PathBuf) -> Result<()> {
    // let appinfo = gio::AppInfo::create_from_commandline(
    //     file.to_str().unwrap(),
    //     None,
    //     gio::AppInfoCreateFlags::empty(),
    // );

    // let launch_context = gio::AppLaunchContext::new();
    // let _ = appinfo.unwrap().launch(&[], Some(&launch_context));

    let mut cmd = std::process::Command::new("gio");
    cmd.arg("launch").arg(file.to_str().unwrap());

    let _hdl = cmd.spawn()?;
    
    // We don't wait for the process to finish, detach it

    // hdl.

    Ok(())
}
