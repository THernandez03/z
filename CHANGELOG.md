# Changelog

## [0.7.0](https://github.com/THernandez03/z/compare/v0.6.0...v0.7.0) (2026-06-29)


### Features

* ✨ Gold-colored version manager and program names in output ([9a2cdf6](https://github.com/THernandez03/z/commit/9a2cdf620f8539d5a9c7d7d57e44f12c04e3d086))

## [0.6.0](https://github.com/THernandez03/z/compare/v0.5.1...v0.6.0) (2026-05-24)


### Features

* ✨ Colored help, -H/-v aliases, styled info/uninstall ([044425c](https://github.com/THernandez03/z/commit/044425c6678bfb46804611257135468a2e8cf613))
* add binary releases and install.sh ([c6a6ff5](https://github.com/THernandez03/z/commit/c6a6ff5bb0798ca0f6e6da175f9d643ba0f1ff61))
* add nightly and edge aliases to master (nightly) channel ([fc31bdf](https://github.com/THernandez03/z/commit/fc31bdf128bb26ef3f601077b61784def5da761e))
* colorized install messages; fix uninstall() return type; add console dep ([9660a56](https://github.com/THernandez03/z/commit/9660a56f6e1a88611a08e504032c2da5ed7dfa6e))
* display from/to version during activation ([d0bc0a0](https://github.com/THernandez03/z/commit/d0bc0a0c9740b16cde03775d3d84bd41e9238f48))
* restructure CLI, add Makefile, update README ([df8d828](https://github.com/THernandez03/z/commit/df8d828c37986ae1b2e81d8fc72f6c3976bdb201))
* skip activation when version is already active ([9689719](https://github.com/THernandez03/z/commit/968971977abf9f73b5e7a35dc010f7d888178bd7))


### Bug Fixes

* 🐛 Strip name prefix from self-update version tag ([4e736a7](https://github.com/THernandez03/z/commit/4e736a70c69ca1227edcc2045a4dae3af3149e9e))
* remove stale uninstall tests, fix needless borrow in install.rs ([bacf328](https://github.com/THernandez03/z/commit/bacf3283d8488c06194a65053bde2d7867c5d433))
* run tests single-threaded to avoid env-var data race between modules ([287dd3b](https://github.com/THernandez03/z/commit/287dd3bb2aba57e500b3c4d2a27957ac0af4687e))
* use real version string instead of 'master' as cache key ([1827003](https://github.com/THernandez03/z/commit/1827003e0176ec54de7bafd46d8a63dd8d3a1b47))


### Documentation

* 📝 Document prune --force and uninstall --yes flags ([57f778b](https://github.com/THernandez03/z/commit/57f778b6f53dbbe6e1153d384b0f67e4a2dbc624))
* add related projects section ([89008da](https://github.com/THernandez03/z/commit/89008da10fe780f3436636721a3e7e299a1b078c))

## [0.5.1](https://github.com/THernandez03/z/compare/z-v0.5.0...z-v0.5.1) (2026-05-24)


### Bug Fixes

* 🐛 Strip name prefix from self-update version tag ([4e736a7](https://github.com/THernandez03/z/commit/4e736a70c69ca1227edcc2045a4dae3af3149e9e))

## [0.5.0](https://github.com/THernandez03/z/compare/z-v0.4.0...z-v0.5.0) (2026-05-24)


### Features

* ✨ Colored help, -H/-v aliases, styled info/uninstall ([044425c](https://github.com/THernandez03/z/commit/044425c6678bfb46804611257135468a2e8cf613))
* add binary releases and install.sh ([c6a6ff5](https://github.com/THernandez03/z/commit/c6a6ff5bb0798ca0f6e6da175f9d643ba0f1ff61))
* add nightly and edge aliases to master (nightly) channel ([fc31bdf](https://github.com/THernandez03/z/commit/fc31bdf128bb26ef3f601077b61784def5da761e))
* colorized install messages; fix uninstall() return type; add console dep ([9660a56](https://github.com/THernandez03/z/commit/9660a56f6e1a88611a08e504032c2da5ed7dfa6e))
* display from/to version during activation ([d0bc0a0](https://github.com/THernandez03/z/commit/d0bc0a0c9740b16cde03775d3d84bd41e9238f48))
* restructure CLI, add Makefile, update README ([df8d828](https://github.com/THernandez03/z/commit/df8d828c37986ae1b2e81d8fc72f6c3976bdb201))
* skip activation when version is already active ([9689719](https://github.com/THernandez03/z/commit/968971977abf9f73b5e7a35dc010f7d888178bd7))


### Bug Fixes

* remove stale uninstall tests, fix needless borrow in install.rs ([bacf328](https://github.com/THernandez03/z/commit/bacf3283d8488c06194a65053bde2d7867c5d433))
* run tests single-threaded to avoid env-var data race between modules ([287dd3b](https://github.com/THernandez03/z/commit/287dd3bb2aba57e500b3c4d2a27957ac0af4687e))
* use real version string instead of 'master' as cache key ([1827003](https://github.com/THernandez03/z/commit/1827003e0176ec54de7bafd46d8a63dd8d3a1b47))


### Documentation

* 📝 Document prune --force and uninstall --yes flags ([57f778b](https://github.com/THernandez03/z/commit/57f778b6f53dbbe6e1153d384b0f67e4a2dbc624))
* add related projects section ([89008da](https://github.com/THernandez03/z/commit/89008da10fe780f3436636721a3e7e299a1b078c))

## [0.4.0](https://github.com/THernandez03/z/compare/v0.3.1...v0.4.0) (2026-05-24)


### Features

* ✨ Add --force to prune and --yes/-y to uninstall ([30ccf82](https://github.com/THernandez03/z/commit/30ccf829a042f0627bd8b185a0e84a729a9655af))
