# Changelog

All notable changes to this project will be documented in this file.

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

