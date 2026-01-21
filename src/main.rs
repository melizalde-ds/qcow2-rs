mod qcow2;

use dotenvy::dotenv;
use qcow2::Qcow2Metadata;
use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the disk location from environment variables
    let mut file = get_file();

    // Create initial buffer
    let initial_buff: [u8; 8] = read_file_bytes(&mut file, 8, 0).try_into().unwrap();
    // Version check
    let version = initial_buff[7];

    // Read full header based on version
    let buff: Vec<u8> = if version == 3 {
        let length_bytes: [u8; 4] = read_file_bytes(&mut file, 4, 100).try_into().unwrap();
        let header_length = u32::from_be_bytes(length_bytes) as usize;
        read_file_bytes(&mut file, header_length, 0)
    } else {
        read_file_bytes(&mut file, 72, 0)
    };

    let header = Qcow2Metadata::try_from(buff).expect("Failed to parse qcow2 metadata");
    println!("QCOW2 Header Metadata: {:#?}", header);
}

fn get_file() -> File {
    let disk_location = env::var("DISK_LOCATION").unwrap_or(String::from("default_disk.qcow2"));
    File::open(disk_location).expect("Failed to open file")
}

fn read_file_bytes(file: &mut File, num_bytes: usize, offset: u64) -> Vec<u8> {
    file.seek(SeekFrom::Start(offset)).expect("Failed to seek");
    let mut buffer = vec![0; num_bytes];
    file.read_exact(&mut buffer)
        .expect("Failed to read specified number of bytes");
    buffer
}
