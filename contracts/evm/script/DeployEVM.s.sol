// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/MockSP1Verifier.sol";
import "../src/EVMPayrollVerifier.sol";

/// @title Deploy EVM Payroll Verifier
/// @notice Deployment script for EVM-compatible chains (Sepolia, Fuse, Polygon, Arbitrum, etc.)
/// @dev Run with: forge script script/DeployEVM.s.sol --rpc-url <RPC_URL> --private-key $PRIVATE_KEY --broadcast
contract DeployEVM is Script {
    /// @notice Predefined verification key for the payroll ZK program
    /// @dev In production, this would be the actual vkey from your SP1 program build
    bytes32 constant PAYROLL_PROGRAM_VKEY = keccak256("EVM_PAYROLL_ZK_PROGRAM_V1");

    function run() external {
        // Get deployer private key from environment
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);

        console.log("=== Deploying to EVM Chain ===");
        console.log("Deployer:", vm.addr(deployerPrivateKey));
        console.log("");

        // Step 1: Deploy MockSP1Verifier
        console.log("Step 1: Deploying MockSP1Verifier...");
        MockSP1Verifier mockVerifier = new MockSP1Verifier();
        console.log("MockSP1Verifier deployed at:", address(mockVerifier));
        console.log("Verifier Hash:", vm.toString(mockVerifier.VERIFIER_HASH()));
        console.log("");

        // Step 2: Deploy EVMPayrollVerifier
        console.log("Step 2: Deploying EVMPayrollVerifier...");
        console.log("Using Payroll Program VKey:", vm.toString(PAYROLL_PROGRAM_VKEY));
        EVMPayrollVerifier payrollVerifier = new EVMPayrollVerifier(
            address(mockVerifier),
            PAYROLL_PROGRAM_VKEY
        );
        console.log("EVMPayrollVerifier deployed at:", address(payrollVerifier));
        console.log("");

        // Step 3: Verify deployment
        console.log("Step 3: Verifying deployment...");
        require(payrollVerifier.getVerifier() == address(mockVerifier), "Verifier mismatch");
        require(payrollVerifier.payrollProgramVKey() == PAYROLL_PROGRAM_VKEY, "VKey mismatch");
        console.log("Deployment verification: PASSED");
        console.log("");

        // Stop broadcasting
        vm.stopBroadcast();

        // Print summary
        console.log("=== DEPLOYMENT SUMMARY ===");
        console.log("");
        console.log("Deployed Contracts:");
        console.log("  MockSP1Verifier:      ", address(mockVerifier));
        console.log("  EVMPayrollVerifier:   ", address(payrollVerifier));
        console.log("");
        console.log("Configuration:");
        console.log("  Payroll Program VKey: ", vm.toString(PAYROLL_PROGRAM_VKEY));
        console.log("");
        console.log("Next Steps:");
        console.log("1. Verify contracts on block explorer");
        console.log("2. Test verifySalary() function with sample data");
        console.log("3. Document deployment addresses");
        console.log("");
        console.log("=== DEPLOYMENT COMPLETE ===");
    }
}
