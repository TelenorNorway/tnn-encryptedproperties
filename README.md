# TNN Encrypted Properties
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?&style=for-the-badge&logo=rust&logoColor=white&color=e57324)](https://www.rust-lang.org/)
[![TNN](https://img.shields.io/badge/TNN-blue?style=for-the-badge)](https://github.com/TelenorNorway/tnn-encryptedproperties)
[![GitHub stars](https://img.shields.io/github/stars/TelenorNorway/tnn-encryptedproperties.svg?style=for-the-badge)](https://github.com/TelenorNorway/tnn-encryptedproperties/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/TelenorNorway/tnn-encryptedproperties.svg?style=for-the-badge)](https://github.com/TelenorNorway/tnn-encryptedproperties/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/TelenorNorway/tnn-encryptedproperties.svg?style=for-the-badge)](https://github.com/TelenorNorway/tnn-encryptedproperties/pulls)

TNN Encrypted Properties is an extension developed by Telenor Norway, aimed at providing a secure method of handling
properties by enabling encryption and decryption using a master password. This extension is designed to integrate
seamlessly with TNN ecosystem for creating CLI tools, aiding in the secure management of sensitive data.

### Usage
#### Encrypting a String
To encrypt a plain text string using your master password, use the following command:
```bash
tnn ep encrypt <MASTER_PASSWORD> <STRING>
```

#### Decrypting an Encrypted String
To decrypt an encrypted string back to its original form using the master password, execute:
```bash
tnn ep decrypt <MASTER_PASSWORD> <ENCRYPTED_STRING>
```

**Repository**: https://github.com/TelenorNorway/tnn
