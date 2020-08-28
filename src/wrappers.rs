#![allow(unused)]

use std::fmt;
use std::ops;
use std::convert::From;
use std::marker::{PhantomData, Sized};

use std::ffi::{CString, CStr};
use std::os::raw::c_char;

macro_rules! impl_object {
    ($name: ident) => {
        impl Object for $name {
            fn new(addr: usize) -> Self { Self(addr) }
            fn try_new(addr: usize) -> Option<Self> { 
                match addr {
                    0 => None,
                    _ => Some(Self(addr))
                }
            }
            fn addr(&self) -> usize { self.0 }
        }

        impl UnrealPointer for $name {
            fn from(addr: usize) -> Self { Self(addr) }
        }
    }
}

pub trait Object: UnrealPointer {
    fn new(addr: usize) -> Self;
    fn try_new(addr: usize) -> Option<Self> where Self: Sized;
    fn addr(&self) -> usize;
}

pub struct ObjectWrapper(pub usize);
impl_object!(ObjectWrapper);

pub trait UnrealPointer {
    fn from(addr: usize) -> Self;
}

macro_rules! impl_struct {
    ($name: ident) => {
        impl UnrealPointer for $name {
            fn from(addr: usize) -> Self {
                unsafe { *(addr as *const Self) }
            }
        }

        impl $name {
            pub fn new() -> Self { Self }
        }
    }
}

// pub trait Actor: Object {
//     fn get_location(&self) -> Vector {
//         unsafe { Actor_GetLocation(self.addr()) }
//     }

//     fn set_location(&self, new_loc: Vector) {
//         unsafe { Actor_SetLocation(self.addr(), new_loc); }
//     }
// }

// pub trait Car: Actor {
//     fn demolish(&self);
//     fn get_last_wheels_hit_ball_time(&self) -> f32;
//     fn get_vehicle_sim(&self) -> VehicleSimWrapper;
// }

// pub trait Wheel: Object {
//     fn get_spin_speed(&self) -> f32;
// }

// pub struct WheelWrapper(pub usize);
// impl_object!(WheelWrapper);

// impl Wheel for WheelWrapper {
//     fn get_spin_speed(&self) -> f32 {
//         unsafe { Wheel_Get_SpinSpeed(self.addr()) }
//     }
// }

// pub trait VehicleSim: Object {
//     fn get_wheels(&self) -> RLArray<WheelWrapper>;
// }

// pub struct VehicleSimWrapper(pub usize);
// impl_object!(VehicleSimWrapper);

// impl VehicleSim for VehicleSimWrapper {
//     fn get_wheels(&self) -> RLArray<WheelWrapper> {
//         unsafe {
//             let mut array = RLArrayRaw::new();
//             let ptr: *mut RLArrayRaw = &mut array as *mut RLArrayRaw;
//             VehicleSim_Get_Wheels(self.addr(), ptr);
//             RLArray::from_raw(array)
//         }
//     }
// }


// impl_object!(CarWrapper);
// impl Actor for CarWrapper {}

// pub struct CarWrapper(pub usize);

// impl Car for CarWrapper {
//     fn demolish(&self) {
//         unsafe { Car_Demolish(self.addr()); }
//     }

//     fn get_last_wheels_hit_ball_time(&self) -> f32 {
//         unsafe { Car_Get_LastWheelsHitBallTime(self.addr()) }
//     }

//     fn get_vehicle_sim(&self) -> VehicleSimWrapper {
//         unsafe { VehicleSimWrapper(Car_Get_VehicleSim(self.addr())) }
//     }
// }


// extern "C" {
//     fn Car_Demolish(p_car: usize);

//     fn Actor_GetLocation(p_actor: usize) -> Vector;
//     fn Actor_SetLocation(p_actor: usize, new_loc: Vector);
    
//     fn Car_Get_LastWheelsHitBallTime(p_car: usize) -> f32;
//     fn Car_Get_VehicleSim(p_car: usize) -> usize;

//     fn VehicleSim_Get_Wheels(p_vehicle_sim: usize, result: *mut RLArrayRaw);

//     fn Wheel_Get_SpinSpeed(p_wheel: usize) -> f32;
// }


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

        if c_name.is_null() { warn!("name ptr is null!"); return String::new(); }
        let name = unsafe { CStr::from_ptr(c_name) };

        match name.to_str() {
            Ok(s) => String::from(s),
            Err(_) => { warn!("cannot convert CStr to &str"); return String::new(); }
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
        if c_value.is_null() { warn!("value ptr is null!"); return String::new(); }
        let value = unsafe { CStr::from_ptr(c_value) };

        match value.to_str() {
            Ok(s) => String::from(s),
            Err(_) => { warn!("cannot convert CStr to &str"); return String::new(); }
        }
    }

    pub fn get_description(&self) -> String {
        let c_value = unsafe { CVar_GetDescription(self.0) };
        if c_value.is_null() { warn!("value ptr is null!"); return String::new(); }
        let value = unsafe { CStr::from_ptr(c_value) };

        match value.to_str() {
            Ok(s) => String::from(s),
            Err(_) => { warn!("cannot convert CStr to &str"); return String::new(); }
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
            T::from(*ptr)
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReplicatedRBState;
impl_struct!(ReplicatedRBState);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VehicleInputs;
impl_struct!(VehicleInputs);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UniqueNetId;
impl_struct!(UniqueNetId);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WheelContactData;
impl_struct!(WheelContactData);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LinearColor;
impl_struct!(LinearColor);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TViewTarget;
impl_struct!(TViewTarget);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WorldContactData;
impl WorldContactData{ pub fn new() -> Self { Self } }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Double;
impl_struct!(Double);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sample;
impl_struct!(Sample);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StickyForceData;
impl_struct!(StickyForceData);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProfileCameraSettings;
impl_struct!(ProfileCameraSettings);


