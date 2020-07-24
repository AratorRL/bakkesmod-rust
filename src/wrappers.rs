#![allow(unused)]

use std::fmt;
use std::ops;
use std::convert::From;

macro_rules! impl_object {
    ($name: ident) => {
        impl Object for $name {
            fn new(addr: usize) -> Self { Self(addr) }
            fn addr(&self) -> usize { self.0 }
        }
    }
}

pub trait Object {
    fn new(addr: usize) -> Self;
    fn addr(&self) -> usize;
}

pub trait Actor: Object {
    fn get_location(&self) -> Vector3 {
        unsafe { Actor_GetLocation(self.addr()) }
    }

    fn set_location(&self, new_loc: Vector3) {
        unsafe { Actor_SetLocation(self.addr(), new_loc); }
    }
}

pub trait Car: Actor {
    fn demolish(&self);
}

impl_object!(CarWrapper);
impl Actor for CarWrapper {}

pub struct CarWrapper(pub usize);

impl Car for CarWrapper {
    fn demolish(&self) {
        unsafe { Car_Demolish(self.addr()); }
    }
}


extern "C" {
    fn Car_Demolish(p_car: usize);

    fn Actor_GetLocation(p_actor: usize) -> Vector3;
    fn Actor_SetLocation(p_actor: usize, new_loc: Vector3);
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {x, y, z}
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => (
        $crate::wrappers::Vector3::new($x as f32, $y as f32, $z as f32)
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

extern "C" {
    fn Canvas_DrawLine(p_canvas: usize, start: Vector2, end: Vector2);
}