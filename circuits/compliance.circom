pragma circom 2.0.0;

include "node_modules/circomlib/circuits/comparators.circom";
include "node_modules/circomlib/circuits/poseidon.circom";

template ComplianceCheck() {
    // Private inputs — never revealed on-chain
    signal input age;
    signal input panHash;
    signal input kycScore;

    // Public inputs
    signal input minAge;
    signal input minKycScore;

    // Output
    signal output isCompliant;

    // Age check: age >= minAge
    component ageCheck = GreaterEqThan(8);
    ageCheck.in[0] <== age;
    ageCheck.in[1] <== minAge;

    // KYC score check: kycScore >= minKycScore
    component kycCheck = GreaterEqThan(8);
    kycCheck.in[0] <== kycScore;
    kycCheck.in[1] <== minKycScore;

    // Both must pass
    isCompliant <== ageCheck.out * kycCheck.out;
}

component main {public [minAge, minKycScore]} = ComplianceCheck();
