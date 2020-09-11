use std::sync::Mutex;
use std::error::Error;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use crate::log_console;
use crate::errors::BakkesModError;

use crate::wrappers::{
    Object,
    Canvas,
    CVar,
    CarWrapper,
    PlayerControllerWrapper,
    EngineTAWrapper,
    CameraWrapper,
    ServerWrapper,
};

trait BakkesMod {
    fn id(&self) -> u64;
    fn add_notifier_callback(&self, addr: usize);
    fn add_on_value_changed_callback(&self, addr: usize);
    fn add_hook_callback(&self, addr: usize);
    fn add_hook_caller_callback(&self, addr: usize);
    fn add_drawable_callback(&self, addr: usize);
    fn add_timeout_callback(&self, addr: usize);
    fn drop_callbacks(&self);
}

static mut BAKKESMOD: &dyn BakkesMod = &Dummy;

pub fn bakkesmod_init(id: u64) {
    let bm_wrapper = Box::new(BakkesModWrapper {
        id,
        notifier_callbacks: Mutex::new(Vec::new()),
        on_value_changed_callbacks: Mutex::new(Vec::new()),
        hook_callbacks: Mutex::new(Vec::new()),
        hook_caller_callbacks: Mutex::new(Vec::new()),
        drawable_callbacks: Mutex::new(Vec::new()),
        timeout_callbacks: Mutex::new(Vec::new()),
    });
    unsafe { BAKKESMOD = Box::leak(bm_wrapper); }
}

pub fn bakkesmod_exit() {
    bakkesmod().drop_callbacks();
}

pub fn get_id() -> u64 {
    bakkesmod().id()
}

fn bakkesmod() -> &'static dyn BakkesMod {
    unsafe { BAKKESMOD }
}

type NotifierCallback = dyn FnMut(Vec<String>);
type OnValueChangedCallback = dyn FnMut(String, CVar);
type HookCallback = dyn FnMut();
type HookWithCallerCallback<T> = dyn FnMut(Box<T>);
type HookWithCallerCallbackInternal = dyn FnMut(usize, usize);
type DrawableCallback = dyn FnMut(Canvas);
type TimeoutCallback = dyn FnMut();


pub fn register_notifier(name: &str, callback: Box<NotifierCallback>) {
    let callback = Box::new(callback);
    let cb_addr = Box::into_raw(callback) as usize;

    let bm = bakkesmod();
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
    let bm = bakkesmod();
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
    let id = bakkesmod().id();
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
    let bm = bakkesmod();
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

    let bm = bakkesmod();
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

pub fn hook_event(name: &str, callback: Box<HookCallback>) {
    hook_event_internal(name, callback, false);
}

pub fn hook_event_post(name: &str, callback: Box<HookCallback>) {
    hook_event_internal(name, callback, true);
}

fn hook_event_internal(name: &str, callback: Box<HookCallback>, post: bool) {
    let callback = Box::new(callback);
    let cb_addr = Box::into_raw(callback) as usize;

    let bm = bakkesmod();
    bm.add_hook_callback(cb_addr);

    let id = bm.id();
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();
    let c_callback = hook_callback as usize;

    if post {
        unsafe { HookEventPost(id, cb_addr, c_name, c_callback); }
    } else {
        unsafe { HookEvent(id, cb_addr, c_name, c_callback); }
    }
}

extern "C" fn hook_callback(addr: usize) {
    let mut closure = unsafe { Box::from_raw(addr as *mut Box<HookCallback>) };
    closure();
    let _ = Box::into_raw(closure);
}

pub fn unhook_event(name: &str) {
    let id = bakkesmod().id();
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();
    unsafe { UnhookEvent(id, c_name); }
}

pub fn hook_event_with_caller<T: Object + 'static>(name: &str, mut callback: Box<HookWithCallerCallback<T>>) {
    let wrapper_callback = Box::new(move |caller: usize, _params: usize| {
        let obj_wrapper = T::new(caller);
        callback(Box::new(obj_wrapper));
    });

    hook_event_with_caller_internal(name, wrapper_callback, false);
}

pub fn hook_event_with_caller_post<T: Object + 'static>(name: &str, mut callback: Box<HookWithCallerCallback<T>>) {
    let wrapper_callback = Box::new(move |caller: usize, _params: usize| {
        let obj_wrapper = T::new(caller);
        callback(Box::new(obj_wrapper));
    });

    hook_event_with_caller_internal(name, wrapper_callback, true);
}

