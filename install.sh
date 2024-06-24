#!/bin/bash

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo could not be found. Please install Rust and Cargo first."
    exit 1
fi

# Clone the repository
git clone https://github.com/lukhas27/mdnav.git
cd mdnav

# Build the project
cargo build --release

# Move the binary to /usr/local/bin
sudo mv target/release/mdnav /usr/local/bin/

# Cleanup
cd ..
rm -rf yourproject

echo "Installation completed! You can now use 'mdnav' command."
