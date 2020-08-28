use crate::wrappers::*;
use crate::generated::*;

pub struct VelcroPickupWrapper(pub usize);
impl_object!(VelcroPickupWrapper);

impl VelcroPickup for VelcroPickupWrapper {}
impl RumblePickupComponent for VelcroPickupWrapper {}
impl CarComponent for VelcroPickupWrapper {}
impl Actor for VelcroPickupWrapper {}

pub trait VelcroPickup : RumblePickupComponent {
    fn get_ball_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_BallVelcro_TA_Get_BallOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_use_real_offset(&self) -> bool {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_bUseRealOffset(self.addr())
        }
    }
    fn get_b_hit(&self) -> bool {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_bHit(self.addr())
        }
    }
    fn get_b_broken(&self) -> bool {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_bBroken(self.addr())
        }
    }
    fn get_after_hit_duration(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_AfterHitDuration(self.addr())
        }
    }
    fn get_post_break_duration(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_PostBreakDuration(self.addr())
        }
    }
    fn get_min_break_force(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_MinBreakForce(self.addr())
        }
    }
    fn get_min_break_time(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_MinBreakTime(self.addr())
        }
    }
    fn get_check_last_touch_rate(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_CheckLastTouchRate(self.addr())
        }
    }
    fn get_welded_ball(&self) -> Option<BallWrapper> {
        unsafe {
            BallWrapper::try_new(SpecialPickup_BallVelcro_TA_Get_WeldedBall(self.addr()))
        }
    }
    fn get_old_ball_mass(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_OldBallMass(self.addr())
        }
    }
    fn get_attach_time(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_AttachTime(self.addr())
        }
    }
    fn get_last_touch_check_time(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_LastTouchCheckTime(self.addr())
        }
    }
    fn get_break_time(&self) -> f32 {
        unsafe {
            SpecialPickup_BallVelcro_TA_Get_BreakTime(self.addr())
        }
    }
    fn do_break(&self) {
        unsafe {
            SpecialPickup_BallVelcro_TA_DoBreak(self.addr());
        }
    }
    fn handle_car_touch(&self, in_ball: BallWrapper, in_car: CarWrapper, hit_type: u8) {
        unsafe {
            SpecialPickup_BallVelcro_TA_HandleCarTouch(self.addr(), in_ball.addr(), in_car.addr(), hit_type);
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_BallVelcro_TA_PickupEnd(self.addr());
        }
    }
    fn on_ball_hit(&self) {
        unsafe {
            SpecialPickup_BallVelcro_TA_OnBallHit(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_BallVelcro_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_BallVelcro_TA_Get_BallOffset(obj: usize, result: *mut Vector);
    fn VelcroPickup_SetBallOffset(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_BallVelcro_TA_Get_bUseRealOffset(obj: usize) -> bool;
    fn VelcroPickup_SetbUseRealOffset(obj: usize, new_val: bool);
    fn SpecialPickup_BallVelcro_TA_Get_bHit(obj: usize) -> bool;
    fn VelcroPickup_SetbHit(obj: usize, new_val: bool);
    fn SpecialPickup_BallVelcro_TA_Get_bBroken(obj: usize) -> bool;
    fn VelcroPickup_SetbBroken(obj: usize, new_val: bool);
    fn SpecialPickup_BallVelcro_TA_Get_AfterHitDuration(obj: usize) -> f32;
    fn VelcroPickup_SetAfterHitDuration(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_Get_PostBreakDuration(obj: usize) -> f32;
    fn VelcroPickup_SetPostBreakDuration(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_Get_MinBreakForce(obj: usize) -> f32;
    fn VelcroPickup_SetMinBreakForce(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_Get_MinBreakTime(obj: usize) -> f32;
    fn VelcroPickup_SetMinBreakTime(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_Get_CheckLastTouchRate(obj: usize) -> f32;
    fn VelcroPickup_SetCheckLastTouchRate(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_Get_WeldedBall(obj: usize) -> usize;
    fn VelcroPickup_SetWeldedBall(obj: usize, new_val: usize);
    fn SpecialPickup_BallVelcro_TA_Get_OldBallMass(obj: usize) -> f32;
    fn VelcroPickup_SetOldBallMass(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_Get_AttachTime(obj: usize) -> f32;
    fn VelcroPickup_SetAttachTime(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_Get_LastTouchCheckTime(obj: usize) -> f32;
    fn VelcroPickup_SetLastTouchCheckTime(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_Get_BreakTime(obj: usize) -> f32;
    fn VelcroPickup_SetBreakTime(obj: usize, new_val: f32);
    fn SpecialPickup_BallVelcro_TA_DoBreak(obj: usize);
    fn SpecialPickup_BallVelcro_TA_HandleCarTouch(obj: usize, InBall: usize, InCar: usize, HitType: u8);
    fn SpecialPickup_BallVelcro_TA_PickupEnd(obj: usize);
    fn SpecialPickup_BallVelcro_TA_OnBallHit(obj: usize);
    fn SpecialPickup_BallVelcro_TA_PickupStart(obj: usize);

}