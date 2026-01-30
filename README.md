<div align="center">

# 🔐 TNN

**Secure String Encryption CLI**

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?&style=for-the-badge&logo=rust&logoColor=white&color=e57324)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.2.0-green.svg?style=for-the-badge)](Cargo.toml)

*A blazingly fast command-line tool for encrypting and decrypting strings using military-grade AES-256-CBC encryption*

[Installation](#-installation) • [Usage](#-usage) 

</div>

---

## 📦 Installation

### Quick Install

```bash
cargo install --path .
```

The `tnn` binary will be installed to `~/.cargo/bin/` (ensure it's in your PATH).

### Build from Source

```bash
# Clone the repository (if not already done)
git clone <your-repo-url>
cd tnn-encryptedproperties

# Build in release mode
cargo build --release

# Binary available at target/release/tnn
```

## 🚀 Usage

### Encrypt a String

Transform plain text into encrypted ciphertext:

```bash
tnn encrypt <MASTER_PASSWORD> <VALUE>
```

**Example:**
```bash
$ tnn encrypt "dGVzdGtleTE2Ynl0ZXNsb25nMTIzNDU2Nzg5MDEyMzQ=" "my secret password"
wsGAOKBLOp8AoqqjcAaPJgsqw9ExCRKa+FiiebBwiuAChasJAdnPbB6U+FT46muZ
```

### Decrypt a String

Recover your original data from encrypted text:

```bash
tnn decrypt <MASTER_PASSWORD> <ENCRYPTED_VALUE>
```

**Example:**
```bash
$ tnn decrypt "dGVzdGtleTE2Ynl0ZXNsb25nMTIzNDU2Nzg5MDEyMzQ=" "wsGAOKBLOp8AoqqjcAaPJgsqw9ExCRKa+FiiebBwiuAChasJAdnPbB6U+FT46muZ"
my secret password
```

### Get Help

```bash
tnn --help           # General help
tnn encrypt --help   # Encrypt command help
tnn decrypt --help   # Decrypt command help
tnn --version        # Check version
```

## 🔑 Master Password

The master password must be a **base64-encoded AES key**. Supported key sizes:

| Key Size | Bits | Bytes (decoded) |
|----------|------|-----------------|
| AES-128  | 128  | 16              |
| AES-192  | 192  | 24              |
| AES-256  | 256  | 32              |

**Generate a secure key:**
```bash
# Generate a 256-bit (32-byte) key
openssl rand -base64 32
```

## 🛠️ Development

```bash
# Run tests
cargo test

# Run with debug info
cargo run -- encrypt "key" "value"

# Check for issues
cargo clippy

# Format code
cargo fmt
```

## 📄 License

This project is licensed under the MIT License.

---

<div align="center">

**Made with ❤️ and Rust**

*If you find this tool useful, consider giving it a ⭐*

</div>
