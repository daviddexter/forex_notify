# Forex notify

Simple program that runs in the background to get the latest forex exchange status

Currently it watches against `USDKES`

This program was built while learning rust.

## Installation

### Create a systemd user service

To create a service that requires no root permissions, it has to be located in `$HOME/.config/systemd/user`

In this, the service file for `forex_notify` will be located in `$HOME/.config/systemd/user/forex_notify.service`

Copy the content below into the service file. Amend as you see fit

```
[Unit]
Description=Forex Service
After=network.target

[Service]
Type=simple
WorkingDirectory=/home/<your-user-name>/.cargo/bin/
ExecStart=/home/<your-user-name>/.cargo/bin/forex_notify
Restart=on-failure
[Install]
WantedBy=default.target
```

### Install from git

Run the below command to install `forex_notify`. The installation path will default to `/home/<your-user-name>/.cargo`

Consult `cargo install` documentation for details

If the installation path is not `/home/<your-user-name>/.cargo`, you will have to amend the service file to the path where the binary is located

```sh
cargo install --git https://github.com/daviddexter/forex_notify.git --tag v0.0.1 forex_notify
```

### Controlling the service

```sh
# Control whether service loads on boot
systemctl --user enable forex_notify.service
systemctl --user disable forex_notify.service

# Manual start and stop
systemctl --user start forex_notify.service
systemctl --user stop forex_notify.service

# Restarting/reloading
systemctl --user daemon-reload  # Run if forex_notify.service file has changed
systemctl --user restart forex_notify.service

```
