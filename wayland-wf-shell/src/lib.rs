// todo: use pkg-config to find wayfire instead of hardcoding the path maybe

use wayland_client;
use wayland_client::protocol::*;

use self::interfaces::*;

use pkg_config;

// const WF_PROTO_PATH: String = "/usr/share/wayfire/protocols/unstable/wayfire-shell-unstable-v2.xml".to_owned();

// i have a feeling this won't work
pub fn wayfire_proto_path() -> String {
    // look up wayfire dependency field pkgdatadir and append 'unstable/wayfire-shell-unstable-v2.xml' to the path
    let wayfire = pkg_config::Config::new()
        .atleast_version("0.7.0")
        .probe("wayfire")
        .unwrap();

    // ex: /usr/share/wayfire/protocols/unstable/wayfire-shell-unstable-v2.xml
    wayfire.include_paths[0].to_str().unwrap().to_string()
        + "/unstable/wayfire-shell-unstable-v2.xml"
}

pub mod interfaces {
    use wayland_client::protocol::__interfaces::*;

    #[cfg(feature = "system_proto")]
    wayland_scanner::generate_interfaces!(
        "/usr/share/wayfire/protocols/unstable/wayfire-shell-unstable-v2.xml"
    );

    // default feature
    #[cfg(not(feature = "system_proto"))]
    wayland_scanner::generate_interfaces!("src/proto/wayfire-shell-unstable-v2.xml");
}

#[cfg(feature = "system_proto")]
wayland_scanner::generate_client_code!(
    "/usr/share/wayfire/protocols/unstable/wayfire-shell-unstable-v2.xml"
);
#[cfg(not(feature = "system_proto"))]
wayland_scanner::generate_client_code!("src/proto/wayfire-shell-unstable-v2.xml");

// okay maybe then
