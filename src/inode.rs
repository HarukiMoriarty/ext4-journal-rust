use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

/// Offsets within the ext4 inode structure
const OFFSET_MODE: u64 = 0x00;
const OFFSET_SIZE: u64 = 0x04;
const OFFSET_BLOCK: u64 = 0x28;
const NUM_BLOCK_POINTERS: usize = 15;

/// Represents a minimal ext4 inode with mode, size, and block pointers.
/// The block pointers are logical block numbers used to locate file data.
#[derive(Debug)]
pub struct Inode {
    /// File type and permissions
    pub mode: u16,
    /// Total file size in bytes
    pub size: u32,
    /// Block pointers: 12 direct, 1 singly indirect, 1 doubly, 1 triply
    pub block: [u32; NUM_BLOCK_POINTERS],
}

impl Inode {
    /// Parse an ext4 inode from a byte buffer
    ///
    /// # Arguments
    /// * `buf` - Byte buffer containing inode data (typically 128 or 256 bytes)
    ///
    /// # Returns
    /// Parsed `Inode` struct
    pub fn parse(buf: &[u8]) -> Self {
        let mut rdr = Cursor::new(buf);

        rdr.set_position(OFFSET_MODE);
        let mode = rdr.read_u16::<LittleEndian>().unwrap();

        rdr.set_position(OFFSET_SIZE);
        let size = rdr.read_u32::<LittleEndian>().unwrap();

        rdr.set_position(OFFSET_BLOCK);
        let mut block = [0u32; NUM_BLOCK_POINTERS];
        for i in 0..NUM_BLOCK_POINTERS {
            block[i] = rdr.read_u32::<LittleEndian>().unwrap();
        }

        Self { mode, size, block }
    }
}
