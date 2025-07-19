const DIR_OFFSET_INODE: usize = 0; // Inode Number (4 bytes)
const DIR_OFFSET_REC_LEN: usize = 4; // Record Length (2 bytes)
const DIR_OFFSET_NAME_LEN: usize = 6; // Name Length (1 byte)
const DIR_OFFSET_FILE_TYPE: usize = 7; // File Type (1 byte)
const DIR_OFFSET_NAME: usize = 8; // File Name (variable length)

// EXT4 file type constants
const EXT4_FT_REG_FILE: u8 = 1; // Regular file
const EXT4_FT_DIR: u8 = 2; // Directory
const EXT4_FT_CHRDEV: u8 = 3; // Character device
const EXT4_FT_BLKDEV: u8 = 4; // Block device
const EXT4_FT_FIFO: u8 = 5; // FIFO
const EXT4_FT_SOCK: u8 = 6; // Socket
const EXT4_FT_SYMLINK: u8 = 7; // Symbolic link

/// Represents a single directory entry in an EXT4 filesystem
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    /// Inode number
    pub inode: u32,
    /// File or directory name
    pub name: String,
    /// File type
    pub file_type: u8,
}

impl DirectoryEntry {
    /// Parse a directory entry from a buffer
    ///
    /// # Arguments
    /// * `buf` - Buffer containing directory entry data
    ///
    /// # Returns
    /// `Some((entry, size))` if successful, where size is the entry length in bytes.
    /// `None` if the data is invalid or insufficient.
    pub fn parse(buf: &[u8]) -> Option<(Self, usize)> {
        // Check minimum size for header fields
        if buf.len() < DIR_OFFSET_NAME {
            return None;
        }

        // Read inode number (4 bytes)
        let inode = u32::from_le_bytes(buf[DIR_OFFSET_INODE..DIR_OFFSET_REC_LEN].try_into().ok()?);

        // Read record length (2 bytes)
        let rec_len = u16::from_le_bytes(
            buf[DIR_OFFSET_REC_LEN..DIR_OFFSET_NAME_LEN]
                .try_into()
                .ok()?,
        ) as usize;

        // Read name length (1 byte)
        let name_len = buf[DIR_OFFSET_NAME_LEN] as usize;

        // Read file type (1 byte)
        let file_type = buf[DIR_OFFSET_FILE_TYPE];

        // Validate entry integrity
        // - inode must be non-zero
        // - rec_len must be non-zero
        // - buffer must be large enough to contain both the name and full entry
        if inode == 0
            || rec_len == 0
            || buf.len() < DIR_OFFSET_NAME + name_len
            || buf.len() < rec_len
        {
            return None;
        }

        // Extract filename (not null-terminated in ext4)
        let name =
            String::from_utf8_lossy(&buf[DIR_OFFSET_NAME..DIR_OFFSET_NAME + name_len]).to_string();

        Some((
            DirectoryEntry {
                inode,
                name,
                file_type,
            },
            rec_len,
        ))
    }

    /// Check if this entry represents a directory
    pub(crate) fn is_directory(&self) -> bool {
        self.file_type == EXT4_FT_DIR
    }

    /// Check if this entry represents a regular file
    pub(crate) fn is_file(&self) -> bool {
        self.file_type == EXT4_FT_REG_FILE
    }

    /// Check if this entry represents a symbolic link
    pub(crate) fn is_symlink(&self) -> bool {
        self.file_type == EXT4_FT_SYMLINK
    }

    /// Check if this entry represents a character device
    pub(crate) fn is_char_device(&self) -> bool {
        self.file_type == EXT4_FT_CHRDEV
    }

    /// Check if this entry represents a block device
    pub(crate) fn is_block_device(&self) -> bool {
        self.file_type == EXT4_FT_BLKDEV
    }

    /// Check if this entry represents a FIFO
    pub(crate) fn is_fifo(&self) -> bool {
        self.file_type == EXT4_FT_FIFO
    }

    /// Check if this entry represents a socket
    pub(crate) fn is_socket(&self) -> bool {
        self.file_type == EXT4_FT_SOCK
    }
}

impl std::fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DirEntry{{ name: {}, inode: {}, type: {} }}",
            self.name, self.inode, self.file_type
        )
    }
}
