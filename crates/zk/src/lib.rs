//! Proof verification and scoped nullifiers.
//!
//! The verifier is a stub. It stands in for zkPassport (or World ID) until the
//! real SDK is wired in. The scoping rule is the part that must not change:
//! each module has its own scope, so one person gets a different nullifier in
//! each module and the two cannot be linked.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    FactCheck,
    Leaks,
}

impl Scope {
    pub fn as_str(self) -> &'static str {
        match self {
            Scope::FactCheck => "cr-factcheck-v1",
            Scope::Leaks => "cr-leaks-v1",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Nullifier(pub String);

/// What the client sends. With zkPassport this is the proof bundle.
#[derive(Debug, Deserialize)]
pub struct ProofRequest {
    /// Stub only: a per-person secret that the real proof keeps hidden.
    pub identity_commitment: String,
    pub nationality: String,
    pub age_over_18: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VerifyError {
    NotCostaRican,
    Underage,
}

pub fn verify(scope: Scope, proof: &ProofRequest) -> Result<Nullifier, VerifyError> {
    if proof.nationality != "CRI" {
        return Err(VerifyError::NotCostaRican);
    }
    if !proof.age_over_18 {
        return Err(VerifyError::Underage);
    }
    Ok(nullifier(scope, &proof.identity_commitment))
}

fn nullifier(scope: Scope, identity_commitment: &str) -> Nullifier {
    let digest = Sha256::new()
        .chain_update(scope.as_str())
        .chain_update([0u8])
        .chain_update(identity_commitment)
        .finalize();
    Nullifier(hex::encode(digest))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proof() -> ProofRequest {
        ProofRequest {
            identity_commitment: "same-person".into(),
            nationality: "CRI".into(),
            age_over_18: true,
        }
    }

    #[test]
    fn same_person_is_unlinkable_across_modules() {
        let a = verify(Scope::FactCheck, &proof()).unwrap();
        let b = verify(Scope::Leaks, &proof()).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn same_person_is_stable_within_a_module() {
        assert_eq!(
            verify(Scope::Leaks, &proof()).unwrap(),
            verify(Scope::Leaks, &proof()).unwrap()
        );
    }

    #[test]
    fn rejects_non_citizens_and_minors() {
        let mut p = proof();
        p.nationality = "USA".into();
        assert_eq!(verify(Scope::Leaks, &p), Err(VerifyError::NotCostaRican));
        let mut p = proof();
        p.age_over_18 = false;
        assert_eq!(verify(Scope::Leaks, &p), Err(VerifyError::Underage));
    }
}
