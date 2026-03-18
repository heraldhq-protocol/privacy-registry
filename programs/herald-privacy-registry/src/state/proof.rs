use anchor_lang::prelude::*;
use light_sdk::instruction::{CompressedProof, ValidityProof};

/// Anchor-compatible wrapper for `CompressedProof`.
///
/// `light_sdk::instruction::CompressedProof` only derives `BorshSerialize` /
/// `BorshDeserialize`, which are incompatible with Anchor's `#[program]` macro.
/// This struct mirrors the same fields and derives `AnchorSerialize` /
/// `AnchorDeserialize` so it can be used as an instruction parameter.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AnchorCompressedProof {
    pub a: [u8; 32],
    pub b: [u8; 64],
    pub c: [u8; 32],
}

impl Default for AnchorCompressedProof {
    fn default() -> Self {
        Self {
            a: [0u8; 32],
            b: [0u8; 64],
            c: [0u8; 32],
        }
    }
}

impl From<AnchorCompressedProof> for CompressedProof {
    fn from(p: AnchorCompressedProof) -> Self {
        CompressedProof {
            a: p.a,
            b: p.b,
            c: p.c,
        }
    }
}

impl From<AnchorCompressedProof> for ValidityProof {
    fn from(p: AnchorCompressedProof) -> Self {
        ValidityProof(Some(CompressedProof::from(p)))
    }
}
