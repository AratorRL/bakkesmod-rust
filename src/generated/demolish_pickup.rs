use crate::wrappers::*;
use crate::generated::*;

pub struct DemolishPickupWrapper(pub usize);
impl_object!(DemolishPickupWrapper);

impl DemolishPickup for DemolishPickupWrapper {}
impl RumblePickupComponent for DemolishPickupWrapper {}
impl CarComponent for DemolishPickupWrapper {}
impl Actor for DemolishPickupWrapper {}

pub trait DemolishPickup : RumblePickupComponent {
    fn get_demolish_target(&self) -> u8 {
        unsafe {
            SpecialPickup_Demolish_TA_Get_DemolishTarget(self.addr())
        }
    }
    fn get_demolish_speed(&self) -> u8 {
        unsafe {
            SpecialPickup_Demolish_TA_Get_DemolishSpeed(self.addr())
        }
    }
    fn get_old_target(&self) -> u8 {
        unsafe {
            SpecialPickup_Demolish_TA_Get_OldTarget(self.addr())
        }
    }
    fn get_old_speed(&self) -> u8 {
        unsafe {
            SpecialPickup_Demolish_TA_Get_OldSpeed(self.addr())
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_Demolish_TA_PickupEnd(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_Demolish_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_Demolish_TA_Get_DemolishTarget(obj: usize) -> u8;
    fn DemolishPickup_SetDemolishTarget(obj: usize, new_val: u8);
    fn SpecialPickup_Demolish_TA_Get_DemolishSpeed(obj: usize) -> u8;
    fn DemolishPickup_SetDemolishSpeed(obj: usize, new_val: u8);
    fn SpecialPickup_Demolish_TA_Get_OldTarget(obj: usize) -> u8;
    fn DemolishPickup_SetOldTarget(obj: usize, new_val: u8);
    fn SpecialPickup_Demolish_TA_Get_OldSpeed(obj: usize) -> u8;
    fn DemolishPickup_SetOldSpeed(obj: usize, new_val: u8);
    fn SpecialPickup_Demolish_TA_PickupEnd(obj: usize);
    fn SpecialPickup_Demolish_TA_PickupStart(obj: usize);

}