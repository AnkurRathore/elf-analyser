# elf-analyser
# ELF Analyzer

ELF Analyzer is a Rust-based command-line tool designed to analyze and parse ELF (Executable and Linking Format) files. The tool extracts and displays key information from ELF files, such as headers, sections, and dynamic linking data, providing insights into their structure and contents.

---

## Features
- Parse and display the ELF header.
- Identify ELF file class (32-bit or 64-bit) and endianness.
- Analyze sections and program headers.
- Support for dynamic linking and relocation entries (in progress).
- Cross-platform compatibility (Linux, macOS, Windows).

---

## Installation

### Prerequisites
- **Rust**: Ensure you have Rust installed. You can install it via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Clone the Repository
```bash
git clone https://github.com/yourusername/elf-analyzer.git
cd elf-analyzer
```

### Build the Project
Use `cargo` to build the project:
```bash
cargo build --release
```

---

## Usage

### Basic Command
Run the tool with an ELF file:
```bash
cargo run -- --file /path/to/elf
```

### Example
```bash
cargo run -- --file /bin/ls
```

Output:
```
ELF Header:
  Magic: [0x7f, 0x45, 0x4c, 0x46]
  Class: 64-bit
  Data: Little-endian
  Version: 1
  OS ABI: System V
  Entry Point: 0x400080
```

---

## Project Structure
```
elf_analyzer/
├── src/
│   ├── main.rs         # Entry point for the application
│   ├── elf_parser.rs   # Module for parsing ELF headers and sections
│   ├── relocation.rs   # Module for handling relocation entries
│   ├── dynamic.rs      # Module for parsing dynamic linking data
│   ├── utils.rs        # Utility functions (e.g., endianness handling)
├── Cargo.toml          # Rust project metadata
├── .gitignore          # Ignored files
├── README.md           # Project documentation
```

---

## Contributing

Contributions are welcome! If you'd like to contribute:
1. Fork the repository.
2. Create a new branch:
   ```bash
   git checkout -b feature/your-feature
   ```
3. Commit your changes:
   ```bash
   git commit -m "Add your feature"
   ```
4. Push to the branch:
   ```bash
   git push origin feature/your-feature
   ```
5. Open a pull request.

---

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Future Enhancements
- Add support for relocation entries.
- Enhance support for dynamic linking.
- Include analysis of symbols and debugging information.

---



