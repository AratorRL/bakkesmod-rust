#![allow(unused)]

use std::fmt;

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
    fn get_location(&self) -> Vector {
        unsafe { Actor_GetLocation(self.addr()) }
    }

    fn set_location(&self, new_loc: Vector) {
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

    fn Actor_GetLocation(p_actor: usize) -> Vector;
    fn Actor_SetLocation(p_actor: usize, new_loc: Vector);
}


#[repr(C)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {x, y, z}
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


#[repr(C)]
pub struct Vector2 {
    x: u32,
    y: u32
}

impl Vector2 {
    pub fn new(x: u32, y: u32) -> Vector2 {
        Vector2 { x, y }
    }
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