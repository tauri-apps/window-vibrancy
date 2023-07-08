# Changelog

## \[0.4.0]

- [`a0f4712`](https://github.com/tauri-apps/window-vibrancy/commit/a0f4712db58cb1cb3685383de4d634fd5bda6383)([#90](https://github.com/tauri-apps/window-vibrancy/pull/90)) On Windows, Add option to specify dark for mica effect.

## \[0.3.2]

- Fix `apply_acrylic` on Windows 11 22H2.
  - [d05ef14](https://github.com/tauri-apps/window-vibrancy/commit/d05ef146b94a8ca66e091e62be112a1c57d14563) fix: fix acrylic on windows 11 22523, closes [#45](https://github.com/tauri-apps/window-vibrancy/pull/45) ([#69](https://github.com/tauri-apps/window-vibrancy/pull/69)) on 2022-10-25
  - [aab46e3](https://github.com/tauri-apps/window-vibrancy/commit/aab46e35eaf014d63920999c4e0132baeb55fc50) publish new versions ([#70](https://github.com/tauri-apps/window-vibrancy/pull/70)) on 2022-10-25
  - [d20161f](https://github.com/tauri-apps/window-vibrancy/commit/d20161fc1892908839e4f7d715e16256b2d96900) fix: fix acrylic on win11 22h2 on 2022-11-18

## \[0.3.1]

- Fix acrylic effect on Windows 11 build 22523 and higher
  - [d05ef14](https://github.com/tauri-apps/window-vibrancy/commit/d05ef146b94a8ca66e091e62be112a1c57d14563) fix: fix acrylic on windows 11 22523, closes [#45](https://github.com/tauri-apps/window-vibrancy/pull/45) ([#69](https://github.com/tauri-apps/window-vibrancy/pull/69)) on 2022-10-25

## \[0.3.0]

- Add a 3rd argument to `apply_vibrancy()` to control the vibrancy state, follows window active state by default.
  - [0a566c6](https://github.com/tauri-apps/window-vibrancy/commit/0a566c6cefca0371ce0e19cce8b9c7c7a7ae1f12) feat: (macos) add vibrancy state parameter ([#63](https://github.com/tauri-apps/window-vibrancy/pull/63)) on 2022-09-19
- Add a 4th argument to `apply_vibrancy()` to control the corner radius of the effect view.
  - [bffac24](https://github.com/tauri-apps/window-vibrancy/commit/bffac24a783dfd6c4d147d7bed6d5abc1d126acf) feat: add rounded corner support on MacOS  ([#26](https://github.com/tauri-apps/window-vibrancy/pull/26)) on 2022-09-19

## \[0.2.0]

- Update `raw-window-handle` dependency to 0.5
  - [aef927b](https://github.com/tauri-apps/window-vibrancy/commit/aef927b7378e834c2b14df13de785770c812c8a0) chore(deps): update raw-window-handle to 0.5 on 2022-07-25

## \[0.1.3]

- Fix `apply_acrylic` effect on Windows 11.
  - [7f4e28f](https://github.com/tauri-apps/window-vibrancy/commit/7f4e28fba82bfc70673cc48ca1aabec2356bdccd) fix(acrylic): pass correct `AccentFlags` to `swca` on 2022-04-29
  - [92ef268](https://github.com/tauri-apps/window-vibrancy/commit/92ef268006686fcdc9b8a3dd09d2b71b5140bd7f) chore: add screenshots ([#37](https://github.com/tauri-apps/window-vibrancy/pull/37)) on 2022-05-23
- Add screenshots
  - [92ef268](https://github.com/tauri-apps/window-vibrancy/commit/92ef268006686fcdc9b8a3dd09d2b71b5140bd7f) chore: add screenshots ([#37](https://github.com/tauri-apps/window-vibrancy/pull/37)) on 2022-05-23

## \[0.1.2]

- Update examples and documentation about macOS `NSVisualEffectMaterial`.
  - [e3e2cc7](https://github.com/tauri-apps/window-vibrancy/commit/e3e2cc7323a830305ef84001edfd7a7678d098d7) docs: update examples and macos NSVisualEffectMaterial on 2022-04-15

## \[0.1.1]

- Update crate docs.
  - [2764ca3](https://github.com/tauri-apps/window-vibrancy/commit/2764ca398661b7f4045b39883914f67e299a7fe4) chore: update crate docs on 2022-03-29

## \[0.1.0]

- Initial Release.
  - [78acb98](https://github.com/tauri-apps/window-vibrancy/commit/78acb9800f9a67ff5793de0b45b78225d91e2947) chore(readme): remove installation section on 2022-03-05
