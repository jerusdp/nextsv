<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Bug Fixes

- Update rust crate clap to 3.2.11
- Update rust crate clap to 3.2.12
- Update rust crate git-conventional to 0.12.0
- Update rust crate clap to 3.2.13
- Update rust crate clap to 3.2.14
- 🐛 Spelling error in error text

### Features

- Create enum of bump levels
- ✨ add patch level of none when no conventional commits are found
- Instead of Level::None return and error NoLevelChange
- Add error for no level change

### Miscellaneous Tasks

- 🎨 Update changelogs
- Update github/codeql-action digest to d8c9c72
- Update ossf/scorecard-action digest to 88c5e32
- Update dependency cimg/rust to v1.62
- Update ossf/scorecard-action digest to d434c40
- Update ossf/scorecard-action digest to ccd0038
- Update github/codeql-action digest to ba95eeb
- Update github/codeql-action digest to b8bd06e
- Update ossf/scorecard-action digest to 0c37758
- Update github/codeql-action digest to 8171514
- Update ossf/scorecard-action digest to 3155d13
- ✨ Add workflow to check  for and release

## [0.3.1] - 2022-07-11

## Fix

- Errors found after cargo release run

## [0.3.0] - 2022-07-11

### Bug Fixes

- Fix errors in drafted Level code
- 🐛 replace tag identification using 'v' with prefix variable

### Documentation

- ✨ Commit based changelog using git cliff application

### Features

- Create enum of bump levels
- ✨ Features for calculation of level or version number
- ✨ Error for case where no conventional commits have been found
- ✨ function to calculate next level based on recent commits
- ✨ Implement display for semantic::Level

### Refactor

- 🎨 separate version calculation into a dedicated function version
- 🎨 move level printing code to separate function for level
- 🎨 Two subcommands for version and level output
- 🎨 Tidy off testing aids

## [0.2.0] - 2022-06-27

### Bug Fixes

- 🐛 Set lower components to 0 on increment

### Features

- ✨ cli based on clap with verbose setting
- ✨ force option on cli to force a specific level of update

### Miscellaneous Tasks

- 🔥 Remove dbg! macros
- 📝 Update release version in Cargo.toml to 0.1.1
- Update version in Cargo.toml to 0.2.0

## [0.1.1] - 2022-06-26

### Bug Fixes

- 🐛 Fix failure to detect separate tag and correct calculation of the next version
- 🐛 Test both other and fix_commits values for patch increment (major=0)

## [0.1.0] - 2022-06-25

### Documentation

- 📝 Update documentation for semantic module to refer to semver standard

### Features

- ✨ Add Semantic version struct and methods to display and increment components
- ✨ Add error module for nextsv library
- ✨ Add dependencies for error ,management
- ✨ add parse method to parse a git tag into a semantic version
- Count conventional commits to last tag
- ✨ abstraction for conventional commit
- ✨ describe a version tag
- Add module references to library and testing code in main, settings updates
- ✨ create function to calculate next semantic version

### Miscellaneous Tasks

- ✨ Initial announcement to reserve crate name
- Add CI to test and check the code
- Update security and changelog notices
- Add cargo release pre-release replacements

### Refactor

- 🎨 Refactor into library and binary
- Tuning updates

<!-- generated by git-cliff -->
<!-- next-url -->
[Unreleased]: https://github.com/jerusdp/nextsv/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/jerusdp/nextsv/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jerusdp/nextsv/compare/v0.2.0...v0.3.0"
[0.2.0]: https://github.com/jerudp/nextsv/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/jerudp/nextsv/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jerudp/nextsv/compare/...v0.1.0
