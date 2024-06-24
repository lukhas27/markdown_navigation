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

# Create the bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Move the binary to ~/.local/bin/
mv target/release/mdnav ~/.local/bin/

# Cleanup
cd ..
rm -rf mdnav

echo "Installation completed! You can now use 'mdnav' command."
