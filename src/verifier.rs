#![cfg_attr(not(feature = "std"), no_std)]

use super::{PreparedVerifyingKey, Proof, VerificationError, VerifyingKey};
// use group::{prime::PrimeCurveAffine, Curve};
// use pairing::{MillerLoopResult, MultiMillerLoop};
// use cosmwasm_std::Uint128;

// use sp_std::ops::{AddAssign};

pub fn prepare_verifying_key(vk: &VerifyingKey) -> PreparedVerifyingKey {
    let alpha_g1_beta_g1 = vk.alpha_g1.checked_mul(vk.beta_g1).unwrap();
    PreparedVerifyingKey {
        alpha_g1_beta_g1: alpha_g1_beta_g1,
        gamma_g1: vk.gamma_g1,
    }
}

pub fn verify_proof(pvk: &PreparedVerifyingKey, proof: &Proof) -> Result<(), VerificationError> {
    // The original verification equation is:
    // a = alpha * beta * gamma

    if pvk.alpha_g1_beta_g1.checked_mul(pvk.gamma_g1).unwrap() == proof.a {
        Ok(())
    } else {
        Err(VerificationError::InvalidProof)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::Uint128;

    #[test]
    fn verify_proof_test() {
        let vk = VerifyingKey {
            alpha_g1: Uint128::new(7),
            beta_g1: Uint128::new(11),
            gamma_g1: Uint128::new(2),
        };
        let pvk = prepare_verifying_key(&vk);
        let proof = Proof {
            a: Uint128::new(154),
        };
        match verify_proof(&pvk, &proof) {
            Ok(()) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        };
    }
}
