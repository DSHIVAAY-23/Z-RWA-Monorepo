# Contributing to Universal Privacy Engine

Thank you for your interest in contributing to the Universal Privacy Engine! This document provides guidelines for contributing to this multi-chain ZK-compliance project.

## 🤝 Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in all interactions.

## 🚀 Getting Started

### Prerequisites

- Rust 1.75+
- Node.js 18+
- Solana CLI 1.18+
- Foundry (for EVM development)
- SP1 toolchain

### Development Setup

```bash
# Clone the repository
git clone https://github.com/DSHIVAAY-23/Z-RWA-Monorepo.git
cd Z-RWA-Monorepo

# Checkout develop branch
git checkout develop

# Install dependencies
cd Z-RWA && anchor build
cd ../contracts/evm && forge build
cd ../ZK-RAG && cargo build
```

## 📋 Development Workflow

### Branch Strategy

- **`main`**: Production-ready code only
- **`develop`**: Active development branch (default)
- **Feature branches**: `feature/your-feature-name`
- **Bug fixes**: `fix/bug-description`

### Making Changes

1. **Create a feature branch** from `develop`:
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our coding standards

3. **Test thoroughly**:
   ```bash
   # Solana tests
   cd Z-RWA && anchor test
   
   # EVM tests
   cd contracts/evm && forge test
   
   # ZK circuit tests
   cd ZK-RAG && cargo test
   ```

4. **Commit with conventional commits**:
   ```bash
   git commit -m "feat: add new verification logic"
   git commit -m "fix: resolve proof generation bug"
   git commit -m "docs: update architecture diagram"
   ```

5. **Push and create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

## 📝 Commit Message Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks
- `perf:` Performance improvements

## 🧪 Testing Requirements

All contributions must include appropriate tests:

- **Unit tests**: For individual functions/modules
- **Integration tests**: For cross-module interactions
- **End-to-end tests**: For complete user flows

### Running Tests

```bash
# Solana
cd Z-RWA && anchor test

# EVM
cd contracts/evm && forge test -vvv

# ZK Circuits
cd ZK-RAG && cargo test --release
```

## 📚 Documentation

- Update relevant README files
- Add inline code comments for complex logic
- Update architecture diagrams if needed
- Document breaking changes in PR description

## 🔍 Code Review Process

1. All PRs require at least one approval
2. CI/CD checks must pass
3. Code coverage should not decrease
4. Follow existing code style and patterns

## 🛡️ Security

- Never commit private keys or secrets
- Report security vulnerabilities privately
- Follow secure coding practices
- Use environment variables for sensitive data

## 🎯 Areas for Contribution

### High Priority
- [ ] Additional chain integrations (Polygon, Arbitrum, etc.)
- [ ] Performance optimizations for proof generation
- [ ] Enhanced documentation and tutorials
- [ ] Additional use case implementations

### Medium Priority
- [ ] UI/UX improvements for demo applications
- [ ] Additional test coverage
- [ ] Benchmarking and profiling tools
- [ ] Developer tooling and scripts

### Good First Issues
- [ ] Documentation improvements
- [ ] Code cleanup and refactoring
- [ ] Adding code comments
- [ ] Fixing typos and formatting

## 📞 Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **Discussions**: For questions and general discussion
- **Discord**: [Join our community](#) (if applicable)

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to the Universal Privacy Engine! 🚀
