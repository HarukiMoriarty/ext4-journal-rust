const OFFSET_INODE: usize = 0; // Inode number (4 bytes)
const OFFSET_REC_LEN: usize = 4; // Record length (2 bytes)
const OFFSET_NAME_LEN: usize = 6; // Name length (1 byte)
const OFFSET_FILE_TYPE: usize = 7; // File type (1 byte)
const OFFSET_NAME: usize = 8; // Start of name (variable length)

/// Represents a single directory entry
#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    /// Inode number
    pub inode: u32,
    /// File or directory name
    pub name: String,
    /// File type (directory, regular file, symlink, etc.)
    pub file_type: u8,
}

impl DirectoryEntry {
    /// Parse a directory entry from buffer
    ///
    /// # Arguments
    /// * `buf` - Buffer containing directory entry data
    ///
    /// # Returns
    /// `Some((entry, size))` if successful, where size is the entry length in bytes
    /// `None` if the data is invalid or insufficient
    pub fn parse(buf: &[u8]) -> Option<(Self, usize)> {
        // Check minimum size for header fields
        if buf.len() < OFFSET_NAME {
            return None;
        }

        // Read inode number (4 bytes, little-endian)
        let inode = u32::from_le_bytes(buf[OFFSET_INODE..OFFSET_REC_LEN].try_into().ok()?);

        // Read record length (2 bytes, little-endian)
        let rec_len =
            u16::from_le_bytes(buf[OFFSET_REC_LEN..OFFSET_NAME_LEN].try_into().ok()?) as usize;

        // Read name length (1 byte)
        let name_len = buf[OFFSET_NAME_LEN] as usize;

        // Read file type (1 byte)
        let file_type = buf[OFFSET_FILE_TYPE];

        // Validate entry integrity
        // - inode must be non-zero
        // - rec_len must be non-zero
        // - buffer must be large enough to contain both the name and full entry
        if inode == 0 || rec_len == 0 || buf.len() < OFFSET_NAME + name_len || buf.len() < rec_len {
            return None;
        }

        // Extract filename (not null-terminated in ext4)
        let name = String::from_utf8_lossy(&buf[OFFSET_NAME..OFFSET_NAME + name_len]).to_string();

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
    pub fn is_directory(&self) -> bool {
        self.file_type == 2 // EXT4_FT_DIR
    }

    /// Check if this entry represents a regular file
    pub fn is_file(&self) -> bool {
        self.file_type == 1 // EXT4_FT_REG_FILE
    }
}
