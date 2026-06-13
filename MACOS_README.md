> **macOS support is currently developer-only.** There is no end-user installer.
> The release ships `libbuttre_platform.dylib` as a developer artifact (universal binary) for use in
> custom host apps. See the bundled README inside `buttre-*-macos-dylib.zip` for linking instructions
> and Gatekeeper quarantine workaround. A Swift IMK shell is planned but not yet built.

# 🍎 buttre macOS - Ready for Development!

## 📦 Build Information

**Version**: 0.1.0 (Phase 1 - Foundation)  
**Status**: ✅ Code Complete, Ready to Build on macOS  
**Architecture**: IMKit + Rust FFI

---

## 🏗️ Architecture

```
┌─────────────────────────────┐
│    macOS Applications       │
│  (TextEdit, Notes, etc.)    │
└──────────┬──────────────────┘
           │
    ┌──────▼──────┐
    │  IMKServer  │  (Objective-C)
    └──────┬──────┘
           │
    ┌──────▼──────┐
    │IMKInputCtrl │  (Objective-C)
    └──────┬──────┘
           │ FFI
    ┌──────▼──────┐
    │   Engine    │  (Rust)
    │buttre-engine│
    └─────────────┘
```

---

## 🚀 Build Instructions (On macOS)

### Prerequisites

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add aarch64 target (for Apple Silicon)
rustup target add aarch64-apple-darwin
```

### Build

```bash
# Make script executable
chmod +x scripts/build-macos.sh

# Run build
./scripts/build-macos.sh
```

**Output**: `target/release/libbuttre_platform.dylib` (host arch; use `installers/macos/build_dylib.sh <version>` for the universal release ZIP)

---

## 📥 Installation

```bash
# Copy to system location (requires sudo)
sudo cp -R build/macos/buttre.app "/Library/Input Methods/"

# Restart Input Method system
killall -9 SystemUIServer
```

### Add Input Source

1. **System Settings** → **Keyboard** → **Input Sources**
2. Click **+** button
3. Select **Vietnamese**
4. Choose **buttre**
5. Click **Add**

---

## 🧪 Testing

### Test 1: Basic Input
1. Open **TextEdit**
2. Switch to buttre (Control + Space or Globe key)
3. Type: `hoaf`
4. **Expected**: `hoà` with underline

### Test 2: Uppercase
1. Type: `Shift+V` `i` `e` `e` `t`
2. **Expected**: `Việt`

### Test 3: Backspace
1. Type: `hoaf` → `hoà`
2. Press **Backspace**
3. **Expected**: `hoa`

### Test 4: Finalization
1. Type: `hoaf` → `hoà`
2. Press **Space**
3. **Expected**: `hoà ` (underline removed)

---

## 📁 Project Structure

```
crates/buttre-platform/
├── src/platforms/macos/
│   ├── mod.rs                  # macOS backend entry
│   └── ffi.rs                  # C ABI exposed to IMKit hosts
└── Cargo.toml                  # cdylib → libbuttre_platform.dylib

installers/macos/
├── build_dylib.sh              # Universal (arm64 + x86_64) release ZIP
└── ARTIFACT_README.md          # Integration notes for the dylib
```

---

## 🔍 Implementation Details

### FFI Functions

```c
// Create engine
void* buttre_engine_new(void);

// Free engine
void buttre_engine_free(void* engine);

// Process key
const char* buttre_engine_process_key(void* engine, unsigned short keycode, BOOL shift);

// Process backspace
const char* buttre_engine_process_backspace(void* engine);
```

### Key Event Flow

```
User types 'h'
    ↓
handleEvent: (NSEvent)
    ↓
buttre_engine_process_key(engine, keycode, shift)
    ↓
Rust: TelexMethod::process('h')
    ↓
Return: "h"
    ↓
setMarkedText: "h" (with underline)
    ↓
Display in app
```

---

## ✅ What's Implemented

- ✅ **IMKServer** - Server initialization
- ✅ **IMKInputController** - Event handling
- ✅ **FFI Bridge** - Rust ↔ Objective-C
- ✅ **Vietnamese Engine** - Telex processing
- ✅ **Composition** - Real-time updates
- ✅ **Backspace** - Intelligent handling
- ✅ **Shift Support** - Uppercase letters
- ✅ **Finalization** - Space/Enter commits

---

## ⏳ What's NOT Implemented Yet

- ❌ **VNI Mode** - Only Telex for now
- ❌ **Candidate UI** - Not needed for Vietnamese
- ❌ **Han Nom** - Planned for Phase 2
- ❌ **Settings UI** - Configuration panel
- ❌ **Icon** - App icon

---

## 🐛 Troubleshooting

### Issue: Build Fails

**Solution**:
- Ensure Xcode Command Line Tools installed
- Check Rust toolchain: `rustup show`
- Verify target: `rustup target list --installed`

### Issue: App Not in Input Sources

**Solution**:
- Check installation path: `/Library/Input Methods/buttre.app`
- Verify Info.plist is correct
- Restart: `killall -9 SystemUIServer`
- Reboot macOS

### Issue: No Composition

**Solution**:
- Check Console.app for logs (filter: "buttre")
- Verify buttre is selected input source
- Try in TextEdit first (best IMKit support)

---

## 📊 Comparison: Windows vs macOS

| Aspect | Windows TSF | macOS IMKit |
|--------|-------------|-------------|
| **Status** | ✅ Complete | ✅ Code Ready |
| **Build** | DLL | App Bundle |
| **Install** | Registry | /Library/Input Methods/ |
| **API** | COM | Objective-C |
| **Composition** | ITfComposition | setMarkedText |
| **Events** | ITfKeyEventSink | handleEvent |

---

## 🚀 Next Steps

### Immediate
1. **Build on macOS** - Run build script
2. **Test** - Verify basic functionality
3. **Debug** - Fix any issues

### Phase 2 (Week 2-3)
- [ ] VNI mode switching
- [ ] Settings panel
- [ ] App icon
- [ ] Localization

### Phase 3 (Week 3-4)
- [ ] Han Nom support
- [ ] Candidate UI
- [ ] Multi-monitor support

---

## 📚 References

- [Input Method Kit Guide](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/InputMethod/InputMethod.html)
- [IMKInputController](https://developer.apple.com/documentation/inputmethodkit/imkinputcontroller)
- [OpenVanilla](https://github.com/openvanilla/openvanilla) - Reference

---

**Status**: Ready for macOS Build  
**Next**: Build and test on macOS machine  
**Timeline**: 1-2 weeks to production-ready

---

*Built with ❤️ using Rust + Objective-C*
