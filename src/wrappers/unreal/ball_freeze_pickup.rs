use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct BallFreezePickupWrapper(pub usize);
impl_object!(BallFreezePickupWrapper);

impl BallFreezePickup for BallFreezePickupWrapper {}
impl TargetedPickup for BallFreezePickupWrapper {}
impl RumblePickupComponent for BallFreezePickupWrapper {}
impl CarComponent for BallFreezePickupWrapper {}
impl Actor for BallFreezePickupWrapper {}

pub trait BallFreezePickup : TargetedPickup {
    fn get_freeze_break_fx_archetype(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(SpecialPickup_BallFreeze_TA_Get_FreezeBreakFXArchetype(self.addr()))
        }
    }
    fn get_freeze_fx_archetype(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(SpecialPickup_BallFreeze_TA_Get_FreezeFXArchetype(self.addr()))
        }
    }
    fn get_b_maintain_momentum(&self) -> bool {
        unsafe {
            SpecialPickup_BallFreeze_TA_Get_bMaintainMomentum(self.addr())
        }
    }
    fn get_b_touched(&self) -> bool {
        unsafe {
            SpecialPickup_BallFreeze_TA_Get_bTouched(self.addr())
        }
    }
    fn get_time_to_stop(&self) -> f32 {
        unsafe {
            SpecialPickup_BallFreeze_TA_Get_TimeToStop(self.addr())
        }
    }
    fn get_stop_momentum_percentage(&self) -> f32 {
        unsafe {
            SpecialPickup_BallFreeze_TA_Get_StopMomentumPercentage(self.addr())
        }
    }
    fn get_ball(&self) -> Option<BallWrapper> {
        unsafe {
            BallWrapper::try_new(SpecialPickup_BallFreeze_TA_Get_Ball(self.addr()))
        }
    }
    fn get_orig_linear_velocity(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_BallFreeze_TA_Get_OrigLinearVelocity(self.addr(), result_ptr);
            result
        }
    }
    fn get_orig_angular_velocity(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_BallFreeze_TA_Get_OrigAngularVelocity(self.addr(), result_ptr);
            result
        }
    }
    fn get_orig_speed(&self) -> f32 {
        unsafe {
            SpecialPickup_BallFreeze_TA_Get_OrigSpeed(self.addr())
        }
    }
    fn get_rep_orig_speed(&self) -> f32 {
        unsafe {
            SpecialPickup_BallFreeze_TA_Get_RepOrigSpeed(self.addr())
        }
    }
    fn get_freeze_fx(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(SpecialPickup_BallFreeze_TA_Get_FreezeFX(self.addr()))
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_BallFreeze_TA_PickupEnd(self.addr());
        }
    }
    fn handle_ball_exploded(&self, in_ball: BallWrapper) {
        unsafe {
            SpecialPickup_BallFreeze_TA_HandleBallExploded(self.addr(), in_ball.addr());
        }
    }
    fn handle_ball_hit(&self, in_ball: BallWrapper, in_car: CarWrapper, hit_type: u8) {
        unsafe {
            SpecialPickup_BallFreeze_TA_HandleBallHit(self.addr(), in_ball.addr(), in_car.addr(), hit_type);
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            SpecialPickup_BallFreeze_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn on_target_changed(&self) {
        unsafe {
            SpecialPickup_BallFreeze_TA_OnTargetChanged(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_BallFreeze_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_BallFreeze_TA_Get_FreezeBreakFXArchetype(obj: usize) -> usize;
    fn BallFreezePickup_SetFreezeBreakFXArchetype(obj: usize, new_val: usize);
    fn SpecialPickup_BallFreeze_TA_Get_FreezeFXArchetype(obj: usize) -> usize;
    fn BallFreezePickup_SetFreezeFXArchetype(obj: usize, new_val: usize);
    fn SpecialPickup_BallFreeze_TA_Get_bMaintainMomentum(obj: usize) -> bool;
    fn BallFreezePickup_SetbMaintainMomentum(obj: usize, new_val: bool);
    fn SpecialPickup_BallFreeze_TA_Get_bTouched(obj: usize) -> bool;
    fn BallFreezePickup_SetbTouched(obj: usize, new_val: bool);
    fn SpecialPickup_BallFreeze_TA_Get_TimeToStop(obj: usize) -> f32;
    fn BallFreezePickup_SetTimeToStop(obj: usize, new_val: f32);
    fn SpecialPickup_BallFreeze_TA_Get_StopMomentumPercentage(obj: usize) -> f32;
    fn BallFreezePickup_SetStopMomentumPercentage(obj: usize, new_val: f32);
    fn SpecialPickup_BallFreeze_TA_Get_Ball(obj: usize) -> usize;
    fn BallFreezePickup_SetBall(obj: usize, new_val: usize);
    fn SpecialPickup_BallFreeze_TA_Get_OrigLinearVelocity(obj: usize, result: *mut Vector);
    fn BallFreezePickup_SetOrigLinearVelocity(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_BallFreeze_TA_Get_OrigAngularVelocity(obj: usize, result: *mut Vector);
    fn BallFreezePickup_SetOrigAngularVelocity(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_BallFreeze_TA_Get_OrigSpeed(obj: usize) -> f32;
    fn BallFreezePickup_SetOrigSpeed(obj: usize, new_val: f32);
    fn SpecialPickup_BallFreeze_TA_Get_RepOrigSpeed(obj: usize) -> f32;
    fn BallFreezePickup_SetRepOrigSpeed(obj: usize, new_val: f32);
    fn SpecialPickup_BallFreeze_TA_Get_FreezeFX(obj: usize) -> usize;
    fn BallFreezePickup_SetFreezeFX(obj: usize, new_val: usize);
    fn SpecialPickup_BallFreeze_TA_PickupEnd(obj: usize);
    fn SpecialPickup_BallFreeze_TA_HandleBallExploded(obj: usize, InBall: usize);
    fn SpecialPickup_BallFreeze_TA_HandleBallHit(obj: usize, InBall: usize, InCar: usize, HitType: u8);
    fn SpecialPickup_BallFreeze_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn SpecialPickup_BallFreeze_TA_OnTargetChanged(obj: usize);
    fn SpecialPickup_BallFreeze_TA_PickupStart(obj: usize);

}