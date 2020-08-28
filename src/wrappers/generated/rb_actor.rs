use crate::wrappers::*;
use super::*;

pub struct RBActorWrapper(pub usize);
impl_object!(RBActorWrapper);

impl RBActor for RBActorWrapper {}
impl Actor for RBActorWrapper {}

pub trait RBActor : Actor {
    fn get_max_linear_speed(&self) -> f32 {
        unsafe {
            RBActor_TA_Get_MaxLinearSpeed(self.addr())
        }
    }
    fn get_max_angular_speed(&self) -> f32 {
        unsafe {
            RBActor_TA_Get_MaxAngularSpeed(self.addr())
        }
    }
    fn get_b_disable_sleeping(&self) -> bool {
        unsafe {
            RBActor_TA_Get_bDisableSleeping(self.addr())
        }
    }
    fn get_b_replay_actor(&self) -> bool {
        unsafe {
            RBActor_TA_Get_bReplayActor(self.addr())
        }
    }
    fn get_b_frozen(&self) -> bool {
        unsafe {
            RBActor_TA_Get_bFrozen(self.addr())
        }
    }
    fn get_b_ignore_syncing(&self) -> bool {
        unsafe {
            RBActor_TA_Get_bIgnoreSyncing(self.addr())
        }
    }
    fn get_b_phys_initialized(&self) -> bool {
        unsafe {
            RBActor_TA_Get_bPhysInitialized(self.addr())
        }
    }
    fn get_old_rb_state(&self) -> ReplicatedRBState {
        unsafe {
            let mut result = ReplicatedRBState::new();
            let result_ptr: *mut ReplicatedRBState = &mut result as *mut ReplicatedRBState;
            RBActor_TA_Get_OldRBState(self.addr(), result_ptr);
            result
        }
    }
    fn get_rb_state(&self) -> ReplicatedRBState {
        unsafe {
            let mut result = ReplicatedRBState::new();
            let result_ptr: *mut ReplicatedRBState = &mut result as *mut ReplicatedRBState;
            RBActor_TA_Get_RBState(self.addr(), result_ptr);
            result
        }
    }
    fn get_replicated_rb_state(&self) -> ReplicatedRBState {
        unsafe {
            let mut result = ReplicatedRBState::new();
            let result_ptr: *mut ReplicatedRBState = &mut result as *mut ReplicatedRBState;
            RBActor_TA_Get_ReplicatedRBState(self.addr(), result_ptr);
            result
        }
    }
    fn get_client_correction_rb_state(&self) -> ReplicatedRBState {
        unsafe {
            let mut result = ReplicatedRBState::new();
            let result_ptr: *mut ReplicatedRBState = &mut result as *mut ReplicatedRBState;
            RBActor_TA_Get_ClientCorrectionRBState(self.addr(), result_ptr);
            result
        }
    }
    fn get_world_contact(&self) -> WorldContactData {
        unsafe {
            let mut result = WorldContactData::new();
            let result_ptr: *mut WorldContactData = &mut result as *mut WorldContactData;
            RBActor_TA_Get_WorldContact(self.addr(), result_ptr);
            result
        }
    }
    fn get_sync_error_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            RBActor_TA_Get_SyncErrorLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_sync_error_angle(&self) -> f32 {
        unsafe {
            RBActor_TA_Get_SyncErrorAngle(self.addr())
        }
    }
    fn get_sync_error_axis(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            RBActor_TA_Get_SyncErrorAxis(self.addr(), result_ptr);
            result
        }
    }
    fn get_fx_actor_archetype(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(RBActor_TA_Get_FXActorArchetype(self.addr()))
        }
    }
    fn get_fx_actor(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(RBActor_TA_Get_FXActor(self.addr()))
        }
    }
    fn get_last_rb_collisions_frame(&self) -> i32 {
        unsafe {
            RBActor_TA_Get_LastRBCollisionsFrame(self.addr())
        }
    }
    fn get_welded_actor(&self) -> Option<RBActorWrapper> {
        unsafe {
            RBActorWrapper::try_new(RBActor_TA_Get_WeldedActor(self.addr()))
        }
    }
    fn get_welded_to(&self) -> Option<RBActorWrapper> {
        unsafe {
            RBActorWrapper::try_new(RBActor_TA_Get_WeldedTo(self.addr()))
        }
    }
    fn get_pre_weld_mass(&self) -> f32 {
        unsafe {
            RBActor_TA_Get_PreWeldMass(self.addr())
        }
    }
    fn set_mass(&self, new_mass: f32) {
        unsafe {
            RBActor_TA_SetMass(self.addr(), new_mass);
        }
    }
    fn set_constrained3_d(&self, linear_lower: Vector, linear_upper: Vector, angular_lower: Vector, angular_upper: Vector) {
        unsafe {
            let mut linear_lower = linear_lower;
            let linear_lower: *mut Vector = &mut linear_lower as *mut Vector;
            let mut linear_upper = linear_upper;
            let linear_upper: *mut Vector = &mut linear_upper as *mut Vector;
            let mut angular_lower = angular_lower;
            let angular_lower: *mut Vector = &mut angular_lower as *mut Vector;
            let mut angular_upper = angular_upper;
            let angular_upper: *mut Vector = &mut angular_upper as *mut Vector;
            RBActor_TA_SetConstrained3D(self.addr(), linear_lower, linear_upper, angular_lower, angular_upper);
        }
    }
    fn set_constrained2_d(&self, b_constrain2_d: bool) {
        unsafe {
            RBActor_TA_SetConstrained2D(self.addr(), b_constrain2_d);
        }
    }
    fn set_physics_state(&self, new_state: ReplicatedRBState) {
        unsafe {
            let mut new_state = new_state;
            let new_state: *mut ReplicatedRBState = &mut new_state as *mut ReplicatedRBState;
            RBActor_TA_SetPhysicsState(self.addr(), new_state);
        }
    }
    fn set_frozen(&self, b_enabled: bool) {
        unsafe {
            RBActor_TA_SetFrozen(self.addr(), b_enabled);
        }
    }
    fn set_max_angular_speed2(&self, new_max_speed: f32) {
        unsafe {
            RBActor_TA_SetMaxAngularSpeed2(self.addr(), new_max_speed);
        }
    }
    fn set_max_linear_speed2(&self, new_max_speed: f32) {
        unsafe {
            RBActor_TA_SetMaxLinearSpeed2(self.addr(), new_max_speed);
        }
    }
    fn un_weld_rb_actor(&self, other: RBActorWrapper) {
        unsafe {
            RBActor_TA_UnWeldRBActor(self.addr(), other.addr());
        }
    }
    fn weld_rb_actor(&self, other: RBActorWrapper, weld_offset: Vector, weld_rotation: Rotator) {
        unsafe {
            let mut weld_offset = weld_offset;
            let weld_offset: *mut Vector = &mut weld_offset as *mut Vector;
            let mut weld_rotation = weld_rotation;
            let weld_rotation: *mut Rotator = &mut weld_rotation as *mut Rotator;
            RBActor_TA_WeldRBActor(self.addr(), other.addr(), weld_offset, weld_rotation);
        }
    }
    fn re_init_rb_phys(&self) {
        unsafe {
            RBActor_TA_ReInitRBPhys(self.addr());
        }
    }
    fn terminate_rb_phys(&self) {
        unsafe {
            RBActor_TA_TerminateRBPhys(self.addr());
        }
    }
    fn get_current_rb_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            RBActor_TA_GetCurrentRBLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_current_rb_state(&self) -> ReplicatedRBState {
        unsafe {
            let mut result = ReplicatedRBState::new();
            let result_ptr: *mut ReplicatedRBState = &mut result as *mut ReplicatedRBState;
            RBActor_TA_GetCurrentRBState(self.addr(), result_ptr);
            result
        }
    }
    fn get_physics_frame(&self) -> i32 {
        unsafe {
            RBActor_TA_GetPhysicsFrame(self.addr())
        }
    }
    fn get_physics_time(&self) -> f32 {
        unsafe {
            RBActor_TA_GetPhysicsTime(self.addr())
        }
    }
    fn init_ak(&self) {
        unsafe {
            RBActor_TA_InitAk(self.addr());
        }
    }

}