fn hook_event_with_caller_internal(name: &str, callback: Box<HookWithCallerCallbackInternal>, post: bool) {
    let callback = Box::new(callback);
    let cb_addr = Box::into_raw(callback) as usize;

    let bm = bakkesmod();
    bm.add_hook_caller_callback(cb_addr);

    let id = bm.id();
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();
    let c_callback = hook_with_caller_callback as usize;

    if post {
        unsafe { HookEventWithCallerPost(id, cb_addr, c_name, c_callback); }
    } else {
        unsafe { HookEventWithCaller(id, cb_addr, c_name, c_callback); }
    }
}

extern "C" fn hook_with_caller_callback(addr: usize, caller: usize, params: usize) {
    let mut closure = unsafe { Box::from_raw(addr as *mut Box<HookWithCallerCallbackInternal>) };
    closure(caller, params);
    let _ = Box::into_raw(closure);
}

pub fn register_drawable(callback: Box<DrawableCallback>) {
    let callback = Box::new(callback);
    let cb_addr = Box::into_raw(callback) as usize;

    let bm = bakkesmod();
    bm.add_drawable_callback(cb_addr);

    let id = bm.id();
    let c_callback = drawable_callback as usize;

    unsafe { RegisterDrawable(id, cb_addr, c_callback); }
}

extern "C" fn drawable_callback(addr: usize, canvas: usize) {
    let mut closure = unsafe { Box::from_raw(addr as *mut Box<DrawableCallback>) };
    let canvas = Canvas::new(canvas);
    closure(canvas);
    let _ = Box::into_raw(closure);
}

pub fn set_timeout(callback: Box<TimeoutCallback>, time: f32) {
    let callback = Box::new(callback);
    let cb_addr = Box::into_raw(callback) as usize;

    let bm = bakkesmod();
    bm.add_timeout_callback(cb_addr);

    let id = bm.id();
    let c_callback = timeout_callback as usize;

    unsafe { SetTimeout(id, cb_addr, c_callback, time); }
}

pub fn execute(callback: Box<TimeoutCallback>) {
    set_timeout(callback, 0.0);
}

extern "C" fn timeout_callback(addr: usize) {
    let mut closure = unsafe { Box::from_raw(addr as *mut Box<TimeoutCallback>) };
    closure();
    let _ = Box::into_raw(closure);
}

pub fn log(text: &str) {
    let id = bakkesmod().id();
    let c_text = CString::new(text).unwrap();
    let c_text: *const c_char = c_text.as_ptr();
    unsafe { LogConsole(id, c_text); }
}


pub fn is_in_game() -> bool {
    unsafe { IsInGame() }
}

pub fn is_in_online_game() -> bool {
    unsafe { IsInOnlineGame() }
}

pub fn is_in_freeplay() -> bool {
    unsafe { IsInFreeplay() }
}

pub fn is_in_custom_training() -> bool {
    unsafe { IsInCustomTraining() }
}

pub fn is_spectating_in_online_game() -> bool {
    unsafe { IsSpectatingInOnlineGame() }
}

pub fn is_paused() -> bool {
    unsafe { IsPaused() }
}
    
pub fn get_online_game() -> Option<ServerWrapper> {
    unsafe { ServerWrapper::try_new(GetOnlineGame()) }
}

pub fn get_game_event_as_server() -> Option<ServerWrapper> {
    unsafe { ServerWrapper::try_new(GetGameEventAsServer()) }
}

pub fn get_local_car() -> Option<CarWrapper> {
    unsafe { CarWrapper::try_new(GetLocalCar()) }
}

pub fn get_camera() -> Option<CameraWrapper> {
    unsafe { CameraWrapper::try_new(GetCamera()) }
}

pub fn get_engine() -> Option<EngineTAWrapper> {
    unsafe { EngineTAWrapper::try_new(GetEngine()) }
}

pub fn get_player_controller() -> Option<PlayerControllerWrapper> {
    unsafe { PlayerControllerWrapper::try_new(GetPlayerController()) }
}


struct Dummy;

