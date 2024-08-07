use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("/proc/stat") {
        println!("windows  not working");
        lines.for_each(|line| {
            if let Ok(cpu_line) = line {
                if cpu_line.starts_with("cpu ") {
                    let parts: Vec<&str> = cpu_line.split_whitespace().collect();
                    let user: u64 = parts[1].parse().unwrap();
                    let nice: u64 = parts[2].parse().unwrap();
                    let system: u64 = parts[3].parse().unwrap();
                    let idle: u64 = parts[4].parse().unwrap();
                    println!(
                        "CPU Usage: User={} Nice={} System={} Idle={}",
                        user, nice, system, idle
                    );
                }
            }
        });
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
