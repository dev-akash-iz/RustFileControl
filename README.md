
---

# RustFileControl – File Manage Tool

A fast, flexible Rust tool to copy, move, and manage files. Skip unwanted folders, filter files, and handle large directory trees efficiently.

---

## Features

* **Custom Filtering** – Exclude folders like `node_modules` or system directories.
* **Efficient Copying** – Queue-based I/O with minimal memory usage.
---

## Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/dev-akash-iz/RustFileControl
cd RustFileControl
cargo build --release
```

Binary is in `target/release/`.

---

## Usage

Create a JSON config:

```json
{
  "process": "copy",
  "sourcePath": "D://Projects",
  "destinationPath": "E://Backup",
  "exclude": ["node_modules", "$RECYCLE.BIN", "System Volume Information"]
}
```

Run:

```bash
cargo run --release 
```

RustFileControl scans, filters, and copies files efficiently.

---

## How It Works

1. **Scan** – Read directories recursively.
2. **Filter** – Skip excluded files/folders.
3. **Queue & Copy** – Buffered, efficient file transfer.

---

## Future Plans

* Multi-threaded & pipelined copy
* more options on Progress reporting & logging
* Move, sync, or mirror modes, others
* Cross-Platform.


---

## License

MIT License – free to use, modify, and distribute.

---
