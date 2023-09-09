use crate::{
    prelude::{FltkError, FltkErrorKind},
    utils::FlString,
};
use fltk_sys::prefs::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

bitflags::bitflags! {
    /// Defines whether preferences are system-wide or not
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Root: i32 {
        /// Returned if storage could not be determined.
        const UNKNOWN_ROOT_TYPE = -1;
        /// Preferences are used system-wide. Deprecated, see SYSTEM_L
        #[doc(hidden)]
        const SYSTEM        =  0;
        /// Preferences apply only to the current user. Deprecated, see USER_L
        #[doc(hidden)]
        const USER          = 1;
        /// Returned if querying memory mapped preferences
        const MEMORY        = 2;
        /// Mask for the values above
        const ROOT_MASK     = 0x00FF;
        /// OR'd by FLTK to read and write core library preferences and options
        const CORE          = 0x0100;
        /// This flag should always be set, it makes sure that floating point
        const C_LOCALE      = 0x1000;
        /// values are written correctly independently of the current locale
        /// Preferences are used system-wide, locale independent
        const SYSTEM_L      = Root::SYSTEM.bits() | Root::C_LOCALE.bits();
        /// Preferences apply only to the current user, locale independent
        const USER_L        = Root::USER.bits() | Root::C_LOCALE.bits();
        /// Same as CORE | SYSTEM | C_LOCALE
        const CORE_SYSTEM_L = Root::CORE.bits() | Root::SYSTEM_L.bits();
        /// Same as CORE | USER | C_LOCALE
        const CORE_USER_L   = Root::CORE.bits() | Root::USER_L.bits();
        // /// Deprecated, same as CORE | SYSTEM. Use CORE_SYSTEM_L instead.
        // const CORE_SYSTEM   = Root::CORE.bits() | Root::SYSTEM.bits();
        // /// Deprecated, same as CORE | USER. Use CORE_USER_L instead.
        // const CORE_USER     = Root::CORE.bits() | Root::USER.bits();
    }
}

bitflags::bitflags! {
    /// Defines whether preferences are system-wide or not
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FileAccess: u32 {
        /// None
        const NONE = 0x0000;
        /// Set this if it is OK for applications to read user preference files.
        const USER_READ_OK = 0x0001;
        /// Set this if it is OK for applications to create and write user preference files.
        const USER_WRITE_OK = 0x0002;
        /// Set this if it is OK for applications to read, create, and write user preference files.
        const USER_OK = FileAccess::USER_READ_OK.bits() | FileAccess::USER_WRITE_OK.bits();
        /// Set this if it is OK for applications to read system wide preference files.
        const SYSTEM_READ_OK = 0x0004;
        /// Set this if it is OK for applications to create and write system wide preference files.
        const SYSTEM_WRITE_OK = 0x0008;
        /// Set this if it is OK for applications to read, create, and write system wide preference files.
        const SYSTEM_OK = FileAccess::SYSTEM_READ_OK.bits() | FileAccess::SYSTEM_WRITE_OK.bits();
        /// Set this if it is OK for applications to read, create, and write any kind of preference files.
        const APP_OK = FileAccess::SYSTEM_OK.bits() | FileAccess::USER_OK.bits();
        /// Set this if it is OK for FLTK to read preference files. USER_READ_OK and/or SYSTEM_READ_OK must also be set.
        const CORE_READ_OK = 0x0010;
        /// Set this if it is OK for FLTK to create or write preference files. USER_WRITE_OK and/or SYSTEM_WRITE_OK must also be set.
        const CORE_WRITE_OK = 0x0020;
        /// Set this if it is OK for FLTK to read, create, or write preference files.
        const CORE_OK = FileAccess::CORE_READ_OK.bits() | FileAccess::CORE_WRITE_OK.bits();
        /// Set this to allow FLTK and applications to read preference files.
        const ALL_READ_OK = FileAccess::USER_READ_OK.bits() | FileAccess::SYSTEM_READ_OK.bits() | FileAccess::CORE_READ_OK.bits();
        /// Set this to allow FLTK and applications to create and write preference files.
        const ALL_WRITE_OK = FileAccess::USER_WRITE_OK.bits() | FileAccess::SYSTEM_WRITE_OK.bits() | FileAccess::CORE_WRITE_OK.bits();
        /// Set this to give FLTK and applications permission to read, write, and create preference files.
        const ALL = FileAccess::ALL_READ_OK.bits() | FileAccess::ALL_WRITE_OK.bits();
    }
}

