use std::error::Error;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use crate::log_console;
use crate::errors::BakkesModError;

use crate::internal;
use crate::internal::{NotifierCallback, OnValueChangedCallback};
use crate::wrappers::cvar::CVar;



pub fn console_print(text: &str) {
    let id = internal::bakkesmod().id();
    let c_text = CString::new(text).unwrap();
    let c_text: *const c_char = c_text.as_ptr();
    unsafe { LogConsole(id, c_text); }
}


pub fn register_notifier(name: &str, callback: Box<NotifierCallback>) {
    let callback = Box::new(callback);
    let cb_addr = Box::into_raw(callback) as usize;

    let bm = internal::bakkesmod();
    bm.add_notifier_callback(cb_addr);

    let id = bm.id();
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();
    let c_desc= CString::new("").unwrap();
    let c_desc: *const c_char = c_desc.as_ptr();
    let c_callback = notifier_callback as usize;

    unsafe { RegisterNotifier(id, cb_addr, c_name, c_callback, c_desc, 0); }
}

extern "C" fn notifier_callback(addr: usize, params: *const *const c_char, len: u32) {
    if len <= 0 { log_console!("callback called but len is <= 0 !"); return; }

    let params_ptr_ptr = params as *const *const c_char;
    if params_ptr_ptr.is_null() { log_console!("ptr to params ptr is null!"); return; }

    let mut rust_params: Vec<String> = Vec::new();

    for i in 0..len {
        let params_ptr = unsafe { *(params_ptr_ptr.offset(i as isize)) as *const c_char };
        if params_ptr.is_null() { log_console!("params ptr is null!"); return; }

        let params_c_str = unsafe { CStr::from_ptr(params_ptr) };
        match params_c_str.to_str() {
            Ok(s) => rust_params.push(String::from(s)),
            Err(_) => { log_console!("params null"); return; }
        };
    }

    let mut closure = unsafe { Box::from_raw(addr as *mut Box<NotifierCallback>) };
    closure(rust_params);
    let _ = Box::into_raw(closure);
}

pub fn remove_notifier(name: &str) -> Result<(), Box<dyn Error>> {
    let bm = internal::bakkesmod();
    let id = bm.id();
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();

    unsafe { 
        if RemoveNotifier(id, c_name) {
            Ok(())
        } else {
            bakkesmod_error!("Could not remove notifier {}", name)
        }
    }
}

pub fn register_cvar(name: &str) -> CVar {
    let id = internal::bakkesmod().id();
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();
    let c_defval = CString::new("").unwrap();
    let c_defval: *const c_char = c_defval.as_ptr();
    let c_desc= CString::new("").unwrap();
    let c_desc: *const c_char = c_desc.as_ptr();
    let cvar = unsafe {
        RegisterCVar(id, c_name, c_defval, c_desc, true, false, 0.0, false, 0.0, false)
    };
    CVar::new(cvar)
}

pub fn remove_cvar(name: &str) -> Result<(), Box<dyn Error>> {
    let bm = internal::bakkesmod();
    let id = bm.id();
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();

    unsafe { 
        if RemoveCvar(id, c_name) {
            Ok(())
        } else {
            bakkesmod_error!("Could not remove cvar {}", name)
        }
    }
}

pub fn get_cvar(name: &str) -> Option<CVar> {
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();
    let cvar = unsafe {
        GetCVar(c_name)
    };

    match cvar {
        0 => None,
        addr => Some(CVar::new(addr))
    }
}

pub fn execute_command(command: &str, log: bool) {
    let c_string = CString::new(command).unwrap();
    let c_string: *const c_char = c_string.as_ptr();
    unsafe { ExecuteCommand(c_string, log); }
}

pub fn get_bind_string_for_key(key: &str) -> String {
    let c_string = CString::new(key).unwrap();
    let c_string: *const c_char = c_string.as_ptr();

    let result: *const c_char = 0 as *const c_char;
    let result_ptr: *const *const c_char = &result as *const *const c_char;

    unsafe { 
        GetBindStringForKey(c_string, result_ptr);
        let result = *result_ptr;
        let c_result = CStr::from_ptr(result);
        match c_result.to_str() {
            Ok(s) => String::from(s),
            Err(_) => String::new()
        }
    }
}

