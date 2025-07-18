use std::fs::File;

use ext4_journal_rust::image;
use ext4_journal_rust::superblock;

fn main() -> std::io::Result<()> {
    let mut file = File::open("ext4.img")?;
    let buf = image::read_block(&mut file, 1024, 1024)?;
    let sb = superblock::Superblock::parse(&buf);

    println!("{:#?}", sb);
    println!("Block size: {} bytes", sb.block_size());

    Ok(())
}
