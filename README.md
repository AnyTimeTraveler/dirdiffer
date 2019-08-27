# dirdiffer

Prints a human readable diff of two directory trees, based on name and filesize.

This program takes two parameters and prints out in which ways the two directories differ.
Which files exist in only one location and which files do exist in both locations, but have different sizes.

# Installation

### Binary (Only on 64-bit x86 processors)

Just download the latest binary from [here](https://github.com/AnyTimeTraveler/dirdiffer/releases) and execute it.
 

### From source
 1. Clone this project
 2. Install rust
 3. Execute build command in this directory: `cargo build --release`
 4. Run the built binary, found here: `target/release/dirdiffer`

# Usage

Quite simple:
./dirdiffer dir1 dir2
