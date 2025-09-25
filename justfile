deploy_dir := "${HOME}/Public"
kumo_bin := "target/debug/kumo"
remote := "desktest.local"
remote_bin := "/virtfs/kumo"

build:
    cargo build

deploy-test: build
    rsync -av {{kumo_bin}} {{deploy_dir}}/kumo

close-test:
    ssh {{remote}} pkill -f kumo || true

run-hack: deploy-test close-test
    ssh {{remote}} env GTK_DEBUG=interactive WAYLAND_DISPLAY=wayland-0 {{remote_bin}}
