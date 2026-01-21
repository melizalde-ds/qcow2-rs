use dotenvy::dotenv;
use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

struct Qcow2Header {
    magic: u32,
    version: u32,
    backing_file_offset: u64,
    backing_file_size: u32,
    cluster_bits: u32,
    size: u64,
    crypt_method: u32,
    l1_size: u32,
    l1_table_offset: u64,
    refcount_table_offset: u64,
    refcount_table_clusters: u32,
    nb_snapshots: u32,
    snapshots_offset: u64,
}

struct Qcow2V3Header {
    incompatible_features: u64,
    compatible_features: u64,
    autoclear_features: u64,
    refcount_order: u32,
    header_length: u32,
    compression_type: Option<u8>,
}

struct Qcow2HeaderExtension {
    extension_type: u32,
    lenght: u32,
    data: Vec<u8>,
}
struct Qcow2Metadata {
    header: Qcow2Header,
    v3_header: Option<Qcow2V3Header>,
    extensions: Option<Vec<Qcow2HeaderExtension>>,
}

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the disk location from environment variables
    let disk_location = get_disk_location();

    // Create unitial buffer
    let initial_buff: [u8; 8];
    initial_buff = read_file_bytes(&disk_location, 8, None).try_into().unwrap();

    // Version check
    let version = initial_buff[7];
    let buff: Vec<u8>;

    if version == 3 {
        let length_bytes: [u8; 4] = read_file_bytes(&disk_location, 4, Some(100))
            .try_into()
            .unwrap();
        let header_length = u32::from_be_bytes(length_bytes) as usize;
        buff = read_file_bytes(&disk_location, header_length, None);
    } else {
        buff = read_file_bytes(&disk_location, 72, None);
    }

    print!("Version: {}\n", version);
    print!("Header Length: {}\n", buff.len());
    print!("{:?}", buff)
}

fn get_disk_location() -> String {
    env::var("DISK_LOCATION").unwrap_or(String::from("default_disk.qcow2"))
}

fn read_file_bytes(path: &str, num_bytes: usize, offset: Option<usize>) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open file");
    let offset = offset.unwrap_or(0);
    file.seek(SeekFrom::Start(offset as u64))
        .expect("Failed to seek");
    let mut buffer = vec![0; num_bytes];
    file.read_exact(&mut buffer)
        .expect("Failed to read specified number of bytes");
    buffer
}

impl TryFrom<Vec<u8>> for Qcow2Metadata {
    type Error = std::io::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        todo!()
    }
}
