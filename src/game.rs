use std::ffi::CString;
use std::os::raw::c_char;

use crate::internal;
use crate::internal::{
    HookCallback,
    HookWithCallerCallback,
    HookWithCallerCallbackInternal,
    DrawableCallback,
    TimeoutCallback
};

use crate::wrappers::Object;

use crate::wrappers::unreal::{
    CarWrapper,
    PlayerControllerWrapper,
    EngineTAWrapper,
    CameraWrapper,
    ServerWrapper,
};

use crate::wrappers::canvas::Canvas;


pub fn hook_event(name: &str, callback: Box<HookCallback>) {
    hook_event_internal(name, callback, false);
}

pub fn hook_event_post(name: &str, callback: Box<HookCallback>) {
    hook_event_internal(name, callback, true);
}

fn hook_event_internal(name: &str, callback: Box<HookCallback>, post: bool) {
    let callback = Box::new(callback);
    let cb_addr = Box::into_raw(callback) as usize;

    let bm =  internal::bakkesmod();
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
    let id =  internal::bakkesmod().id();
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

    let bm =  internal::bakkesmod();
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

    let bm =  internal::bakkesmod();
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

    let bm =  internal::bakkesmod();
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


#[allow(unused)]
extern "C" {
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