use std::ffi::{CString, CStr};
use std::os::raw::c_char;

pub struct CVar(usize);

impl CVar {
    pub fn new(addr: usize) -> CVar {
        CVar(addr)
    }

    pub fn addr(&self) -> usize { self.0 }

    pub fn get_name(&self) -> String {
        let c_name = unsafe { CVar_GetName(self.0) };

        if c_name.is_null() { log_console!("name ptr is null!"); return String::new(); }
        let name = unsafe { CStr::from_ptr(c_name) };

        match name.to_str() {
            Ok(s) => String::from(s),
            Err(_) => { log_console!("cannot convert CStr to &str"); return String::new(); }
        }
    }

    pub fn get_int_value(&self) -> i32 {
        unsafe { CVar_GetIntValue(self.0) }
    }

    pub fn get_float_value(&self) -> f32 {
        unsafe { CVar_GetFloatValue(self.0) }
    }

    pub fn get_bool_value(&self) -> bool {
        unsafe { CVar_GetBoolValue(self.0) }
    }

    pub fn get_string_value(&self) -> String {
        let c_value = unsafe { CVar_GetStringValue(self.0) };
        if c_value.is_null() { log_console!("value ptr is null!"); return String::new(); }
        let value = unsafe { CStr::from_ptr(c_value) };

        match value.to_str() {
            Ok(s) => String::from(s),
            Err(_) => { log_console!("cannot convert CStr to &str"); return String::new(); }
        }
    }

    pub fn get_description(&self) -> String {
        let c_value = unsafe { CVar_GetDescription(self.0) };
        if c_value.is_null() { log_console!("value ptr is null!"); return String::new(); }
        let value = unsafe { CStr::from_ptr(c_value) };

        match value.to_str() {
            Ok(s) => String::from(s),
            Err(_) => { log_console!("cannot convert CStr to &str"); return String::new(); }
        }
    }

    pub fn set_string_value(&self, value: &str) {
        let c_value = CString::new(value).unwrap();
        let c_value: *const c_char = c_value.as_ptr();
        unsafe { CVar_SetStringValue(self.0, c_value); }
    }

    pub fn set_int_value(&self, value: i32) {
        unsafe { CVar_SetIntValue(self.0, value); }
    }

    pub fn set_float_value(&self, value: f32) {
        unsafe { CVar_SetFloatValue(self.0, value); }
    }
}

extern "C" {
    fn CVar_GetName(p_cvar: usize) -> *const c_char;
    fn CVar_GetIntValue(p_cvar: usize) -> i32;
    fn CVar_GetFloatValue(p_cvar: usize) -> f32;
    fn CVar_GetBoolValue(p_cvar: usize) -> bool;
    fn CVar_GetStringValue(p_cvar: usize) -> *const c_char;
    fn CVar_GetDescription(p_cvar: usize) -> *const c_char;
    fn CVar_notify(p_cvar: usize);
    fn CVar_SetStringValue(p_cvar: usize, value: *const c_char);
    fn CVar_SetIntValue(p_cvar: usize, value: i32);
    fn CVar_SetFloatValue(p_cvar: usize, value: f32);
}