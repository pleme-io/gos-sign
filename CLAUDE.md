# gos-sign

Sign GrapheneOS factory images and OTA packages. AVB key management. Consumes VbmetaParser, ApkSignatureVerifier, BootImageParser traits.

## Build

```bash
nix build
nix run .#gos-sign
cargo build
```

## Architecture

- Binary: `gos-sign`
- Language: Rust (edition 2024, rust-version 1.91.0)
- License: MIT
- Nix: substrate `rust-tool-release-flake.nix` pattern
