use async_channel;
use gio::prelude::{AppInfoExt, AppLaunchContextExt};
use glib::VariantDict;
use session::AdoptionRequest;
use stable_eyre::{eyre::OptionExt, Result};
use std::path::{Path, PathBuf};
use zbus::zvariant::{OwnedObjectPath, Value};
pub mod session;
use crate::runtime;



pub fn appid_from_desktop(path: &str) -> Option<String> {
    let path = Path::new(path);
    path.file_stem().map(|s| s.to_string_lossy().to_string())
}

