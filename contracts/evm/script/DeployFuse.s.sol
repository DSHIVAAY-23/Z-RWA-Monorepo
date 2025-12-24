// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/MockSP1Verifier.sol";
import "../src/FusePayrollVerifier.sol";

/// @title Deploy Fuse Payroll Verifier
/// @notice Deployment script for Fuse Spark Testnet (Chain ID: 123)
/// @dev Run with: forge script script/DeployFuse.s.sol --rpc-url https://rpc.fusespark.io --private-key $PRIVATE_KEY --broadcast
contract DeployFuse is Script {
    /// @notice Predefined verification key for the payroll ZK program
    /// @dev In production, this would be the actual vkey from your SP1 program build
    bytes32 constant PAYROLL_PROGRAM_VKEY = keccak256("FUSE_PAYROLL_ZK_PROGRAM_V1");

    function run() external {
        // Get deployer private key from environment
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);

        console.log("=== Deploying to Fuse Spark Testnet ===");
        console.log("Chain ID: 123");
        console.log("RPC URL: https://rpc.fusespark.io");
        console.log("Deployer:", vm.addr(deployerPrivateKey));
        console.log("");

        // Step 1: Deploy MockSP1Verifier
        console.log("Step 1: Deploying MockSP1Verifier...");
        MockSP1Verifier mockVerifier = new MockSP1Verifier();
        console.log("MockSP1Verifier deployed at:", address(mockVerifier));
        console.log("Verifier Hash:", vm.toString(mockVerifier.VERIFIER_HASH()));
        console.log("");

        // Step 2: Deploy FusePayrollVerifier
        console.log("Step 2: Deploying FusePayrollVerifier...");
        console.log("Using Payroll Program VKey:", vm.toString(PAYROLL_PROGRAM_VKEY));
        FusePayrollVerifier payrollVerifier = new FusePayrollVerifier(
            address(mockVerifier),
            PAYROLL_PROGRAM_VKEY
        );
        console.log("FusePayrollVerifier deployed at:", address(payrollVerifier));
        console.log("");

        // Step 3: Verify deployment
        console.log("Step 3: Verifying deployment...");
        require(payrollVerifier.getVerifier() == address(mockVerifier), "Verifier mismatch");
        require(payrollVerifier.payrollProgramVKey() == PAYROLL_PROGRAM_VKEY, "VKey mismatch");
        console.log("Deployment verification: PASSED");
        console.log("");

        // Stop broadcasting
        vm.stopBroadcast();

        // Print summary for grant documentation
        console.log("=== DEPLOYMENT SUMMARY ===");
        console.log("Network: Fuse Spark Testnet");
        console.log("Chain ID: 123");
        console.log("");
        console.log("Deployed Contracts:");
        console.log("  MockSP1Verifier:      ", address(mockVerifier));
        console.log("  FusePayrollVerifier:  ", address(payrollVerifier));
        console.log("");
        console.log("Configuration:");
        console.log("  Payroll Program VKey: ", vm.toString(PAYROLL_PROGRAM_VKEY));
        console.log("");
        console.log("Next Steps:");
        console.log("1. Verify contracts on Fuse block explorer");
        console.log("2. Test verifySalary() function with sample data");
        console.log("3. Document deployment addresses for grant submission");
        console.log("");
        console.log("=== DEPLOYMENT COMPLETE ===");
    }
}
