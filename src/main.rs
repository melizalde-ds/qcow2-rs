use dotenvy::dotenv;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the disk location from environment variables
    let disk_location = get_disk_location();

    // Load the QCOW2 disk image
    let qcow2_image = load_qcow2_image(&disk_location);
    print!("Loaded QCOW2 image of size: {} bytes\n", qcow2_image.len());

    // Read and prints the header information (first 72 bytes for QCOW2 or at least 104 bytes for QCOW2 v3)
    let version = qcow2_image[7];
    let header_size = qcow2_image[103] as usize;
    let header = &qcow2_image[..header_size];

    print!("QCOW2 Version: {}\n", version);
    println!("QCOW2 Header Size: {} bytes", header_size);
    print!("QCOW2 Header: {:x?}\n", header);
}

fn get_disk_location() -> String {
    env::var("DISK_LOCATION").unwrap_or(String::from("default_disk.qcow2"))
}

fn load_qcow2_image(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open QCOW2 file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read QCOW2 file");
    buffer
}
