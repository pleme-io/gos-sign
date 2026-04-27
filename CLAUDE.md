# gos-sign

> **★★★ CSE / Knowable Construction.** This repo operates under **Constructive Substrate Engineering** — canonical specification at [`pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`](https://github.com/pleme-io/theory/blob/main/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md). The Compounding Directive (operational rules: solve once, load-bearing fixes only, idiom-first, models stay current, direction beats velocity) is in the org-level pleme-io/CLAUDE.md ★★★ section. Read both before non-trivial changes.


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
