use dotenvy::dotenv;
use sysinfo::System;
mod byte;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::prelude::*;

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

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");
    let mut sys = System::new_all();
    sys.refresh_all();
    let ram_used = byte::Byte::new(sys.used_memory());
    let total_ram = byte::Byte::new(sys.total_memory());
    println!("** Proc DB Monitor Log **");
    let current_date = chrono::Local::now();
    println!("Date: {}", current_date);
    println!("RAM: {} / {} bytes", ram_used, total_ram);

    let conn_options = MySqlConnectOptions::new()
        .host(std::env::var("DB_HOST").unwrap().as_str())
        .port(std::env::var("DB_PORT").unwrap().parse::<u16>().unwrap())
        .username(std::env::var("DB_USER").unwrap().as_str())
        .password(std::env::var("DB_PASS").unwrap().as_str())
        .database(std::env::var("DB_NAME").unwrap().as_str());
    let mut conn = sqlx::MySqlConnection::connect_with(&conn_options)
        .await
        .expect("Failed to connect to database");
    let row: (String, String) =
        sqlx::query_as("SHOW STATUS WHERE `variable_name` = 'Threads_connected'")
            .fetch_one(&mut conn)
            .await
            .expect("Failed to fetch row");
    let threads = row.1.parse::<u32>().expect("Failed to parse threads value");
    println!("Threads connected: {}", threads);
}
