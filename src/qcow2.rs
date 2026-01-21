use std::io::{Read, Seek, SeekFrom};
use std::{fs::File, io::Error};

pub const QCOW2_MAGIC: u32 = 0x514649fb;

pub trait ValidateQcow2Struct {
    fn is_valid(&self, file: &mut File) -> Result<bool, Error>;
}

#[derive(Debug)]
pub struct Qcow2Header {
    pub version: u32,
    pub backing_file_offset: u64,
    pub backing_file_size: u32,
    pub cluster_bits: u32,
    pub size: u64,
    pub crypt_method: u32,
    pub l1_size: u32,
    pub l1_table_offset: u64,
    pub refcount_table_offset: u64,
    pub refcount_table_clusters: u32,
    pub nb_snapshots: u32,
    pub snapshots_offset: u64,
}

impl Qcow2Header {
    fn validate_backing(&self, file: &mut File) -> Result<bool, Error> {
        if self.backing_file_offset == 0 && self.backing_file_size == 0 {
            Ok(true)
        } else if self.backing_file_offset > 0 && self.backing_file_size > 0 {
            file.seek(SeekFrom::Start(self.backing_file_offset))
                .expect("Failed to seek");
            let mut buffer = vec![0; self.backing_file_size as usize];
            file.read_exact(&mut buffer)
                .expect("Failed to read specified number of bytes");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn validate_cluster_bits(&self) -> Result<bool, Error> {
        if self.cluster_bits >= 9 && self.cluster_bits <= 21 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn validate_crypt_method(&self) -> Result<bool, Error> {
        if self.crypt_method <= 2 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn validate_l1_size(&self) -> Result<bool, Error> {
        if self.size == 0 && self.l1_size != 0 {
            return Ok(false);
        }
        Ok(true)
    }
}

#[derive(Debug)]
pub struct Qcow2V3Header {
    pub incompatible_features: u64,
    pub compatible_features: u64,
    pub autoclear_features: u64,
    pub refcount_order: u32,
    pub header_length: u32,
    pub compression_type: Option<u8>,
}

#[derive(Debug)]
pub struct Qcow2HeaderExtension {
    pub extension_type: u32,
    pub length: u32,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct Qcow2Metadata {
    pub header: Qcow2Header,
    pub v3_header: Option<Qcow2V3Header>,
    pub extensions: Option<Vec<Qcow2HeaderExtension>>,
}

impl TryFrom<Vec<u8>> for Qcow2Metadata {
    type Error = std::io::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let length = value.len();
        let header = Qcow2Header {
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

impl ValidateQcow2Struct for Qcow2Header {
    fn is_valid(&self, file: &mut File) -> Result<bool, Error> {
        let backing_valid = self.validate_backing(file)?;
        let cluster_bits_valid = self.validate_cluster_bits()?;
        let crypt_method_valid = self.validate_crypt_method()?;
        let l1_size_valid = self.validate_l1_size()?;
        if backing_valid && cluster_bits_valid && crypt_method_valid && l1_size_valid {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl ValidateQcow2Struct for Qcow2Metadata {
    fn is_valid(&self, file: &mut File) -> Result<bool, Error> {
        self.header.is_valid(file)
    }
}
