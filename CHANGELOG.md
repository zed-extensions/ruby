# Changelog

All notable changes to this project will be documented in this file.

## [0.15.0] - 2025-11-01

### Features

- *(rdbg)* Prefer exe from gemset

### Performance

- *(gemset)* Cache env() result with OnceLock

### Styling

- Run rustfmt

### Testing

- *(bundler)* Fix args comparison in mock test

### Miscellaneous Tasks

- *(bundle)* Use generic type parameter
- Replace wildcard imports with explicit types

## [0.14.1] - 2025-10-28

### Bug Fixes

- *(rdbg)* Remove `rdbg` subdirectory from debugger path (#197)
- Improve completion highlights (#200)

### Miscellaneous Tasks

- Update changelog
- *(deps)* Update pnpm to v10.18.2 (#190)
- *(deps)* Update rust crate serde to v1.0.228
- *(deps)* Update rust crate regex to v1.12.2 (#193)
- *(deps)* Update pnpm to v10.19.0 (#192)
- *(deps)* Update rust crate serde_json to v1.0.145 (#182)
- *(ruby)* Prefix tree-sitter captures with underscore (#198)

## [0.14.0] - 2025-10-10

### Features

- JS/ERB language support (#188)

### Bug Fixes

- Rename all composite langs for ERB templates

### Miscellaneous Tasks

- Update CHANGELOG.md
- *(deps)* Update embedded-template to v0.25.0

## [0.13.5] - 2025-10-07

### Bug Fixes

- Update language names to better represent embedded templates (#160)
- *(gems)* Improve environment handling and PATH resolution (#189)

### Miscellaneous Tasks

- Fix CHANGELOG.md file

## [0.13.4] - 2025-09-26

### Miscellaneous Tasks

- Bump extension API for enhanced Windows support (#179)

## [0.13.3] - 2025-09-24

### Bug Fixes

- *(deps)* Update rust crate regex to v1.11.2 (#168)

### Performance

- *(gemset)* Use LazyLock for regex compilation

### Miscellaneous Tasks

- Update CHANGELOG.md
- *(deps)* Update pnpm to v10.15.1 (#171)
- *(deps)* Update pnpm to v10.17.1 (#183)

## [0.13.2] - 2025-09-11

### Features

- *(gemset)* Store and re-use Worktree shell env (#176)

### Bug Fixes

- *(gemset)* Add `cwd` to Gemset and set `RBENV_DIR` env var (#173)

## [0.13.1] - 2025-08-24

### Bug Fixes

- *(deps)* Update rust crate serde_json to v1.0.141 (#151)
- *(deps)* Update rust crate serde_json to v1.0.143 (#155)
- *(ruby)* Resolve precedence issue with import keyword detection (#163)
- *(gemset)* Pass `GEM_PATH` env variable to commands (#164)
- *(rdbg)* Ensure we always have current dir (#165)

### Styling

- Allow conventional commits that start with a lowercase letter

### Testing

- Add tests for `root_path` and `shell_env` (#159)

### Miscellaneous Tasks

- Enable semantic commits for renovate
- *(deps)* Update pnpm to v10.14.0 (#156)
- *(deps)* Update pnpm to v10.15.0 (#161)
- *(release)* Prepare release v0.13.1 (#166)

## [0.13.0] - 2025-07-26

### Bug Fixes

- *(ruby-lsp)* Disable `onTypeFormatting` feature (#142)
- *(gemset)* Preserve env vars (#150)
- *(ruby)* Expose `name` and `RUBY_TEST_NAME` captures on `test_` (#153)

### Refactor

- Replace String with PathBuf for path handling (#146)

### Documentation

- Add CHANGELOG.md file

### Performance

- Remove unnecessary string allocations in command output handling

### Miscellaneous Tasks

- Add conventional commits linting (#147)
- Add `git-cliff` for conventional changelog (#148)

## [0.12.0] - 2025-07-05

### Features

- *(ruby)* Add `gem uninstall` functionality (#135)
- *(ruby)* Add basic support for displaying dbg variables (#125)

### Bug Fixes

- *(rdbg)* Improve debugger argument handling (#136)

## [0.11.0] - 2025-07-02

### Features

- *(rdbg)* Support RUBY_DEBUG env vars for host and port (#123)
- *(rdbg)* Support attach requests for `rdbg` debugger (#124)

## [0.10.1] - 2025-06-27

### Bug Fixes

- *(rdbg)* Remove `RUBY_DEBUG_OPEN` environment variable (#119)

## [0.10.0] - 2025-06-26

### Features

- *(herb)* Add Herb LSP (#110)

### Bug Fixes

- *(rdbg)* Use gemset-aware detection (#114)

### Miscellaneous Tasks

- Modernize string formatting with string interpolation (#118)

## [0.9.0] - 2025-06-08

### Features

- Add sorbet Ruby LSP server option (#104)

### Ruby

- Fix runnable queries
- Make `LanguageServer::get_executable_args` an instance method (#105)

## [0.8.0] - 2025-06-02

### Ruby

- Update tasks format (#100)

## [0.0.8] - 2024-07-18

### Tasks

- Provide task variables from matching runnable ranges in task modal (zed-industries/zed#12237)

## [0.0.1] - 2024-05-10