extern "C" {
    fn RBActor_TA_Get_MaxLinearSpeed(obj: usize) -> f32;
    fn RBActorWrapper_SetMaxLinearSpeed(obj: usize, new_val: f32);
    fn RBActor_TA_Get_MaxAngularSpeed(obj: usize) -> f32;
    fn RBActorWrapper_SetMaxAngularSpeed(obj: usize, new_val: f32);
    fn RBActor_TA_Get_bDisableSleeping(obj: usize) -> bool;
    fn RBActorWrapper_SetbDisableSleeping(obj: usize, new_val: bool);
    fn RBActor_TA_Get_bReplayActor(obj: usize) -> bool;
    fn RBActorWrapper_SetbReplayActor(obj: usize, new_val: bool);
    fn RBActor_TA_Get_bFrozen(obj: usize) -> bool;
    fn RBActorWrapper_SetbFrozen(obj: usize, new_val: bool);
    fn RBActor_TA_Get_bIgnoreSyncing(obj: usize) -> bool;
    fn RBActorWrapper_SetbIgnoreSyncing(obj: usize, new_val: bool);
    fn RBActor_TA_Get_bPhysInitialized(obj: usize) -> bool;
    fn RBActor_TA_Get_OldRBState(obj: usize, result: *mut ReplicatedRBState);
    fn RBActorWrapper_SetOldRBState(obj: usize, new_val: *mut ReplicatedRBState);
    fn RBActor_TA_Get_RBState(obj: usize, result: *mut ReplicatedRBState);
    fn RBActorWrapper_SetRBState(obj: usize, new_val: *mut ReplicatedRBState);
    fn RBActor_TA_Get_ReplicatedRBState(obj: usize, result: *mut ReplicatedRBState);
    fn RBActorWrapper_SetReplicatedRBState(obj: usize, new_val: *mut ReplicatedRBState);
    fn RBActor_TA_Get_ClientCorrectionRBState(obj: usize, result: *mut ReplicatedRBState);
    fn RBActorWrapper_SetClientCorrectionRBState(obj: usize, new_val: *mut ReplicatedRBState);
    fn RBActor_TA_Get_WorldContact(obj: usize, result: *mut WorldContactData);
    fn RBActorWrapper_SetWorldContact(obj: usize, new_val: *mut WorldContactData);
    fn RBActor_TA_Get_SyncErrorLocation(obj: usize, result: *mut Vector);
    fn RBActor_TA_Get_SyncErrorAngle(obj: usize) -> f32;
    fn RBActor_TA_Get_SyncErrorAxis(obj: usize, result: *mut Vector);
    fn RBActor_TA_Get_FXActorArchetype(obj: usize) -> usize;
    fn RBActorWrapper_SetFXActorArchetype(obj: usize, new_val: usize);
    fn RBActor_TA_Get_FXActor(obj: usize) -> usize;
    fn RBActorWrapper_SetFXActor(obj: usize, new_val: usize);
    fn RBActor_TA_Get_LastRBCollisionsFrame(obj: usize) -> i32;
    fn RBActor_TA_Get_WeldedActor(obj: usize) -> usize;
    fn RBActor_TA_Get_WeldedTo(obj: usize) -> usize;
    fn RBActor_TA_Get_PreWeldMass(obj: usize) -> f32;
    fn RBActor_TA_SetMass(obj: usize, NewMass: f32);
    fn RBActor_TA_SetConstrained3D(obj: usize, LinearLower: *mut Vector, LinearUpper: *mut Vector, AngularLower: *mut Vector, AngularUpper: *mut Vector);
    fn RBActor_TA_SetConstrained2D(obj: usize, bConstrain2D: bool);
    fn RBActor_TA_SetPhysicsState(obj: usize, NewState: *mut ReplicatedRBState);
    fn RBActor_TA_SetFrozen(obj: usize, bEnabled: bool);
    fn RBActor_TA_SetMaxAngularSpeed2(obj: usize, NewMaxSpeed: f32);
    fn RBActor_TA_SetMaxLinearSpeed2(obj: usize, NewMaxSpeed: f32);
    fn RBActor_TA_UnWeldRBActor(obj: usize, Other: usize);
    fn RBActor_TA_WeldRBActor(obj: usize, Other: usize, WeldOffset: *mut Vector, WeldRotation: *mut Rotator);
    fn RBActor_TA_ReInitRBPhys(obj: usize);
    fn RBActor_TA_TerminateRBPhys(obj: usize);
    fn RBActor_TA_GetCurrentRBLocation(obj: usize, result: *mut Vector);
    fn RBActor_TA_GetCurrentRBState(obj: usize, result: *mut ReplicatedRBState);
    fn RBActor_TA_GetPhysicsFrame(obj: usize) -> i32;
    fn RBActor_TA_GetPhysicsTime(obj: usize) -> f32;
    fn RBActor_TA_InitAk(obj: usize);

}