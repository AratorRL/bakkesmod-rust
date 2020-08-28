use crate::wrappers::*;
use super::*;

pub struct CarWrapper(pub usize);
impl_object!(CarWrapper);

impl Car for CarWrapper {}
impl Vehicle for CarWrapper {}
impl RBActor for CarWrapper {}
impl Actor for CarWrapper {}

pub trait Car : Vehicle {
    fn demolish(&self) {
        unsafe { Car_Demolish(self.addr()); }
    }

    fn get_default_car_components(&self) -> RLArray<CarComponentWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            Car_TA_Get_DefaultCarComponents(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_flip_component(&self) -> Option<FlipCarComponentWrapper> {
        unsafe {
            FlipCarComponentWrapper::try_new(Car_TA_Get_FlipComponent(self.addr()))
        }
    }
    fn get_demolish_target(&self) -> u8 {
        unsafe {
            Car_TA_Get_DemolishTarget(self.addr())
        }
    }
    fn get_demolish_speed(&self) -> u8 {
        unsafe {
            Car_TA_Get_DemolishSpeed(self.addr())
        }
    }
    fn get_b_loadout_set(&self) -> bool {
        unsafe {
            Car_TA_Get_bLoadoutSet(self.addr())
        }
    }
    fn get_b_demolish_on_opposing_ground(&self) -> bool {
        unsafe {
            Car_TA_Get_bDemolishOnOpposingGround(self.addr())
        }
    }
    fn get_b_was_on_opposing_ground(&self) -> bool {
        unsafe {
            Car_TA_Get_bWasOnOpposingGround(self.addr())
        }
    }
    fn get_b_demolish_on_goal_zone(&self) -> bool {
        unsafe {
            Car_TA_Get_bDemolishOnGoalZone(self.addr())
        }
    }
    fn get_b_was_in_goal_zone(&self) -> bool {
        unsafe {
            Car_TA_Get_bWasInGoalZone(self.addr())
        }
    }
    fn get_b_override_handbrake_on(&self) -> bool {
        unsafe {
            Car_TA_Get_bOverrideHandbrakeOn(self.addr())
        }
    }
    fn get_b_override_boost_on(&self) -> bool {
        unsafe {
            Car_TA_Get_bOverrideBoostOn(self.addr())
        }
    }
    fn get_exit_fx_archetype(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(Car_TA_Get_ExitFXArchetype(self.addr()))
        }
    }
    fn get_max_time_for_dodge(&self) -> f32 {
        unsafe {
            Car_TA_Get_MaxTimeForDodge(self.addr())
        }
    }
    fn get_last_wheels_hit_ball_time(&self) -> f32 {
        unsafe {
            Car_TA_Get_LastWheelsHitBallTime(self.addr())
        }
    }
    fn get_replicated_car_scale(&self) -> f32 {
        unsafe {
            Car_TA_Get_ReplicatedCarScale(self.addr())
        }
    }
    fn get_body_fx_actor(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(Car_TA_Get_BodyFXActor(self.addr()))
        }
    }
    fn get_attacker_pri(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(Car_TA_Get_AttackerPRI(self.addr()))
        }
    }
    fn get_mouse_accel(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Car_TA_Get_MouseAccel(self.addr(), result_ptr);
            result
        }
    }
    fn get_mouse_air_accel(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Car_TA_Get_MouseAirAccel(self.addr(), result_ptr);
            result
        }
    }
    fn get_attached_pickup(&self) -> Option<RumblePickupComponentWrapper> {
        unsafe {
            RumblePickupComponentWrapper::try_new(Car_TA_Get_AttachedPickup(self.addr()))
        }
    }
    fn get_replay_focus_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Car_TA_Get_ReplayFocusOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_added_ball_force_multiplier(&self) -> f32 {
        unsafe {
            Car_TA_Get_AddedBallForceMultiplier(self.addr())
        }
    }
    fn get_added_car_force_multiplier(&self) -> f32 {
        unsafe {
            Car_TA_Get_AddedCarForceMultiplier(self.addr())
        }
    }
    fn get_game_event(&self) -> Option<GameEventWrapper> {
        unsafe {
            GameEventWrapper::try_new(Car_TA_Get_GameEvent(self.addr()))
        }
    }
    fn get_max_drive_backwards_speed(&self) -> f32 {
        unsafe {
            Car_TA_GetMaxDriveBackwardsSpeed(self.addr())
        }
    }
    fn get_max_drive_forward_speed(&self) -> f32 {
        unsafe {
            Car_TA_GetMaxDriveForwardSpeed(self.addr())
        }
    }
    fn get_replay_focus_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Car_TA_GetReplayFocusLocation(self.addr(), result_ptr);
            result
        }
    }
    fn on_pickup_changed(&self, in_pickup: RumblePickupComponentWrapper) {
        unsafe {
            Car_TA_OnPickupChanged(self.addr(), in_pickup.addr());
        }
    }
    fn set_attached_pickup2(&self, in_pickup: RumblePickupComponentWrapper) {
        unsafe {
            Car_TA_SetAttachedPickup2(self.addr(), in_pickup.addr());
        }
    }
    fn enable_podium_mode(&self) {
        unsafe {
            Car_TA_EnablePodiumMode(self.addr());
        }
    }
    fn copy_push_factor_curve(&self) {
        unsafe {
            Car_TA_CopyPushFactorCurve(self.addr());
        }
    }
    fn update_ball_indicator(&self) {
        unsafe {
            Car_TA_UpdateBallIndicator(self.addr());
        }
    }
    fn fell_out_of_world(&self) {
        unsafe {
            Car_TA_FellOutOfWorld(self.addr());
        }
    }
    fn demolish_destroy_timer(&self) {
        unsafe {
            Car_TA_DemolishDestroyTimer(self.addr());
        }
    }
    fn teleport(&self, spawn_location: Vector, spawn_rotation: Rotator, b_stop_velocity: bool, b_update_rotation: bool, extra_force: f32) -> bool {
        unsafe {
            let mut spawn_location = spawn_location;
            let spawn_location: *mut Vector = &mut spawn_location as *mut Vector;
            let mut spawn_rotation = spawn_rotation;
            let spawn_rotation: *mut Rotator = &mut spawn_rotation as *mut Rotator;
            Car_TA_Teleport(self.addr(), spawn_location, spawn_rotation, b_stop_velocity, b_update_rotation, extra_force)
        }
    }
    fn on_jump_released(&self) {
        unsafe {
            Car_TA_OnJumpReleased(self.addr());
        }
    }
    fn on_jump_pressed(&self) {
        unsafe {
            Car_TA_OnJumpPressed(self.addr());
        }
    }
    fn can_demolish(&self, hit_car: CarWrapper) -> bool {
        unsafe {
            Car_TA_CanDemolish(self.addr(), hit_car.addr())
        }
    }
    fn should_demolish(&self, hit_car: CarWrapper, hit_location: Vector, hit_normal: Vector, result: u8) -> bool {
        unsafe {
            let mut hit_location = hit_location;
            let hit_location: *mut Vector = &mut hit_location as *mut Vector;
            let mut hit_normal = hit_normal;
            let hit_normal: *mut Vector = &mut hit_normal as *mut Vector;
            Car_TA_ShouldDemolish(self.addr(), hit_car.addr(), hit_location, hit_normal, result)
        }
    }
    fn apply_car_impact_forces(&self, other_car: CarWrapper, hit_location: Vector, hit_normal: Vector) -> u8 {
        unsafe {
            let mut hit_location = hit_location;
            let hit_location: *mut Vector = &mut hit_location as *mut Vector;
            let mut hit_normal = hit_normal;
            let hit_normal: *mut Vector = &mut hit_normal as *mut Vector;
            Car_TA_ApplyCarImpactForces(self.addr(), other_car.addr(), hit_location, hit_normal)
        }
    }
    fn is_bumper_hit(&self, other_car: CarWrapper, hit_location: Vector) -> bool {
        unsafe {
            let mut hit_location = hit_location;
            let hit_location: *mut Vector = &mut hit_location as *mut Vector;
            Car_TA_IsBumperHit(self.addr(), other_car.addr(), hit_location)
        }
    }
    fn apply_ball_impact_forces(&self, ball: BallWrapper, hit_location: Vector) {
        unsafe {
            let mut hit_location = hit_location;
            let hit_location: *mut Vector = &mut hit_location as *mut Vector;
            Car_TA_ApplyBallImpactForces(self.addr(), ball.addr(), hit_location);
        }
    }
    fn is_dodging(&self) -> bool {
        unsafe {
            Car_TA_IsDodging(self.addr())
        }
    }
    fn on_hit_ball(&self, ball: BallWrapper, hit_location: Vector, hit_normal: Vector) {
        unsafe {
            let mut hit_location = hit_location;
            let hit_location: *mut Vector = &mut hit_location as *mut Vector;
            let mut hit_normal = hit_normal;
            let hit_normal: *mut Vector = &mut hit_normal as *mut Vector;
            Car_TA_OnHitBall(self.addr(), ball.addr(), hit_location, hit_normal);
        }
    }
    fn any_wheel_touching_ground(&self) -> bool {
        unsafe {
            Car_TA_AnyWheelTouchingGround(self.addr())
        }
    }
    fn give_car_component(&self, component_archetype: CarComponentWrapper, activator: PriWrapper) -> Option<CarComponentWrapper> {
        unsafe {
            CarComponentWrapper::try_new(Car_TA_GiveCarComponent(self.addr(), component_archetype.addr(), activator.addr()))
        }
    }
    fn add_default_car_components(&self) {
        unsafe {
            Car_TA_AddDefaultCarComponents(self.addr());
        }
    }
    fn detach_primitive_component(&self, component: PrimitiveComponentWrapper) {
        unsafe {
            Car_TA_DetachPrimitiveComponent(self.addr(), component.addr());
        }
    }
    fn handle_wheel_ball_hit(&self, wheel: WheelWrapper) {
        unsafe {
            Car_TA_HandleWheelBallHit(self.addr(), wheel.addr());
        }
    }
    fn respawn_in_place(&self) {
        unsafe {
            Car_TA_RespawnInPlace(self.addr());
        }
    }
    fn set_car_scale(&self, new_scale: f32) {
        unsafe {
            Car_TA_SetCarScale(self.addr(), new_scale);
        }
    }
    fn on_club_colors_changed(&self) {
        unsafe {
            Car_TA_OnClubColorsChanged(self.addr());
        }
    }
    fn handle_team_changed(&self, my_pri: PriXWrapper) {
        unsafe {
            Car_TA_HandleTeamChanged(self.addr(), my_pri.addr());
        }
    }
    fn update_team_loadout(&self) -> bool {
        unsafe {
            Car_TA_UpdateTeamLoadout(self.addr())
        }
    }
    fn init_team_paint(&self) {
        unsafe {
            Car_TA_InitTeamPaint(self.addr());
        }
    }
    fn get_loadout_team_index(&self) -> i32 {
        unsafe {
            Car_TA_GetLoadoutTeamIndex(self.addr())
        }
    }
    fn get_preview_team_index(&self) -> i32 {
        unsafe {
            Car_TA_GetPreviewTeamIndex(self.addr())
        }
    }
    fn has_team(&self) -> bool {
        unsafe {
            Car_TA_HasTeam(self.addr())
        }
    }
    fn handle_loadout_selected(&self, my_pri: PriWrapper) {
        unsafe {
            Car_TA_HandleLoadoutSelected(self.addr(), my_pri.addr());
        }
    }
    fn handle_game_event_changed(&self, my_pri: PriWrapper) {
        unsafe {
            Car_TA_HandleGameEventChanged(self.addr(), my_pri.addr());
        }
    }
    fn on_pri_changed(&self) {
        unsafe {
            Car_TA_OnPRIChanged(self.addr());
        }
    }
    fn on_controller_changed(&self) {
        unsafe {
            Car_TA_OnControllerChanged(self.addr());
        }
    }

}

