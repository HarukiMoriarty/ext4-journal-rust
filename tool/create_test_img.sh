# Step 1: Create blank image
dd if=/dev/zero of=ext4.img bs=1M count=32

# Step 2: Format it with ext4
mkfs.ext4 -F ext4.img

# Step 3: Mount and add directories
mkdir -p mnt
sudo mount -o loop ext4.img mnt
sudo mkdir -p mnt/home/zyu379
sudo umount mnt
