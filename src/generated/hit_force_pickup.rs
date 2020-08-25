use crate::wrappers::*;
use crate::generated::*;

pub struct HitForcePickupWrapper(pub usize);
impl_object!(HitForcePickupWrapper);

impl HitForcePickup for HitForcePickupWrapper {}
impl RumblePickupComponent for HitForcePickupWrapper {}
impl CarComponent for HitForcePickupWrapper {}
impl Actor for HitForcePickupWrapper {}

pub trait HitForcePickup : RumblePickupComponent {
	fn get_b_ball_force(&self) -> bool {
		unsafe {
			SpecialPickup_HitForce_TA_Get_bBallForce(self.addr())
		}
	}
	fn get_b_car_force(&self) -> bool {
		unsafe {
			SpecialPickup_HitForce_TA_Get_bCarForce(self.addr())
		}
	}
	fn get_b_demolish_cars(&self) -> bool {
		unsafe {
			SpecialPickup_HitForce_TA_Get_bDemolishCars(self.addr())
		}
	}
	fn get_ball_hit_force(&self) -> f32 {
		unsafe {
			SpecialPickup_HitForce_TA_Get_BallHitForce(self.addr())
		}
	}
	fn get_car_hit_force(&self) -> f32 {
		unsafe {
			SpecialPickup_HitForce_TA_Get_CarHitForce(self.addr())
		}
	}
	fn get_min_fx_time(&self) -> f32 {
		unsafe {
			SpecialPickup_HitForce_TA_Get_MinFXTime(self.addr())
		}
	}
	fn get_orig_ball_hit_force(&self) -> f32 {
		unsafe {
			SpecialPickup_HitForce_TA_Get_OrigBallHitForce(self.addr())
		}
	}
	fn get_orig_car_hit_force(&self) -> f32 {
		unsafe {
			SpecialPickup_HitForce_TA_Get_OrigCarHitForce(self.addr())
		}
	}
	fn get_last_fx_time(&self) -> f32 {
		unsafe {
			SpecialPickup_HitForce_TA_Get_LastFXTime(self.addr())
		}
	}
	fn pickup_end(&self) {
		unsafe {
			SpecialPickup_HitForce_TA_PickupEnd(self.addr());
		}
	}
	fn pickup_start(&self) {
		unsafe {
			SpecialPickup_HitForce_TA_PickupStart(self.addr());
		}
	}

}

extern "C" {
	fn SpecialPickup_HitForce_TA_Get_bBallForce(obj: usize) -> bool;
	fn HitForcePickup_SetbBallForce(obj: usize, new_val: bool);
	fn SpecialPickup_HitForce_TA_Get_bCarForce(obj: usize) -> bool;
	fn HitForcePickup_SetbCarForce(obj: usize, new_val: bool);
	fn SpecialPickup_HitForce_TA_Get_bDemolishCars(obj: usize) -> bool;
	fn HitForcePickup_SetbDemolishCars(obj: usize, new_val: bool);
	fn SpecialPickup_HitForce_TA_Get_BallHitForce(obj: usize) -> f32;
	fn HitForcePickup_SetBallHitForce(obj: usize, new_val: f32);
	fn SpecialPickup_HitForce_TA_Get_CarHitForce(obj: usize) -> f32;
	fn HitForcePickup_SetCarHitForce(obj: usize, new_val: f32);
	fn SpecialPickup_HitForce_TA_Get_MinFXTime(obj: usize) -> f32;
	fn HitForcePickup_SetMinFXTime(obj: usize, new_val: f32);
	fn SpecialPickup_HitForce_TA_Get_OrigBallHitForce(obj: usize) -> f32;
	fn HitForcePickup_SetOrigBallHitForce(obj: usize, new_val: f32);
	fn SpecialPickup_HitForce_TA_Get_OrigCarHitForce(obj: usize) -> f32;
	fn HitForcePickup_SetOrigCarHitForce(obj: usize, new_val: f32);
	fn SpecialPickup_HitForce_TA_Get_LastFXTime(obj: usize) -> f32;
	fn HitForcePickup_SetLastFXTime(obj: usize, new_val: f32);
	fn SpecialPickup_HitForce_TA_PickupEnd(obj: usize);
	fn SpecialPickup_HitForce_TA_PickupStart(obj: usize);

}