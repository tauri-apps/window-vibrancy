---
'window-vibrancy': patch
---

Added the `LiquidGlassOptions::interactive` option, which sets `NSGlassEffectView.effectIsInteractive` (macOS 27.0+) to enable the glass' visual response to user interactions. Also added `effectIsInteractive`, `setEffectIsInteractive` and `supportsEffectIsInteractive` methods to `NSGlassEffectViewTagged`.