/// Provides methods to store user settings between application starts
#[derive(Debug)]
pub struct Preferences {
    inner: *mut Fl_Preferences,
}

impl Drop for Preferences {
    fn drop(&mut self) {
        unsafe { Fl_Preferences_delete(self.inner) }
    }
}

impl Clone for Preferences {
    fn clone(&self) -> Self {
        let inner = unsafe { Fl_Preferences_copy(self.inner) };
        assert!(!inner.is_null());
        Self { inner }
    }
}

impl Preferences {
    /// Set filesystem access
    pub fn set_file_access(flags: FileAccess) {
        unsafe { Fl_Preferences_set_file_access(flags.bits()) }
    }

    /// Get filesystem access flags
    pub fn file_access() -> FileAccess {
        let ret = unsafe { Fl_Preferences_file_access() };
        FileAccess::from_bits_retain(ret)
    }

    /// Create a new Preferences object
    pub fn new(root: Root, vendor: &str, application: &str) -> Option<Self> {
        unsafe {
            let vendor = CString::safe_new(vendor);
            let application = CString::safe_new(application);
            let inner = Fl_Preferences_new(root.bits(), vendor.as_ptr(), application.as_ptr());
            if inner.is_null() {
                None
            } else {
                Some(Self { inner })
            }
        }
    }

    /// Create a preferences object entry inside the group
    pub fn new_group(parent: &mut Preferences, group: &str) -> Option<Self> {
        unsafe {
            let group = CString::safe_new(group);
            let inner = Fl_Preferences_from_parent_group(parent.inner, group.as_ptr());
            if inner.is_null() {
                None
            } else {
                Some(Self { inner })
            }
        }
    }

    /// Get the prefs path
    pub fn path(&self) -> String {
        unsafe {
            let path = Fl_Preferences_path(self.inner);
            CStr::from_ptr(path).to_string_lossy().to_string()
        }
    }

    /// Get the filename
    pub fn filename(&self) -> Result<(std::path::PathBuf, Root), FltkError> {
        unsafe {
            let mut v: Vec<c_char> = vec![0; 250];
            let ret = Fl_Preferences_filename(self.inner, v.as_mut_ptr(), 250);
            if ret == -1 {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                Ok((
                    std::path::PathBuf::from(
                        CStr::from_ptr(v.as_ptr()).to_string_lossy().to_string(),
                    ),
                    Root::from_bits_retain(ret),
                ))
            }
        }
    }

    /// Get the userdata path
    pub fn get_userdata_path(&self) -> Result<std::path::PathBuf, FltkError> {
        unsafe {
            let mut v: Vec<c_char> = vec![0; 250];
            let ret = Fl_Preferences_filename(self.inner, v.as_mut_ptr(), 250);
            if ret == 0 {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                Ok(std::path::PathBuf::from(
                    CStr::from_ptr(v.as_ptr()).to_string_lossy().to_string(),
                ))
            }
        }
    }

