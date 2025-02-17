# RustLock

Rust ransomware, using AES+RSA. For more info read M3str3/ransomware/README.md

## Key Files
- **compile.py**: Helper script for building in lock or unlock mode.
- **build.rs**: Sets up build-time variables.
- **src/locker.rs**: Main entry point for file encryption.
- **src/unlocker.rs**: Main entry point for file decryption.
- **src/antireversing.rs**: Implements basic anti-debug measures.
- **src/cypher/** & **src/decypher/**: Handle encryption/decryption.
- **keys/**: Stores RSA public and private keys.
