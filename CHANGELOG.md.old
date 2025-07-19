## 0.12.0 (2025-07-05)

* Add snippets (#53) ([d42b759](https://github.com/zed-extensions/ruby/commit/d42b759)), closes [#53](https://github.com/zed-extensions/ruby/issues/53)
* Add support for debug locators (#130) ([8c87b24](https://github.com/zed-extensions/ruby/commit/8c87b24)), closes [#130](https://github.com/zed-extensions/ruby/issues/130)
* v0.12.0 ([f9a5294](https://github.com/zed-extensions/ruby/commit/f9a5294))
* fix(rdbg): Improve debugger argument handling (#136) ([f12cabe](https://github.com/zed-extensions/ruby/commit/f12cabe)), closes [#136](https://github.com/zed-extensions/ruby/issues/136)
* feat(ruby): add `gem uninstall` functionality (#135) ([28d5155](https://github.com/zed-extensions/ruby/commit/28d5155)), closes [#135](https://github.com/zed-extensions/ruby/issues/135)
* feat(ruby): Add basic support for displaying dbg variables (#125) ([0cd47e0](https://github.com/zed-extensions/ruby/commit/0cd47e0)), closes [#125](https://github.com/zed-extensions/ruby/issues/125)



## 0.11.0 (2025-07-02)

* Ensure command args aren't parsed by rdbg (#128) ([b7453ac](https://github.com/zed-extensions/ruby/commit/b7453ac)), closes [#128](https://github.com/zed-extensions/ruby/issues/128)
* Ensure cwd actually has a default (#127) ([ca5a38b](https://github.com/zed-extensions/ruby/commit/ca5a38b)), closes [#127](https://github.com/zed-extensions/ruby/issues/127) [#126](https://github.com/zed-extensions/ruby/issues/126)
* Support Rake outlines (#131) ([f2de9d5](https://github.com/zed-extensions/ruby/commit/f2de9d5)), closes [#131](https://github.com/zed-extensions/ruby/issues/131)
* Update Rust crate serde to v1.0.219 (#116) ([3a39e84](https://github.com/zed-extensions/ruby/commit/3a39e84)), closes [#116](https://github.com/zed-extensions/ruby/issues/116)
* Update Rust crate serde_json to v1.0.140 (#117) ([97b120f](https://github.com/zed-extensions/ruby/commit/97b120f)), closes [#117](https://github.com/zed-extensions/ruby/issues/117)
* v0.11.0 ([3ef5f61](https://github.com/zed-extensions/ruby/commit/3ef5f61))
* feat(rdbg): support attach requests for `rdbg` debugger (#124) ([075038f](https://github.com/zed-extensions/ruby/commit/075038f)), closes [#124](https://github.com/zed-extensions/ruby/issues/124)
* feat(rdbg): support RUBY_DEBUG env vars for host and port (#123) ([d937b31](https://github.com/zed-extensions/ruby/commit/d937b31)), closes [#123](https://github.com/zed-extensions/ruby/issues/123)



## <small>0.10.1 (2025-06-27)</small>

* Bump to v0.10.1 (#120) ([4aa75a9](https://github.com/zed-extensions/ruby/commit/4aa75a9)), closes [#120](https://github.com/zed-extensions/ruby/issues/120)
* Changed all impls to use string slices for better perf (#111) ([aefcb77](https://github.com/zed-extensions/ruby/commit/aefcb77)), closes [#111](https://github.com/zed-extensions/ruby/issues/111)
* ERB injection for other source languages (#113) ([14a913e](https://github.com/zed-extensions/ruby/commit/14a913e)), closes [#113](https://github.com/zed-extensions/ruby/issues/113)
* Fix version in Cargo.toml for v0.10.1 (#122) ([a30847c](https://github.com/zed-extensions/ruby/commit/a30847c)), closes [#122](https://github.com/zed-extensions/ruby/issues/122) [#120](https://github.com/zed-extensions/ruby/issues/120)
* fix(rdbg): remove `RUBY_DEBUG_OPEN` environment variable (#119) ([fa8ffcc](https://github.com/zed-extensions/ruby/commit/fa8ffcc)), closes [#119](https://github.com/zed-extensions/ruby/issues/119)

### BREAKING CHANGE

- ERB injection now applies to additional source languages beyond Ruby. If you rely on previous behavior that only supported Ruby, you may need to update your usage or configuration to accommodate changes in how ERB templates are injected and processed for other languages, from `ERB` to `HTML/ERB`:

Before:

```jsonc
{
 "ERB": {
    "formatter": "prettier",
    "prettier": {
        "allowed": true
    }
}
```

After:

```jsonc
{
 "HTML/ERB": {
    "formatter": "prettier",
    "prettier": {
        "allowed": true
    }
}
```

## 0.10.0 (2025-06-26)

* Debugger prototype for Ruby (#96) ([4ad6185](https://github.com/zed-extensions/ruby/commit/4ad6185)), closes [#96](https://github.com/zed-extensions/ruby/issues/96)
* Update `herb-language-server` executable path (#115) ([e3daf6f](https://github.com/zed-extensions/ruby/commit/e3daf6f)), closes [#115](https://github.com/zed-extensions/ruby/issues/115) [#110](https://github.com/zed-extensions/ruby/issues/110)
* Update Rust crate zed_extension_api to 0.6.0 (#108) ([95ac4cd](https://github.com/zed-extensions/ruby/commit/95ac4cd)), closes [#108](https://github.com/zed-extensions/ruby/issues/108)
* v0.10.0 ([43ff8ea](https://github.com/zed-extensions/ruby/commit/43ff8ea))
* fix(rdbg): use gemset-aware detection (#114) ([2c053e5](https://github.com/zed-extensions/ruby/commit/2c053e5)), closes [#114](https://github.com/zed-extensions/ruby/issues/114)
* chore: modernize string formatting with string interpolation (#118) ([58796c3](https://github.com/zed-extensions/ruby/commit/58796c3)), closes [#118](https://github.com/zed-extensions/ruby/issues/118)
* feat(herb): add Herb LSP (#110) ([25f768d](https://github.com/zed-extensions/ruby/commit/25f768d)), closes [#110](https://github.com/zed-extensions/ruby/issues/110)



## 0.9.0 (2025-06-08)

* Add support for the `steep` LSP (#102) ([1f802be](https://github.com/zed-extensions/ruby/commit/1f802be)), closes [#102](https://github.com/zed-extensions/ruby/issues/102)
* fixup! ruby: Fix runnable queries ([6f5532e](https://github.com/zed-extensions/ruby/commit/6f5532e))
* Refactor Zed API imports and Result types ([16f8a30](https://github.com/zed-extensions/ruby/commit/16f8a30))
* v0.9.0 (#106) ([12fc524](https://github.com/zed-extensions/ruby/commit/12fc524)), closes [#106](https://github.com/zed-extensions/ruby/issues/106)
* feat: add sorbet Ruby LSP server option (#104) ([8587f26](https://github.com/zed-extensions/ruby/commit/8587f26)), closes [#104](https://github.com/zed-extensions/ruby/issues/104) [#9](https://github.com/zed-extensions/ruby/issues/9) [notchairmk/zed-sorbet#6](https://github.com/notchairmk/zed-sorbet/issues/6)
* ruby: Fix runnable queries ([9d649a4](https://github.com/zed-extensions/ruby/commit/9d649a4))
* ruby: Make `LanguageServer::get_executable_args` an instance method (#105) ([8cf135d](https://github.com/zed-extensions/ruby/commit/8cf135d)), closes [#105](https://github.com/zed-extensions/ruby/issues/105)



## 0.8.0 (2025-06-02)

* Add `@RUBY_TEST_NAME` capture for test nodes (#97) ([7c84df6](https://github.com/zed-extensions/ruby/commit/7c84df6)), closes [#97](https://github.com/zed-extensions/ruby/issues/97)
* Adds .irbrc to language config (#101) ([fcf72bc](https://github.com/zed-extensions/ruby/commit/fcf72bc)), closes [#101](https://github.com/zed-extensions/ruby/issues/101)
* Expose `runnables` captures (#94) ([0d06487](https://github.com/zed-extensions/ruby/commit/0d06487)), closes [#94](https://github.com/zed-extensions/ruby/issues/94)
* Extract and unify command execution logic (#98) ([0e208b0](https://github.com/zed-extensions/ruby/commit/0e208b0)), closes [#98](https://github.com/zed-extensions/ruby/issues/98)
* Update EditorConfig config (#74) ([6bbadba](https://github.com/zed-extensions/ruby/commit/6bbadba)), closes [#74](https://github.com/zed-extensions/ruby/issues/74)
* Update GitHub Actions to use wasm32-wasip2 target (#99) ([a5c1439](https://github.com/zed-extensions/ruby/commit/a5c1439)), closes [#99](https://github.com/zed-extensions/ruby/issues/99)
* Update Rust crate zed_extension_api to 0.5.0 (#86) ([2bd9e29](https://github.com/zed-extensions/ruby/commit/2bd9e29)), closes [#86](https://github.com/zed-extensions/ruby/issues/86)
* v0.8.0 (#103) ([f239973](https://github.com/zed-extensions/ruby/commit/f239973)), closes [#103](https://github.com/zed-extensions/ruby/issues/103) [#94](https://github.com/zed-extensions/ruby/issues/94) [#86](https://github.com/zed-extensions/ruby/issues/86) [#100](https://github.com/zed-extensions/ruby/issues/100) [#101](https://github.com/zed-extensions/ruby/issues/101)
* ruby: Update tasks format (#100) ([70cd7b6](https://github.com/zed-extensions/ruby/commit/70cd7b6)), closes [#100](https://github.com/zed-extensions/ruby/issues/100)



## <small>0.7.3 (2025-05-08)</small>

* Add environment variables support to Bundler (#91) ([6433c7c](https://github.com/zed-extensions/ruby/commit/6433c7c)), closes [#91](https://github.com/zed-extensions/ruby/issues/91)
* Add GitHub workflow for running linters and tests (#88) ([b319276](https://github.com/zed-extensions/ruby/commit/b319276)), closes [#88](https://github.com/zed-extensions/ruby/issues/88)
* Check `PATH` for starting a LS out of the project gemset (#90) ([b22ef1a](https://github.com/zed-extensions/ruby/commit/b22ef1a)), closes [#90](https://github.com/zed-extensions/ruby/issues/90)
* Refactor command execution in bundler and gemset modules (#87) ([f7da1e6](https://github.com/zed-extensions/ruby/commit/f7da1e6)), closes [#87](https://github.com/zed-extensions/ruby/issues/87)
* v0.7.2 ([61477a9](https://github.com/zed-extensions/ruby/commit/61477a9))
* v0.7.3 ([ec7c1ef](https://github.com/zed-extensions/ruby/commit/ec7c1ef))



## <small>0.7.1 (2025-05-07)</small>

* Fix gem environment and language server configuration (#85) ([6d0b198](https://github.com/zed-extensions/ruby/commit/6d0b198)), closes [#85](https://github.com/zed-extensions/ruby/issues/85)
* Improve language server binary lookup in PATH (#84) ([a844047](https://github.com/zed-extensions/ruby/commit/a844047)), closes [#84](https://github.com/zed-extensions/ruby/issues/84)
* v0.7.1 ([c4bd1df](https://github.com/zed-extensions/ruby/commit/c4bd1df))



## 0.7.0 (2025-05-06)

* Downgrade `zed_extension_api` to v0.4.0 ([842b0cd](https://github.com/zed-extensions/ruby/commit/842b0cd))
* Use new process API for starting language servers (#48) ([d6bf9e4](https://github.com/zed-extensions/ruby/commit/d6bf9e4)), closes [#48](https://github.com/zed-extensions/ruby/issues/48)
* v0.7.0 ([9935ecf](https://github.com/zed-extensions/ruby/commit/9935ecf))



## 0.6.0 (2025-05-05)

* Fix RSpec outlines (#78) ([2d2b5dd](https://github.com/zed-extensions/ruby/commit/2d2b5dd)), closes [#78](https://github.com/zed-extensions/ruby/issues/78) [#77](https://github.com/zed-extensions/ruby/issues/77)
* Update Rust crate zed_extension_api to 0.5.0 (#44) ([047b3fd](https://github.com/zed-extensions/ruby/commit/047b3fd)), closes [#44](https://github.com/zed-extensions/ruby/issues/44)
* v0.6.0 (#80) ([785ebe4](https://github.com/zed-extensions/ruby/commit/785ebe4)), closes [#80](https://github.com/zed-extensions/ruby/issues/80)



## <small>0.5.6 (2025-04-30)</small>

* Add `completion_query_characters` (#49) ([310d2db](https://github.com/zed-extensions/ruby/commit/310d2db)), closes [#49](https://github.com/zed-extensions/ruby/issues/49)
* Add constants to outline view (#60) ([7044d0c](https://github.com/zed-extensions/ruby/commit/7044d0c)), closes [#60](https://github.com/zed-extensions/ruby/issues/60)
* Add minimal README (#54) ([aa0c857](https://github.com/zed-extensions/ruby/commit/aa0c857)), closes [#54](https://github.com/zed-extensions/ruby/issues/54)
* Add some basic tasks (#57) ([66bb9c2](https://github.com/zed-extensions/ruby/commit/66bb9c2)), closes [#57](https://github.com/zed-extensions/ruby/issues/57)
* Add task to evaluate selected text as Ruby (#73) ([825e779](https://github.com/zed-extensions/ruby/commit/825e779)), closes [#73](https://github.com/zed-extensions/ruby/issues/73)
* Capture constants with assignments only for outline panel (#61) ([d3b4ff6](https://github.com/zed-extensions/ruby/commit/d3b4ff6)), closes [#61](https://github.com/zed-extensions/ruby/issues/61)
* Fix missing Tailwind completions (#62) ([bdbd0b3](https://github.com/zed-extensions/ruby/commit/bdbd0b3)), closes [#62](https://github.com/zed-extensions/ruby/issues/62)
* Fix visibility keywords in outline (#65) ([fae9c93](https://github.com/zed-extensions/ruby/commit/fae9c93)), closes [#65](https://github.com/zed-extensions/ruby/issues/65) [#64](https://github.com/zed-extensions/ruby/issues/64)
* Improve Ruby outlines with support for macros (#70) ([eed46d3](https://github.com/zed-extensions/ruby/commit/eed46d3)), closes [#70](https://github.com/zed-extensions/ruby/issues/70) [#45](https://github.com/zed-extensions/ruby/issues/45)
* Include method modifiers in outline (#66) ([ee12679](https://github.com/zed-extensions/ruby/commit/ee12679)), closes [#66](https://github.com/zed-extensions/ruby/issues/66) [#65](https://github.com/zed-extensions/ruby/issues/65)
* Include singleton class in outline (#67) ([279c7fe](https://github.com/zed-extensions/ruby/commit/279c7fe)), closes [#67](https://github.com/zed-extensions/ruby/issues/67)
* Mark instance & class variables as "@variable.special" (#63) ([80fec8e](https://github.com/zed-extensions/ruby/commit/80fec8e)), closes [#63](https://github.com/zed-extensions/ruby/issues/63)
* Remove `else` from outdent triggers (#42) ([35ef4d4](https://github.com/zed-extensions/ruby/commit/35ef4d4)), closes [#42](https://github.com/zed-extensions/ruby/issues/42)
* Remove default `tasks.json` (#36) ([662ca25](https://github.com/zed-extensions/ruby/commit/662ca25)), closes [#36](https://github.com/zed-extensions/ruby/issues/36)
* Specify `?` and `!` as word characters (#58) ([34ab7c3](https://github.com/zed-extensions/ruby/commit/34ab7c3)), closes [#58](https://github.com/zed-extensions/ruby/issues/58)
* v0.5.6 ([4f28e5e](https://github.com/zed-extensions/ruby/commit/4f28e5e))



## <small>0.4.6 (2025-02-17)</small>

* Add textobjects support (#34) ([9cc5a77](https://github.com/zed-extensions/ruby/commit/9cc5a77)), closes [#34](https://github.com/zed-extensions/ruby/issues/34)
* Bump version to v0.4.6 ([a910ec7](https://github.com/zed-extensions/ruby/commit/a910ec7))
* Move `language_server_command` to trait implementation ([5014496](https://github.com/zed-extensions/ruby/commit/5014496))



## <small>0.4.5 (2025-02-13)</small>

* Add back in support for Minitest/RSpec symbols (#31) ([372ce12](https://github.com/zed-extensions/ruby/commit/372ce12)), closes [#31](https://github.com/zed-extensions/ruby/issues/31)
* Bump version to v0.4.5 ([1a68f52](https://github.com/zed-extensions/ruby/commit/1a68f52))



## <small>0.4.4 (2025-02-04)</small>

* Bump version to v0.4.4 ([c678571](https://github.com/zed-extensions/ruby/commit/c678571))
* Move variable highlighting rules to top level ([8e8cd34](https://github.com/zed-extensions/ruby/commit/8e8cd34))



## <small>0.4.3 (2025-02-04)</small>

* Add highlighting for module and exception keywords ([cf08167](https://github.com/zed-extensions/ruby/commit/cf08167))
* Bump version to v0.4.3 ([1e5d3b3](https://github.com/zed-extensions/ruby/commit/1e5d3b3))
* Change `identifier` and `global_variable` highlighting ([a5a818d](https://github.com/zed-extensions/ruby/commit/a5a818d))
* Extract common LSPs stuff into `LanguageServer` trait (#24) ([9fbf632](https://github.com/zed-extensions/ruby/commit/9fbf632)), closes [#24](https://github.com/zed-extensions/ruby/issues/24)
* Improve import keyword highlighting ([1c44a89](https://github.com/zed-extensions/ruby/commit/1c44a89))
* Remove .rdoc from the list of Ruby files (#26) ([1109c8b](https://github.com/zed-extensions/ruby/commit/1109c8b)), closes [#26](https://github.com/zed-extensions/ruby/issues/26)
* Remove redundant `overrides.scm` ([2bc9866](https://github.com/zed-extensions/ruby/commit/2bc9866))
* Separate `nil` from boolean literals ([5a666ca](https://github.com/zed-extensions/ruby/commit/5a666ca))
* Update tree-sitter-embedded-template to v0.23.2 ([7ef8f53](https://github.com/zed-extensions/ruby/commit/7ef8f53))
* Update tree-sitter-rbs ([528e4dd](https://github.com/zed-extensions/ruby/commit/528e4dd))
* Update tree-sitter-ruby to v0.23.1 ([fd2eaae](https://github.com/zed-extensions/ruby/commit/fd2eaae))



## <small>0.3.3 (2024-12-18)</small>

* Add `use_bundler` configuration option (#19) ([49921fe](https://github.com/zed-extensions/ruby/commit/49921fe)), closes [#19](https://github.com/zed-extensions/ruby/issues/19)
* Add label details to ruby-lsp completions (#13) ([2eda2ac](https://github.com/zed-extensions/ruby/commit/2eda2ac)), closes [#13](https://github.com/zed-extensions/ruby/issues/13)
* Bump to 0.2.3 (#15) ([aea5080](https://github.com/zed-extensions/ruby/commit/aea5080)), closes [#15](https://github.com/zed-extensions/ruby/issues/15)
* Bump version to 0.3.3 (#23) ([f0225e8](https://github.com/zed-extensions/ruby/commit/f0225e8)), closes [#23](https://github.com/zed-extensions/ruby/issues/23)
* Configure Renovate (#18) ([f5df067](https://github.com/zed-extensions/ruby/commit/f5df067)), closes [#18](https://github.com/zed-extensions/ruby/issues/18)
* Update `Cargo.lock` file (#20) ([bf5122c](https://github.com/zed-extensions/ruby/commit/bf5122c)), closes [#20](https://github.com/zed-extensions/ruby/issues/20)
* Update config.toml to include additional Ruby-related files (#5) ([784b0cb](https://github.com/zed-extensions/ruby/commit/784b0cb)), closes [#5](https://github.com/zed-extensions/ruby/issues/5) [#4](https://github.com/zed-extensions/ruby/issues/4) [#3](https://github.com/zed-extensions/ruby/issues/3)
* Upgrade `zed_extension_api` to v0.2.0 (#12) ([d540f23](https://github.com/zed-extensions/ruby/commit/d540f23)), closes [#12](https://github.com/zed-extensions/ruby/issues/12)
* Use bundler for `solargraph` and `rubocop` (#22) ([8de492b](https://github.com/zed-extensions/ruby/commit/8de492b)), closes [#22](https://github.com/zed-extensions/ruby/issues/22)



## <small>0.2.2 (2024-10-15)</small>

* bump to 0.2.1 (change repo url) ([2696061](https://github.com/zed-extensions/ruby/commit/2696061))
* Bump version to v0.2.2 (#2) ([0c890fc](https://github.com/zed-extensions/ruby/commit/0c890fc)), closes [#2](https://github.com/zed-extensions/ruby/issues/2)
* Fix several style lints (zed-industries/zed#17488) ([9a6f239](https://github.com/zed-extensions/ruby/commit/9a6f239)), closes [zed-industries/zed#17488](https://github.com/zed-industries/zed/issues/17488)
* Improve syntax highlights (zed-industries/zed#18728) ([961cecc](https://github.com/zed-extensions/ruby/commit/961cecc)), closes [zed-industries/zed#18728](https://github.com/zed-industries/zed/issues/18728) [zed-industries/zed#18722](https://github.com/zed-industries/zed/issues/18722) [zed-industries/zed#18722](https://github.com/zed-industries/zed/issues/18722)



## 0.2.0 (2024-08-30)

* Bump version to v0.2.0 (zed-industries/zed#17128) ([f40c010](https://github.com/zed-extensions/ruby/commit/f40c010)), closes [zed-industries/zed#17128](https://github.com/zed-industries/zed/issues/17128) [zed-industries/zed#16752](https://github.com/zed-industries/zed/issues/16752) [zed-industries/zed#16892](https://github.com/zed-industries/zed/issues/16892) [zed-industries/zed#16893](https://github.com/zed-industries/zed/issues/16893) [zed-industries/zed#16907](https://github.com/zed-industries/zed/issues/16907)
* Rename "rbs" language to "RBS" (zed-industries/zed#16893) ([5cab643](https://github.com/zed-extensions/ruby/commit/5cab643)), closes [zed-industries/zed#16893](https://github.com/zed-industries/zed/issues/16893)
* Replace default tasks with a stub message (zed-industries/zed#16752) ([be81351](https://github.com/zed-extensions/ruby/commit/be81351)), closes [zed-industries/zed#16752](https://github.com/zed-industries/zed/issues/16752) [zed-industries/zed#12579](https://github.com/zed-industries/zed/issues/12579)
* Update tree-sitter grammar for the Ruby language (zed-industries/zed#16892) ([d635d8b](https://github.com/zed-extensions/ruby/commit/d635d8b)), closes [zed-industries/zed#16892](https://github.com/zed-industries/zed/issues/16892)
* Upgrade `zed_extension_api` to v0.1.0 (zed-industries/zed#16907) ([2d3f5dc](https://github.com/zed-extensions/ruby/commit/2d3f5dc)), closes [zed-industries/zed#16907](https://github.com/zed-industries/zed/issues/16907)



## 0.1.0 (2024-08-06)

* Add support for *.rbs files (zed-industries/zed#15778) ([4f0b961](https://github.com/zed-extensions/ruby/commit/4f0b961)), closes [zed-industries/zed#15778](https://github.com/zed-industries/zed/issues/15778)
* Adjust language servers languages (zed-industries/zed#15297) ([53a6dcd](https://github.com/zed-extensions/ruby/commit/53a6dcd)), closes [zed-industries/zed#15297](https://github.com/zed-industries/zed/issues/15297)
* Bump to v0.1.0 (zed-industries/zed#15855) ([9554654](https://github.com/zed-extensions/ruby/commit/9554654)), closes [zed-industries/zed#15855](https://github.com/zed-industries/zed/issues/15855)
* Recognize `Steepfile`s as Ruby (zed-industries/zed#15762) ([9e24367](https://github.com/zed-extensions/ruby/commit/9e24367)), closes [zed-industries/zed#15762](https://github.com/zed-industries/zed/issues/15762)
* Support "binary" settings for Rubocop and Solargraph (zed-industries/zed#15110) ([b7512c9](https://github.com/zed-extensions/ruby/commit/b7512c9)), closes [zed-industries/zed#15110](https://github.com/zed-industries/zed/issues/15110)



## <small>0.0.8 (2024-07-18)</small>

* Add a new injection for regular expressions (zed-industries/zed#12533) ([d2cb413](https://github.com/zed-extensions/ruby/commit/d2cb413)), closes [zed-industries/zed#12533](https://github.com/zed-industries/zed/issues/12533)
* Add Podfile as Ruby extension language path suffix (zed-industries/zed#12392) ([f2fb122](https://github.com/zed-extensions/ruby/commit/f2fb122)), closes [zed-industries/zed#12392](https://github.com/zed-industries/zed/issues/12392) [zed-industries/extensions#803](https://github.com/zed-industries/extensions/issues/803)
* Add proper indentation for singleton methods (zed-industries/zed#12535) ([33ed96c](https://github.com/zed-extensions/ruby/commit/33ed96c)), closes [zed-industries/zed#12535](https://github.com/zed-industries/zed/issues/12535)
* Add support for "rubocop" language server (zed-industries/zed#14661) ([78953ea](https://github.com/zed-extensions/ruby/commit/78953ea)), closes [zed-industries/zed#14661](https://github.com/zed-industries/zed/issues/14661)
* Allow opt-in to Tailwind LS in string (zed-industries/zed#12742) ([dde08e4](https://github.com/zed-extensions/ruby/commit/dde08e4)), closes [zed-industries/zed#12742](https://github.com/zed-industries/zed/issues/12742) [zed-industries/zed#12728](https://github.com/zed-industries/zed/issues/12728)
* Auto detect some DSLs (zed-industries/zed#14693) ([2ea6a6a](https://github.com/zed-extensions/ruby/commit/2ea6a6a)), closes [zed-industries/zed#14693](https://github.com/zed-industries/zed/issues/14693)
* Bump to v0.0.6 (zed-industries/zed#12395) ([af95abd](https://github.com/zed-extensions/ruby/commit/af95abd)), closes [zed-industries/zed#12395](https://github.com/zed-industries/zed/issues/12395) [zed-industries/zed#12392](https://github.com/zed-industries/zed/issues/12392)
* Bump to v0.0.8 (zed-industries/zed#14707) ([136c338](https://github.com/zed-extensions/ruby/commit/136c338)), closes [zed-industries/zed#14707](https://github.com/zed-industries/zed/issues/14707)
* bump version to 0.0.5 (zed-industries/zed#12330) ([07fbf34](https://github.com/zed-extensions/ruby/commit/07fbf34)), closes [zed-industries/zed#12330](https://github.com/zed-industries/zed/issues/12330)
* Bump version to 0.0.7 (zed-industries/zed#12743) ([f905dd5](https://github.com/zed-extensions/ruby/commit/f905dd5)), closes [zed-industries/zed#12743](https://github.com/zed-industries/zed/issues/12743)
* Capture the heredoc content only and downcase the language (zed-industries/zed#12532) ([f143273](https://github.com/zed-extensions/ruby/commit/f143273)), closes [zed-industries/zed#12532](https://github.com/zed-industries/zed/issues/12532)
* Fix injections query location (zed-industries/zed#12534) ([04b7ce1](https://github.com/zed-extensions/ruby/commit/04b7ce1)), closes [zed-industries/zed#12534](https://github.com/zed-industries/zed/issues/12534) [zed-industries/zed#12532](https://github.com/zed-industries/zed/issues/12532)
* Remove outline for running tests (zed-industries/zed#12642) ([4d679a7](https://github.com/zed-extensions/ruby/commit/4d679a7)), closes [zed-industries/zed#12642](https://github.com/zed-industries/zed/issues/12642)
* Update tree-sitter grammar version (zed-industries/zed#13216) ([a57b367](https://github.com/zed-extensions/ruby/commit/a57b367)), closes [zed-industries/zed#13216](https://github.com/zed-industries/zed/issues/13216)
* tasks: Provide task variables from matching runnable ranges in task modal (zed-industries/zed#12237) ([481ea91](https://github.com/zed-extensions/ruby/commit/481ea91)), closes [zed-industries/zed#12237](https://github.com/zed-industries/zed/issues/12237) [zed-industries/zed#12003](https://github.com/zed-industries/zed/issues/12003)



## <small>0.0.4 (2024-05-21)</small>

* Add support for running tests (zed-industries/zed#12052) ([94e0868](https://github.com/zed-extensions/ruby/commit/94e0868)), closes [zed-industries/zed#12052](https://github.com/zed-industries/zed/issues/12052)
* Bump to v0.0.4 (zed-industries/zed#12101) ([48c83e9](https://github.com/zed-extensions/ruby/commit/48c83e9)), closes [zed-industries/zed#12101](https://github.com/zed-industries/zed/issues/12101) [zed-industries/zed#11869](https://github.com/zed-industries/zed/issues/11869) [zed-industries/zed#12012](https://github.com/zed-industries/zed/issues/12012) [zed-industries/zed#12052](https://github.com/zed-industries/zed/issues/12052)
* Pass initialization options to LSPs (zed-industries/zed#12012) ([064c5f5](https://github.com/zed-extensions/ruby/commit/064c5f5)), closes [zed-industries/zed#12012](https://github.com/zed-industries/zed/issues/12012)
* Use two spaces per indentation level (zed-industries/zed#11869) ([f74449d](https://github.com/zed-extensions/ruby/commit/f74449d)), closes [zed-industries/zed#11869](https://github.com/zed-industries/zed/issues/11869)



## <small>0.0.3 (2024-05-14)</small>

* Bump to v0.0.3 (zed-industries/zed#11833) ([4625f99](https://github.com/zed-extensions/ruby/commit/4625f99)), closes [zed-industries/zed#11833](https://github.com/zed-industries/zed/issues/11833) [zed-industries/zed#11825](https://github.com/zed-industries/zed/issues/11825)
* Fix solargraph completion highlighting (zed-industries/zed#11825) ([0146b0d](https://github.com/zed-extensions/ruby/commit/0146b0d)), closes [zed-industries/zed#11825](https://github.com/zed-industries/zed/issues/11825)



## <small>0.0.2 (2024-05-13)</small>

* Add `embedded_template` grammar (zed-industries/zed#11677) ([8376d46](https://github.com/zed-extensions/ruby/commit/8376d46)), closes [zed-industries/zed#11677](https://github.com/zed-industries/zed/issues/11677)
* Add ruby-lsp as an experimental language server (zed-industries/zed#11768) ([756db00](https://github.com/zed-extensions/ruby/commit/756db00)), closes [zed-industries/zed#11768](https://github.com/zed-industries/zed/issues/11768) [zed-industries/zed#4834](https://github.com/zed-industries/zed/issues/4834)
* Bump to v0.0.2 (zed-industries/zed#11769) ([facb601](https://github.com/zed-extensions/ruby/commit/facb601)), closes [zed-industries/zed#11769](https://github.com/zed-industries/zed/issues/11769) [zed-industries/zed#11768](https://github.com/zed-industries/zed/issues/11768)
* Move injections to extension (zed-industries/zed#11664) ([58c9aa2](https://github.com/zed-extensions/ruby/commit/58c9aa2)), closes [zed-industries/zed#11664](https://github.com/zed-industries/zed/issues/11664) [zed-industries/zed#8796](https://github.com/zed-industries/zed/issues/8796) [zed-industries/zed#11360](https://github.com/zed-industries/zed/issues/11360)



## <small>0.0.1 (2024-05-10)</small>

* Add crate licenses. (zed-industries/zed#4158) ([091cb42](https://github.com/zed-extensions/ruby/commit/091cb42)), closes [zed-industries/zed#4158](https://github.com/zed-industries/zed/issues/4158)
* Extract Ruby extension (zed-industries/zed#11360) ([ef5e675](https://github.com/zed-extensions/ruby/commit/ef5e675)), closes [zed-industries/zed#11360](https://github.com/zed-industries/zed/issues/11360)
