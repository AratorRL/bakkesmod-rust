use crate::wrappers::*;
use crate::generated::*;

pub struct BoostModPickupWrapper(pub usize);
impl_object!(BoostModPickupWrapper);

impl BoostModPickup for BoostModPickupWrapper {}
impl RumblePickupComponent for BoostModPickupWrapper {}
impl CarComponent for BoostModPickupWrapper {}
impl Actor for BoostModPickupWrapper {}

pub trait BoostModPickup : RumblePickupComponent {
	fn get_b_unlimited_boost(&self) -> bool {
		unsafe {
			SpecialPickup_BoostMod_TA_Get_bUnlimitedBoost(self.addr())
		}
	}
	fn get_boost_strength(&self) -> f32 {
		unsafe {
			SpecialPickup_BoostMod_TA_Get_BoostStrength(self.addr())
		}
	}
	fn get_old_boost_strength(&self) -> f32 {
		unsafe {
			SpecialPickup_BoostMod_TA_Get_OldBoostStrength(self.addr())
		}
	}
	fn pickup_end(&self) {
		unsafe {
			SpecialPickup_BoostMod_TA_PickupEnd(self.addr());
		}
	}
	fn pickup_start(&self) {
		unsafe {
			SpecialPickup_BoostMod_TA_PickupStart(self.addr());
		}
	}

}

extern "C" {
	fn SpecialPickup_BoostMod_TA_Get_bUnlimitedBoost(obj: usize) -> bool;
	fn BoostModPickup_SetbUnlimitedBoost(obj: usize, new_val: bool);
	fn SpecialPickup_BoostMod_TA_Get_BoostStrength(obj: usize) -> f32;
	fn BoostModPickup_SetBoostStrength(obj: usize, new_val: f32);
	fn SpecialPickup_BoostMod_TA_Get_OldBoostStrength(obj: usize) -> f32;
	fn BoostModPickup_SetOldBoostStrength(obj: usize, new_val: f32);
	fn SpecialPickup_BoostMod_TA_PickupEnd(obj: usize);
	fn SpecialPickup_BoostMod_TA_PickupStart(obj: usize);

}