use crate::wrappers::*;
use super::*;

pub struct SwapperPickupWrapper(pub usize);
impl_object!(SwapperPickupWrapper);

impl SwapperPickup for SwapperPickupWrapper {}
impl TargetedPickup for SwapperPickupWrapper {}
impl RumblePickupComponent for SwapperPickupWrapper {}
impl CarComponent for SwapperPickupWrapper {}
impl Actor for SwapperPickupWrapper {}

pub trait SwapperPickup : TargetedPickup {
    fn get_other_car(&self) -> Option<CarWrapper> {
        unsafe {
            CarWrapper::try_new(SpecialPickup_Swapper_TA_Get_OtherCar(self.addr()))
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_Swapper_TA_PickupEnd(self.addr());
        }
    }
    fn on_target_changed(&self) {
        unsafe {
            SpecialPickup_Swapper_TA_OnTargetChanged(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_Swapper_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_Swapper_TA_Get_OtherCar(obj: usize) -> usize;
    fn SwapperPickup_SetOtherCar(obj: usize, new_val: usize);
    fn SpecialPickup_Swapper_TA_PickupEnd(obj: usize);
    fn SpecialPickup_Swapper_TA_OnTargetChanged(obj: usize);
    fn SpecialPickup_Swapper_TA_PickupStart(obj: usize);

}