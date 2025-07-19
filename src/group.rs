use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

/// Offset of the inode table block field in a 32-byte group descriptor.
const GROUP_DESC_OFFSET_INODE_TABLE_BLOCK: u64 = 0x08;

/// Represents a single ext4 block group descriptor.
/// Each block group has its own inode table.
#[derive(Debug)]
pub(crate) struct GroupDescriptor {
    /// Block number where this group's inode table starts
    pub(crate) inode_table_block: u32,
}

impl GroupDescriptor {
    /// Parses a 32-byte ext4 group descriptor
    ///
    /// # Arguments
    /// * `buf` - A byte slice containing one group descriptor (must be at least 12 bytes)
    ///
    /// # Returns
    /// Parsed `GroupDescriptor` with the inode table block number
    pub(crate) fn parse(buf: &[u8]) -> Self {
        let mut rdr = Cursor::new(buf);
        rdr.set_position(GROUP_DESC_OFFSET_INODE_TABLE_BLOCK);
        let inode_table_block = rdr.read_u32::<LittleEndian>().unwrap();
        Self { inode_table_block }
    }
}
