# HDD Keep-Alive Service

This Rust-based utility prevents external Hard Drives from spinning down by performing periodic "heartbeat" writes. It is specifically designed for cases where standard tools like `hdparm` fail to override the drive's internal firmware power management.

## How It Works

The script monitors `/proc/diskstats` to track drive activity:

1. **Check Interval:** Every 10 minutes.
2. **Activity Detected:** If external activity is detected, it writes to a hidden file and resets the keep-alive counter.
3. **Idle State:** If no activity is detected, it writes to a `.keepalive.txt` file on the drive to force it to stay awake.
4. **Auto-Stop:** After ~60 minutes of zero user activity, the script stops writing to the drive to allow it to eventually rest, only resuming once it detects manual I/O activity again.

---

## Configuration

Before compiling, ensure the constants in `main.rs` match your environment:

* `MOUNT_PATH`: `/mnt/drive1`
* `DRIVE`: `sda`
* `KEEPALIVE_FILE`: `/mnt/drive1/.keepalive.txt`

---

## Installation & Build

### 1. Cross-Compilation for Raspberry Pi

Use `cross` to target the Pi 4's architecture:

```bash
cross build --release --target aarch64-unknown-linux-gnu

```

### 2. Systemd Service Setup

Create a service file to ensure the script runs in the background:
`sudo nano /etc/systemd/system/hdd-keepalive.service`

```ini
[Unit]
Description=HDD Keep-Alive Script
After=mnt-drive1.mount

[Service]
ExecStart=/usr/local/bin/hdd-keepalive
Restart=always
User=root

[Install]
WantedBy=multi-user.target

```

---

## Commands

Use these commands to manage the service on your server:

| Action | Command |
| --- | --- |
| **Start Service** | `sudo systemctl start hdd-keepalive` |
| **Stop Service** | `sudo systemctl stop hdd-keepalive` |
| **Restart Service** | `sudo systemctl restart hdd-keepalive` |
| **Check Logs** | `journalctl -u hdd-keepalive -f` |

---

## Logic Summary

* **Timer:** 600 seconds (10 minutes).
* **Sensitivity:** Triggered if `reads + writes` increase by more than 15 units.
* **Threshold:** Performs up to 5 keep-alive writes (approx. 50-60 mins) before entering a passive wait state.
