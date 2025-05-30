use std::fs;
use std::time::Duration;
use std::io;

pub struct SysInfo {
    pub kernel_version: String,
    pub uptime: Duration,
    pub load_average: (f64, f64, f64),
}

impl SysInfo {
    pub fn gather() -> io::Result<Self> {
        Ok(Self {
            kernel_version: read_kernel_version()?,
            uptime: read_uptime()?,
            load_average: read_load_average()?,
        })
    }
}

fn read_kernel_version() -> io::Result<String> {
    let contents = fs::read_to_string("/proc/version")?;
    // We want to clean up the string to remove unnecessary parts, such as the compiler version
    let version = contents.split_whitespace().nth(2).unwrap_or("unknown").to_string();
    Ok(version.to_string())
}

fn read_uptime() -> io::Result<Duration> {
    let contents = fs::read_to_string("/proc/uptime")?;
    let seconds = contents.split_whitespace().next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to read uptime"))?
        .parse::<f64>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid uptime format"))?;
    Ok(Duration::from_secs_f64(seconds))
}

fn read_load_average() -> io::Result<(f64, f64, f64)> {
    let contents = fs::read_to_string("/proc/loadavg")?;
    let parts: Vec<&str> = contents.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid load average format"));
    }

    let one = parts[0].parse::<f64>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let five = parts[1].parse::<f64>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let fifteen = parts[2].parse::<f64>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok((one, five, fifteen))
}
