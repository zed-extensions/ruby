# Ruby extension for Zed

[Documentation](https://zed.dev/docs/languages/ruby)

## Ruby command resolution

The extension uses configured `lsp.<server>.binary.path` values first. If
`use_bundler` is enabled, it checks Bundler and launches through
`bundle exec <server>`. Otherwise it falls back to `worktree.which` or the
extension-managed gemset.

```sh
cargo test
```

This is not a replacement for fixing Zed's command spawning behavior:
https://github.com/zed-industries/zed/issues/57170. Debugging expects `rdbg` to
be available from the project environment.

On macOS, extension-side Ruby, Bundler, and gem probes run through
`/bin/sh -c 'exec "$0" "$@"' ...`. Bundler-mode LSP startup likewise uses shell
resolution so version managers (rbenv, chruby, mise, asdf) are honored. Gemset
LSP startup runs `ruby <gemset-binstub>` from the worktree root so the project
Ruby is activated; DAP startup uses Zed worktree command resolution directly.
