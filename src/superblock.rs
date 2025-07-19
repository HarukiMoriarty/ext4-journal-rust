use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Fixed offsets for superblock fields
const SUPERBLOCK_OFFSET_INODES_COUNT: u64 = 0x00; // Total inodes count
const SUPERBLOCK_OFFSET_BLOCKS_COUNT: u64 = 0x04; // Total blocks count
const SUPERBLOCK_OFFSET_LOG_BLOCK_SIZE: u64 = 0x18; // Log2 of block size
const SUPERBLOCK_OFFSET_INODES_PER_GROUP: u64 = 0x28; // Number of inodes per block group
const SUPERBLOCK_OFFSET_INODE_SIZE: u64 = 0x58; // Size of inode structure
const SUPERBLOCK_OFFSET_VOLUME_NAME: u64 = 0x78; // Volume name/label
const SUPERBLOCK_VOLUME_NAME_LENGTH: usize = 16; // Maximum volume name length

/// Represents the ext4 superblock structure
///
/// The superblock contains critical metadata about the entire filesystem,
/// including layout information, feature flags, and filesystem parameters.
/// It is typically located at byte offset 1024 in the filesystem.
#[derive(Debug, Clone)]
pub(crate) struct Superblock {
    /// Total number of inodes in the filesystem
    pub(crate) inodes_count: u32,

    /// Total number of blocks in the filesystem
    pub(crate) blocks_count: u32,

    /// Log base 2 of the block size
    ///
    /// The actual block size is calculated as: 1024 << log_block_size
    /// - log_block_size = 0 → 1024 bytes
    /// - log_block_size = 1 → 2048 bytes  
    /// - log_block_size = 2 → 4096 bytes
    pub(crate) log_block_size: u32,

    /// Number of inodes per group
    pub(crate) inodes_per_group: u32,

    /// Size of each inode structure in bytes
    pub(crate) inode_size: u16,

    /// Volume name/label (up to 16 characters)
    ///
    /// Human-readable name for the filesystem, null-terminated
    pub(crate) volume_name: String,
}

impl Superblock {
    /// Parse a superblock from a byte buffer
    ///
    /// # Arguments
    /// * `buf` - Byte buffer containing the superblock data (minimum 1024 bytes)
    ///
    /// # Returns
    /// A new `Superblock` instance with parsed values
    ///
    /// # Panics
    /// This function will panic if:
    /// - The buffer is too small to contain the required fields
    /// - Any read operation fails (should not happen with valid input)
    pub(crate) fn parse(buf: &[u8]) -> Self {
        let mut reader = Cursor::new(buf);

        // Read total inodes count (4 bytes at offset 0x00)
        reader.set_position(SUPERBLOCK_OFFSET_INODES_COUNT);
        let inodes_count = reader
            .read_u32::<LittleEndian>()
            .expect("Failed to read inodes count");

        // Read total blocks count (4 bytes at offset 0x04)
        reader.set_position(SUPERBLOCK_OFFSET_BLOCKS_COUNT);
        let blocks_count = reader
            .read_u32::<LittleEndian>()
            .expect("Failed to read blocks count");

        // Read log block size (4 bytes at offset 0x18)
        reader.set_position(SUPERBLOCK_OFFSET_LOG_BLOCK_SIZE);
        let log_block_size = reader
            .read_u32::<LittleEndian>()
            .expect("Failed to read log block size");

        // Read inodes per group (4 bytes at offset 0x28)
        reader.set_position(SUPERBLOCK_OFFSET_INODES_PER_GROUP);
        let inodes_per_group = reader
            .read_u32::<LittleEndian>()
            .expect("Failed to read inodes per group");

        // Read inode size (2 bytes at offset 0x58)
        reader.set_position(SUPERBLOCK_OFFSET_INODE_SIZE);
        let inode_size = reader
            .read_u16::<LittleEndian>()
            .expect("Failed to read inode size");

        // Read volume name (16 bytes at offset 0x78)
        reader.set_position(SUPERBLOCK_OFFSET_VOLUME_NAME);
        let mut name_buffer = [0u8; SUPERBLOCK_VOLUME_NAME_LENGTH];
        reader
            .read_exact(&mut name_buffer)
            .expect("Failed to read volume name");

        // Convert volume name to UTF-8 string, removing null terminators
        let volume_name = String::from_utf8_lossy(&name_buffer)
            .trim_end_matches('\0')
            .to_string();

        Self {
            inodes_count,
            blocks_count,
            log_block_size,
            inodes_per_group,
            inode_size,
            volume_name,
        }
    }

    /// Calculate the actual block size in bytes
    ///
    /// # Returns
    /// Block size in bytes, calculated as 1024 * 2^log_block_size
    pub(crate) fn block_size(&self) -> u32 {
        1024 << self.log_block_size
    }
}

impl std::fmt::Display for Superblock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EXT4 Filesystem '{}': {} inodes ({} per group), {} blocks ({} bytes each), inode size: {} bytes",
            self.volume_name,
            self.inodes_count,
            self.inodes_per_group,
            self.blocks_count,
            self.block_size(),
            self.inode_size
        )
    }
}
