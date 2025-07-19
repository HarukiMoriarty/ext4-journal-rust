use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

/// Offsets within the ext4 inode structure
const INODE_OFFSET_MODE: u64 = 0x00;
const INODE_OFFSET_SIZE: u64 = 0x04;
const INODE_OFFSET_FLAGS: u64 = 0x20;
const INODE_OFFSET_BLOCK: u64 = 0x28;
const EXT4_EXTENTS_FLAG: u32 = 0x00080000;

/// Parsed extent header
/// 12 bytes at start of i_block
#[derive(Debug)]
pub(crate) struct ExtentHeader {
    pub magic: u16,
    pub entry_count: u16,
    pub max_entry_count: u16,
    pub tree_depth: u16,
    pub generation: u32,
}

impl ExtentHeader {
    pub fn parse(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);
        let magic = cursor.read_u16::<LittleEndian>().unwrap();
        assert_eq!(magic, 0xf30a, "Invalid extent header magic");

        let entry_count = cursor.read_u16::<LittleEndian>().unwrap();
        let max_entry_count = cursor.read_u16::<LittleEndian>().unwrap();
        let tree_depth = cursor.read_u16::<LittleEndian>().unwrap();
        let generation = cursor.read_u32::<LittleEndian>().unwrap();

        Self {
            magic,
            entry_count,
            max_entry_count,
            tree_depth,
            generation,
        }
    }
}

/// Leaf extent entry
/// 12 bytes per extent if depth == 0
#[derive(Debug)]
pub(crate) struct Extent {
    pub logical_block: u32,  // Logical block index in file
    pub block_count: u16,    // Number of blocks this extent covers
    pub start_block_hi: u16, // Upper 16 bits of physical block
    pub start_block_lo: u32, // Lower 32 bits of physical block
}

impl Extent {
    pub fn parse(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);
        let logical_block = cursor.read_u32::<LittleEndian>().unwrap();
        let block_count = cursor.read_u16::<LittleEndian>().unwrap();
        let start_block_hi = cursor.read_u16::<LittleEndian>().unwrap();
        let start_block_lo = cursor.read_u32::<LittleEndian>().unwrap();

        Self {
            logical_block,
            block_count,
            start_block_hi,
            start_block_lo,
        }
    }

    /// Returns the starting physical block number as u64
    pub fn physical_block_start(&self) -> u64 {
        ((self.start_block_hi as u64) << 32) | (self.start_block_lo as u64)
    }
}

/// Represents a parsed inode, assuming extent-based layout
#[derive(Debug)]
pub(crate) struct Inode {
    pub inode_mode: u16,
    pub inode_size: u32,
    pub extent_blocks: Vec<u64>, // All resolved physical block numbers
    pub extent_header: ExtentHeader,
    pub extents: Vec<Extent>, // All parsed extent entries
}

impl Inode {
    pub(crate) fn parse(inode_bytes: &[u8]) -> Self {
        let mut cursor = Cursor::new(inode_bytes);

        cursor.set_position(INODE_OFFSET_MODE);
        let inode_mode = cursor.read_u16::<LittleEndian>().unwrap();

        cursor.set_position(INODE_OFFSET_SIZE);
        let inode_size = cursor.read_u32::<LittleEndian>().unwrap();

        cursor.set_position(INODE_OFFSET_FLAGS);
        let inode_flags = cursor.read_u32::<LittleEndian>().unwrap();
        assert!(
            inode_flags & EXT4_EXTENTS_FLAG != 0,
            "Expected inode with extents enabled"
        );

        cursor.set_position(INODE_OFFSET_BLOCK);
        let mut i_block_raw = [0u8; 60];
        cursor.read_exact(&mut i_block_raw).unwrap();

        // Parse extent header and assert depth = 0
        let extent_header = ExtentHeader::parse(&i_block_raw[..12]);
        assert_eq!(
            extent_header.tree_depth, 0,
            "Extent trees with depth > 0 are not supported"
        );

        // Parse extent entries
        let mut extents = Vec::new();
        let mut extent_blocks = Vec::new();
        for i in 0..extent_header.entry_count {
            let offset = 12 + (i as usize) * 12;
            let extent = Extent::parse(&i_block_raw[offset..offset + 12]);

            let physical_start = extent.physical_block_start();
            for j in 0..extent.block_count as u64 {
                extent_blocks.push(physical_start + j);
            }

            extents.push(extent);
        }

        println!("Parsed inode:");
        println!("  Mode: 0x{:04x}", inode_mode);
        println!("  Size: {}", inode_size);
        println!("  Extent header: {:?}", extent_header);
        println!("  Extents: {:?}", extents);
        println!("  Resolved physical blocks: {:?}", extent_blocks);

        Self {
            inode_mode,
            inode_size,
            extent_blocks,
            extent_header,
            extents,
        }
    }
}
