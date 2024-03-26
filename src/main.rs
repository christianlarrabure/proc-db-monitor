use std::collections::HashMap;

use sysinfo::{System};
use dotenvy::dotenv;
use regex::Regex;
mod byte;

pub struct ProcessInfo {
    pub working_directory: String,
    pub pids: Vec<u32>,
    pub ram: byte::Byte,
}

impl ProcessInfo {
    pub fn new(working_directory: String, pids: Vec<u32>, ram: byte::Byte) -> ProcessInfo {
        ProcessInfo {
            working_directory,
            pids,
            ram,
        }
    }

    pub fn add(&mut self, pid: u32, ram: byte::Byte) {
        self.pids.push(pid);
        self.ram.0 += ram.0;
    }
}

fn main() {
    dotenv().expect("Failed to read .env file");
    let PROCESS_REGEX = std::env::var("PROCESS_REGEX").expect("PROCESS_REGEX must be set");
    let process_regex = Regex::new(&PROCESS_REGEX).expect("Invalid regex in PROCESS_REGEX");
    let mut sys = System::new_all();
    sys.refresh_all();
    let ram_used = byte::Byte::new(sys.used_memory());
    let total_ram = byte::Byte::new(sys.total_memory());
    println!("** Proc DB Monitor Log **");
    println!("RAM: {} / {} bytes", ram_used, total_ram);
    let mut processes: HashMap<String, ProcessInfo> = HashMap::new();
    for (pid, process) in sys.processes() {
        let  cwd = process.cwd();
        if cwd.is_none() {
            continue;
        }
        let cwd = cwd.unwrap();
        // match cwd against a regex
        if !process_regex.is_match(cwd.to_str().unwrap()) {
            continue;
        }
        let working_directory = cwd.to_str().unwrap().to_string();
        let ram = byte::Byte::new(process.memory());
        let pid = process.pid().as_u32();
        processes.entry(working_directory.clone()).or_insert(ProcessInfo::new(working_directory, vec![], byte::Byte::new(0))).add(pid, ram);
    }
    for (working_directory, process_info) in processes {
        println!("{}: {} pids, {} RAM", working_directory, process_info.pids.len(), process_info.ram);
    }
}
