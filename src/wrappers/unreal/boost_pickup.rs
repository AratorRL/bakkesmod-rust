use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct BoostPickupWrapper(pub usize);
impl_object!(BoostPickupWrapper);

impl BoostPickup for BoostPickupWrapper {}
impl VehiclePickup for BoostPickupWrapper {}
impl Actor for BoostPickupWrapper {}

pub trait BoostPickup : VehiclePickup {
    fn get_boost_amount(&self) -> f32 {
        unsafe {
            VehiclePickup_Boost_TA_Get_BoostAmount(self.addr())
        }
    }
    fn get_boost_type(&self) -> u8 {
        unsafe {
            VehiclePickup_Boost_TA_Get_BoostType(self.addr())
        }
    }
    fn play_picked_up_fx(&self) {
        unsafe {
            VehiclePickup_Boost_TA_PlayPickedUpFX(self.addr());
        }
    }
    fn pickup(&self, car: CarWrapper) {
        unsafe {
            VehiclePickup_Boost_TA_Pickup(self.addr(), car.addr());
        }
    }
    fn can_pickup(&self, car: CarWrapper) -> bool {
        unsafe {
            VehiclePickup_Boost_TA_CanPickup(self.addr(), car.addr())
        }
    }

}

extern "C" {
    fn VehiclePickup_Boost_TA_Get_BoostAmount(obj: usize) -> f32;
    fn BoostPickupWrapper_SetBoostAmount(obj: usize, new_val: f32);
    fn VehiclePickup_Boost_TA_Get_BoostType(obj: usize) -> u8;
    fn BoostPickupWrapper_SetBoostType(obj: usize, new_val: u8);
    fn VehiclePickup_Boost_TA_PlayPickedUpFX(obj: usize);
    fn VehiclePickup_Boost_TA_Pickup(obj: usize, Car: usize);
    fn VehiclePickup_Boost_TA_CanPickup(obj: usize, Car: usize) -> bool;

}