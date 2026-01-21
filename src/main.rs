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

    // Load the QCOW2 disk image
    let qcow2_image = load_qcow2_image(&disk_location);
    print!("Loaded QCOW2 image of size: {} bytes\n", qcow2_image.len());

    // Parse and display QCOW2 header information
    let version = qcow2_image[7];
    print!("QCOW2 version: {}\n", version);

    let header_length: u32;
    if version == 3 {
        header_length = u32::from_be_bytes(qcow2_image[100..104].try_into().unwrap());
    } else {
        header_length = 72;
    }
    print!("QCOW2 header length: {} bytes\n", header_length);
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
