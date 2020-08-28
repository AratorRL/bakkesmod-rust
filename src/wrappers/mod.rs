#![allow(unused)]

use std::fmt;
use std::ops;
use std::convert::From;
use std::marker::{PhantomData, Sized};

use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[macro_use] mod macros;

mod generated;
pub use generated::*;

pub trait UnrealPointer {
    fn from_ptr(addr: usize) -> Self;
}

pub trait Object: UnrealPointer {
    fn new(addr: usize) -> Self;
    fn try_new(addr: usize) -> Option<Self> where Self: Sized;
    fn addr(&self) -> usize;
}


pub struct ObjectWrapper(pub usize);
impl_object!(ObjectWrapper);


#[repr(C)]
pub struct RLArrayRaw {
    data: usize,
    count: u32,
    max: u32
}

impl RLArrayRaw {
    pub fn new() -> RLArrayRaw {
        RLArrayRaw { data: 0, count: 0, max: 0 }
    }
}

#[repr(C)]
pub struct RLArray<T: UnrealPointer> {
    pub data: *mut usize,
    count: u32,
    max: u32,
    phantom: PhantomData<T>
}

impl<T: UnrealPointer> RLArray<T> {
    pub fn from_raw(raw: RLArrayRaw) -> RLArray<T> {
        RLArray { data: raw.data as *mut usize, count: 0, max: 0, phantom: PhantomData }
    }

    pub fn to_raw(&self) -> RLArrayRaw {
        RLArrayRaw { data: self.data as usize, count: 0, max: 0 }
    }

    pub fn len(&self) -> isize {
        self.count as isize
    }

    pub fn get(&self, index: isize) -> T {
        unsafe { 
            let ptr = self.data.offset(index);
            T::from_ptr(*ptr)
        }
    }
}

#[repr(C)]
pub struct RLString {
    data: usize,
    count: u32,
    max: u32
}

impl RLString {
    pub fn new() -> RLString {
        RLString { data: 0, count: 0, max: 0 }
    }
}


#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => (
        $crate::wrappers::Vector::from($x as f32, $y as f32, $z as f32)
    );
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }
}

impl ops::Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => (
        $crate::wrappers::Vector2::new($x as i32, $y as i32)
    );
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    pub fn new() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn from(x: f32, y: f32, z: f32) -> Vector {
        Vector {x, y, z}
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rotator {
    pitch: i32,
    yaw: i32,
    roll: i32
}

impl Rotator {
    pub fn new() -> Rotator {
        Rotator { pitch: 0, yaw: 0, roll:0 }
    }

    pub fn from(pitch: i32, yaw: i32, roll:i32) -> Rotator {
        Rotator { pitch, yaw, roll }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quat {
    pub fn new() -> Quat {
        Quat { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn from(x: f32, y: f32, z: f32, w: f32) -> Quat {
        Quat { x, y, z, w }
    }
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn new() -> Color {
        Color { r: 0, g: 0, b: 0, a:0 }
    }

    pub fn from(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {r, g, b, a}
    }
}



#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReplicatedRBState {
    quat: Quat,
    loc: Vector,
    lin_vel: Vector,
    ang_vel: Vector,
    time: f32,
    b_sleeping: bool,
    b_new_data: bool
}
impl_unreal_pointer_struct!(ReplicatedRBState);

impl ReplicatedRBState {
    pub fn new() -> Self {
        Self {
            quat: Quat::new(),
            loc: Vector::new(),
            lin_vel: Vector::new(),
            ang_vel: Vector::new(),
            time: 0.0,
            b_sleeping: false,
            b_new_data: false,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VehicleInputs {
    throttle: f32,
    steer: f32,
    pitch: f32,
    yaw: f32,
    roll: f32,
    dodge_forward: f32,
    dodge_strafe: f32,
    jump: bool,
    activate_boost: bool,
    holding_boost: bool,
    handbrake: bool,
    jumped: bool,
}
impl_unreal_pointer_struct!(VehicleInputs);

impl VehicleInputs {
    pub fn new() -> Self {
        Self {
            throttle: 0.0,
            steer: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
            dodge_forward: 0.0,
            dodge_strafe: 0.0,
            jump: false,
            activate_boost: false,
            holding_boost: false,
            handbrake: false,
            jumped: false,
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct UniqueNetId(usize);
impl_unreal_pointer_struct!(UniqueNetId);
struct_default_new!(UniqueNetId);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WheelContactData(usize);
impl_unreal_pointer_struct!(WheelContactData);
struct_default_new!(WheelContactData);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LinearColor(usize);
impl_unreal_pointer_struct!(LinearColor);
struct_default_new!(LinearColor);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TViewTarget(usize);
impl_unreal_pointer_struct!(TViewTarget);
struct_default_new!(TViewTarget);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WorldContactData(usize);
impl_unreal_pointer_struct!(WorldContactData);
struct_default_new!(WorldContactData);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Double(usize);
impl_unreal_pointer_struct!(Double);
struct_default_new!(Double);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sample(usize);
impl_unreal_pointer_struct!(Sample);
struct_default_new!(Sample);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StickyForceData(usize);
impl_unreal_pointer_struct!(StickyForceData);
struct_default_new!(StickyForceData);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProfileCameraSettings(usize);
impl_unreal_pointer_struct!(ProfileCameraSettings);
struct_default_new!(ProfileCameraSettings);




pub struct Canvas(usize);

impl Canvas {
    pub fn new(addr: usize) -> Canvas {
        Canvas(addr)
    }

    pub fn draw_line(&self, start: Vector2, end: Vector2) {
        unsafe { Canvas_DrawLine(self.0, start, end); }
    }
}

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
    fn Canvas_DrawLine(p_canvas: usize, start: Vector2, end: Vector2);

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