pub fn set_bind(key: &str, command: &str) {
    let c_key = CString::new(key).unwrap();
    let c_key: *const c_char = c_key.as_ptr();
    let c_command = CString::new(command).unwrap();
    let c_command: *const c_char = c_command.as_ptr();

    unsafe { SetBind(c_key, c_command); }
}

pub fn get_alias(alias: &str) -> String {
    let c_string = CString::new(alias).unwrap();
    let c_string: *const c_char = c_string.as_ptr();

    let result: *const c_char = 0 as *const c_char;
    let result_ptr: *const *const c_char = &result as *const *const c_char;

    unsafe { 
        GetAlias(c_string, result_ptr);
        let result = *result_ptr;
        let c_result = CStr::from_ptr(result);
        match c_result.to_str() {
            Ok(s) => String::from(s),
            Err(_) => String::new()
        }
    }
}

pub fn set_alias(key: &str, script: &str) {
    let c_key = CString::new(key).unwrap();
    let c_key: *const c_char = c_key.as_ptr();
    let c_script = CString::new(script).unwrap();
    let c_script: *const c_char = c_script.as_ptr();

    unsafe { SetBind(c_key, c_script); }
}

pub fn backup_cfg(path: &str) {
    let c_path = CString::new(path).unwrap();
    let c_path: *const c_char = c_path.as_ptr();
    unsafe { BackupCfg(c_path); }
}

pub fn backup_binds(path: &str) {
    let c_path = CString::new(path).unwrap();
    let c_path: *const c_char = c_path.as_ptr();
    unsafe { BackupBinds(c_path); }
}

pub fn load_cfg(path: &str) {
    let c_path = CString::new(path).unwrap();
    let c_path: *const c_char = c_path.as_ptr();
    unsafe { LoadCfg(c_path); }
}

pub fn add_on_value_changed(cvar: &CVar, callback: Box<OnValueChangedCallback>) {
    let callback = Box::new(callback);
    let cb_addr = Box::into_raw(callback) as usize;

    let bm = internal::bakkesmod();
    bm.add_on_value_changed_callback(cb_addr);
    let id = bm.id();

    let c_callback = on_value_changed_callback as usize;

    unsafe { CVar_AddOnValueChanged(cvar.addr(), id, cb_addr, c_callback); }
}

extern "C" fn on_value_changed_callback(addr: usize, _old: *const c_char, cvar: usize) {
    let mut closure = unsafe { Box::from_raw(addr as *mut Box<OnValueChangedCallback>) };

    // TODO: use old string value
    closure(String::new(), CVar::new(cvar));
    let _ = Box::into_raw(closure);
}


#[allow(unused)]
extern "C" {
    fn LogConsole(id: u64, text: *const c_char);
    fn RegisterNotifier(id: u64, user_data: usize, cvar: *const c_char, callback: usize, description: *const c_char, permissions: u8);
    fn RemoveNotifier(id: u64, cvar: *const c_char) -> bool;
    fn RegisterCVar(id: u64, cvar: *const c_char, default_value: *const c_char, desc: *const c_char, searchable: bool, has_min: bool, min: f32, has_max: bool, max: f32, save_to_cfg: bool) -> usize;
    fn RemoveCvar(id: u64, cvar: *const c_char) -> bool;
    fn GetCVar(name: *const c_char) -> usize;

    fn ExecuteCommand(command: *const c_char, log: bool);
    fn GetBindStringForKey(key: *const c_char, result: *const *const c_char);
    fn SetBind(key: *const c_char, command: *const c_char);
    fn GetAlias(alias: *const c_char, result: *const *const c_char);
    fn SetAlias(key: *const c_char, script: *const c_char);
    fn BackupCfg(path: *const c_char);
    fn BackupBinds(path: *const c_char);
    fn LoadCfg(path: *const c_char);

    fn CVar_AddOnValueChanged(p_cvar: usize, id: u64, user_data: usize, callback: usize);
    fn CVar_RemoveOnValueChanged(p_cvar: usize, id: u64);
}