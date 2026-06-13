# buttre Project Roadmap

> Strategic plan for buttre development across platforms and features

**Last Updated**: 2026-05-19
**Version**: 0.6.3-alpha
**Status**: Windows Core & Installers Complete, Cross-Platform Expansion In Progress

---

## Vision

Build a **modern, performant, cross-platform Vietnamese input method** that:
- Delivers sub-millisecond keystroke processing
- Supports all major platforms (Windows, macOS, Linux)
- Provides flexible input methods (Telex, VNI, VIQR, Hán Nôm)
- Maintains absolute privacy (zero telemetry)
- Enables community contributions through open source

---

## Current Status

### ✅ Completed (v0.6.3-alpha)

**Core Engine**:
- [x] 7-stage processing pipeline (config-driven, recompute-from-raw)
- [x] Telex input method (full support)
- [x] VNI input method (full support)
- [x] Vietnamese orthography rules (100% compliant)
- [x] Tone mark positioning (Old & New styles)
- [x] English fallback mode (undo handling)
- [x] Flexible typing (permutation support)
- [x] 600+ integration tests
- [x] Performance optimization (sub-ms processing)

**Windows Platform**:
- [x] TSF (Text Services Framework) implementation
- [x] COM DLL registration (fixed CLSID alignment)
- [x] Composition string support
- [x] Key event handling
- [x] Manual testing guide

**Cross-Platform Installers** (v0.6.3-alpha):
- [x] Windows MSI via cargo-wix (perMachine scope, CLSID registration)
- [x] Linux .deb + .rpm via cargo-deb & cargo-generate-rpm (with IBus integration)
- [x] macOS dylib artifact (developer release, unsigned)
- [x] GitHub Actions 3-platform release matrix (softprops/action-gh-release@v2)
- [x] CLSID fix: TSF DLL registration aligned across platforms

**Infrastructure**:
- [x] Cargo workspace setup
- [x] Multi-crate architecture (engine, core, platform, test)
- [x] Clippy lints & code quality checks
- [x] Release optimization (LTO, size optimization)
- [x] Release artifacts on GitHub (Windows MSI, Linux .deb/.rpm, macOS dylib)

---

## Roadmap by Phase

### Phase 1: Windows Installers & Stability (Q1–Q2 2026)

**Goal**: Multi-platform signed-less release artifacts

**Completed Tasks** (v0.6.3-alpha):
- [x] Fix CLSID mismatch (E6B8A6C0-1234-5678-9ABC-DEF012345678)
- [x] Windows MSI via cargo-wix (perMachine, CLSID + profile registration)
- [x] Linux .deb + .rpm via cargo-deb/cargo-generate-rpm (IBus integration)
- [x] macOS dylib artifact (unsigned, developer-only)
- [x] GitHub Actions 3-platform matrix (windows/ubuntu/macos latest, parallel jobs)
- [x] Updated CHANGELOG.md with all installer entries

**Remaining Tasks** (Q2 2026):
- [ ] Fix known test failures
  - [ ] `test_find_best_permutation_thuwowfngf` (duplicate 'w' handling)
  - [ ] `test_telex_settings` / `test_vni_settings` (ToneStyle mismatch)
- [ ] Manual testing & bug fixes
  - [ ] Test in Notepad, Word, VS Code, browsers
  - [ ] Test .deb/.rpm on Ubuntu 22.04+
  - [ ] Fix edge cases discovered in real usage
- [ ] Documentation updates
  - [ ] WINDOWS_README.md (SmartScreen bypass workaround)
  - [ ] LINUX_README.md (IBus cache refresh)
  - [ ] MACOS_README.md (quarantine workaround)
  - [ ] User manual (Vietnamese)

**Deliverable**: buttre 0.6.3-alpha with cross-platform installers; buttre 1.0 for Windows (Q2 2026)

---

### Phase 2: macOS Implementation (Q2 2026)

**Goal**: Native macOS input method

**Architecture**:
```
┌─────────────────────────────────────┐
│     macOS Application               │
└────────────┬────────────────────────┘
             │ Text Input
             ▼
┌─────────────────────────────────────┐
│  Text Input Management (TIM)        │
└────────────┬────────────────────────┘
             │ IMKServer Protocol
             ▼
┌─────────────────────────────────────┐
│      buttre.app (Bundle)             │
│  ┌───────────────────────────────┐  │
│  │  IMKServer (Obj-C)            │  │
│  │  ├─ IMKInputController        │  │
│  │  └─ Rust Core (FFI)           │  │
│  │      └─ buttre-engine          │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

**Tasks**:
- [ ] Research IMKit framework
  - [ ] Study Apple documentation
  - [ ] Analyze existing input methods (e.g., GoTiengViet)
  - [ ] Determine FFI strategy
- [ ] Create `buttre-macos` crate
  - [ ] Objective-C bridge (using `objc` crate)
  - [ ] IMKServer implementation
  - [ ] IMKInputController wrapper
  - [ ] Key event handling
- [ ] Integrate with `buttre-engine`
  - [ ] Action mapping (Replace → setMarkedText)
  - [ ] Candidate window (for Hán Nôm)
- [ ] Build & packaging
  - [ ] .app bundle creation
  - [ ] Code signing (Developer ID)
  - [ ] Notarization for Gatekeeper
- [ ] Testing
  - [ ] Test in TextEdit, Notes, Safari, Chrome
  - [ ] Performance testing
- [ ] Distribution
  - [ ] DMG installer
  - [ ] Homebrew cask (optional)

**Deliverable**: buttre 1.0 for macOS

**Reference**: See `docs/archive/MACOS_IMPLEMENTATION_PLAN.md`

---

### Phase 3: Linux Implementation (Q3 2026)

**Goal**: IBus input method for Linux

**Architecture**:
```
┌─────────────────────────────────────┐
│     Linux Applications              │
└────────────┬────────────────────────┘
             │ GTK/Qt Input Context
             ▼