    /// Set the value of an entry
    pub fn set_int(&mut self, entry: &str, val: i32) -> Result<(), FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let ret = Fl_Preferences_seti(self.inner, entry.as_ptr(), val);
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to set entry")))
            } else {
                Ok(())
            }
        }
    }

    /// Get the value of an entry
    pub fn get_int(&mut self, entry: &str) -> Result<i32, FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let mut i = 0;
            let ret = Fl_Preferences_geti(self.inner, entry.as_ptr(), &mut i, 0);
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to get entry")))
            } else {
                Ok(i)
            }
        }
    }

    /// Set the value of an entry
    pub fn set_float(&mut self, entry: &str, val: f32) -> Result<(), FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let ret = Fl_Preferences_setf(self.inner, entry.as_ptr(), val);
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to set entry")))
            } else {
                Ok(())
            }
        }
    }

    /// Set the value of an entry
    pub fn set_float_with_precision(
        &mut self,
        entry: &str,
        val: f32,
        precision: u16,
    ) -> Result<(), FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let ret = Fl_Preferences_setfp(self.inner, entry.as_ptr(), val, precision as i32);
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to set entry")))
            } else {
                Ok(())
            }
        }
    }

    /// Get the value of an entry
    pub fn get_float(&mut self, entry: &str) -> Result<f32, FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let mut i = 0.0;
            let ret = Fl_Preferences_getf(self.inner, entry.as_ptr(), &mut i, 0.0);
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to get entry")))
            } else {
                Ok(i)
            }
        }
    }

    /// Set the value of an entry
    pub fn set_double(&mut self, entry: &str, val: f64) -> Result<(), FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let ret = Fl_Preferences_setd(self.inner, entry.as_ptr(), val);
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to set entry")))
            } else {
                Ok(())
            }
        }
    }

    /// Set the value of an entry
    pub fn set_double_with_precision(
        &mut self,
        entry: &str,
        val: f64,
        precision: u16,
    ) -> Result<(), FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let ret = Fl_Preferences_setdp(self.inner, entry.as_ptr(), val, precision as i32);
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to set entry")))
            } else {
                Ok(())
            }
        }
    }

    /// Get the value of an entry
    pub fn get_double(&mut self, entry: &str) -> Result<f64, FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let mut i = 0.0;
            let ret = Fl_Preferences_getd(self.inner, entry.as_ptr(), &mut i, 0.0);
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to get entry")))
            } else {
                Ok(i)
            }
        }
    }

    /// Set the value of an entry
    pub fn set_str(&mut self, entry: &str, val: &str) -> Result<(), FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let val = CString::safe_new(val);
            let ret = Fl_Preferences_sets(self.inner, entry.as_ptr(), val.as_ptr());
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to set entry")))
            } else {
                Ok(())
            }
        }
    }

    /// Get the value of an entry
    pub fn get_str(&mut self, entry: &str) -> Result<String, FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let sz = Fl_Preferences_size(self.inner, entry.as_ptr());
            let mut val: Vec<c_char> = vec![0; (sz + 1) as usize];
            let ret = Fl_Preferences_gets(
                self.inner,
                entry.as_ptr(),
                val.as_mut_ptr(),
                "\0".as_ptr() as _,
                sz + 1,
            );
            if ret == 0 {
                Err(FltkError::Unknown(String::from("Failed to get entry")))
            } else {
                Ok(CStr::from_ptr(val.as_ptr()).to_string_lossy().to_string())
            }
        }
    }

    /// Return the pref's name
    pub fn name(&mut self) -> Option<String> {
        unsafe {
            let ptr = Fl_Preferences_name(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
            }
        }
    }

    /// Return the number of groups
    pub fn groups(&mut self) -> i32 {
        unsafe { Fl_Preferences_groups(self.inner) }
    }

    /// Check whether a group exists
    pub fn group_exists(&mut self, key: &str) -> bool {
        unsafe {
            let key = CString::safe_new(key);
            Fl_Preferences_group_exists(self.inner, key.as_ptr()) != 0
        }
    }

    /// Delete a group
    pub fn delete_group(&mut self, group: &str) -> Result<(), FltkError> {
        unsafe {
            let group = CString::safe_new(group);
            let ret = Fl_Preferences_delete_group(self.inner, group.as_ptr());
            if ret == 0 {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            } else {
                Ok(())
            }
        }
    }

    /// Delete all groups
    pub fn delete_all_groups(&mut self) -> Result<(), FltkError> {
        unsafe {
            let ret = Fl_Preferences_delete_all_groups(self.inner);
            if ret == 0 {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            } else {
                Ok(())
            }
        }
    }

    /// Return the number of entries
    pub fn entries(&mut self) -> i32 {
        unsafe { Fl_Preferences_entries(self.inner) }
    }

    /// Check whether an entry exists
    pub fn entry_exists(&mut self, key: &str) -> bool {
        unsafe {
            let key = CString::safe_new(key);
            Fl_Preferences_entry_exists(self.inner, key.as_ptr()) != 0
        }
    }

    /// Delete an entry
    pub fn delete_entry(&mut self, entry: &str) -> Result<(), FltkError> {
        unsafe {
            let entry = CString::safe_new(entry);
            let ret = Fl_Preferences_delete_entry(self.inner, entry.as_ptr());
            if ret == 0 {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            } else {
                Ok(())
            }
        }
    }

    /// Delete all entries
    pub fn delete_all_entries(&mut self) -> Result<(), FltkError> {
        unsafe {
            let ret = Fl_Preferences_delete_all_entries(self.inner);
            if ret == 0 {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            } else {
                Ok(())
            }
        }
    }

    /// Clear prefs
    pub fn clear(&mut self) -> Result<(), FltkError> {
        unsafe {
            let ret = Fl_Preferences_clear(self.inner);
            if ret == 0 {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            } else {
                Ok(())
            }
        }
    }
}
