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
}
struct Qcow2Metadata {
    header: Qcow2Header,
    v3_header: Option<Qcow2V3Header>,
}

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the disk location from environment variables
    let disk_location = get_disk_location();

    // Create Header Buffer
    let mut initial_buff: [u8; 72] = [0u8; 72];

    // Load the first 72 bytes of the QCOW2 file
    let mut file = File::open(&disk_location).expect("Failed to open QCOW2 file");
    file.read_exact(&mut initial_buff)
        .expect("Failed to read QCOW2 header");

    // Parse and display QCOW2 header information
    let version = initial_buff[7];
    print!("QCOW2 version: {}\n", version);

    if version == 3 {
        let mut v3_buff: [u8; 104] = [0u8; 104];
        file.read_exact(&mut v3_buff)
            .expect("Failed to read QCOW2 v3 header");
    }
}

fn get_disk_location() -> String {
    env::var("DISK_LOCATION").unwrap_or(String::from("default_disk.qcow2"))
}
