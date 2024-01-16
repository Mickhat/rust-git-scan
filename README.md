
# Rust Git Scan

## Description
Rust Git Scan is a command-line tool written in Rust for scanning domains for `.git/` folders. This tool is useful for bug bounty hunting and security analysis.

## Features
- Scan a single domain for exposed `.git/` folder.
- Scan multiple domains from a file.
- Supports both `http` and `https` schemes.
- Colored output for better readability.

## Usage
1. **Single Domain Scan**: Use the `-d` flag followed by the domain name.
   Example: `rust-git-scan -d example.com`
2. **Multiple Domains Scan**: Use the `-f` flag followed by the file path containing a list of domains.
   Example: `rust-git-scan -f domains.txt`

## Installation
Ensure you have Rust and Cargo installed on your system. Then, clone the repository and build the project:
```
git clone <repository-url>
cd rust-git-scan
cargo build --release
```
The executable will be available in the `target/release` directory.

## Dependencies
- `reqwest` for making HTTP requests.
- `clap` for parsing command-line arguments.
- `colored` for colored console output.

## Contributing
Contributions to the Rust Git Scan project are welcome! Please submit a pull request or open an issue for any features or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