extern "C" {
    fn Car_Demolish(p_car: usize);

    fn Car_TA_Get_DefaultCarComponents(obj: usize, result: *mut RLArrayRaw);
    fn Car_TA_Get_FlipComponent(obj: usize) -> usize;
    fn Car_TA_Get_DemolishTarget(obj: usize) -> u8;
    fn CarWrapper_SetDemolishTarget(obj: usize, new_val: u8);
    fn Car_TA_Get_DemolishSpeed(obj: usize) -> u8;
    fn CarWrapper_SetDemolishSpeed(obj: usize, new_val: u8);
    fn Car_TA_Get_bLoadoutSet(obj: usize) -> bool;
    fn CarWrapper_SetbLoadoutSet(obj: usize, new_val: bool);
    fn Car_TA_Get_bDemolishOnOpposingGround(obj: usize) -> bool;
    fn CarWrapper_SetbDemolishOnOpposingGround(obj: usize, new_val: bool);
    fn Car_TA_Get_bWasOnOpposingGround(obj: usize) -> bool;
    fn CarWrapper_SetbWasOnOpposingGround(obj: usize, new_val: bool);
    fn Car_TA_Get_bDemolishOnGoalZone(obj: usize) -> bool;
    fn CarWrapper_SetbDemolishOnGoalZone(obj: usize, new_val: bool);
    fn Car_TA_Get_bWasInGoalZone(obj: usize) -> bool;
    fn CarWrapper_SetbWasInGoalZone(obj: usize, new_val: bool);
    fn Car_TA_Get_bOverrideHandbrakeOn(obj: usize) -> bool;
    fn CarWrapper_SetbOverrideHandbrakeOn(obj: usize, new_val: bool);
    fn Car_TA_Get_bOverrideBoostOn(obj: usize) -> bool;
    fn CarWrapper_SetbOverrideBoostOn(obj: usize, new_val: bool);
    fn Car_TA_Get_ExitFXArchetype(obj: usize) -> usize;
    fn CarWrapper_SetExitFXArchetype(obj: usize, new_val: usize);
    fn Car_TA_Get_MaxTimeForDodge(obj: usize) -> f32;
    fn CarWrapper_SetMaxTimeForDodge(obj: usize, new_val: f32);
    fn Car_TA_Get_LastWheelsHitBallTime(obj: usize) -> f32;
    fn CarWrapper_SetLastWheelsHitBallTime(obj: usize, new_val: f32);
    fn Car_TA_Get_ReplicatedCarScale(obj: usize) -> f32;
    fn CarWrapper_SetReplicatedCarScale(obj: usize, new_val: f32);
    fn Car_TA_Get_BodyFXActor(obj: usize) -> usize;
    fn CarWrapper_SetBodyFXActor(obj: usize, new_val: usize);
    fn Car_TA_Get_AttackerPRI(obj: usize) -> usize;
    fn CarWrapper_SetAttackerPRI(obj: usize, new_val: usize);
    fn Car_TA_Get_MouseAccel(obj: usize, result: *mut Vector);
    fn CarWrapper_SetMouseAccel(obj: usize, new_val: *mut Vector);
    fn Car_TA_Get_MouseAirAccel(obj: usize, result: *mut Vector);
    fn CarWrapper_SetMouseAirAccel(obj: usize, new_val: *mut Vector);
    fn Car_TA_Get_AttachedPickup(obj: usize) -> usize;
    fn CarWrapper_SetAttachedPickup(obj: usize, new_val: usize);
    fn Car_TA_Get_ReplayFocusOffset(obj: usize, result: *mut Vector);
    fn CarWrapper_SetReplayFocusOffset(obj: usize, new_val: *mut Vector);
    fn Car_TA_Get_AddedBallForceMultiplier(obj: usize) -> f32;
    fn CarWrapper_SetAddedBallForceMultiplier(obj: usize, new_val: f32);
    fn Car_TA_Get_AddedCarForceMultiplier(obj: usize) -> f32;
    fn CarWrapper_SetAddedCarForceMultiplier(obj: usize, new_val: f32);
    fn Car_TA_Get_GameEvent(obj: usize) -> usize;
    fn CarWrapper_SetGameEvent(obj: usize, new_val: usize);
    fn Car_TA_GetMaxDriveBackwardsSpeed(obj: usize) -> f32;
    fn Car_TA_GetMaxDriveForwardSpeed(obj: usize) -> f32;
    fn Car_TA_GetReplayFocusLocation(obj: usize, result: *mut Vector);
    fn Car_TA_OnPickupChanged(obj: usize, InPickup: usize);
    fn Car_TA_SetAttachedPickup2(obj: usize, InPickup: usize);
    fn Car_TA_EnablePodiumMode(obj: usize);
    fn Car_TA_CopyPushFactorCurve(obj: usize);
    fn Car_TA_UpdateBallIndicator(obj: usize);
    fn Car_TA_FellOutOfWorld(obj: usize);
    fn Car_TA_DemolishDestroyTimer(obj: usize);
    fn Car_TA_Demolish(obj: usize, Demolisher: usize, InCustomDemoFX: usize);
    fn Car_TA_Teleport(obj: usize, SpawnLocation: *mut Vector, SpawnRotation: *mut Rotator, bStopVelocity: bool, bUpdateRotation: bool, ExtraForce: f32) -> bool;
    fn Car_TA_OnJumpReleased(obj: usize);
    fn Car_TA_OnJumpPressed(obj: usize);
    fn Car_TA_CanDemolish(obj: usize, HitCar: usize) -> bool;
    fn Car_TA_ShouldDemolish(obj: usize, HitCar: usize, HitLocation: *mut Vector, HitNormal: *mut Vector, Result: u8) -> bool;
    fn Car_TA_ApplyCarImpactForces(obj: usize, OtherCar: usize, HitLocation: *mut Vector, HitNormal: *mut Vector) -> u8;
    fn Car_TA_IsBumperHit(obj: usize, OtherCar: usize, HitLocation: *mut Vector) -> bool;
    fn Car_TA_ApplyBallImpactForces(obj: usize, Ball: usize, HitLocation: *mut Vector);
    fn Car_TA_IsDodging(obj: usize) -> bool;
    fn Car_TA_OnHitBall(obj: usize, Ball: usize, HitLocation: *mut Vector, HitNormal: *mut Vector);
    fn Car_TA_AnyWheelTouchingGround(obj: usize) -> bool;
    fn Car_TA_GiveCarComponent(obj: usize, ComponentArchetype: usize, Activator: usize) -> usize;
    fn Car_TA_AddDefaultCarComponents(obj: usize);
    fn Car_TA_DetachPrimitiveComponent(obj: usize, Component: usize);
    fn Car_TA_HandleWheelBallHit(obj: usize, Wheel: usize);
    fn Car_TA_RespawnInPlace(obj: usize);
    fn Car_TA_SetCarScale(obj: usize, NewScale: f32);
    fn Car_TA_OnClubColorsChanged(obj: usize);
    fn Car_TA_HandleTeamChanged(obj: usize, MyPRI: usize);
    fn Car_TA_UpdateTeamLoadout(obj: usize) -> bool;
    fn Car_TA_InitTeamPaint(obj: usize);
    fn Car_TA_GetLoadoutTeamIndex(obj: usize) -> i32;
    fn Car_TA_GetPreviewTeamIndex(obj: usize) -> i32;
    fn Car_TA_HasTeam(obj: usize) -> bool;
    fn Car_TA_HandleLoadoutSelected(obj: usize, MyPRI: usize);
    fn Car_TA_HandleGameEventChanged(obj: usize, MyPRI: usize);
    fn Car_TA_OnPRIChanged(obj: usize);
    fn Car_TA_OnControllerChanged(obj: usize);
}