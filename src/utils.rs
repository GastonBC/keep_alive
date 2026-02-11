use std::{fs, io::{BufReader, Write}};
use std::io::BufRead;
use chrono;

pub fn get_io_count(drive: &str) -> u64 {
    let file = match fs::File::open("/proc/diskstats") {
        Ok(f) => f,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        if line.contains(drive) {
            let fields: Vec<&str> = line.split_whitespace().collect();
            // fields[3] = reads, fields[7] = writes
            let reads = fields.get(3).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            let writes = fields.get(7).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
            return reads + writes;
        }
    }
    0       
}

pub fn is_mounted(path_to_check: &str) -> bool {
    fs::metadata(path_to_check).is_ok()
}

pub fn write_to_dummy(dummy_file: &str, counter: &u8) -> std::io::Result<()> {
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let timestamp = now.format("%Y-%m-%d_%H:%M");
    let content = format!("keepalive {} {}/4", timestamp, counter);

    // List of files to update
    let files_to_update = [dummy_file, ".keep_alive_copy.txt"];

    for path in files_to_update {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;

        writeln!(file, "{}", content)?;
        file.sync_all()?;
    }
    
    println!("Activity triggered: Write successful to both files.");
    Ok(())
}
