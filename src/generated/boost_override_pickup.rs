use crate::wrappers::*;
use crate::generated::*;

pub struct BoostOverridePickupWrapper(pub usize);
impl_object!(BoostOverridePickupWrapper);

impl BoostOverridePickup for BoostOverridePickupWrapper {}
impl TargetedPickup for BoostOverridePickupWrapper {}
impl RumblePickupComponent for BoostOverridePickupWrapper {}
impl CarComponent for BoostOverridePickupWrapper {}
impl Actor for BoostOverridePickupWrapper {}

pub trait BoostOverridePickup : TargetedPickup {
	fn get_other_car(&self) -> Option<CarWrapper> {
		unsafe {
			CarWrapper::try_new(SpecialPickup_BoostOverride_TA_Get_OtherCar(self.addr()))
		}
	}
	fn pickup_end(&self) {
		unsafe {
			SpecialPickup_BoostOverride_TA_PickupEnd(self.addr());
		}
	}
	fn on_target_changed(&self) {
		unsafe {
			SpecialPickup_BoostOverride_TA_OnTargetChanged(self.addr());
		}
	}
	fn pickup_start(&self) {
		unsafe {
			SpecialPickup_BoostOverride_TA_PickupStart(self.addr());
		}
	}

}

extern "C" {
	fn SpecialPickup_BoostOverride_TA_Get_OtherCar(obj: usize) -> usize;
	fn BoostOverridePickup_SetOtherCar(obj: usize, new_val: usize);
	fn SpecialPickup_BoostOverride_TA_PickupEnd(obj: usize);
	fn SpecialPickup_BoostOverride_TA_OnTargetChanged(obj: usize);
	fn SpecialPickup_BoostOverride_TA_PickupStart(obj: usize);

}