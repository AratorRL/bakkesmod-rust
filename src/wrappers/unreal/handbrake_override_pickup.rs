use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct HandbrakeOverridePickupWrapper(pub usize);
impl_object!(HandbrakeOverridePickupWrapper);

impl HandbrakeOverridePickup for HandbrakeOverridePickupWrapper {}
impl TargetedPickup for HandbrakeOverridePickupWrapper {}
impl RumblePickupComponent for HandbrakeOverridePickupWrapper {}
impl CarComponent for HandbrakeOverridePickupWrapper {}
impl Actor for HandbrakeOverridePickupWrapper {}

pub trait HandbrakeOverridePickup : TargetedPickup {
    fn get_other_car(&self) -> Option<CarWrapper> {
        unsafe {
            CarWrapper::try_new(SpecialPickup_HandbrakeOverride_TA_Get_OtherCar(self.addr()))
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_HandbrakeOverride_TA_PickupEnd(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_HandbrakeOverride_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_HandbrakeOverride_TA_Get_OtherCar(obj: usize) -> usize;
    fn HandbrakeOverridePickup_SetOtherCar(obj: usize, new_val: usize);
    fn SpecialPickup_HandbrakeOverride_TA_PickupEnd(obj: usize);
    fn SpecialPickup_HandbrakeOverride_TA_PickupStart(obj: usize);

}