┌─────────────────────────────────────┐
│     IBus Daemon (ibus-daemon)       │
└────────────┬────────────────────────┘
             │ D-Bus IPC
             ▼
┌─────────────────────────────────────┐
│      buttre IBus Engine              │
│  ┌───────────────────────────────┐  │
│  │   D-Bus Interface (Rust)      │  │
│  │   └─ buttre-engine             │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

**Tasks**:
- [ ] Research IBus architecture
  - [ ] Study IBus protocol
  - [ ] Analyze ibus-bamboo (Go reference)
  - [ ] D-Bus communication in Rust (using `zbus`)
- [ ] Create `buttre-linux` crate
  - [ ] D-Bus interface implementation
  - [ ] Process key event handler
  - [ ] Preedit text management
  - [ ] Candidate window (for Hán Nôm)
- [ ] Integration
  - [ ] buttre-engine integration
  - [ ] Action mapping (Replace → update_preedit_text)
- [ ] Build & packaging
  - [ ] Shared object (.so) compilation
  - [ ] Desktop file creation
  - [ ] IBus component XML
- [ ] Distribution
  - [ ] .deb package (Ubuntu/Debian)
  - [ ] .rpm package (Fedora/RHEL)
  - [ ] AUR package (Arch Linux)
  - [ ] Flatpak (optional)
- [ ] Testing
  - [ ] Test in gedit, LibreOffice, Firefox
  - [ ] Wayland support verification
  - [ ] X11 fallback testing

**Deliverable**: buttre 1.0 for Linux (IBus)

**Future**: Fcitx5 support (Phase 3.5)

**Reference**: See `docs/archive/LINUX_IMPLEMENTATION_PLAN.md`

---

### Phase 4: Hán Nôm Support (Q4 2026)

**Goal**: Full Hán Nôm (chữ Nôm) input method

**Features**:
- [ ] Dictionary-based input
  - [ ] 48,510 Hán Nôm character database (from rime-han-nom-data)
  - [ ] SQLite FTS5 full-text search
  - [ ] Keyword-based lookup
  - [ ] Optimized index (minimal size)
- [ ] Candidate window
  - [ ] Show multiple candidates
  - [ ] Navigate with arrow keys / number keys
  - [ ] Preview character details (Nom Meaning, Sino-Vietnamese)
- [ ] Input modes
  - [ ] Vietnamese pronunciation input (e.g., "người" → 𠊛)
  - [ ] Sino-Vietnamese input (e.g., "nhân" → 人)
  - [ ] Keyword search (e.g., "person" → 人, 𠊛)
- [ ] Pipeline integration
  - [ ] Stage 11: Dictionary Lookup
  - [ ] Stage 12: Output Generation (candidates)
- [ ] Testing & documentation
  - [ ] Test data from classical texts
  - [ ] User guide for Hán Nôm input

**Deliverable**: buttre 1.5 with Hán Nôm support

**Reference**:
- `docs/archive/NOM_DEVELOPER_GUIDE.md`
- `docs/archive/NOM_DATABASE_OPTIMIZATION.md`
- `docs/archive/NOM_INPUT_ANALYSIS.md`

---

### Phase 5: Advanced Features (2027)

**Goal**: Enhance user experience with advanced features

**Features Under Consideration**:
- [ ] **Auto-completion**
  - [ ] Word-level prediction
  - [ ] Phrase-level suggestion
  - [ ] User dictionary learning
- [ ] **Spelling correction**
  - [ ] Fuzzy matching for typos
  - [ ] Suggestion ranking
- [ ] **User customization**
  - [ ] Custom key bindings
  - [ ] Custom transformation rules
  - [ ] Custom dictionary
- [ ] **Multi-language support** (UI)
  - [ ] English UI
  - [ ] Vietnamese UI
- [ ] **Minority languages** (stretch goal)
  - [ ] Tày-Nùng script
  - [ ] Chăm script
  - [ ] Hmong script
- [ ] **Cloud sync** (opt-in)
  - [ ] Sync user dictionary across devices
  - [ ] Privacy-preserving (encrypted)

**Note**: These features are **under discussion**. Implementation depends on:
- Community demand
- Team availability
- Technical feasibility
- Privacy considerations

---

## Platform Priority Matrix

