# Ruby extension for Zed

[Documentation](https://zed.dev/docs/languages/ruby)

## Command-free LSP build

The default build does not run extension-side process commands for Ruby LSP
startup. It uses configured `lsp.<server>.binary.path` values first, then falls
back to `worktree.which`. If `use_bundler` is enabled, it launches through
`bundle exec <server>` without probing Bundler.

```sh
cargo test
```

To enable the command API path for project gem detection and extension-managed
language server/debug gem installation, build with:

```sh
cargo test --features command_api
```

This is not a replacement for fixing Zed's command spawning behavior:
https://github.com/zed-industries/zed/issues/57170. The command-free profile
expects `bundle` or the language server executable to be available from the
project environment. Debugging expects `rdbg` to be available from that same
environment.
