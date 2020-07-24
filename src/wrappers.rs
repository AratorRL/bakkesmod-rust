#![allow(unused)]

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

    // TODO: impl Debug trait instead
    pub fn to_string(&self) -> String {
        format!("{}, {}, {}", self.x, self.y, self.z)
    }
}