| Platform | Priority | Status | Target |
|----------|----------|--------|--------|
| Windows  | High     | ✅ Done (TSF) | 1.0 (Q1 2026) |
| macOS    | High     | 🚧 Planned (IMKit) | 1.0 (Q2 2026) |
| Linux    | High     | 🚧 Planned (IBus) | 1.0 (Q3 2026) |
| ChromeOS | Low      | ⏳ Future | TBD |
| Android  | Low      | ⏳ Future | TBD |
| iOS      | Low      | ⏳ Future | TBD |

**Notes**:
- Desktop platforms (Windows/macOS/Linux) are **top priority**
- Mobile platforms (Android/iOS) require different architecture (virtual keyboard vs IME)
- ChromeOS can potentially reuse Linux (IBus) implementation

---

## Technical Debt & Refactoring

### Known Issues

**Pre-existing Test Failures**:
1. `test_find_best_permutation_thuwowfngf` (stage6_permutation.rs)
   - **Issue**: Duplicate transform mark handling appends extra 'w'
   - **Priority**: Medium (affects edge case)
   - **Fix**: Improve permutation duplicate detection

2. `test_telex_settings` / `test_vni_settings` (presets.rs)
   - **Issue**: Test expects ToneStyle::New but preset uses ToneStyle::Old
   - **Priority**: Low (test vs preset mismatch)
   - **Fix**: Align test expectations with preset defaults

**Architecture Improvements** (Future):
- [ ] **Error handling**: Replace `anyhow` with custom error types in library code
- [ ] **Logging**: Replace debug file writing with proper `tracing` integration
- [ ] **Configuration**: Centralized config management (TOML file + UI)
- [ ] **Modularity**: Extract platform-agnostic UI components

---

## Community & Ecosystem

### Open Source Strategy

**Goals**:
- Build a **vibrant community** around buttre
- Encourage **contributions** from developers and linguists
- Provide **documentation** for contributors
- Maintain **high code quality** standards

**Community Initiatives**:
- [ ] **Contributing Guide** (CONTRIBUTING.md)
  - [ ] How to build from source
  - [ ] How to run tests
  - [ ] Code review process
  - [ ] PR guidelines
- [ ] **Issue templates**
  - [ ] Bug report template
  - [ ] Feature request template
  - [ ] Q&A template
- [ ] **GitHub Discussions**
  - [ ] General discussion
  - [ ] Feature proposals
  - [ ] Showcase (user projects)
- [ ] **Documentation site**
  - [ ] User manual
  - [ ] Developer guide
  - [ ] API documentation

### Licensing

**Current**: Mozilla Public License 2.0 (MPL-2.0)

**Why MPL-2.0?**
- ✅ **Copyleft for modifications**: Changes to buttre code must be open-sourced
- ✅ **Compatible with proprietary**: Can be integrated into proprietary apps
- ✅ **File-level copyleft**: Only modified files need to be shared, not entire project
- ✅ **Patent grant**: Protection against patent claims

**License unchanged**: No plans to change license

---

## Timeline Summary

| Quarter | Focus | Deliverable |
|---------|-------|-------------|
| Q1–Q2 2026 | Installers & Windows Polish | buttre 0.6.3-alpha (installers), buttre 1.0 Windows (stable) |
| Q2 2026 | macOS Implementation | buttre 1.0 macOS |
| Q3 2026 | Linux Implementation | buttre 1.0 Linux (IBus) |
| Q4 2026 | Hán Nôm Support | buttre 1.5 (all platforms) |
| 2027    | Advanced Features | buttre 2.0 (auto-complete, etc.) |

**Note**: Timeline is **aspirational** and depends on:
- Core team availability (this is a **passion project**, not commercial)
- Community contributions
- Platform complexity
- Bug severity

**Flexibility**: We prioritize **quality over speed**. Releases may be delayed to ensure stability.

---

## How to Contribute

Interested in contributing to buttre? Here's how:

1. **Code Contributions**:
   - Check open issues labeled `good first issue`
   - Read `docs/CODING_GUIDE.md` for coding standards
   - Submit PR with tests and documentation

2. **Testing & Feedback**:
   - Try beta releases and report bugs
   - Test on different platforms and applications
   - Provide UX feedback

3. **Documentation**:
   - Improve user guides
   - Write tutorials
   - Translate documentation

4. **Linguistics**:
   - Help with Hán Nôm dictionary
   - Validate Vietnamese orthography rules
   - Support minority language scripts

**Join us**: [GitHub Discussions](https://github.com/dxsl-org/buttre/discussions)

---

## Contact & Resources

- **GitHub**: [https://github.com/dxsl-org/buttre](https://github.com/dxsl-org/buttre)
- **Issues**: [https://github.com/dxsl-org/buttre/issues](https://github.com/dxsl-org/buttre/issues)
- **Discussions**: [https://github.com/dxsl-org/buttre/discussions](https://github.com/dxsl-org/buttre/discussions)
- **Documentation**: `docs/` folder in repository

---

**Last Updated**: 2026-05-19

_This roadmap is a living document and will be updated as the project evolves._
