use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let disk_location = env::var("DISK_LOCATION").unwrap_or(String::from("default_disk.qcow2"));
    println!("Disk location: {:?}", disk_location);
}
