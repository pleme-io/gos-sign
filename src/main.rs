use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "gos-sign", about = "GrapheneOS release signing")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate signing keys
    GenerateKeys,
    /// Sign an OTA update package
    SignOta,
    /// Sign a factory image
    SignFactory,
}

/// A signing key with algorithm metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SigningKey {
    pub algorithm: String,
    pub fingerprint: String,
    pub created_at: String,
}

/// A signed artifact with provenance information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedArtifact {
    pub artifact_path: String,
    pub signature: String,
    pub signer_fingerprint: String,
    pub signed_at: String,
}

/// Compute a BLAKE3 hash of raw artifact data.
#[must_use]
pub fn compute_artifact_hash(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::GenerateKeys => {
            println!("gos-sign: generating signing keys");
        }
        Command::SignOta => {
            println!("gos-sign: signing OTA package");
        }
        Command::SignFactory => {
            println!("gos-sign: signing factory image");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_determinism() {
        let data = b"GrapheneOS OTA package contents";
        let h1 = compute_artifact_hash(data);
        let h2 = compute_artifact_hash(data);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64); // BLAKE3 hex = 64 chars
    }

    #[test]
    fn different_data_different_hash() {
        let h1 = compute_artifact_hash(b"factory-image-v1");
        let h2 = compute_artifact_hash(b"factory-image-v2");
        assert_ne!(h1, h2);
    }

    #[test]
    fn signing_key_serialization() {
        let key = SigningKey {
            algorithm: "Ed25519".to_string(),
            fingerprint: "SHA256:abc123def456".to_string(),
            created_at: "2026-03-22T00:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&key).unwrap();
        let deserialized: SigningKey = serde_json::from_str(&json).unwrap();
        assert_eq!(key, deserialized);
    }

    #[test]
    fn signed_artifact_serialization() {
        let artifact = SignedArtifact {
            artifact_path: "husky-ota-2026030100.zip".to_string(),
            signature: "base64sig==".to_string(),
            signer_fingerprint: "SHA256:abc123".to_string(),
            signed_at: "2026-03-22T12:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&artifact).unwrap();
        let deserialized: SignedArtifact = serde_json::from_str(&json).unwrap();
        assert_eq!(artifact, deserialized);
    }

    #[test]
    fn empty_data_has_valid_hash() {
        let hash = compute_artifact_hash(b"");
        assert_eq!(hash.len(), 64);
        // BLAKE3 hash of empty input is well-defined
        let hash2 = compute_artifact_hash(b"");
        assert_eq!(hash, hash2);
    }
}
