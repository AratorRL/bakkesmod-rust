use crate::wrappers::*;
use super::*;

pub struct CarSpeedPickupWrapper(pub usize);
impl_object!(CarSpeedPickupWrapper);

impl CarSpeedPickup for CarSpeedPickupWrapper {}
impl RumblePickupComponent for CarSpeedPickupWrapper {}
impl CarComponent for CarSpeedPickupWrapper {}
impl Actor for CarSpeedPickupWrapper {}

pub trait CarSpeedPickup : RumblePickupComponent {
    fn get_gravity_scale(&self) -> f32 {
        unsafe {
            SpecialPickup_CarGravity_TA_Get_GravityScale(self.addr())
        }
    }
    fn get_added_force(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_CarGravity_TA_Get_AddedForce(self.addr(), result_ptr);
            result
        }
    }
    fn get_orig_gravity_scale(&self) -> f32 {
        unsafe {
            SpecialPickup_CarGravity_TA_Get_OrigGravityScale(self.addr())
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_CarGravity_TA_PickupEnd(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_CarGravity_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_CarGravity_TA_Get_GravityScale(obj: usize) -> f32;
    fn CarSpeedPickup_SetGravityScale(obj: usize, new_val: f32);
    fn SpecialPickup_CarGravity_TA_Get_AddedForce(obj: usize, result: *mut Vector);
    fn CarSpeedPickup_SetAddedForce(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_CarGravity_TA_Get_OrigGravityScale(obj: usize) -> f32;
    fn CarSpeedPickup_SetOrigGravityScale(obj: usize, new_val: f32);
    fn SpecialPickup_CarGravity_TA_PickupEnd(obj: usize);
    fn SpecialPickup_CarGravity_TA_PickupStart(obj: usize);

}