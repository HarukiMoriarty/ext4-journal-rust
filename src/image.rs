use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// Reads a block of data from a file at a specific offset
///
/// # Arguments
/// * `file` - A mutable reference to an open File handle
/// * `offset` - The byte offset from the start of the file where reading should begin
/// * `size` - The number of bytes to read from the file
///
/// # Returns
/// * `Ok(Vec<u8>)` - A vector containing the read data on success
/// * `Err(std::io::Error)` - An IO error if seeking or reading fails
///
/// # Errors
/// This function will return an error if:
/// * The file seek operation fails (e.g., invalid offset)
/// * The file read operation fails (e.g., unexpected EOF, permission issues)
/// * The file doesn't contain enough data to read the requested size
///
/// # Examples
/// ```rust
/// use std::fs::File;
///
/// let mut file = File::open("filesystem.img")?;
///
/// // Read the first 1024 bytes (typical superblock size)
/// let superblock_data = read_block(&mut file, 1024, 1024)?;
///
/// // Read a 4KB block at offset 8192
/// let block_data = read_block(&mut file, 8192, 4096)?;
/// ```
pub fn read_block(file: &mut File, offset: u64, size: usize) -> std::io::Result<Vec<u8>> {
    // Allocate buffer with the requested size
    let mut buf = vec![0u8; size];

    // Seek to the specified offset from the beginning of the file
    file.seek(SeekFrom::Start(offset))?;

    // Read exactly the requested number of bytes
    // This will return an error if EOF is reached before reading all bytes
    file.read_exact(&mut buf)?;

    // Return the buffer containing the read data
    Ok(buf)
}
