#![cfg_attr(not(feature = "std"), no_std)]

use cosmwasm_std::Uint128; // used to manage private and public inputs - all inputs are prime numbers
use sp_std::prelude::*;

mod verifier;

pub use self::verifier::*;

// Proof info - defined as alpha * beta * gamma
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

// Verify keys - composited to three different prime numbers
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone)]
pub struct VerifyingKey {
    pub alpha_g1: Uint128,
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