impl BakkesMod for Dummy {
    fn id(&self) -> u64 { 0 }
    fn add_notifier_callback(&self, _: usize) {}
    fn add_on_value_changed_callback(&self, _: usize) {}
    fn add_hook_callback(&self, _: usize) {}
    fn add_hook_caller_callback(&self, _: usize) {}
    fn add_drawable_callback(&self, _: usize) {}
    fn add_timeout_callback(&self, _: usize) {}
    fn drop_callbacks(&self) {}
}

struct BakkesModWrapper {
    pub id: u64,
    pub notifier_callbacks: Mutex<Vec<usize>>,
    pub on_value_changed_callbacks: Mutex<Vec<usize>>,
    pub hook_callbacks: Mutex<Vec<usize>>,
    pub hook_caller_callbacks: Mutex<Vec<usize>>,
    pub drawable_callbacks: Mutex<Vec<usize>>,
    pub timeout_callbacks: Mutex<Vec<usize>>,
}

impl BakkesMod for BakkesModWrapper {
    fn id(&self) -> u64 {
        self.id
    }

    fn add_notifier_callback(&self, addr: usize) {
        let mut callbacks = self.notifier_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_on_value_changed_callback(&self, addr: usize) {
        let mut callbacks = self.on_value_changed_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_hook_callback(&self, addr: usize) {
        let mut callbacks = self.hook_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_hook_caller_callback(&self, addr: usize) {
        let mut callbacks = self.hook_caller_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_drawable_callback(&self, addr: usize) {
        let mut callbacks = self.drawable_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_timeout_callback(&self, addr: usize) {
        let mut callbacks = self.timeout_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn drop_callbacks(&self) {
        let notifiers = self.notifier_callbacks.lock().unwrap();
        for addr in notifiers.iter() {
            let _ = unsafe { Box::from_raw(*addr as *mut Box<NotifierCallback>) };
        }

        let on_value_changed = self.on_value_changed_callbacks.lock().unwrap();
        for addr in on_value_changed.iter() {
            let _ = unsafe { Box::from_raw(*addr as *mut Box<OnValueChangedCallback>) };
        }

        let hooks = self.hook_callbacks.lock().unwrap();
        for addr in hooks.iter() {
            let _ = unsafe { Box::from_raw(*addr as *mut Box<HookCallback>) };
        }

        let hooks_caller = self.hook_caller_callbacks.lock().unwrap();
        for addr in hooks_caller.iter() {
            let _ = unsafe { Box::from_raw(*addr as *mut Box<HookWithCallerCallbackInternal>) };
        }

        let drawables = self.drawable_callbacks.lock().unwrap();
        for addr in drawables.iter() {
            let _ = unsafe { Box::from_raw(*addr as *mut Box<DrawableCallback>) };
        }

        let timeouts = self.timeout_callbacks.lock().unwrap();
        for addr in timeouts.iter() {
            let _ = unsafe { Box::from_raw(*addr as *mut Box<TimeoutCallback>) };
        }
    }
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

    fn HookEvent(id: u64, user_data: usize, event_name: *const c_char, callback: usize);
    fn HookEventPost(id: u64, user_data: usize, event_name: *const c_char, callback: usize);
    fn UnhookEvent(id: u64, event_name: *const c_char);
    fn HookEventWithCaller(id: u64, user_data: usize, event_name: *const c_char, callback: usize);
    fn HookEventWithCallerPost(id: u64, user_data: usize, event_name: *const c_char, callback: usize);
    fn UnhookEventPost(id: u64, event_name: *const c_char);
    fn RegisterDrawable(id: u64, user_data: usize, callback: usize);
    fn UnregisterDrawables(id: u64);
    fn SetTimeout(id: u64, user_data: usize, callback: usize, time: f32);
    fn Execute(id: u64, user_data: usize, callback: usize);

    fn CVar_AddOnValueChanged(p_cvar: usize, id: u64, user_data: usize, callback: usize);
    fn CVar_RemoveOnValueChanged(p_cvar: usize, id: u64);

    fn IsInGame() -> bool;
    fn IsInOnlineGame() -> bool;
    fn IsInFreeplay() -> bool;
    fn IsInReplay() -> bool;
    fn IsInCustomTraining() -> bool;
    fn IsSpectatingInOnlineGame() -> bool;
    fn IsPaused() -> bool;
    
    fn GetOnlineGame() -> usize;
    fn GetGameEventAsServer() -> usize;
    fn GetLocalCar() -> usize;
    fn GetCamera() -> usize;
    fn GetEngine() -> usize;
    fn GetPlayerController() -> usize;
}