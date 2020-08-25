use crate::wrappers::*;
use crate::generated::*;

pub struct GravityPickupWrapper(pub usize);
impl_object!(GravityPickupWrapper);

impl GravityPickup for GravityPickupWrapper {}
impl RumblePickupComponent for GravityPickupWrapper {}
impl CarComponent for GravityPickupWrapper {}
impl Actor for GravityPickupWrapper {}

pub trait GravityPickup : RumblePickupComponent {
	fn get_ball_gravity(&self) -> f32 {
		unsafe {
			SpecialPickup_BallGravity_TA_Get_BallGravity(self.addr())
		}
	}
	fn get_range(&self) -> f32 {
		unsafe {
			SpecialPickup_BallGravity_TA_Get_Range(self.addr())
		}
	}
	fn get_offset(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			SpecialPickup_BallGravity_TA_Get_Offset(self.addr(), result_ptr);
			result
		}
	}
	fn get_b_deactivate_on_touch(&self) -> bool {
		unsafe {
			SpecialPickup_BallGravity_TA_Get_bDeactivateOnTouch(self.addr())
		}
	}
	fn get_record_ball_hit_rate(&self) -> f32 {
		unsafe {
			SpecialPickup_BallGravity_TA_Get_RecordBallHitRate(self.addr())
		}
	}
	fn get_last_recorded_ball_hit_time(&self) -> f32 {
		unsafe {
			SpecialPickup_BallGravity_TA_Get_LastRecordedBallHitTime(self.addr())
		}
	}
	fn get_prev_ball(&self) -> BallWrapper {
		unsafe {
			BallWrapper::new(SpecialPickup_BallGravity_TA_Get_PrevBall(self.addr()))
		}
	}
	fn update_visual(&self) {
		unsafe {
			SpecialPickup_BallGravity_TA_UpdateVisual(self.addr());
		}
	}
	fn apply_forces(&self, active_time: f32) {
		unsafe {
			SpecialPickup_BallGravity_TA_ApplyForces(self.addr(), active_time);
		}
	}
	fn pickup_end(&self) {
		unsafe {
			SpecialPickup_BallGravity_TA_PickupEnd(self.addr());
		}
	}
	fn pickup_start(&self) {
		unsafe {
			SpecialPickup_BallGravity_TA_PickupStart(self.addr());
		}
	}

}

extern "C" {
	fn SpecialPickup_BallGravity_TA_Get_BallGravity(obj: usize) -> f32;
	fn GravityPickup_SetBallGravity(obj: usize, new_val: f32);
	fn SpecialPickup_BallGravity_TA_Get_Range(obj: usize) -> f32;
	fn GravityPickup_SetRange(obj: usize, new_val: f32);
	fn SpecialPickup_BallGravity_TA_Get_Offset(obj: usize, result: *mut Vector);
	fn GravityPickup_SetOffset(obj: usize, new_val: *mut Vector);
	fn SpecialPickup_BallGravity_TA_Get_bDeactivateOnTouch(obj: usize) -> bool;
	fn GravityPickup_SetbDeactivateOnTouch(obj: usize, new_val: bool);
	fn SpecialPickup_BallGravity_TA_Get_RecordBallHitRate(obj: usize) -> f32;
	fn GravityPickup_SetRecordBallHitRate(obj: usize, new_val: f32);
	fn SpecialPickup_BallGravity_TA_Get_LastRecordedBallHitTime(obj: usize) -> f32;
	fn GravityPickup_SetLastRecordedBallHitTime(obj: usize, new_val: f32);
	fn SpecialPickup_BallGravity_TA_Get_PrevBall(obj: usize) -> usize;
	fn GravityPickup_SetPrevBall(obj: usize, new_val: usize);
	fn SpecialPickup_BallGravity_TA_UpdateVisual(obj: usize);
	fn SpecialPickup_BallGravity_TA_ApplyForces(obj: usize, ActiveTime: f32);
	fn SpecialPickup_BallGravity_TA_PickupEnd(obj: usize);
	fn SpecialPickup_BallGravity_TA_PickupStart(obj: usize);

}