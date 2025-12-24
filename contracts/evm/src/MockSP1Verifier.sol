// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./ISP1Verifier.sol";

/// @title Mock SP1 Verifier
/// @notice A mock implementation of ISP1Verifier for testing on networks without official SP1 deployment
/// @dev This contract always succeeds verification - FOR DEMO PURPOSES ONLY
contract MockSP1Verifier is ISP1VerifierWithHash {
    /// @notice Mock verifier hash for compatibility
    bytes32 public constant VERIFIER_HASH = keccak256("MOCK_SP1_VERIFIER_V1");

    /// @notice Mock verification function that always succeeds
    /// @dev In production, this would perform actual ZK proof verification
    /// @param programVKey The verification key for the RISC-V program
    /// @param publicValues The public values encoded as bytes
    /// @param proofBytes The proof of the program execution
    function verifyProof(
        bytes32 programVKey,
        bytes calldata publicValues,
        bytes calldata proofBytes
    ) external pure override {
        // Mock implementation - always succeeds
        // In production, this would revert if proof is invalid
        require(programVKey != bytes32(0), "MockSP1Verifier: Invalid vkey");
        require(publicValues.length > 0, "MockSP1Verifier: Empty public values");
        require(proofBytes.length > 0, "MockSP1Verifier: Empty proof");
        
        // Proof verification would happen here in production
        // For demo purposes, we just validate inputs are non-empty
    }
}
