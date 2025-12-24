// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./ISP1Verifier.sol";

/// @title EVM Payroll Verifier
/// @notice Privacy-preserving payroll verification using ZK proofs
/// @dev Verifies salary payments without revealing actual amounts on-chain
contract EVMPayrollVerifier {
    /// @notice SP1 Verifier contract for proof verification
    ISP1Verifier public immutable verifier;

    /// @notice Verification key for the payroll ZK program
    bytes32 public immutable payrollProgramVKey;

    /// @notice Track verified pay periods per employee
    /// @dev Mapping: employee address => pay period => verified status
    mapping(address => mapping(uint256 => bool)) public verifiedPeriods;

    /// @notice Emitted when a salary is successfully verified
    /// @param employee The employee address
    /// @param payPeriod The pay period identifier (e.g., YYYYMM format)
    /// @param payrollHash The hash of the payroll data from HR database
    event SalaryVerified(
        address indexed employee,
        uint256 indexed payPeriod,
        bytes32 payrollHash
    );

    /// @notice Emitted when verification fails
    /// @param employee The employee address
    /// @param payPeriod The pay period identifier
    /// @param reason The failure reason
    event VerificationFailed(
        address indexed employee,
        uint256 indexed payPeriod,
        string reason
    );

    /// @notice Initialize the payroll verifier
    /// @param _verifier Address of the SP1 verifier contract
    /// @param _vkey Verification key for the payroll ZK program
    constructor(address _verifier, bytes32 _vkey) {
        require(_verifier != address(0), "EVMPayrollVerifier: Invalid verifier address");
        require(_vkey != bytes32(0), "EVMPayrollVerifier: Invalid vkey");
        
        verifier = ISP1Verifier(_verifier);
        payrollProgramVKey = _vkey;
    }

    /// @notice Verify a salary payment using a ZK proof
    /// @dev The proof confirms the salary matches the HR database without revealing the amount
    /// @param employee The employee receiving the salary
    /// @param payPeriod The pay period (e.g., 202412 for December 2024)
    /// @param payrollHash Hash of the payroll data (employee + period + amount)
    /// @param proof The ZK proof bytes
    function verifySalary(
        address employee,
        uint256 payPeriod,
        bytes32 payrollHash,
        bytes calldata proof
    ) external {
        // Validate inputs
        require(employee != address(0), "EVMPayrollVerifier: Invalid employee");
        require(payPeriod > 0, "EVMPayrollVerifier: Invalid pay period");
        require(payrollHash != bytes32(0), "EVMPayrollVerifier: Invalid payroll hash");
        
        // Prevent double verification
        if (verifiedPeriods[employee][payPeriod]) {
            emit VerificationFailed(employee, payPeriod, "Already verified");
            revert("EVMPayrollVerifier: Period already verified");
        }

        // Encode public values: employee address, pay period, and payroll hash
        bytes memory publicValues = abi.encode(employee, payPeriod, payrollHash);

        // Verify the ZK proof
        try verifier.verifyProof(payrollProgramVKey, publicValues, proof) {
            // Mark period as verified
            verifiedPeriods[employee][payPeriod] = true;
            
            // Emit success event
            emit SalaryVerified(employee, payPeriod, payrollHash);
        } catch Error(string memory reason) {
            emit VerificationFailed(employee, payPeriod, reason);
            revert(string(abi.encodePacked("EVMPayrollVerifier: Proof verification failed - ", reason)));
        } catch {
            emit VerificationFailed(employee, payPeriod, "Unknown error");
            revert("EVMPayrollVerifier: Proof verification failed");
        }
    }

    /// @notice Check if a pay period has been verified for an employee
    /// @param employee The employee address
    /// @param payPeriod The pay period to check
    /// @return True if the period has been verified
    function isVerified(address employee, uint256 payPeriod) external view returns (bool) {
        return verifiedPeriods[employee][payPeriod];
    }

    /// @notice Get the verifier address
    /// @return The address of the SP1 verifier contract
    function getVerifier() external view returns (address) {
        return address(verifier);
    }
}
