# Hacking Kumo Shell

## Prerequisites

Kumo is meant to be run in a Wayland compositor supporting at least the [`wlr_layer_shell`](https://wayland.app/protocols/wlr-layer-shell-unstable-v1)
protocol, which is used to create fullscreen and overlay windows.

## Development Environment

We primarily use the [Zed Editor](https://zed.dev) as our text editor and environment of choice.

This repository provides tasks to quickly start debugging Kumo Shell in Zed.

### Compositors

In development, we develop using primarily either:

- [LabWC](https://github.com/labwc/labwc) for quick iteration and hacking,
- [Miriway](https://github.com/miriway/miriway) for full integration testing inside a virtual machine, or
- [Wayfire](https://github.com/WayfireWM/wayfire) as an alternative compositor for GPU-composited integration tests.

We haven't decided on the actual compositor that the KIRI desktop will end up on, but for now we prototype the desktop using either
Miriway, LabWC or Wayfire.

For development environments for either of these compositors, check out the [dev](dev/) directory for more info.
