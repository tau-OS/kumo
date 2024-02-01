// todo: use pkg-config to find wayfire instead of hardcoding the path, and copying the file manually

use wayland_client;
use wayland_client::protocol::*;

use self::interfaces::*;

pub mod interfaces {
    use wayland_client::protocol::__interfaces::*;

    wayland_scanner::generate_interfaces!("src/proto/wayfire-shell-unstable-v2.xml");
}

wayland_scanner::generate_client_code!("src/proto/wayfire-shell-unstable-v2.xml");
