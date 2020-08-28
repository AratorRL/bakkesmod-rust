use crate::wrappers::*;
use super::*;

pub struct GrapplingHookPickupWrapper(pub usize);
impl_object!(GrapplingHookPickupWrapper);

impl GrapplingHookPickup for GrapplingHookPickupWrapper {}
impl TargetedPickup for GrapplingHookPickupWrapper {}
impl RumblePickupComponent for GrapplingHookPickupWrapper {}
impl CarComponent for GrapplingHookPickupWrapper {}
impl Actor for GrapplingHookPickupWrapper {}

pub trait GrapplingHookPickup : TargetedPickup {
    fn get_impulse(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_Impulse(self.addr())
        }
    }
    fn get_force(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_Force(self.addr())
        }
    }
    fn get_max_rope_length(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_MaxRopeLength(self.addr())
        }
    }
    fn get_prediction_speed(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_PredictionSpeed(self.addr())
        }
    }
    fn get_b_deactivate_on_touch(&self) -> bool {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_bDeactivateOnTouch(self.addr())
        }
    }
    fn get_b_instant(&self) -> bool {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_bInstant(self.addr())
        }
    }
    fn get_b_blocked(&self) -> bool {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_bBlocked(self.addr())
        }
    }
    fn get_b_attached_to_ball(&self) -> bool {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_bAttachedToBall(self.addr())
        }
    }
    fn get_rope_mesh_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_GrapplingHook_TA_Get_RopeMeshScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_rope_mesh_initial_size(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_RopeMeshInitialSize(self.addr())
        }
    }
    fn get_rope_rotation_offset(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            SpecialPickup_GrapplingHook_TA_Get_RopeRotationOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_hook_mesh_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_GrapplingHook_TA_Get_HookMeshScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_hook_mesh_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_GrapplingHook_TA_Get_HookMeshOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_hook_rotation_offset(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            SpecialPickup_GrapplingHook_TA_Get_HookRotationOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_hit_distance_offset(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_HitDistanceOffset(self.addr())
        }
    }
    fn get_after_attach_duration(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_AfterAttachDuration(self.addr())
        }
    }
    fn get_blocked_required_move_distance(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_BlockedRequiredMoveDistance(self.addr())
        }
    }
    fn get_blocked_required_move_time(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_BlockedRequiredMoveTime(self.addr())
        }
    }
    fn get_blocked_start_time(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_BlockedStartTime(self.addr())
        }
    }
    fn get_blocked_start_pos(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_GrapplingHook_TA_Get_BlockedStartPos(self.addr(), result_ptr);
            result
        }
    }
    fn get_ball(&self) -> Option<BallWrapper> {
        unsafe {
            BallWrapper::try_new(SpecialPickup_GrapplingHook_TA_Get_Ball(self.addr()))
        }
    }
    fn get_rope_origin(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_GrapplingHook_TA_Get_RopeOrigin(self.addr(), result_ptr);
            result
        }
    }
    fn get_rope_to_time(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_RopeToTime(self.addr())
        }
    }
    fn get_current_rope_length(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_CurrentRopeLength(self.addr())
        }
    }
    fn get_attach_time(&self) -> f32 {
        unsafe {
            SpecialPickup_GrapplingHook_TA_Get_AttachTime(self.addr())
        }
    }
    fn handle_ball_exploded(&self, in_ball: BallWrapper) {
        unsafe {
            SpecialPickup_GrapplingHook_TA_HandleBallExploded(self.addr(), in_ball.addr());
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_GrapplingHook_TA_PickupEnd(self.addr());
        }
    }
    fn scale_mesh_to_location(&self, new_location: Vector, target_location: Vector) {
        unsafe {
            let mut new_location = new_location;
            let new_location: *mut Vector = &mut new_location as *mut Vector;
            let mut target_location = target_location;
            let target_location: *mut Vector = &mut target_location as *mut Vector;
            SpecialPickup_GrapplingHook_TA_ScaleMeshToLocation(self.addr(), new_location, target_location);
        }
    }
    fn get_predicted_ball_location(&self, in_ball: BallWrapper) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_GrapplingHook_TA_GetPredictedBallLocation(self.addr(), in_ball.addr(), result_ptr);
            result
        }
    }
    fn get_targeted_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_GrapplingHook_TA_GetTargetedLocation(self.addr(), result_ptr);
            result
        }
    }
    fn update_visual(&self, delta_time: f32) {
        unsafe {
            SpecialPickup_GrapplingHook_TA_UpdateVisual(self.addr(), delta_time);
        }
    }
    fn pickup_tick(&self, delta_time: f32) {
        unsafe {
            SpecialPickup_GrapplingHook_TA_PickupTick(self.addr(), delta_time);
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            SpecialPickup_GrapplingHook_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn do_attach(&self) {
        unsafe {
            SpecialPickup_GrapplingHook_TA_DoAttach(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_GrapplingHook_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_GrapplingHook_TA_Get_Impulse(obj: usize) -> f32;
    fn GrapplingHookPickup_SetImpulse(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_Force(obj: usize) -> f32;
    fn GrapplingHookPickup_SetForce(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_MaxRopeLength(obj: usize) -> f32;
    fn GrapplingHookPickup_SetMaxRopeLength(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_PredictionSpeed(obj: usize) -> f32;
    fn GrapplingHookPickup_SetPredictionSpeed(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_bDeactivateOnTouch(obj: usize) -> bool;
    fn GrapplingHookPickup_SetbDeactivateOnTouch(obj: usize, new_val: bool);
    fn SpecialPickup_GrapplingHook_TA_Get_bInstant(obj: usize) -> bool;
    fn GrapplingHookPickup_SetbInstant(obj: usize, new_val: bool);
    fn SpecialPickup_GrapplingHook_TA_Get_bBlocked(obj: usize) -> bool;
    fn GrapplingHookPickup_SetbBlocked(obj: usize, new_val: bool);
    fn SpecialPickup_GrapplingHook_TA_Get_bAttachedToBall(obj: usize) -> bool;
    fn GrapplingHookPickup_SetbAttachedToBall(obj: usize, new_val: bool);
    fn SpecialPickup_GrapplingHook_TA_Get_RopeMeshScale(obj: usize, result: *mut Vector);
    fn GrapplingHookPickup_SetRopeMeshScale(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_GrapplingHook_TA_Get_RopeMeshInitialSize(obj: usize) -> f32;
    fn GrapplingHookPickup_SetRopeMeshInitialSize(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_RopeRotationOffset(obj: usize, result: *mut Rotator);
    fn GrapplingHookPickup_SetRopeRotationOffset(obj: usize, new_val: *mut Rotator);
    fn SpecialPickup_GrapplingHook_TA_Get_HookMeshScale(obj: usize, result: *mut Vector);
    fn GrapplingHookPickup_SetHookMeshScale(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_GrapplingHook_TA_Get_HookMeshOffset(obj: usize, result: *mut Vector);
    fn GrapplingHookPickup_SetHookMeshOffset(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_GrapplingHook_TA_Get_HookRotationOffset(obj: usize, result: *mut Rotator);
    fn GrapplingHookPickup_SetHookRotationOffset(obj: usize, new_val: *mut Rotator);
    fn SpecialPickup_GrapplingHook_TA_Get_HitDistanceOffset(obj: usize) -> f32;
    fn GrapplingHookPickup_SetHitDistanceOffset(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_AfterAttachDuration(obj: usize) -> f32;
    fn GrapplingHookPickup_SetAfterAttachDuration(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_BlockedRequiredMoveDistance(obj: usize) -> f32;
    fn GrapplingHookPickup_SetBlockedRequiredMoveDistance(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_BlockedRequiredMoveTime(obj: usize) -> f32;
    fn GrapplingHookPickup_SetBlockedRequiredMoveTime(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_BlockedStartTime(obj: usize) -> f32;
    fn GrapplingHookPickup_SetBlockedStartTime(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_BlockedStartPos(obj: usize, result: *mut Vector);
    fn GrapplingHookPickup_SetBlockedStartPos(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_GrapplingHook_TA_Get_Ball(obj: usize) -> usize;
    fn GrapplingHookPickup_SetBall(obj: usize, new_val: usize);
    fn SpecialPickup_GrapplingHook_TA_Get_RopeOrigin(obj: usize, result: *mut Vector);
    fn GrapplingHookPickup_SetRopeOrigin(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_GrapplingHook_TA_Get_RopeToTime(obj: usize) -> f32;
    fn GrapplingHookPickup_SetRopeToTime(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_CurrentRopeLength(obj: usize) -> f32;
    fn GrapplingHookPickup_SetCurrentRopeLength(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_Get_AttachTime(obj: usize) -> f32;
    fn GrapplingHookPickup_SetAttachTime(obj: usize, new_val: f32);
    fn SpecialPickup_GrapplingHook_TA_HandleBallExploded(obj: usize, InBall: usize);
    fn SpecialPickup_GrapplingHook_TA_PickupEnd(obj: usize);
    fn SpecialPickup_GrapplingHook_TA_ScaleMeshToLocation(obj: usize, NewLocation: *mut Vector, TargetLocation: *mut Vector);
    fn SpecialPickup_GrapplingHook_TA_GetPredictedBallLocation(obj: usize, InBall: usize, result: *mut Vector);
    fn SpecialPickup_GrapplingHook_TA_GetTargetedLocation(obj: usize, result: *mut Vector);
    fn SpecialPickup_GrapplingHook_TA_UpdateVisual(obj: usize, DeltaTime: f32);
    fn SpecialPickup_GrapplingHook_TA_PickupTick(obj: usize, DeltaTime: f32);
    fn SpecialPickup_GrapplingHook_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn SpecialPickup_GrapplingHook_TA_DoAttach(obj: usize);
    fn SpecialPickup_GrapplingHook_TA_PickupStart(obj: usize);

}