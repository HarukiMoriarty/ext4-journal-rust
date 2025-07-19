mod group;
mod image;
mod inode;
mod superblock;

use crate::group::GroupDescriptor;
use crate::image::read_block;
use crate::inode::Inode;
use crate::superblock::Superblock;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// Represents an ext4 filesystem with read access
pub struct FileSystem {
    /// File handle to the filesystem image or device
    device: File,
    /// Parsed superblock containing filesystem metadata
    pub superblock: Superblock,
}

impl FileSystem {
    /// Open and initialize an ext4 filesystem
    ///
    /// # Arguments
    /// * `path` - Path to filesystem image or device file
    ///
    /// # Returns
    /// Initialized FileSystem instance with parsed superblock
    pub fn open(path: &str) -> std::io::Result<Self> {
        let mut device = File::open(path)?;

        // Read superblock at standard location (offset 1024, size 1024)
        let buf = read_block(&mut device, 1024, 1024)?;
        let sb = Superblock::parse(&buf);

        Ok(FileSystem {
            device,
            superblock: sb,
        })
    }

    /// Read and parse an inode by its number
    ///
    /// # Arguments
    /// * `inode_num` - Inode number (1-indexed)
    ///
    /// # Returns
    /// Parsed Inode structure
    pub fn read_inode(&mut self, inode_num: u32) -> std::io::Result<Inode> {
        let block_size = self.superblock.block_size() as u64;
        let inode_size = self.superblock.inode_size as u64;
        let inodes_per_group = self.superblock.inodes_per_group;

        // Convert to 0-indexed
        let inode_index = inode_num - 1;

        // Determine which block group contains this inode
        let group_index = inode_index / inodes_per_group;
        let local_index = inode_index % inodes_per_group;

        // Get group descriptor to find inode table location
        let group = self.read_group_desc(group_index)?;
        let inode_table_block = group.inode_table_block;

        // Calculate byte offset of the specific inode
        let inode_table_offset = inode_table_block as u64 * block_size;
        let inode_offset = inode_table_offset + (local_index as u64 * inode_size);

        // Read and parse the inode data
        let mut buf = vec![0u8; inode_size as usize];
        self.device.seek(SeekFrom::Start(inode_offset))?;
        self.device.read_exact(&mut buf)?;

        Ok(Inode::parse(&buf))
    }

    /// Read a block group descriptor by index
    ///
    /// # Arguments
    /// * `group_index` - 0-indexed block group number
    ///
    /// # Returns
    /// Parsed GroupDescriptor for the specified group
    fn read_group_desc(&mut self, group_index: u32) -> std::io::Result<GroupDescriptor> {
        let block_size = self.superblock.block_size();

        // Group descriptor table location depends on block size
        let desc_table_offset = if block_size == 1024 {
            2048 // Block 2 for 1KB blocks (superblock takes blocks 0-1)
        } else {
            block_size // Block 1 for larger blocks (superblock is block 0)
        };

        // Each group descriptor is 32 bytes
        let offset = desc_table_offset as u64 + group_index as u64 * 32;

        // Read and parse group descriptor
        let mut buf = [0u8; 32];
        self.device.seek(SeekFrom::Start(offset))?;
        self.device.read_exact(&mut buf)?;

        Ok(GroupDescriptor::parse(&buf))
    }
}
