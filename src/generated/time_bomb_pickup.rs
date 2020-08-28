use crate::wrappers::*;
use crate::generated::*;

pub struct TimeBombPickupWrapper(pub usize);
impl_object!(TimeBombPickupWrapper);

impl TimeBombPickup for TimeBombPickupWrapper {}
impl RumblePickupComponent for TimeBombPickupWrapper {}
impl CarComponent for TimeBombPickupWrapper {}
impl Actor for TimeBombPickupWrapper {}

pub trait TimeBombPickup : RumblePickupComponent {
    fn get_radius(&self) -> f32 {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_Radius(self.addr())
        }
    }
    fn get_almost_ready_duration(&self) -> f32 {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_AlmostReadyDuration(self.addr())
        }
    }
    fn get_start_mat_speed(&self) -> f32 {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_StartMatSpeed(self.addr())
        }
    }
    fn get_almost_ready_mat_speed(&self) -> f32 {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_AlmostReadyMatSpeed(self.addr())
        }
    }
    fn get_impulse_force(&self) -> f32 {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_ImpulseForce(self.addr())
        }
    }
    fn get_car_vertical_force(&self) -> f32 {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_CarVerticalForce(self.addr())
        }
    }
    fn get_car_torque(&self) -> f32 {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_CarTorque(self.addr())
        }
    }
    fn get_b_demolish(&self) -> bool {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_bDemolish(self.addr())
        }
    }
    fn get_b_impulse(&self) -> bool {
        unsafe {
            SpecialPickup_TimeBomb_TA_Get_bImpulse(self.addr())
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_TimeBomb_TA_PickupEnd(self.addr());
        }
    }
    fn almost_ready(&self) {
        unsafe {
            SpecialPickup_TimeBomb_TA_AlmostReady(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_TimeBomb_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_TimeBomb_TA_Get_Radius(obj: usize) -> f32;
    fn TimeBombPickup_SetRadius(obj: usize, new_val: f32);
    fn SpecialPickup_TimeBomb_TA_Get_AlmostReadyDuration(obj: usize) -> f32;
    fn TimeBombPickup_SetAlmostReadyDuration(obj: usize, new_val: f32);
    fn SpecialPickup_TimeBomb_TA_Get_StartMatSpeed(obj: usize) -> f32;
    fn TimeBombPickup_SetStartMatSpeed(obj: usize, new_val: f32);
    fn SpecialPickup_TimeBomb_TA_Get_AlmostReadyMatSpeed(obj: usize) -> f32;
    fn TimeBombPickup_SetAlmostReadyMatSpeed(obj: usize, new_val: f32);
    fn SpecialPickup_TimeBomb_TA_Get_ImpulseForce(obj: usize) -> f32;
    fn TimeBombPickup_SetImpulseForce(obj: usize, new_val: f32);
    fn SpecialPickup_TimeBomb_TA_Get_CarVerticalForce(obj: usize) -> f32;
    fn TimeBombPickup_SetCarVerticalForce(obj: usize, new_val: f32);
    fn SpecialPickup_TimeBomb_TA_Get_CarTorque(obj: usize) -> f32;
    fn TimeBombPickup_SetCarTorque(obj: usize, new_val: f32);
    fn SpecialPickup_TimeBomb_TA_Get_bDemolish(obj: usize) -> bool;
    fn TimeBombPickup_SetbDemolish(obj: usize, new_val: bool);
    fn SpecialPickup_TimeBomb_TA_Get_bImpulse(obj: usize) -> bool;
    fn TimeBombPickup_SetbImpulse(obj: usize, new_val: bool);
    fn SpecialPickup_TimeBomb_TA_PickupEnd(obj: usize);
    fn SpecialPickup_TimeBomb_TA_AlmostReady(obj: usize);
    fn SpecialPickup_TimeBomb_TA_PickupStart(obj: usize);

}