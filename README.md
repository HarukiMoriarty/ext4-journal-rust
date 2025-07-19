# ext4-journal-rust

An in-progress **ext4 journal filesystem** parser for toy use in user space.

This project first took root during last semester’s *Advanced OS* course, where I was introduced to the beauty (and chaos) of ext4 journaling. It was love at first sight — but alas, no time back then to write my own. Now that I have more freedom, I’ve decided to revisit the idea.

The goal here isn't performance (it runs in user space after all), but **a deeper understanding of how filesystems work**, especially one as battle-tested and widely used as ext4 in Linux. Rust felt like the right partner for this journey: safe, expressive, and honestly, kind of fun.

## Quick Start

To create a test image:

```bash
$ bash ./tool/create_test_img.sh
```

Then run tests:
```
$ cargo test
```

## TODO:
1. Inode block pointer offset is hardcoded (`0x3C` for 256-byte inodes). Be careful: this changes for 128-byte inodes (`0x28`).
