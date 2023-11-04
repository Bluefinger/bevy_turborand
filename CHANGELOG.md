## [unreleased]

### Chore

- Prepare release

## [0.7.0] - 2023-11-04

### Chore

- Update to latest bevy version

## [0.6.0] - 2023-07-10

### Chore

- Update to latest bevy version
- Prepare release
- Prepare release

### Feat

- Updated to stable indexing release of Turborand

## [0.4.3] - 2022-12-20

### Chore

- Prepare release
- Fix support versions table

### Feat

- Reflection support

### Test

- Fix windows newline issues

## [0.4.2] - 2022-11-22

### Chore

- Prepare for release
- Add prelude module
- Prepare for release

### Doc

- Use prelude in examples

## [0.4.1] - 2022-11-18

### Feat

- *_mut sampling methods

## [0.4.0] - 2022-11-13

### Chore

- Update example
- Update to latest bevy version
- Prepare for release

### Feat

- Add ForkableCore trait and update to latest bevy

### Fix

- Limit tags to 5
- Properly enable serde dep

## [0.3.0] - 2022-08-19

### Chore

- Clippy fixes
- Enable Miri to run plus tests on secure Rng variants
- Migrate to 0.6 turborand, prep docs and README
- Complete intro sentence about ChaCha.
- Prepare for release

### Docs

- Provide updated docs for reworked traits and structs

### Feat

- Traitification rework, Secure RNG, granular features

### Fix

- Additional Miri flags to silence known behaviours from bevy
- Fix import warning when building with no default features

## [0.2.0] - 2022-07-30

### Chore

- Update README example
- Upgrade turborand, fill_bytes method
- Add CHANGELOG for releases
- Prepare for release

### Doc

- Better examples on get_mut() usage and use cases

### Feat

- Enable serialize and rand features

### Refactor

- Delegated methods, must_use, const new, Atomic GlobalRng
- No more atomics, delegated global methods

### Test

- Enable CI testing for WASM
- Enable extra determinism test on WASM

## [0.1.0] - 2022-06-23

### Chore

- Enable CI
- Remove default features of bevy

### Feat

- Initial commit, let there be Rng

### Fix

- Fix button links
- Install linux dependencies

