use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct BattarangPickupWrapper(pub usize);
impl_object!(BattarangPickupWrapper);

impl BattarangPickup for BattarangPickupWrapper {}
impl BallLassoPickup for BattarangPickupWrapper {}
impl SpringPickup for BattarangPickupWrapper {}
impl TargetedPickup for BattarangPickupWrapper {}
impl RumblePickupComponent for BattarangPickupWrapper {}
impl CarComponent for BattarangPickupWrapper {}
impl Actor for BattarangPickupWrapper {}

pub trait BattarangPickup : BallLassoPickup {
    fn get_spin_speed(&self) -> f32 {
        unsafe {
            SpecialPickup_Batarang_TA_Get_SpinSpeed(self.addr())
        }
    }
    fn get_cur_rotation(&self) -> f32 {
        unsafe {
            SpecialPickup_Batarang_TA_Get_CurRotation(self.addr())
        }
    }

}

extern "C" {
    fn SpecialPickup_Batarang_TA_Get_SpinSpeed(obj: usize) -> f32;
    fn BattarangPickup_SetSpinSpeed(obj: usize, new_val: f32);
    fn SpecialPickup_Batarang_TA_Get_CurRotation(obj: usize) -> f32;
    fn BattarangPickup_SetCurRotation(obj: usize, new_val: f32);

}