#!/bin/bash

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo could not be found. Please install Rust and Cargo first."
    exit 1
fi

# Clone the repository
git clone https://github.com/yourusername/yourproject.git
cd yourproject || exit

# Build the project
cargo build --release

# Move the binary to /usr/local/bin
sudo mv target/release/yourproject /usr/local/bin/

# Cleanup
cd ..
rm -rf yourproject

echo "Installation completed! You can now use 'yourproject' command."
