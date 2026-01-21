use dotenvy::dotenv;
use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
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

#[derive(Debug)]
struct Qcow2V3Header {
    incompatible_features: u64,
    compatible_features: u64,
    autoclear_features: u64,
    refcount_order: u32,
    header_length: u32,
    compression_type: Option<u8>,
}

#[derive(Debug)]
struct Qcow2HeaderExtension {
    extension_type: u32,
    length: u32,
    data: Vec<u8>,
}

#[derive(Debug)]
struct Qcow2Metadata {
    header: Qcow2Header,
    v3_header: Option<Qcow2V3Header>,
    extensions: Option<Vec<Qcow2HeaderExtension>>,
}

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the disk location from environment variables
    let mut file = get_file();

    // Create unitial buffer
    let initial_buff: [u8; 8];
    initial_buff = read_file_bytes(&mut file, 8, 0).try_into().unwrap();
    // Version check
    let version = initial_buff[7];
    let buff: Vec<u8>;

    if version == 3 {
        let length_bytes: [u8; 4] = read_file_bytes(&mut file, 4, 100).try_into().unwrap();
        let header_length = u32::from_be_bytes(length_bytes) as usize;
        buff = read_file_bytes(&mut file, header_length, 0);
    } else {
        buff = read_file_bytes(&mut file, 72, 0);
    }

    let header = Qcow2Metadata::try_from(buff).expect("Failed to parse qcow2 metadata");
    println!("QCOW2 Header Metadata: {:#?}", header);
}

fn get_file() -> File {
    let disk_location = env::var("DISK_LOCATION").unwrap_or(String::from("default_disk.qcow2"));
    File::open(disk_location).expect("Failed to open file")
}

fn read_file_bytes(file: &mut File, num_bytes: usize, offset: u64) -> Vec<u8> {
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
        let length = value.len();
        let header = Qcow2Header {
            magic: u32::from_be_bytes(value[0..4].try_into().unwrap()),
            version: u32::from_be_bytes(value[4..8].try_into().unwrap()),
            backing_file_offset: u64::from_be_bytes(value[8..16].try_into().unwrap()),
            backing_file_size: u32::from_be_bytes(value[16..20].try_into().unwrap()),
            cluster_bits: u32::from_be_bytes(value[20..24].try_into().unwrap()),
            size: u64::from_be_bytes(value[24..32].try_into().unwrap()),
            crypt_method: u32::from_be_bytes(value[32..36].try_into().unwrap()),
            l1_size: u32::from_be_bytes(value[36..40].try_into().unwrap()),
            l1_table_offset: u64::from_be_bytes(value[40..48].try_into().unwrap()),
            refcount_table_offset: u64::from_be_bytes(value[48..56].try_into().unwrap()),
            refcount_table_clusters: u32::from_be_bytes(value[56..60].try_into().unwrap()),
            nb_snapshots: u32::from_be_bytes(value[60..64].try_into().unwrap()),
            snapshots_offset: u64::from_be_bytes(value[64..72].try_into().unwrap()),
        };
        if length > 72 {
            let v3_header = Qcow2V3Header {
                incompatible_features: u64::from_be_bytes(value[72..80].try_into().unwrap()),
                compatible_features: u64::from_be_bytes(value[80..88].try_into().unwrap()),
                autoclear_features: u64::from_be_bytes(value[88..96].try_into().unwrap()),
                refcount_order: u32::from_be_bytes(value[96..100].try_into().unwrap()),
                header_length: u32::from_be_bytes(value[100..104].try_into().unwrap()),
                compression_type: if length > 104 { Some(value[104]) } else { None },
            };
            Ok(Self {
                header,
                v3_header: Some(v3_header),
                extensions: None,
            })
        } else {
            Ok(Self {
                header,
                v3_header: None,
                extensions: None,
            })
        }
    }
}
