#!/bin/bash
set -e

# Step 1: Create a 32MB blank image
dd if=/dev/zero of=ext4.img bs=1M count=32

# Step 2: Format it with ext4 (default extents-enabled)
mkfs.ext4 -F ext4.img

# Step 3: Mount and populate
mkdir -p mnt
sudo mount -o loop ext4.img mnt

# Create directory and file
sudo mkdir -p mnt/home/zyu379

# Write content to file
echo "hello from ext4 test" | sudo tee mnt/home/zyu379/test_file.txt > /dev/null

# sync to flush writes
sync

# Step 4: Unmount
sudo umount mnt
