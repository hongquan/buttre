// SPDX-License-Identifier: MPL-2.0
// Display Attributes for buttre TSF
//
// **Tests**: Integration tests for this module are located in `crates/buttre-platform/tests/platform_windows_tsf_tests.rs`.
//
// Defines visual styles for composition text (underline, colors, etc.)

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::UI::TextServices::*;

// Constants for System Colors
const COLOR_WINDOW: i32 = 5;
const COLOR_WINDOWTEXT: i32 = 8;

// GUID for Input Attribute (Dashed or Dot underline)
// {E6B8A6C2-1234-5678-9ABC-DEF012345678}
pub const GUID_DISPLAY_ATTRIBUTE_INPUT: GUID = GUID::from_u128(0xE6B8A6C2_1234_5678_9ABC_DEF012345678);

// GUID for Converted/Selected Attribute (Solid underline / Thick)
// {E6B8A6C3-1234-5678-9ABC-DEF012345678}
pub const GUID_DISPLAY_ATTRIBUTE_CONVERTED: GUID = GUID::from_u128(0xE6B8A6C3_1234_5678_9ABC_DEF012345678);

/// Info for a specific display attribute
#[implement(ITfDisplayAttributeInfo)]
pub struct DisplayAttributeInfo {
    guid: GUID,
    description: HSTRING,
    info: TF_DISPLAYATTRIBUTE,
}

impl DisplayAttributeInfo {
    pub fn new(guid: GUID, description: &str, info: TF_DISPLAYATTRIBUTE) -> Self {
        Self {
            guid,
            description: description.into(),
            info,
        }
    }

    /// Create standard "Input" attribute (Dotted underline)
    pub fn create_input() -> Self {
        let info = TF_DISPLAYATTRIBUTE {
            crText: TF_DA_COLOR { 
                r#type: TF_CT_SYSCOLOR, 
                Anonymous: TF_DA_COLOR_0 { nIndex: COLOR_WINDOWTEXT }
            },
            crBk: TF_DA_COLOR { 
                r#type: TF_CT_SYSCOLOR, 
                Anonymous: TF_DA_COLOR_0 { nIndex: COLOR_WINDOW }
            },
            lsStyle: TF_LS_DOT,
            fBoldLine: BOOL(0),
            crLine: TF_DA_COLOR { 
                r#type: TF_CT_SYSCOLOR, 
                Anonymous: TF_DA_COLOR_0 { nIndex: COLOR_WINDOWTEXT }
            },
            bAttr: TF_ATTR_INPUT,
        };
        
        Self::new(GUID_DISPLAY_ATTRIBUTE_INPUT, "buttre Input", info)
    }

    /// Create standard "Converted" attribute (Solid underline)
    pub fn create_converted() -> Self {
         let info = TF_DISPLAYATTRIBUTE {
            crText: TF_DA_COLOR { 
                r#type: TF_CT_SYSCOLOR, 
                Anonymous: TF_DA_COLOR_0 { nIndex: COLOR_WINDOWTEXT }
            },
            crBk: TF_DA_COLOR { 
                r#type: TF_CT_SYSCOLOR, 
                Anonymous: TF_DA_COLOR_0 { nIndex: COLOR_WINDOW }
            },
            lsStyle: TF_LS_SOLID,
            fBoldLine: BOOL(1), // Thick line
            crLine: TF_DA_COLOR { 
                r#type: TF_CT_SYSCOLOR, 
                Anonymous: TF_DA_COLOR_0 { nIndex: COLOR_WINDOWTEXT }
            },
            bAttr: TF_ATTR_TARGET_CONVERTED,
        };
        
        Self::new(GUID_DISPLAY_ATTRIBUTE_CONVERTED, "buttre Converted", info)
    }
}

impl ITfDisplayAttributeInfo_Impl for DisplayAttributeInfo_Impl {
    fn GetGUID(&self) -> Result<GUID> {
        Ok(self.this.guid)
    }

    fn GetDescription(&self) -> Result<BSTR> {
        Ok(BSTR::from(self.this.description.to_string()))
    }

    fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> Result<()> {
        // SAFETY:
        // 1. pda pointer is provided by TSF framework - valid during call
        // 2. We check for null before dereferencing
        // 3. TF_DISPLAYATTRIBUTE is a POD struct - safe to copy
        // 4. self.this.info is a valid TF_DISPLAYATTRIBUTE initialized in create_*
        unsafe {
            if !pda.is_null() {
                *pda = self.this.info;
            }
        }
        Ok(())
    }

    fn SetAttributeInfo(&self, _pda: *const TF_DISPLAYATTRIBUTE) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn Reset(&self) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};

/// Enumerator for Display Attributes
#[implement(IEnumTfDisplayAttributeInfo)]
pub struct DisplayAttributeEnum {
    index: AtomicUsize,
}

impl Default for DisplayAttributeEnum {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayAttributeEnum {
    pub fn new() -> Self {
        Self {
            index: AtomicUsize::new(0),
        }
    }
}

impl IEnumTfDisplayAttributeInfo_Impl for DisplayAttributeEnum_Impl {
    fn Clone(&self) -> Result<IEnumTfDisplayAttributeInfo> {
        let new_enum = DisplayAttributeEnum::new();
        new_enum.index.store(self.this.index.load(Ordering::SeqCst), Ordering::SeqCst);
        Ok(new_enum.into())
    }

    fn Next(
        &self,
        count: u32,
        items: *mut Option<ITfDisplayAttributeInfo>,
        fetched: *mut u32,
    ) -> Result<()> {
        let count = count as usize;
        let mut fetched_count = 0;
        let current_index = self.this.index.load(Ordering::SeqCst);

        // SAFETY:
        // 1. items and fetched pointers are provided by COM caller - valid during call
        // 2. We check for null before dereferencing
        // 3. items.add(i) computes valid pointer offset within caller's array
        // 4. We only iterate up to count, which caller guarantees array size for
        // 5. DisplayAttributeInfo::create_* creates valid COM objects
        // 6. Writing Some(attr) to items[i] is safe - proper COM interface
        // 7. fetched is written with actual count - caller expects u32
        unsafe {
            if !items.is_null() {
                // We have 2 attributes: Input [0] and Converted [1]
                for i in 0..count {
                    let idx = current_index + i;
                    if idx >= 2 {
                        break;
                    }

                    let attr = if idx == 0 {
                        DisplayAttributeInfo::create_input().into()
                    } else {
                        DisplayAttributeInfo::create_converted().into()
                    };

                    *items.add(i) = Some(attr);
                    fetched_count += 1;
                }
            }

            if !fetched.is_null() {
                *fetched = fetched_count as u32;
            }
        }

        self.this.index.fetch_add(fetched_count, Ordering::SeqCst);

        Ok(())
    }

    fn Reset(&self) -> Result<()> {
        self.this.index.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn Skip(&self, count: u32) -> Result<()> {
        self.this.index.fetch_add(count as usize, Ordering::SeqCst);
        Ok(())
    }
}

