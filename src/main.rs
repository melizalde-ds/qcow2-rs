use dotenvy::dotenv;
use std::env;
use std::fs::File;
use std::io::Read;

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
    let mut initial_buff: [u8; 72];
    initial_buff = read_file_bytes(&disk_location, 72).try_into().unwrap();

    // Version check
    let version = initial_buff[7];
    if version == 3 {
        let mut v3_buff: [u8; 104];
        v3_buff = read_file_bytes(&disk_location, 104).try_into().unwrap();
    }
}

fn get_disk_location() -> String {
    env::var("DISK_LOCATION").unwrap_or(String::from("default_disk.qcow2"))
}

fn read_file_bytes(path: &str, num_bytes: usize) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buffer = vec![0; num_bytes];
    file.read_exact(&mut buffer)
        .expect("Failed to read specified number of bytes");
    buffer
}

impl TryFrom<Vec<u8>> for Qcow2Metadata {
    type Error = std::io::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        
    }
}
