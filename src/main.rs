use ext4_journal_rust::FileSystem;

fn main() -> std::io::Result<()> {
    let mut fs = FileSystem::open("ext4.img")?;

    println!("{}", fs.superblock.summary());

    let root_inode = fs.read_inode(2)?;
    println!("\nRoot Inode:");
    println!("{:#?}", root_inode);

    Ok(())
}
