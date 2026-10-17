# Known Issues

This page tracks known limitations, build constraints, and verification gaps
for Lunola Scripta as of the 1.0.0 delivery.

---

## Build Requirements

### Rust version

The project requires **rustc >= 1.95** (specified in `Cargo.toml` as
`rust-version = "1.95"`). Verified with **rustc 1.99.1 stable** — `cargo check -p wordcraft-ui-egui` passes.

```sh
rustup update stable
```

### Krilla (PDF backend) git dependency

The `krilla` PDF writing library is pulled from a git dependency
(`https://github.com/storytold/craft-libs`, rev `c348732`). If the git
repository is unreachable, private, or the revision is unavailable, the
build will fail at the dependency resolution step. Ensure network access
to GitHub or a vendored copy of the repository is available.

---

## Verification Status

### Automated tests

**`cargo check -p wordcraft-ui-egui`**: ✅ PASS (rustc 1.99.1)

The test suite (~800 tests plus property-based tests) was not
executed as part of this delivery. The test suite should be run in CI or on a machine with rustc >= 1.95:

```sh
cargo xtask ci    # formatting, clippy, tests, asset-attribution, layering, wasm
```

### GUI verification

**Not performed.** No manual or automated GUI testing was conducted. The
screenshots in the README were rendered by the offscreen UI test harness
(`cargo run -p lunola-scripta-ui-egui --example ui_shot`) but have not been
manually reviewed for visual regressions.

### WebAssembly build

**Not verified.** The wasm target build (`cargo xtask ci` includes a wasm
check) was not run.

---

## Known Functional Limitations

### Stability

- The ROADMAP documents ~21 open core-path bugs as of the last review.
- There is open field evidence of a startup crash and several hangs
  (see `docs/gaps.md` #3 for details).

### File format fidelity

- **Charts and SmartArt:** dropped silently on .docx import; not yet
  supported (parity: 3%).
- **OLE objects:** not preserved through a round trip.
- **Content controls:** lost on import/export.
- **`.doc` writing:** read-only; no write support for Word 97-2003 format.

### Pagination fidelity

- Column balancing and footnote continuation do not match Word exactly.
- No Aptos-metric font yet.

### Printing

- No native printing dialog; PDF export is the recommended print workflow
  (see `docs/roadmap.md` printing update, 2026-10-12).

### Localization

- UI strings are available in 20 languages, but depth of proofing tools
  (spelling, grammar, hyphenation, thesaurus) is at ~27% parity with Word.
- Right-to-left and complex script support is at ~50% parity.

---

## Platform-Specific Issues

### Windows

- DirectX 12 is the default graphics backend; some AMD drivers crash before
  the window appears. Set `WGPU_BACKEND=vulkan` or `WGPU_BACKEND=dx12` to
  work around this (see README "Downloads" section for details).
- The 32-bit build uses OpenGL even when a DirectX 12 GPU is present.

### Linux (Wayland)

- Runs through XWayland by default for file drag-and-drop support.
- Native Wayland backend available via `WORDCRAFT_WAYLAND=1` (trades
  drag-and-drop for sharper text with fractional scaling).

---

## Missing Documentation

- No video tutorials or screen-cast walkthroughs exist yet.
- API reference docs (rustdoc) are not published.
- The MCP server protocol documentation (`docs/mcp.md`) is in draft form
  and has not been validated against a live MCP session.
