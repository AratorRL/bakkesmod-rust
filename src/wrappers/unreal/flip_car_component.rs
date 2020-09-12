use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct FlipCarComponentWrapper(pub usize);
impl_object!(FlipCarComponentWrapper);

impl FlipCarComponent for FlipCarComponentWrapper {}
impl CarComponent for FlipCarComponentWrapper {}
impl Actor for FlipCarComponentWrapper {}

pub trait FlipCarComponent : CarComponent {
    fn get_flip_car_impulse(&self) -> f32 {
        unsafe {
            CarComponent_FlipCar_TA_Get_FlipCarImpulse(self.addr())
        }
    }
    fn get_flip_car_torque(&self) -> f32 {
        unsafe {
            CarComponent_FlipCar_TA_Get_FlipCarTorque(self.addr())
        }
    }
    fn get_flip_car_time(&self) -> f32 {
        unsafe {
            CarComponent_FlipCar_TA_Get_FlipCarTime(self.addr())
        }
    }
    fn get_b_flip_right(&self) -> bool {
        unsafe {
            CarComponent_FlipCar_TA_Get_bFlipRight(self.addr())
        }
    }
    fn init_flip(&self) {
        unsafe {
            CarComponent_FlipCar_TA_InitFlip(self.addr());
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            CarComponent_FlipCar_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn can_activate(&self) -> bool {
        unsafe {
            CarComponent_FlipCar_TA_CanActivate(self.addr())
        }
    }
    fn on_created(&self) {
        unsafe {
            CarComponent_FlipCar_TA_OnCreated(self.addr());
        }
    }

}

extern "C" {
    fn CarComponent_FlipCar_TA_Get_FlipCarImpulse(obj: usize) -> f32;
    fn FlipCarComponentWrapper_SetFlipCarImpulse(obj: usize, new_val: f32);
    fn CarComponent_FlipCar_TA_Get_FlipCarTorque(obj: usize) -> f32;
    fn FlipCarComponentWrapper_SetFlipCarTorque(obj: usize, new_val: f32);
    fn CarComponent_FlipCar_TA_Get_FlipCarTime(obj: usize) -> f32;
    fn FlipCarComponentWrapper_SetFlipCarTime(obj: usize, new_val: f32);
    fn CarComponent_FlipCar_TA_Get_bFlipRight(obj: usize) -> bool;
    fn FlipCarComponentWrapper_SetbFlipRight(obj: usize, new_val: bool);
    fn CarComponent_FlipCar_TA_InitFlip(obj: usize);
    fn CarComponent_FlipCar_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn CarComponent_FlipCar_TA_CanActivate(obj: usize) -> bool;
    fn CarComponent_FlipCar_TA_OnCreated(obj: usize);

}