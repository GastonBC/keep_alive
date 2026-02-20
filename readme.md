# HDD Keep-Alive Service

This Rust-based utility prevents external Hard Drives from spinning down by performing periodic "heartbeat" writes. It is specifically designed for cases where standard tools like `hdparm` fail to override the drive's internal firmware power management.

## How It Works

The script monitors `/proc/diskstats` to track drive activity:

1. **Check Interval:** Every 10 minutes (`600s`).
2. **Activity Detected:** If external activity is detected (I/O increases by >15), it resets the keep-alive counter.
3. **Keep-Alive State:** While the counter is active, it writes to a `.keepalive.txt` file on the drive to force it to stay awake.
4. **Auto-Stop:** Once the counter exceeds the defined number of loops, the script enters a passive state to allow the drive to rest, only resuming once new manual I/O activity is detected.

---

## Configuration

### Static Constants

Before compiling, ensure these in `utils.rs` (or `main.rs`) match your environment:

* `MOUNT_PATH`: `/mnt/drive1`
* `DRIVE`: `sda`
* `KEEPALIVE_FILE`: `/mnt/drive1/.keepalive.txt`

### Dynamic Timer (No Restart Required)

The service reads the allowed idle duration from a configuration file at the start of every loop. You can change the total keep-alive window without restarting the service.

* **Config File:** `keep_alive.conf` (located in the same folder as the binary).
* **Format:** A single integer representing the number of minutes.
* **Default:** If the file is missing, it defaults to your `DEFAULT_TIMER` constant.

---

## Installation & Build

### 1. Cross-Compilation for Raspberry Pi

Use `cross` to target the Pi 4's architecture from your laptop:

```bash
cross build --release --target aarch64-unknown-linux-gnu

```

### 2. Deployment

1. Move the binary to `/usr/local/bin/keep_alive`.
2. Ensure it is executable: `sudo chmod +x /usr/local/bin/keep_alive`.
3. Create the service file: `sudo nano /etc/systemd/system/hdd-keepalive.service`.

```ini
[Unit]
Description=HDD Keep-Alive Script
After=mnt-drive1.mount

[Service]
ExecStart=/usr/local/bin/keep_alive
Restart=always
User=root
WorkingDirectory=/usr/local/bin

[Install]
WantedBy=multi-user.target

```

---

## Commands

| Action | Command | Note |
| --- | --- | --- |
| **Apply New Code** | `sudo systemctl restart hdd-keepalive` | Required if you update the binary file. |
| **Update Timer** | `echo 2400 > keep_alive.conf` | Changes applied on next loop (no restart). |
| **Start Service** | `sudo systemctl start hdd-keepalive` |  |
| **Stop Service** | `sudo systemctl stop hdd-keepalive` |  |
| **Check Logs** | `journalctl -u hdd-keepalive -f` |  |

---

## Logic Summary

* **Check Frequency:** 600 seconds (10 minutes).
* **Sensitivity:** Triggered if `reads + writes` increase by more than 15 units.
* **Persistence:** The `counter` determines how many 10-minute cycles to stay awake after the last detected activity.