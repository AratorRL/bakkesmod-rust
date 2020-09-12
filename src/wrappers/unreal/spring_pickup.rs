use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct SpringPickupWrapper(pub usize);
impl_object!(SpringPickupWrapper);

impl SpringPickup for SpringPickupWrapper {}
impl TargetedPickup for SpringPickupWrapper {}
impl RumblePickupComponent for SpringPickupWrapper {}
impl CarComponent for SpringPickupWrapper {}
impl Actor for SpringPickupWrapper {}

pub trait SpringPickup : TargetedPickup {
    fn get_force(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_Force(self.addr())
        }
    }
    fn get_vertical_force(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_VerticalForce(self.addr())
        }
    }
    fn get_torque(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Spring_TA_Get_Torque(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_apply_relative_force(&self) -> bool {
        unsafe {
            SpecialPickup_Spring_TA_Get_bApplyRelativeForce(self.addr())
        }
    }
    fn get_b_apply_constant_force(&self) -> bool {
        unsafe {
            SpecialPickup_Spring_TA_Get_bApplyConstantForce(self.addr())
        }
    }
    fn get_b_break_constant_force_with_hit(&self) -> bool {
        unsafe {
            SpecialPickup_Spring_TA_Get_bBreakConstantForceWithHit(self.addr())
        }
    }
    fn get_b_apply_relative_constant_force(&self) -> bool {
        unsafe {
            SpecialPickup_Spring_TA_Get_bApplyRelativeConstantForce(self.addr())
        }
    }
    fn get_b_instant(&self) -> bool {
        unsafe {
            SpecialPickup_Spring_TA_Get_bInstant(self.addr())
        }
    }
    fn get_b_follow_after_hit(&self) -> bool {
        unsafe {
            SpecialPickup_Spring_TA_Get_bFollowAfterHit(self.addr())
        }
    }
    fn get_b_springed(&self) -> bool {
        unsafe {
            SpecialPickup_Spring_TA_Get_bSpringed(self.addr())
        }
    }
    fn get_relative_force_normal_direction(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_RelativeForceNormalDirection(self.addr())
        }
    }
    fn get_max_spring_length(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_MaxSpringLength(self.addr())
        }
    }
    fn get_constant_force(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_ConstantForce(self.addr())
        }
    }
    fn get_from_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Spring_TA_Get_FromOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_spring_mesh_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Spring_TA_Get_SpringMeshScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_spring_mesh_initial_size(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_SpringMeshInitialSize(self.addr())
        }
    }
    fn get_spring_rotation_offset(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            SpecialPickup_Spring_TA_Get_SpringRotationOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_hitting_mesh_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Spring_TA_Get_HittingMeshScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_hitting_mesh_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Spring_TA_Get_HittingMeshOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_hitting_rotation_offset(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            SpecialPickup_Spring_TA_Get_HittingRotationOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_hit_distance_offset(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_HitDistanceOffset(self.addr())
        }
    }
    fn get_after_spring_duration(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_AfterSpringDuration(self.addr())
        }
    }
    fn get_ball_hit_type(&self) -> u8 {
        unsafe {
            SpecialPickup_Spring_TA_Get_BallHitType(self.addr())
        }
    }
    fn get_min_spring_length(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_MinSpringLength(self.addr())
        }
    }
    fn get_welded_force_scalar(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_WeldedForceScalar(self.addr())
        }
    }
    fn get_welded_vertical_force(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_WeldedVerticalForce(self.addr())
        }
    }
    fn get_current_spring_length(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_CurrentSpringLength(self.addr())
        }
    }
    fn get_springed_time(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_SpringedTime(self.addr())
        }
    }
    fn get_after_spring_time(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_AfterSpringTime(self.addr())
        }
    }
    fn get_spring_to_time(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_SpringToTime(self.addr())
        }
    }
    fn get_spring_origin(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Spring_TA_Get_SpringOrigin(self.addr(), result_ptr);
            result
        }
    }
    fn get_springed_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Spring_TA_Get_SpringedLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_springed_normal(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Spring_TA_Get_SpringedNormal(self.addr(), result_ptr);
            result
        }
    }
    fn get_springed_length(&self) -> f32 {
        unsafe {
            SpecialPickup_Spring_TA_Get_SpringedLength(self.addr())
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_Spring_TA_PickupEnd(self.addr());
        }
    }
    fn handle_car_touched_ball(&self, ball: BallWrapper, other_car: CarWrapper, hit_type: u8) {
        unsafe {
            SpecialPickup_Spring_TA_HandleCarTouchedBall(self.addr(), ball.addr(), other_car.addr(), hit_type);
        }
    }
    fn scale_spring_mesh_to_location(&self, new_location: Vector, target_loction: Vector) {
        unsafe {
            let mut new_location = new_location;
            let new_location: *mut Vector = &mut new_location as *mut Vector;
            let mut target_loction = target_loction;
            let target_loction: *mut Vector = &mut target_loction as *mut Vector;
            SpecialPickup_Spring_TA_ScaleSpringMeshToLocation(self.addr(), new_location, target_loction);
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            SpecialPickup_Spring_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn get_relative_constant_force(&self, direction: Vector) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            let mut direction = direction;
            let direction: *mut Vector = &mut direction as *mut Vector;
            SpecialPickup_Spring_TA_GetRelativeConstantForce(self.addr(), direction, result_ptr);
            result
        }
    }
    fn get_relative_impulse(&self, direction: Vector) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            let mut direction = direction;
            let direction: *mut Vector = &mut direction as *mut Vector;
            SpecialPickup_Spring_TA_GetRelativeImpulse(self.addr(), direction, result_ptr);
            result
        }
    }
    fn do_spring(&self, b_first_hit: bool) {
        unsafe {
            SpecialPickup_Spring_TA_DoSpring(self.addr(), b_first_hit);
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_Spring_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_Spring_TA_Get_Force(obj: usize) -> f32;
    fn SpringPickup_SetForce(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_VerticalForce(obj: usize) -> f32;
    fn SpringPickup_SetVerticalForce(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_Torque(obj: usize, result: *mut Vector);
    fn SpringPickup_SetTorque(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Spring_TA_Get_bApplyRelativeForce(obj: usize) -> bool;
    fn SpringPickup_SetbApplyRelativeForce(obj: usize, new_val: bool);
    fn SpecialPickup_Spring_TA_Get_bApplyConstantForce(obj: usize) -> bool;
    fn SpringPickup_SetbApplyConstantForce(obj: usize, new_val: bool);
    fn SpecialPickup_Spring_TA_Get_bBreakConstantForceWithHit(obj: usize) -> bool;
    fn SpringPickup_SetbBreakConstantForceWithHit(obj: usize, new_val: bool);
    fn SpecialPickup_Spring_TA_Get_bApplyRelativeConstantForce(obj: usize) -> bool;
    fn SpringPickup_SetbApplyRelativeConstantForce(obj: usize, new_val: bool);
    fn SpecialPickup_Spring_TA_Get_bInstant(obj: usize) -> bool;
    fn SpringPickup_SetbInstant(obj: usize, new_val: bool);
    fn SpecialPickup_Spring_TA_Get_bFollowAfterHit(obj: usize) -> bool;
    fn SpringPickup_SetbFollowAfterHit(obj: usize, new_val: bool);
    fn SpecialPickup_Spring_TA_Get_bSpringed(obj: usize) -> bool;
    fn SpringPickup_SetbSpringed(obj: usize, new_val: bool);
    fn SpecialPickup_Spring_TA_Get_RelativeForceNormalDirection(obj: usize) -> f32;
    fn SpringPickup_SetRelativeForceNormalDirection(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_MaxSpringLength(obj: usize) -> f32;
    fn SpringPickup_SetMaxSpringLength(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_ConstantForce(obj: usize) -> f32;
    fn SpringPickup_SetConstantForce(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_FromOffset(obj: usize, result: *mut Vector);
    fn SpringPickup_SetFromOffset(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Spring_TA_Get_SpringMeshScale(obj: usize, result: *mut Vector);
    fn SpringPickup_SetSpringMeshScale(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Spring_TA_Get_SpringMeshInitialSize(obj: usize) -> f32;
    fn SpringPickup_SetSpringMeshInitialSize(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_SpringRotationOffset(obj: usize, result: *mut Rotator);
    fn SpringPickup_SetSpringRotationOffset(obj: usize, new_val: *mut Rotator);
    fn SpecialPickup_Spring_TA_Get_HittingMeshScale(obj: usize, result: *mut Vector);
    fn SpringPickup_SetHittingMeshScale(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Spring_TA_Get_HittingMeshOffset(obj: usize, result: *mut Vector);
    fn SpringPickup_SetHittingMeshOffset(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Spring_TA_Get_HittingRotationOffset(obj: usize, result: *mut Rotator);
    fn SpringPickup_SetHittingRotationOffset(obj: usize, new_val: *mut Rotator);
    fn SpecialPickup_Spring_TA_Get_HitDistanceOffset(obj: usize) -> f32;
    fn SpringPickup_SetHitDistanceOffset(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_AfterSpringDuration(obj: usize) -> f32;
    fn SpringPickup_SetAfterSpringDuration(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_BallHitType(obj: usize) -> u8;
    fn SpringPickup_SetBallHitType(obj: usize, new_val: u8);
    fn SpecialPickup_Spring_TA_Get_MinSpringLength(obj: usize) -> f32;
    fn SpringPickup_SetMinSpringLength(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_WeldedForceScalar(obj: usize) -> f32;
    fn SpringPickup_SetWeldedForceScalar(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_WeldedVerticalForce(obj: usize) -> f32;
    fn SpringPickup_SetWeldedVerticalForce(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_CurrentSpringLength(obj: usize) -> f32;
    fn SpringPickup_SetCurrentSpringLength(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_SpringedTime(obj: usize) -> f32;
    fn SpringPickup_SetSpringedTime(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_AfterSpringTime(obj: usize) -> f32;
    fn SpringPickup_SetAfterSpringTime(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_SpringToTime(obj: usize) -> f32;
    fn SpringPickup_SetSpringToTime(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_Get_SpringOrigin(obj: usize, result: *mut Vector);
    fn SpringPickup_SetSpringOrigin(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Spring_TA_Get_SpringedLocation(obj: usize, result: *mut Vector);
    fn SpringPickup_SetSpringedLocation(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Spring_TA_Get_SpringedNormal(obj: usize, result: *mut Vector);
    fn SpringPickup_SetSpringedNormal(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Spring_TA_Get_SpringedLength(obj: usize) -> f32;
    fn SpringPickup_SetSpringedLength(obj: usize, new_val: f32);
    fn SpecialPickup_Spring_TA_PickupEnd(obj: usize);
    fn SpecialPickup_Spring_TA_HandleCarTouchedBall(obj: usize, Ball: usize, OtherCar: usize, HitType: u8);
    fn SpecialPickup_Spring_TA_ScaleSpringMeshToLocation(obj: usize, NewLocation: *mut Vector, TargetLoction: *mut Vector);
    fn SpecialPickup_Spring_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn SpecialPickup_Spring_TA_GetRelativeConstantForce(obj: usize, Direction: *mut Vector, result: *mut Vector);
    fn SpecialPickup_Spring_TA_GetRelativeImpulse(obj: usize, Direction: *mut Vector, result: *mut Vector);
    fn SpecialPickup_Spring_TA_DoSpring(obj: usize, bFirstHit: bool);
    fn SpecialPickup_Spring_TA_PickupStart(obj: usize);

}