use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Represents the superblock structure of this ext4 filesystem
/// The superblock contains metadata about the filesystem including
/// the number of inodes, blocks, and other configuration parameters
#[derive(Debug)]
pub struct Superblock {
    /// Total number of inodes in the filesystem
    pub inodes_count: u32,
    /// Total number of blocks in the filesystem
    pub blocks_count: u32,
    /// Log base 2 of the block size (block_size = 1024 << log_block_size)
    pub log_block_size: u32,
    /// Size of each inode structure in bytes
    pub inode_size: u16,
    /// Volume name/label (up to 16 characters)
    pub volume_name: String,
}

impl Superblock {
    /// Parse a superblock from a byte buffer
    ///
    /// # Arguments
    /// * `buf` - Byte buffer containing the superblock data
    ///
    /// # Returns
    /// A new Superblock instance with parsed values
    ///
    /// # Panics
    /// This function will panic if the buffer is too small or if reading fails
    pub fn parse(buf: &[u8]) -> Self {
        let mut rdr = Cursor::new(buf);

        // Start reading from the beginning of the buffer
        rdr.set_position(0);

        // Read inodes count (offset 0x00, 4 bytes)
        let inodes_count = rdr.read_u32::<LittleEndian>().unwrap();

        // Read blocks count (offset 0x04, 4 bytes)
        let blocks_count = rdr.read_u32::<LittleEndian>().unwrap();

        // Skip to log_block_size field (offset 0x18, 4 bytes)
        rdr.set_position(24);
        let log_block_size = rdr.read_u32::<LittleEndian>().unwrap();

        // Skip to inode_size field (offset 0x58, 2 bytes)
        rdr.set_position(88);
        let inode_size = rdr.read_u16::<LittleEndian>().unwrap();

        // Skip to volume name field (offset 0x78, 16 bytes)
        rdr.set_position(120);
        let mut name_buf = [0u8; 16];
        rdr.read_exact(&mut name_buf).unwrap();

        // Convert volume name to string, removing null terminators
        let volume_name = String::from_utf8_lossy(&name_buf)
            .trim_end_matches('\0')
            .to_string();

        Self {
            inodes_count,
            blocks_count,
            log_block_size,
            inode_size,
            volume_name,
        }
    }

    /// Calculate the actual block size in bytes
    ///
    /// # Returns
    /// The block size in bytes (1024 * 2^log_block_size)
    ///
    /// # Examples
    /// ```
    /// // If log_block_size is 0, block size is 1024 bytes
    /// // If log_block_size is 1, block size is 2048 bytes
    /// // If log_block_size is 2, block size is 4096 bytes
    /// ```
    pub fn block_size(&self) -> u32 {
        1024 << self.log_block_size
    }
}
