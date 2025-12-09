# Hydration Notifier

A simple service that reminds you to drink water at regular intervals via notification.

## Installation

Install via Systemd

1. Build and Install `hydration-notifier` using Cargo:

```sh
cargo install --path .
```

2. Create a systemd service file at `~/.config/systemd/user/hydration-notifier.service` with the following content:

```ini
[Unit]
Description=Hydration Notifier Service
After=network.target

[Service]
ExecStart=/home/<username>/.cargo/bin/hydration-notifier
Restart=always

[Install]
WantedBy=multi-user.target
```

3. Enable and start the service:

```sh
systemctl --user daemon-reload
systemctl --user enable hydration-notifier.service
systemctl --user start hydration-notifier.service
```

4. Verify the service status:

```sh
systemctl --user status hydration-notifier.service
```
