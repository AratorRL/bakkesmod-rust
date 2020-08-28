use crate::wrappers::*;
use super::*;

pub struct BallWrapper(pub usize);
impl_object!(BallWrapper);

impl Ball for BallWrapper {}
impl RBActor for BallWrapper {}
impl Actor for BallWrapper {}

pub trait Ball : RBActor {
    fn get_end_of_game_fx_archetype(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(Ball_TA_Get_EndOfGameFXArchetype(self.addr()))
        }
    }
    fn get_b_allow_player_explosion_override(&self) -> bool {
        unsafe {
            Ball_TA_Get_bAllowPlayerExplosionOverride(self.addr())
        }
    }
    fn get_b_notify_ground_hit(&self) -> bool {
        unsafe {
            Ball_TA_Get_bNotifyGroundHit(self.addr())
        }
    }
    fn get_b_end_of_game_hidden(&self) -> bool {
        unsafe {
            Ball_TA_Get_bEndOfGameHidden(self.addr())
        }
    }
    fn get_b_fade_in(&self) -> bool {
        unsafe {
            Ball_TA_Get_bFadeIn(self.addr())
        }
    }
    fn get_b_fade_out(&self) -> bool {
        unsafe {
            Ball_TA_Get_bFadeOut(self.addr())
        }
    }
    fn get_b_prediction_on_ground(&self) -> bool {
        unsafe {
            Ball_TA_Get_bPredictionOnGround(self.addr())
        }
    }
    fn get_b_can_be_attached(&self) -> bool {
        unsafe {
            Ball_TA_Get_bCanBeAttached(self.addr())
        }
    }
    fn get_b_item_freeze(&self) -> bool {
        unsafe {
            Ball_TA_Get_bItemFreeze(self.addr())
        }
    }
    fn get_magnus_coefficient(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Ball_TA_Get_MagnusCoefficient(self.addr(), result_ptr);
            result
        }
    }
    fn get_radius(&self) -> f32 {
        unsafe {
            Ball_TA_Get_Radius(self.addr())
        }
    }
    fn get_visual_radius(&self) -> f32 {
        unsafe {
            Ball_TA_Get_VisualRadius(self.addr())
        }
    }
    fn get_last_calculate_car_hit(&self) -> f32 {
        unsafe {
            Ball_TA_Get_LastCalculateCarHit(self.addr())
        }
    }
    fn get_initial_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Ball_TA_Get_InitialLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_last_hit_world_time(&self) -> f32 {
        unsafe {
            Ball_TA_Get_LastHitWorldTime(self.addr())
        }
    }
    fn get_replicated_ball_scale(&self) -> f32 {
        unsafe {
            Ball_TA_Get_ReplicatedBallScale(self.addr())
        }
    }
    fn get_replicated_world_bounce_scale(&self) -> f32 {
        unsafe {
            Ball_TA_Get_ReplicatedWorldBounceScale(self.addr())
        }
    }
    fn get_replicated_ball_gravity_scale(&self) -> f32 {
        unsafe {
            Ball_TA_Get_ReplicatedBallGravityScale(self.addr())
        }
    }
    fn get_replicated_ball_max_linear_speed_scale(&self) -> f32 {
        unsafe {
            Ball_TA_Get_ReplicatedBallMaxLinearSpeedScale(self.addr())
        }
    }
    fn get_replicated_added_car_bounce_scale(&self) -> f32 {
        unsafe {
            Ball_TA_Get_ReplicatedAddedCarBounceScale(self.addr())
        }
    }
    fn get_additional_car_ground_bounce_scale_z(&self) -> f32 {
        unsafe {
            Ball_TA_Get_AdditionalCarGroundBounceScaleZ(self.addr())
        }
    }
    fn get_additional_car_ground_bounce_scale_xy(&self) -> f32 {
        unsafe {
            Ball_TA_Get_AdditionalCarGroundBounceScaleXY(self.addr())
        }
    }
    fn get_hit_team_num(&self) -> u8 {
        unsafe {
            Ball_TA_Get_HitTeamNum(self.addr())
        }
    }
    fn get_game_event(&self) -> Option<ServerWrapper> {
        unsafe {
            ServerWrapper::try_new(Ball_TA_Get_GameEvent(self.addr()))
        }
    }
    fn get_explosion_time(&self) -> f32 {
        unsafe {
            Ball_TA_Get_ExplosionTime(self.addr())
        }
    }
    fn get_old_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Ball_TA_Get_OldLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_prediction_timestep(&self) -> f32 {
        unsafe {
            Ball_TA_Get_PredictionTimestep(self.addr())
        }
    }
    fn get_last_prediction_time(&self) -> f32 {
        unsafe {
            Ball_TA_Get_LastPredictionTime(self.addr())
        }
    }
    fn get_ground_force(&self) -> f32 {
        unsafe {
            Ball_TA_Get_GroundForce(self.addr())
        }
    }
    fn get_current_affector(&self) -> Option<CarWrapper> {
        unsafe {
            CarWrapper::try_new(Ball_TA_Get_CurrentAffector(self.addr()))
        }
    }
    fn get_trajectory_start_velocity(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Ball_TA_GetTrajectoryStartVelocity(self.addr(), result_ptr);
            result
        }
    }
    fn get_trajectory_start_rotation(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            Ball_TA_GetTrajectoryStartRotation(self.addr(), result_ptr);
            result
        }
    }
    fn get_trajectory_start_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Ball_TA_GetTrajectoryStartLocation(self.addr(), result_ptr);
            result
        }
    }
    fn should_draw_trajectory(&self) -> bool {
        unsafe {
            Ball_TA_ShouldDrawTrajectory(self.addr())
        }
    }
    fn get_additional_car_bounce_scale_z(&self, car: CarWrapper) -> f32 {
        unsafe {
            Ball_TA_GetAdditionalCarBounceScaleZ(self.addr(), car.addr())
        }
    }
    fn set_end_of_game_hidden(&self) {
        unsafe {
            Ball_TA_SetEndOfGameHidden(self.addr());
        }
    }
    fn explode(&self, explosion_goal: GoalWrapper, explode_location: Vector, scorer: PriWrapper) {
        unsafe {
            let mut explode_location = explode_location;
            let explode_location: *mut Vector = &mut explode_location as *mut Vector;
            Ball_TA_Explode(self.addr(), explosion_goal.addr(), explode_location, scorer.addr());
        }
    }
    fn do_destroy(&self) {
        unsafe {
            Ball_TA_DoDestroy(self.addr());
        }
    }
    fn do_explode(&self) {
        unsafe {
            Ball_TA_DoExplode(self.addr());
        }
    }
    fn launch(&self, launch_position: Vector, launch_direction: Vector) {
        unsafe {
            let mut launch_position = launch_position;
            let launch_position: *mut Vector = &mut launch_position as *mut Vector;
            let mut launch_direction = launch_direction;
            let launch_direction: *mut Vector = &mut launch_direction as *mut Vector;
            Ball_TA_Launch(self.addr(), launch_position, launch_direction);
        }
    }
    fn on_car_touch(&self, hit_car: CarWrapper, hit_type: u8) {
        unsafe {
            Ball_TA_OnCarTouch(self.addr(), hit_car.addr(), hit_type);
        }
    }
    fn record_car_hit(&self, hit_car: CarWrapper, hit_location: Vector, hit_normal: Vector, hit_type: u8) {
        unsafe {
            let mut hit_location = hit_location;
            let hit_location: *mut Vector = &mut hit_location as *mut Vector;
            let mut hit_normal = hit_normal;
            let hit_normal: *mut Vector = &mut hit_normal as *mut Vector;
            Ball_TA_RecordCarHit(self.addr(), hit_car.addr(), hit_location, hit_normal, hit_type);
        }
    }
    fn is_ground_hit(&self, hit_normal: Vector) -> bool {
        unsafe {
            let mut hit_normal = hit_normal;
            let hit_normal: *mut Vector = &mut hit_normal as *mut Vector;
            Ball_TA_IsGroundHit(self.addr(), hit_normal)
        }
    }
    fn fell_out_of_world(&self) {
        unsafe {
            Ball_TA_FellOutOfWorld(self.addr());
        }
    }
    fn is_round_active(&self) -> bool {
        unsafe {
            Ball_TA_IsRoundActive(self.addr())
        }
    }
    fn turn_off(&self) {
        unsafe {
            Ball_TA_TurnOff(self.addr());
        }
    }
    fn init_ak(&self) {
        unsafe {
            Ball_TA_InitAk(self.addr());
        }
    }
    fn set_world_bounce_scale(&self, new_scale: f32) {
        unsafe {
            Ball_TA_SetWorldBounceScale(self.addr(), new_scale);
        }
    }
    fn set_car_bounce_scale(&self, new_scale: f32) {
        unsafe {
            Ball_TA_SetCarBounceScale(self.addr(), new_scale);
        }
    }
    fn set_ball_max_linear_speed_scale(&self, in_max_linear_speed_scale: f32) {
        unsafe {
            Ball_TA_SetBallMaxLinearSpeedScale(self.addr(), in_max_linear_speed_scale);
        }
    }
    fn set_ball_gravity_scale(&self, in_ball_gravity_scale: f32) {
        unsafe {
            Ball_TA_SetBallGravityScale(self.addr(), in_ball_gravity_scale);
        }
    }
    fn set_ball_scale(&self, new_scale: f32) {
        unsafe {
            Ball_TA_SetBallScale(self.addr(), new_scale);
        }
    }
    fn event_hit_goal(&self, ball: BallWrapper, goal: GoalWrapper) {
        unsafe {
            Ball_TA_EventHitGoal(self.addr(), ball.addr(), goal.addr());
        }
    }

}

