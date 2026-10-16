# Changelog

All notable changes to Lunola Scripta are documented in this file.

## [1.0.0] - 2026-10-16

### Rebranding: WordCraft → Lunola Scripta

This release marks the completion of the Lunola Scripta rebranding. The product
is now **Lunola Scripta**, part of the Lunola ecosystem of creative tools.
The internal crate names (`wordcraft-*`) remain unchanged to avoid breaking
file-format identifiers and downstream dependents.

### Branding & Visual Identity

- **New product name:** Lunola Scripta (tagline: "Write Your World.")
- **Accent color:** #328BE6 (Lunola blue) applied throughout the UI
- **New logo and icons:** app icon, splash screen, and menu bar icons updated
  to the Lunola Scripta brand assets
- **README refreshed:** complete rewrite of the README with Lunola Scripta
  branding, ecosystem links, and updated screenshots

### UI Modernization

- Ribbon and dialog styling updated to match the Lunola design language
- Dark mode accent colors aligned to the new brand palette
- Welcome / File tab (Backstage) start page rebranded
- About window and preferences dialog rebranded

### Icons & Assets

- Added `docs/brand/` directory with Lunola Scripta SVG logos (light/dark)
- Application icon set added at `assets/app-icon/hicolor/`
- Splash screen updated with new branding
- Favicon for the web app updated

### Ecosystem Integration

- README "Crafting Apps" section: Lunola Scripta presented alongside
  PhotoCraft, VectorCraft, FilmCraft, LightCraft, PdfCraft, EffectCraft,
  and DesignCraft
- Links to getlunola.com and the Lunola Discord
- "Made by the Lunola team" attribution in the app and documentation

### Documentation

- **CHANGELOG.md** (this file) created
- **KNOWN_ISSUES.md** created — build requirements, test status, limitations
- **DELIVERY.md** created — phase-by-phase delivery summary
- **ROADMAP.md** updated with delivery documentation links
- **README.md** updated with links to all new documentation

### Breaking Changes

- None. All document formats, command IDs, and crate names are unchanged.
