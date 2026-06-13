# buttre TSF - Manual Testing Guide

## 📦 Build Output

**DLL Location**: `target/release/buttre_platform.dll`

**Build Date**: 2025-12-15  
**Version**: 0.1.0 (Week 5 - Vietnamese Engine + Polish)

---

## 🚀 Installation & Registration

### Step 1: Copy DLL to System Location

```powershell
# Run as Administrator
$dllPath = "target\release\buttre_platform.dll"
$systemPath = "$env:ProgramFiles\buttre\buttre_platform.dll"

# Create directory
New-Item -ItemType Directory -Force -Path "$env:ProgramFiles\buttre"

# Copy DLL
Copy-Item $dllPath $systemPath -Force
```

### Step 2: Register COM Server

```powershell
# Run as Administrator
regsvr32 "$env:ProgramFiles\buttre\buttre_platform.dll"
```

**Expected Output**: "DllRegisterServer in buttre_platform.dll succeeded"

### Step 3: Enable TSF Service

1. Open **Settings** → **Time & Language** → **Language**
2. Click **Preferred languages** → **Add a language**
3. Search for "Vietnamese" → Add
4. Click **Vietnamese** → **Options**
5. Under **Keyboards**, click **Add a keyboard**
6. Look for **buttre** in the list
7. Select **buttre** and click **Add**

---

## 🧪 Testing Checklist

### Basic Functionality

#### Test 1: Telex Input (Lowercase)
1. Open **Notepad**
2. Switch to buttre input (Windows + Space)
3. Type: `hoaf`
4. **Expected**: `hoà` (with grave accent)

#### Test 2: Telex Input (Uppercase)
1. Type: `Shift+V` `i` `e` `e` `t`
2. **Expected**: `Việt`

#### Test 3: Complex Word
1. Type: `t` `o` `a` `n` `f`
2. **Expected**: `toàn`

#### Test 4: Backspace
1. Type: `h` `o` `a` `f` → `hoà`
2. Press **Backspace**
3. **Expected**: `hoa` (accent removed)
4. Press **Backspace** again
5. **Expected**: `ho`

#### Test 5: Space Finalization
1. Type: `h` `o` `a` `f` → `hoà`
2. Press **Space**
3. **Expected**: `hoà ` (composition finalized, cursor after space)

#### Test 6: Enter Finalization
1. Type: `h` `o` `a` `f` → `hoà`
2. Press **Enter**
3. **Expected**: `hoà` on first line, cursor on new line

### Advanced Tests

#### Test 7: VNI Mode (Future)
*Note: Currently only Telex is active. VNI mode switching not yet implemented in UI.*

#### Test 8: Multiple Words
1. Type: `t` `i` `e` `e` `n` `g` **Space** `v` `i` `e` `e` `t`
2. **Expected**: `tiếng việt`

#### Test 9: Tone Changes
1. Type: `h` `o` `a` `f` → `hoà`
2. Press `z` (remove tone)
3. **Expected**: `hoa`
4. Press `s` (acute accent)
5. **Expected**: `hoá`

#### Test 10: Display Attributes
1. Type: `h` `o` `a`
2. **Expected**: Text should have **dotted underline** during composition
3. Press **Space**
4. **Expected**: Underline disappears (composition finalized)

---

## 🐛 Troubleshooting

### Issue: DLL Registration Failed

**Symptoms**: `regsvr32` returns error

**Solutions**:
1. Run PowerShell as Administrator
2. Check DLL path is correct
3. Ensure no antivirus blocking
4. Check Windows Event Viewer for details

### Issue: buttre Not in Keyboard List

**Symptoms**: Can't find buttre in keyboard options

**Solutions**:
1. Verify DLL is registered: Check `HKEY_CLASSES_ROOT\CLSID\{E6B8A6C0-1234-5678-9ABC-DEF012345678}`
2. Restart Windows Explorer: `taskkill /f /im explorer.exe && start explorer.exe`
3. Reboot system
4. Check TSF category registration in registry

### Issue: No Composition Display

**Symptoms**: Typing shows nothing or direct characters

**Solutions**:
1. Verify buttre is active input method (check language bar)
2. Switch input methods: Windows + Space
3. Check logs: `%TEMP%\buttre_tsf.log`
4. Restart application (Notepad, etc.)

### Issue: Backspace Not Working

**Symptoms**: Backspace deletes entire word instead of intelligent backspace

