#![allow(unused)]

use std::fmt;
use std::ops;
use std::convert::From;
use std::marker::{PhantomData, Sized};

use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use crate::log_console;

#[macro_use] mod macros;

mod generated;
pub use generated::*;

mod mmr;
pub use mmr::*;

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
    pub fn from(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn new() -> Vector2 {
        Vector2 { x: 0, y: 0 }
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
        $crate::wrappers::Vector2::from($x as i32, $y as i32)
    );
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2f {
    x: f32,
    y: f32
}

impl Vector2f {
    pub fn new_from(x: f32, y: f32) -> Vector2f {
        Vector2f { x, y }
    }

    pub fn new() -> Vector2f {
        Vector2f { x: 0.0, y: 0.0 }
    }
}

impl From<Vector2> for Vector2f {
    fn from(vec2: Vector2) -> Self {
        let x: f32 = vec2.x as f32;
        let y: f32 = vec2.y as f32;
        Vector2f { x, y }
    }
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

#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        $crate::wrappers::Color::from($r as u8, $g as u8, $b as u8, $a as u8)
    );
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
pub struct SkillRating {
    mu: f32,
    sigma: f32
}
impl_unreal_pointer_struct!(SkillRating);

impl SkillRating {
    pub fn new() -> Self {
        SkillRating { mu: 0.0, sigma: 0.0 }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SkillRank {
    tier: i32,
    division: i32,
    matches_player: i32
}
impl_unreal_pointer_struct!(SkillRank);

impl SkillRank {
    pub fn new() -> Self {
        SkillRank { tier: 0, division: 0, matches_player: 0 }
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
pub struct POV(usize);
impl_unreal_pointer_struct!(POV);
struct_default_new!(POV);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LinearColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}
impl_unreal_pointer_struct!(LinearColor);

impl LinearColor {
    pub fn new() -> Self {
        LinearColor { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }

    pub fn from(r: f32, g: f32, b: f32, a: f32) -> Self {
        LinearColor { r, g, b, a }
    }
}

#[macro_export]
macro_rules! lin_color {
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        $crate::wrappers::LinearColor::from($r as f32, $g as f32, $b as f32, $a as f32)
    );
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PredictionInfo(usize);
impl_unreal_pointer_struct!(PredictionInfo);
struct_default_new!(PredictionInfo);

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

    pub fn addr(&self) -> usize {
        self.0
    }

    pub fn set_position<T: Into<Vector2f>>(&self, pos: T) {
        let pos: Vector2f = pos.into();
        let pos = &pos as *const Vector2f;
        unsafe { Canvas_SetPosition(self.addr(), pos); }
    }

    pub fn get_position_float(&self) -> Vector2f {
        let mut result = Vector2f::new();
        let result_ptr = &result as *const Vector2f;
        unsafe { Canvas_GetPositionFloat(self.addr(), result_ptr); }
        result
    }

    pub fn set_color_chars(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        unsafe { Canvas_SetColor_chars(self.addr(), red, green, blue, alpha); }
    }

    pub fn set_color(&self, color: LinearColor) {
        let color = &color as *const LinearColor;
        unsafe { Canvas_SetColor(self.addr(), color); }
    }

    pub fn get_color(&self) -> LinearColor {
        let mut result = LinearColor::new();
        let result_ptr = &result as *const LinearColor;
        unsafe { Canvas_GetColor(self.addr(), result_ptr); }
        result
    }

    pub fn draw_box<T: Into<Vector2f>>(&self, size: T) {
        let size: Vector2f = size.into();
        let size = &size as *const Vector2f;
        unsafe { Canvas_DrawBox(self.addr(), size); }
    }

    pub fn fill_box<T: Into<Vector2f>>(&self, size: T) {
        let size: Vector2f = size.into();
        let size = &size as *const Vector2f;
        unsafe { Canvas_FillBox(self.addr(), size); }
    }

    pub fn fill_triangle<T: Into<Vector2f>>(&self, p1: T, p2: T, p3: T) {
        let p1: Vector2f = p1.into();
        let p1 = &p1 as *const Vector2f;
        let p2: Vector2f = p2.into();
        let p2 = &p2 as *const Vector2f;
        let p3: Vector2f = p3.into();
        let p3 = &p3 as *const Vector2f;
        unsafe { Canvas_FillTriangle(self.addr(), p1, p2, p3); }
    }

    pub fn fill_triangle_color<T: Into<Vector2f>>(&self, p1: T, p2: T, p3: T, color: LinearColor) {
        let p1: Vector2f = p1.into();
        let p1 = &p1 as *const Vector2f;
        let p2: Vector2f = p2.into();
        let p2 = &p2 as *const Vector2f;
        let p3: Vector2f = p3.into();
        let p3 = &p3 as *const Vector2f;
        let color = &color as *const LinearColor;
        unsafe { Canvas_FillTriangle_color(self.addr(), p1, p2, p3, color); }
    }

    pub fn draw_string(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        unsafe { Canvas_DrawString(self.addr(), c_text); }
    }

    pub fn draw_string_scale(&self, text: &str, x_scale: f32, y_scale: f32) {
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        unsafe { Canvas_DrawString_pos(self.addr(), c_text, x_scale, y_scale); }
    }

    pub fn get_string_size(&self, text: &str, x_scale: f32, y_scale: f32) -> Vector2f {
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        let mut result = Vector2f::new();
        let result_ptr = &result as *const Vector2f;
        unsafe { Canvas_GetStringSize(self.addr(), c_text, x_scale, y_scale, result_ptr); }
        result
    }

    pub fn draw_line<T: Into<Vector2f>>(&self, start: T, end: T) {
        let start: Vector2f = start.into();
        let start = &start as *const Vector2f;
        let end: Vector2f = end.into();
        let end = &end as *const Vector2f;
        unsafe { Canvas_DrawLine(self.addr(), start, end); }
    }

    pub fn draw_line_width<T: Into<Vector2f>>(&self, start: T, end: T, width: f32) {
        let start: Vector2f = start.into();
        let start = &start as *const Vector2f;
        let end: Vector2f = end.into();
        let end = &end as *const Vector2f;
        unsafe { Canvas_DrawLine_width(self.addr(), start, end, width); }
    }

    pub fn draw_rect<T: Into<Vector2f>>(&self, start: T, end: T) {
        let start: Vector2f = start.into();
        let start = &start as *const Vector2f;
        let end: Vector2f = end.into();
        let end = &end as *const Vector2f;
        unsafe { Canvas_DrawRect(self.addr(), start, end); }
    }

    pub fn project(&self, location: Vector) -> Vector2 {
        let location = &location as *const Vector;
        let mut result = Vector2::new();
        let result_ptr = &result as *const Vector2;
        unsafe { Canvas_Project(self.addr(), location, result_ptr); }
        result
    }

    pub fn project_f(&self, location: Vector) -> Vector2f {
        let location = &location as *const Vector;
        let mut result = Vector2f::new();
        let result_ptr = &result as *const Vector2f;
        unsafe { Canvas_ProjectF(self.addr(), location, result_ptr); }
        result
    }
    pub fn get_size(&self) -> Vector2 {
        let mut result = Vector2::new();
        let result_ptr = &result as *const Vector2;
        unsafe { Canvas_GetSize(self.addr(), result_ptr); }
        result
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
    fn Canvas_SetPosition(obj: usize, pos: *const Vector2f);
    fn Canvas_GetPositionFloat(obj: usize, result: *const Vector2f);
    fn Canvas_SetColor_chars(obj: usize, Red: u8, Green: u8, Blue: u8, Alpha: u8);
    fn Canvas_SetColor(obj: usize, color: *const LinearColor);
    fn Canvas_GetColor(obj: usize, result: *const LinearColor);
    fn Canvas_DrawBox(obj: usize, size: *const Vector2f);
    fn Canvas_FillBox(obj: usize, size: *const Vector2f);
    fn Canvas_FillTriangle(obj: usize, p1: *const Vector2f, p2: *const Vector2f, p3: *const Vector2f);
    fn Canvas_FillTriangle_color(obj: usize, p1: *const Vector2f, p2: *const Vector2f, p3: *const Vector2f, color: *const LinearColor);
    fn Canvas_DrawString(obj: usize, text: *const c_char);
    fn Canvas_DrawString_pos(obj: usize, text: *const c_char, xScale: f32, yScale: f32);
    fn Canvas_GetStringSize(obj: usize, text: *const c_char, xScale: f32, yScale: f32, result: *const Vector2f);
    fn Canvas_DrawLine(obj: usize, start: *const Vector2f, end: *const Vector2f);
    fn Canvas_DrawLine_width(obj: usize, start: *const Vector2f, end: *const Vector2f, width: f32);
    fn Canvas_DrawRect(obj: usize, start: *const Vector2f, end: *const Vector2f);
    fn Canvas_SetPositionI(obj: usize, pos: *const Vector2);
    fn Canvas_GetPositionI(obj: usize, result: *const Vector2);
    fn Canvas_DrawBoxI(obj: usize, size: *const Vector2);
    fn Canvas_FillBoxI(obj: usize, size: *const Vector2);
    fn Canvas_FillTriangleI(obj: usize, p1: *const Vector2, p2: *const Vector2, p3: *const Vector2);
    fn Canvas_FillTriangle_colorI(obj: usize, p1: *const Vector2, p2: *const Vector2, p3: *const Vector2, color: *const LinearColor);
    fn Canvas_DrawLineI(obj: usize, start: *const Vector2, end: *const Vector2);
    fn Canvas_DrawLineWidthI(obj: usize, start: *const Vector2, end: *const Vector2, width: f32);
    fn Canvas_DrawRectI(obj: usize, start: *const Vector2, end: *const Vector2);
    fn Canvas_Project(obj: usize, location: *const Vector, result: *const Vector2);
    fn Canvas_ProjectF(obj: usize, location: *const Vector, result: *const Vector2f);
    fn Canvas_GetSize(obj: usize, result: *const Vector2);

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