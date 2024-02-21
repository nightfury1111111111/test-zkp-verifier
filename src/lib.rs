#![cfg_attr(not(feature = "std"), no_std)]

// use pairing::{Engine, MultiMillerLoop};
use cosmwasm_std::Uint128;
// use codec::{Decode, Encode};
use sp_std::prelude::*;
// use sp_std::sync::Arc;

mod verifier;

pub use self::verifier::*;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Default, Eq)]
pub struct Proof {
    pub a: Uint128,
}

impl PartialEq for Proof {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone)]
pub struct VerifyingKey {
    // alpha in g1 for verifying and for creating A/C elements of
    // proof. Never the point at infinity.
    pub alpha_g1: Uint128,

    // beta in g1 and g2 for verifying and for creating B/C elements
    // of proof. Never the point at infinity.
    pub beta_g1: Uint128,
    pub gamma_g1: Uint128,
}

impl PartialEq for VerifyingKey {
    fn eq(&self, other: &Self) -> bool {
        self.alpha_g1 == other.alpha_g1
            && self.beta_g1 == other.beta_g1
            && self.gamma_g1 == other.gamma_g1
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct PreparedVerifyingKey {
    /// Pairing result of alpha*beta
    alpha_g1_beta_g1: Uint128,
    /// -gamma in G2
    gamma_g1: Uint128,
}

/// This is an error that could occur during circuit synthesis contexts,
/// such as CRS generation or proving.
#[derive(Debug)]
pub enum SynthesisError {
    /// During synthesis, we lacked knowledge of a variable assignment.
    AssignmentMissing,
    /// During synthesis, we divided by zero.
    DivisionByZero,
    /// During synthesis, we constructed an unsatisfiable constraint system.
    Unsatisfiable,
    /// During synthesis, our polynomials ended up being too high of degree
    PolynomialDegreeTooLarge,
    /// During proof generation, we encountered an identity in the CRS
    UnexpectedIdentity,
    /// During proof generation, we encountered an I/O error with the CRS
    IoError,
    MalformedVerifyingKey,
    /// During CRS generation, we observed an unconstrained auxiliary variable
    UnconstrainedVariable,
}

/// An error during verification.
#[derive(Debug, Clone)]
pub enum VerificationError {
    /// Verification was attempted with a malformed verifying key.
    InvalidVerifyingKey,
    /// Proof verification failed.
    InvalidProof,
}
