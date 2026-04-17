# Changelog

# 0.2.0 (2026-04-17)

### Features

- **channels:** add Telegram and SMS channel support to registry ([0387945](https://github.com/heraldhq-protocol/privacy-registry/commit/0387945bbe8e575e694f65a282e01f3a899359db))
  - Extended `IdentityAccount` with channel settings, encrypted Telegram chat IDs, and encrypted phone numbers
  - Added instructions: `register_telegram`, `register_sms`, `update_channels`, `remove_channel`
  - GDPR erasure compliance via `remove_channel` instruction
  - Permissionless lazy migration instruction `migrate_channels`

### Bug Fixes

- **test:** fix identity registration and protocol authorization tests ([6a6850f](https://github.com/heraldhq-protocol/privacy-registry/commit/6a6850f56e82b9d8241093155cf630dd25328af5))
  - Updated protocol authorization tests to expect 'AccountNotInitialized' instead of 'Unauthorized'
  - Added idempotency to test suite with 'before' hook to delete stale identity accounts
  - Fixed 'account already in use' errors during identity registration

# 0.1.0 (2026-03-22)

### Bug Fixes

- use glob re-exports in instructions/mod.rs to fix #[program] macro resolution ([2efb96f](https://github.com/heraldhq-protocol/privacy-registry/commit/2efb96f78697016e9f38444a4ad6e733f1b53800))

### Features

- **billing:** implement Phase 2 on-chain subscription payments ([3294107](https://github.com/heraldhq-protocol/privacy-registry/commit/3294107fa1f3d039a41c0132ba5f618d0ca5d97d))
- **billing:** implement Phase 2 on-chain subscription payments ([b6f4d1d](https://github.com/heraldhq-protocol/privacy-registry/commit/b6f4d1d5eff88384b1244fdc40a8bc2cf423d0e1))
- **program:** full Herald Privacy Registry implementation ([26d47ec](https://github.com/heraldhq-protocol/privacy-registry/commit/26d47ec924780c40e9603fe16bf16745236bca95))

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

## [1.0.0] - 2026-03-18

### Features

- Implement subscription billing, security hardening, and documentation
- Expand `ProtocolRegistryAccount` with subscription and billing fields
- Enforce on-chain tier limits and subscription expiry
- Harden `write_receipt` with multi-stage security checks
- Add 10 core instructions covering full protocol lifecycle

### Security

- Resolved 12 findings (documented in `docs/SECURITY.md`)