extern "C" {
    fn Ball_TA_Get_EndOfGameFXArchetype(obj: usize) -> usize;
    fn BallWrapper_SetEndOfGameFXArchetype(obj: usize, new_val: usize);
    fn Ball_TA_Get_bAllowPlayerExplosionOverride(obj: usize) -> bool;
    fn BallWrapper_SetbAllowPlayerExplosionOverride(obj: usize, new_val: bool);
    fn Ball_TA_Get_bNotifyGroundHit(obj: usize) -> bool;
    fn BallWrapper_SetbNotifyGroundHit(obj: usize, new_val: bool);
    fn Ball_TA_Get_bEndOfGameHidden(obj: usize) -> bool;
    fn BallWrapper_SetbEndOfGameHidden(obj: usize, new_val: bool);
    fn Ball_TA_Get_bFadeIn(obj: usize) -> bool;
    fn BallWrapper_SetbFadeIn(obj: usize, new_val: bool);
    fn Ball_TA_Get_bFadeOut(obj: usize) -> bool;
    fn BallWrapper_SetbFadeOut(obj: usize, new_val: bool);
    fn Ball_TA_Get_bPredictionOnGround(obj: usize) -> bool;
    fn BallWrapper_SetbPredictionOnGround(obj: usize, new_val: bool);
    fn Ball_TA_Get_bCanBeAttached(obj: usize) -> bool;
    fn BallWrapper_SetbCanBeAttached(obj: usize, new_val: bool);
    fn Ball_TA_Get_bItemFreeze(obj: usize) -> bool;
    fn BallWrapper_SetbItemFreeze(obj: usize, new_val: bool);
    fn Ball_TA_Get_MagnusCoefficient(obj: usize, result: *mut Vector);
    fn BallWrapper_SetMagnusCoefficient(obj: usize, new_val: *mut Vector);
    fn Ball_TA_Get_Radius(obj: usize) -> f32;
    fn BallWrapper_SetRadius(obj: usize, new_val: f32);
    fn Ball_TA_Get_VisualRadius(obj: usize) -> f32;
    fn BallWrapper_SetVisualRadius(obj: usize, new_val: f32);
    fn Ball_TA_Get_LastCalculateCarHit(obj: usize) -> f32;
    fn Ball_TA_Get_InitialLocation(obj: usize, result: *mut Vector);
    fn BallWrapper_SetInitialLocation(obj: usize, new_val: *mut Vector);
    fn Ball_TA_Get_LastHitWorldTime(obj: usize) -> f32;
    fn Ball_TA_Get_ReplicatedBallScale(obj: usize) -> f32;
    fn BallWrapper_SetReplicatedBallScale(obj: usize, new_val: f32);
    fn Ball_TA_Get_ReplicatedWorldBounceScale(obj: usize) -> f32;
    fn BallWrapper_SetReplicatedWorldBounceScale(obj: usize, new_val: f32);
    fn Ball_TA_Get_ReplicatedBallGravityScale(obj: usize) -> f32;
    fn BallWrapper_SetReplicatedBallGravityScale(obj: usize, new_val: f32);
    fn Ball_TA_Get_ReplicatedBallMaxLinearSpeedScale(obj: usize) -> f32;
    fn BallWrapper_SetReplicatedBallMaxLinearSpeedScale(obj: usize, new_val: f32);
    fn Ball_TA_Get_ReplicatedAddedCarBounceScale(obj: usize) -> f32;
    fn BallWrapper_SetReplicatedAddedCarBounceScale(obj: usize, new_val: f32);
    fn Ball_TA_Get_AdditionalCarGroundBounceScaleZ(obj: usize) -> f32;
    fn BallWrapper_SetAdditionalCarGroundBounceScaleZ(obj: usize, new_val: f32);
    fn Ball_TA_Get_AdditionalCarGroundBounceScaleXY(obj: usize) -> f32;
    fn BallWrapper_SetAdditionalCarGroundBounceScaleXY(obj: usize, new_val: f32);
    fn Ball_TA_Get_HitTeamNum(obj: usize) -> u8;
    fn BallWrapper_SetHitTeamNum(obj: usize, new_val: u8);
    fn Ball_TA_Get_GameEvent(obj: usize) -> usize;
    fn Ball_TA_Get_ExplosionTime(obj: usize) -> f32;
    fn BallWrapper_SetExplosionTime(obj: usize, new_val: f32);
    fn Ball_TA_Get_OldLocation(obj: usize, result: *mut Vector);
    fn BallWrapper_SetOldLocation(obj: usize, new_val: *mut Vector);
    fn Ball_TA_Get_PredictionTimestep(obj: usize) -> f32;
    fn BallWrapper_SetPredictionTimestep(obj: usize, new_val: f32);
    fn Ball_TA_Get_LastPredictionTime(obj: usize) -> f32;
    fn BallWrapper_SetLastPredictionTime(obj: usize, new_val: f32);
    fn Ball_TA_Get_GroundForce(obj: usize) -> f32;
    fn BallWrapper_SetGroundForce(obj: usize, new_val: f32);
    fn Ball_TA_Get_CurrentAffector(obj: usize) -> usize;
    fn BallWrapper_SetCurrentAffector(obj: usize, new_val: usize);
    fn Ball_TA_GetTrajectoryStartVelocity(obj: usize, result: *mut Vector);
    fn Ball_TA_GetTrajectoryStartRotation(obj: usize, result: *mut Rotator);
    fn Ball_TA_GetTrajectoryStartLocation(obj: usize, result: *mut Vector);
    fn Ball_TA_ShouldDrawTrajectory(obj: usize) -> bool;
    fn Ball_TA_GetAdditionalCarBounceScaleZ(obj: usize, Car: usize) -> f32;
    fn Ball_TA_SetEndOfGameHidden(obj: usize);
    fn Ball_TA_Explode(obj: usize, ExplosionGoal: usize, ExplodeLocation: *mut Vector, Scorer: usize);
    fn Ball_TA_DoDestroy(obj: usize);
    fn Ball_TA_DoExplode(obj: usize);
    fn Ball_TA_Launch(obj: usize, LaunchPosition: *mut Vector, LaunchDirection: *mut Vector);
    fn Ball_TA_OnCarTouch(obj: usize, HitCar: usize, HitType: u8);
    fn Ball_TA_RecordCarHit(obj: usize, HitCar: usize, HitLocation: *mut Vector, HitNormal: *mut Vector, HitType: u8);
    fn Ball_TA_IsGroundHit(obj: usize, HitNormal: *mut Vector) -> bool;
    fn Ball_TA_FellOutOfWorld(obj: usize);
    fn Ball_TA_IsRoundActive(obj: usize) -> bool;
    fn Ball_TA_TurnOff(obj: usize);
    fn Ball_TA_InitAk(obj: usize);
    fn Ball_TA_SetWorldBounceScale(obj: usize, NewScale: f32);
    fn Ball_TA_SetCarBounceScale(obj: usize, NewScale: f32);
    fn Ball_TA_SetBallMaxLinearSpeedScale(obj: usize, InMaxLinearSpeedScale: f32);
    fn Ball_TA_SetBallGravityScale(obj: usize, InBallGravityScale: f32);
    fn Ball_TA_SetBallScale(obj: usize, NewScale: f32);
    fn Ball_TA_EventHitGoal(obj: usize, Ball: usize, Goal: usize);

}