**Solutions**:
1. This is expected behavior in some apps
2. Try in Notepad first (best TSF support)
3. Check if composition is active (should have underline)

---

## 📊 Expected Behavior

### Composition States

1. **No Composition**
   - Normal typing
   - No underline
   - Direct character output

2. **Active Composition**
   - Dotted underline under text
   - Real-time updates as you type
   - Backspace removes last modification

3. **Finalized**
   - Underline disappears
   - Text committed to document
   - Engine reset

### Key Mappings (Telex)

| Keys | Output | Description |
|------|--------|-------------|
| `aa` | `â` | Circumflex |
| `aw` | `ă` | Breve |
| `dd` | `đ` | D-stroke |
| `ee` | `ê` | Circumflex |
| `oo` | `ô` | Circumflex |
| `ow` | `ơ` | Horn |
| `uw` | `ư` | Horn |
| `w` (after vowel) | Add horn/breve | Modifier |
| `f` | Grave (`) | Tone |
| `s` | Acute (´) | Tone |
| `r` | Hook (?) | Tone |
| `x` | Tilde (~) | Tone |
| `j` | Dot below (.) | Tone |
| `z` | Remove tone | Undo |

---

## 🔍 Debug Information

### Log Location
Logs are written to: `%TEMP%\buttre_tsf.log`

View logs:
```powershell
Get-Content "$env:TEMP\buttre_tsf.log" -Tail 50 -Wait
```

### Registry Locations

**COM Registration**:
- `HKEY_CLASSES_ROOT\CLSID\{E6B8A6C0-1234-5678-9ABC-DEF012345678}`

**TSF Categories**:
- `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}`

**Language Profile**:
- `HKEY_CURRENT_USER\Software\Microsoft\CTF\Assemblies\0x0000042a` (Vietnamese)

---

## 📝 Testing Notes

### What's Implemented (Week 5)
- ✅ Vietnamese Telex input
- ✅ Real-time composition
- ✅ Intelligent backspace
- ✅ Uppercase/lowercase (Shift support)
- ✅ Auto-finalize on Space/Enter
- ✅ Display attributes (dotted underline)
- ✅ Tone marks (f, s, r, x, j, z)
- ✅ Diacritics (aa, aw, dd, ee, oo, ow, uw, w)

### What's NOT Implemented Yet
- ❌ VNI input method (code exists, no UI to switch)
- ❌ Candidate UI (not needed for Vietnamese)
- ❌ Han Nom support
- ❌ Settings UI
- ❌ Hotkey configuration
- ❌ Mode indicator

### Known Limitations
1. **App Compatibility**: Works best in Notepad, Word. May have issues in some apps.
2. **No Mode Indicator**: Can't see which input method is active (buttre vs English)
3. **No VNI Switch**: Can't switch to VNI mode (hardcoded to Telex)
4. **Numbers with Shift**: Shift+number not handled (e.g., Shift+1 = !)

---

## 🎯 Success Criteria

### Minimum Viable
- [ ] DLL registers successfully
- [ ] Appears in keyboard list
- [ ] Can switch to buttre input
- [ ] Basic Telex works (`hoaf` → `hoà`)
- [ ] Backspace works
- [ ] Space finalizes

### Full Feature
- [ ] All Telex combinations work
- [ ] Uppercase input works
- [ ] Tone changes work (z, then s)
- [ ] Multiple words work
- [ ] Works in Notepad
- [ ] Works in Word
- [ ] No crashes

---

## 🆘 Support

If you encounter issues:

1. **Check logs**: `%TEMP%\buttre_tsf.log`
2. **Check Event Viewer**: Windows Logs → Application
3. **Verify registration**: Check registry keys
4. **Reboot**: Sometimes TSF needs a reboot to pick up changes

---

## 🔄 Uninstallation

### Step 1: Remove from Language Settings
1. Settings → Language → Vietnamese → Options
2. Remove **buttre** keyboard

### Step 2: Unregister DLL
```powershell
# Run as Administrator
regsvr32 /u "$env:ProgramFiles\buttre\buttre_platform.dll"
```

### Step 3: Delete Files
```powershell
Remove-Item "$env:ProgramFiles\buttre" -Recurse -Force
```

---

**Build Version**: Week 5 - Vietnamese Engine + Polish  
**Test Date**: _____________  
**Tested By**: _____________  
**Result**: ⬜ Pass ⬜ Fail ⬜ Partial

**Notes**:
```


```
