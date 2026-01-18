# Dispenser 📦

```
                      ______   __                                          
                     |   _  \ |  |.-----.-----.-----.-----.-----.-----.----.
                     |.  |   \|  ||__ --|  _  |  -__|     |__ --|  -__|   _|
                     |.  |    |__||_____|   __|_____|__|__|_____|_____|__|  
                     |:  1    /        |__|                                
                     |::.. . /                                             
                     `------'                                              

                             An advance all in one file and folder 
                                      management tool.
```

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)

## Overview

**Dispenser** is a powerful command-line tool written in Rust that provides advanced file and folder management capabilities. It allows you to retrieve detailed metadata, calculate checksums, and organize files efficiently with various sorting options.

## Features

- **📊 Metadata Retrieval**: Get comprehensive information about files and directories
  - Name, type, size, location
  - Creation, modification, and access timestamps
  - File permissions (read, write, execute)
  - Recursive directory traversal with configurable depth

- **Checksum Calculation**: Compute SHA-256 checksums for file integrity verification

- **Smart File Sorting**: Organize files automatically based on:
  - File extension (`se`)
  - Date created (`sdc`)
  - Date modified (`sdm`)
  - Date accessed (`sda`)

- **High Performance**: Built with Rust for speed and reliability

- **Flexible Input**: Process multiple files and directories in a single command

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/param-jasani/dispenser.git
cd dispenser

# Build the project
cargo build --release

# The binary will be available at target/release/dispenser
```

### Install via Cargo

```bash
cargo install --path .
```

## Usage

### Basic Syntax

```bash
dispenser [OPTIONS] --paths <PATHS>...
```

### Options

| Flag | Long Form | Description | Default |
|------|-----------|-------------|---------|
| `-p` | `--paths <PATHS>` | Path(s) to file or directory (comma-separated) | *Required* |
| `-s` | `--sort <METHOD>` | Sort files by method | `sn` (no sort) |
| `-m` | `--metadata` | Retrieve metadata of the given path(s) | `false` |
| `-d` | `--depth <DEPTH>` | Depth level to traverse for directories | `0` |
| | `--checksum` | Calculate SHA-256 checksum of files | `false` |

### Sorting Methods

- `sn` - No sorting (default)
- `se` - Sort by extension
- `sdc` - Sort by date created
- `sdm` - Sort by date modified
- `sda` - Sort by date accessed

### Examples

#### 1. Display File Metadata

```bash
dispenser -p ./myfile.txt -m
```

#### 2. Display Directory Metadata with Depth

```bash
dispenser -p ./myfolder -m -d 2
```

This will traverse the directory up to 2 levels deep.

#### 3. Calculate Checksum

```bash
dispenser -p ./document.pdf --checksum
```

#### 4. Sort Files by Extension

```bash
dispenser -p ./downloads -s se -d 1
```

This creates subdirectories based on file extensions and moves files accordingly.

#### 5. Process Multiple Paths

```bash
dispenser -p ./file1.txt,./file2.pdf,./folder1 -m
```

#### 6. Sort Files by Date Modified

```bash
dispenser -p ./documents -s sdm -d 1
```

Creates date-based folders (YYYY-MM-DD) and organizes files by modification date.

#### 7. Combined Operations

```bash
dispenser -p ./folder -m -d 3 --checksum
```

Displays metadata for all items up to 3 levels deep AND calculates checksums for all files.

## Project Structure

```
dispenser/
├── src/
│   ├── main.rs                    # Entry point and CLI argument parsing
│   ├── display_props.rs           # Functions for displaying file/directory properties
│   ├── path_error_handler.rs      # Path validation and error handling utilities
│   ├── sorter.rs                  # File sorting logic
│   └── properties/
│       ├── mod.rs                 # Module declarations and AccessMethods enum
│       ├── traits.rs              # Common traits (Info, Dates, Permission, etc.)
│       ├── file_props.rs          # FileProperties struct and implementations
│       └── dir_props.rs           # DirectoryProperties struct and implementations
├── Cargo.toml                     # Project dependencies and metadata
└── README.md                      # This file
```

## 🔧 Development

### Running Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy
```

### Build in Debug Mode

```bash
cargo build
```

### Run in Development

```bash
cargo run -- -p ./example -m
```

## Dependencies

- **[clap](https://crates.io/crates/clap)** (v4.5.49) - Command-line argument parsing with derive macros
- **[chrono](https://crates.io/crates/chrono)** (v0.4.42) - Date and time handling
- **[faccess](https://crates.io/crates/faccess)** (v0.2.4) - Cross-platform file permission checks
- **[sha256](https://crates.io/crates/sha256)** (v1.6.0) - SHA-256 hashing
- **[tempfile](https://crates.io/crates/tempfile)** (v3.24.0) - Temporary file creation (dev dependency)

## Use Cases

1. **File Organization**: Automatically sort messy downloads folders by file type or date
2. **System Auditing**: Generate metadata reports for compliance and documentation
3. **Data Integrity**: Verify file checksums for backup and transfer validation
4. **Disk Analysis**: Analyze directory structures and file distributions
5. **Automation**: Integrate into shell scripts for automated file management workflows

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Param Jasani**

- GitHub: [@param-jasani](https://github.com/param-jasani)
- LinkedIn: [param-jasani](https://www.linkedin.com/in/param-jasani/)
- Linktree: [ParamJasani](https://linktr.ee/ParamJasani)

## Acknowledgments

- Built with ❤️ using Rust
- ASCII art generated for project branding
- Inspired by the need for a simple yet powerful file management tool

## Performance

Dispenser is built with performance in mind:

- **Fast**: Rust's zero-cost abstractions ensure minimal overhead
- **Memory Safe**: No buffer overflows or memory leaks
- **Concurrent**: Efficient processing of large directory structures
- **Cross-platform**: Works on Windows, macOS, and Linux

## Known Issues

None at the moment! If you find a bug, please [open an issue](https://github.com/param-jasani/dispenser/issues).

## Future Enhancements

- [ ] Support for more hash algorithms (MD5, SHA-512)
- [ ] Parallel file processing for improved performance
- [ ] Export metadata to JSON/CSV formats
- [ ] File filtering based on size, date ranges, or patterns
- [ ] Interactive TUI mode
- [ ] Configuration file support

---

<div align="center">

**If you find this project useful, please consider giving it a ⭐!**

Made with 🦀 Rust

</div>
