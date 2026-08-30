# Changelog

All notable changes to this project will be documented in this file.

## [0.16.17] - 2026-08-30

### Bug Fixes

- *(ruby)* Scope string overrides to literal content (#319)

### Testing

- Move `MockCommandExecutor` to a shared location (#316)

### Miscellaneous Tasks

- Update CHANGELOG.md

## [0.16.16] - 2026-08-29

### Features

- *(ruby-lsp)* Attach Ruby LSP to Slim buffers (#311)
- Remove command_api (#315)

### Miscellaneous Tasks

- *(deps)* Update rust crate tree-sitter to v0.26.13 (#312)

## [0.16.15] - 2026-08-13

### Features

- *(erb)* Support strict locals highlighting (#309)

### Styling

- Fix clippy warnings (#307)

### Miscellaneous Tasks

- *(deps)* Update rust crate tree-sitter to v0.26.11 (#302)
- *(deps)* Update rust crate anyhow to v1.0.104 (#304)
- *(deps)* Update rust crate serde_json to v1.0.151 (#306)
- *(deps)* Update rust crate regex to v1.13.1 (#303)
- *(deps)* Update rust crate serde to v1.0.229 (#305)
- *(deps)* Update rust crate tree-sitter to v0.26.12 (#310)

## [0.16.14] - 2026-07-06

### Features

- *(ruby)* Highlight instance vars, method defs, and keyword params distinctly (#298)

### Miscellaneous Tasks

- *(deps)* Update rust crate regex to v1.12.4 (#295)
- *(deps)* Update rust crate tree-sitter to v0.26.9 (#290)
- *(deps)* Update rust crate serde_json to v1.0.150 (#293)
- *(deps)* Update rust crate insta to v1.48.0 (#297)
- *(deps)* Update rust crate tree-sitter to v0.26.10 (#301)
- *(deps)* Update rust crate anyhow to v1.0.103 (#300)

## [0.16.13] - 2026-05-26

### Features

- Command-free build (#291)

## [0.16.12] - 2026-05-08

### Features

- *(ruby)* Add Fuzzy Ruby Server language server support (#283)

## [0.16.11] - 2026-04-02

### Features

- Add keyword subcategories to Ruby highlights (#275)

### Bug Fixes

- *(ruby)* Exclude strings from bracket colorization (#281)

### Miscellaneous Tasks

- *(deps)* Update zed-industries/zed digest to f92b498 (#277)
- *(deps)* Update rust crate insta to v1.47.2 (#280)
- *(deps)* Update rust crate tree-sitter to v0.26.8 (#279)

## [0.16.10] - 2026-03-22

### Bug Fixes

- *(herb)* Use `shell` env from a Worktree (#270)

### Miscellaneous Tasks

- *(deps)* Update zed-industries/zed digest to aabc967 (#266)
- *(deps)* Update rust crate tree-sitter to v0.26.7 (#273)

## [0.16.9] - 2026-03-13

### Features

- *(rbs)* Add highlight rules for inline annotations (#262)

### Miscellaneous Tasks

- Bump version to 0.16.9 (#268)

## [0.16.8] - 2026-03-02

### Features

- Introduce Kanayago LSP that provides real-time Ruby syntax checking (#224)

### Miscellaneous Tasks

- *(deps)* Update rust crate anyhow to v1.0.102 (#256)
- *(deps)* Update rust crate tree-sitter to v0.26.6 (#257)
- *(ruby)* Format queries (#260)

## [0.16.7] - 2026-02-15

### Miscellaneous Tasks

- *(deps)* Update rust crate regex to v1.12.3 (#245)
- *(deps)* Update rust crate insta to v1.46.3 (#244)
- *(deps)* Update rust crate tree-sitter to 0.26 (#242)

## [0.16.6] - 2026-02-10

### Features

- *(ruby)* Add additional textobjects queries (#249)

### Bug Fixes

- *(ruby)* Improve auto indent (#246)
- Add missing fields to `rdbg` DAP schema (#248)

### Refactor

- Switch to `anyhow` (#247)

### Testing

- *(gemset)* Make tests path-separator agnostic (#236)
- *(bundler)* Make tests path-separator agnostic (#237)
- Testing `tree-sitter` queries (#235)

### Miscellaneous Tasks

- *(deps)* Update rust crate serde_json to v1.0.149 (#219)
- Add `brackers`, `indents`, `outline` to tree-sitter LS config (#250)
- *(build)* Switch to crates-based tree-sitter versions (#252)
- Bump version to 0.16.6 (#239)

## [0.16.5] - 2026-01-24

### Bug Fixes

- *(ruby)* Fix runnable detection (#234)

### Miscellaneous Tasks

- Update changelog (#232)

## [0.16.4] - 2026-01-11

### Features

- *(gemset)* Install gems per Ruby version to avoid compat issues (#231)

### Bug Fixes

- *(ruby)* Use `string_content` capture for Ruby test name extraction (#229)

## [0.16.3] - 2026-01-09

### Bug Fixes

- *(ruby)* Convert snippets from global to language-scoped (#221)
- *(ruby)* Add more RSpec test methods (#227)

### Miscellaneous Tasks

- Remove `extensions::run_tests` workflow (#218)

## [0.16.2] - 2025-12-08

### Miscellaneous Tasks

- Add support for `ts_query_ls` language server (#211)
- Remove `commitlint` (#215)
- Remove obsolete `pnpm` lockfile (#217)

## [0.16.1] - 2025-11-27

### Bug Fixes

- *(solargraph)* Stop converting constant names to uppercase
- *(ruby-lsp)* Stop converting constant names to uppercase

### Miscellaneous Tasks

- *(deps)* Update pnpm to v10.23.0 (#207)
- *(deps)* Update RBS grammar to v0.2.2 (#209)

## [0.16.0] - 2025-11-11

### Features

- Add support for `zed-comment` (#203)
- Add `zed-comment` support for all `ERB` languages (#205)
- Add RBS Inline syntax highlighting support (#206)

### Miscellaneous Tasks

- *(deps)* Update pnpm to v10.20.0 (#201)

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

