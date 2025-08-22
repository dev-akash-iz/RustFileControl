# 🦀 RustFileControl – File Manage Tool

A **high-performance, multi-threaded** file management tool built in Rust.
Copy, move, or manage large directories efficiently while skipping unwanted folders and controlling CPU usage.

Perfect for developers who want **fast, configurable, and safe file operations**.

---

## ✨ Features

* 🦀 **Smart Filtering** – Exclude folders like `node_modules`, `$RECYCLE.BIN`, or system directories.
* ⚡ **Efficient Copying** – Queue-based I/O with low memory usage.
* 🧵 **Multi-Threading** – Utilizes all CPU cores for maximum speed.
* 🔍 **Config-Driven** – Everything controlled via a simple JSON file.
* 📂 **Recursive Scan** – Works with huge directory trees safely.

---

## 🚀 Installation

```bash
# Install Rust (if not already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone & build
git clone https://github.com/dev-akash-iz/RustFileControl
cd RustFileControl
cargo build --release
```

✅ The compiled binary will be available in:
`target/release/`

---

## ⚡ Usage

Create a `config.json` file with your desired options:

```json
{
  "process": "copy",
  "source_path": "D://Projects",
  "destination_path": "E://Backup",
  "exclude": ["node_modules", "$RECYCLE.BIN", "System Volume Information"],
  "multi_threading": true,
  "cpu_usage_percent": 100
}
```

Run RustFileControl:

```bash
cargo run --release
```

🦀 The tool will **scan → filter → copy/move** your files using multiple threads if enabled.

---

## 🔹 Configuration Keys Explained

* **`process`** – The operation to perform:
    * `"copy"` → copy files from source to destination
    * *(future plans: `"move"`, `"sync"`, `"delete"`)*


* **`source_path`** – Path to the folder **where files will be read from**.


* **`destination_path`** – Path to **where files will be copied or moved to**.

* **`exclude`** – Array of folders or files to **skip during the operation**. Useful for ignoring system folders or large cache directories.

* **`multi_threading`** – Enable/disable **multi-threaded processing**:

    * `true` → utilize multiple threads for faster performance
    * `false` → single-threaded (less CPU load)

* **`cpu_usage_percent`** – Maximum CPU usage allowed:

    * `100` → use all available cores fully
    * `50` → use half of total CPU power
    * Helps balance speed vs system load

---

## 🔧 How It Works

1. **Scan** – Recursively traverse the source directory.
2. **Filter** – Skip any folders or files listed in `exclude`.
3. **Copy** – Buffered, multi-threaded operations for fast and safe file management.

---

## 🛠 Future implementation

* 📊 Advanced **progress reporting & logging**
* 🔄 **Move, sync, mirror** modes
* 🗑 Smart **cache cleanup** (delete only included folders)

---

## 📜 License

MIT License – free to use, modify, and distribute.
