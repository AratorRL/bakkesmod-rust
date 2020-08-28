use crate::wrappers::*;
use crate::generated::*;

pub struct TutorialWrapper(pub usize);
impl_object!(TutorialWrapper);

impl Tutorial for TutorialWrapper {}
impl Server for TutorialWrapper {}
impl TeamGameEvent for TutorialWrapper {}
impl GameEvent for TutorialWrapper {}
impl Actor for TutorialWrapper {}

pub trait Tutorial : Server {
    fn get_total_field_extent(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            GameEvent_Tutorial_TA_Get_TotalFieldExtent(self.addr(), result_ptr);
            result
        }
    }
    fn get_team_num(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_TeamNum(self.addr())
        }
    }
    fn get_ball_goal_num(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_BallGoalNum(self.addr())
        }
    }
    fn get_b_only_score_in_ball_goal_num(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_Get_bOnlyScoreInBallGoalNum(self.addr())
        }
    }
    fn get_b_redo_round(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_Get_bRedoRound(self.addr())
        }
    }
    fn get_b_allow_super_boost(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_Get_bAllowSuperBoost(self.addr())
        }
    }
    fn get_b_displayed_redo_penalty_message(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_Get_bDisplayedRedoPenaltyMessage(self.addr())
        }
    }
    fn get_b_show_boost_meter(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_Get_bShowBoostMeter(self.addr())
        }
    }
    fn get_difficulty(&self) -> u8 {
        unsafe {
            GameEvent_Tutorial_TA_Get_Difficulty(self.addr())
        }
    }
    fn get_debug_rotation_type(&self) -> u8 {
        unsafe {
            GameEvent_Tutorial_TA_Get_DebugRotationType(self.addr())
        }
    }
    fn get_goal_depth(&self) -> f32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_GoalDepth(self.addr())
        }
    }
    fn get_game_event_rounds(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_GameEventRounds(self.addr())
        }
    }
    fn get_event_start_time(&self) -> f32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_EventStartTime(self.addr())
        }
    }
    fn get_ball_initial_velocity(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            GameEvent_Tutorial_TA_Get_BallInitialVelocity(self.addr(), result_ptr);
            result
        }
    }
    fn get_spawn_index_type_override(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_SpawnIndexTypeOverride(self.addr())
        }
    }
    fn get_wave_index(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_WaveIndex(self.addr())
        }
    }
    fn get_wave_spawn_count(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_WaveSpawnCount(self.addr())
        }
    }
    fn get_random_spawn_index(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_RandomSpawnIndex(self.addr())
        }
    }
    fn get_start_message_archetype(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            GameEvent_Tutorial_TA_Get_StartMessageArchetype(self.addr(), result_ptr);
            result
        }
    }
    fn get_ball_spawn_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            GameEvent_Tutorial_TA_Get_BallSpawnLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_points_scored_this_round(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_PointsScoredThisRound(self.addr())
        }
    }
    fn get_ball_spawn_count(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_BallSpawnCount(self.addr())
        }
    }
    fn get_ball_bounce_scale(&self) -> f32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_BallBounceScale(self.addr())
        }
    }
    fn get_current_debug_step_x(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_CurrentDebugStepX(self.addr())
        }
    }
    fn get_current_debug_step_y(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_CurrentDebugStepY(self.addr())
        }
    }
    fn get_current_debug_step_z(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_CurrentDebugStepZ(self.addr())
        }
    }
    fn get_redo_count(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_RedoCount(self.addr())
        }
    }
    fn get_redo_total(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_Get_RedoTotal(self.addr())
        }
    }
    fn init_intro(&self) {
        unsafe {
            GameEvent_Tutorial_TA_InitIntro(self.addr());
        }
    }
    fn on_loading_movie_closed(&self) {
        unsafe {
            GameEvent_Tutorial_TA_OnLoadingMovieClosed(self.addr());
        }
    }
    fn start_timers(&self) {
        unsafe {
            GameEvent_Tutorial_TA_StartTimers(self.addr());
        }
    }
    fn update_mvp(&self) {
        unsafe {
            GameEvent_Tutorial_TA_UpdateMVP(self.addr());
        }
    }
    fn allow_dynamic_crowd(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_AllowDynamicCrowd(self.addr())
        }
    }
    fn set_tutorial_tip(&self, new_tip: RLString) {
        unsafe {
            let mut new_tip = new_tip;
            let new_tip: *mut RLString = &mut new_tip as *mut RLString;
            GameEvent_Tutorial_TA_SetTutorialTip(self.addr(), new_tip);
        }
    }
    fn set_show_boost_meter(&self, b_show: bool) {
        unsafe {
            GameEvent_Tutorial_TA_SetShowBoostMeter(self.addr(), b_show);
        }
    }
    fn get_step_loc(&self, steps: i32, total_dist: f32, b_increment: bool, out_current_step: i32) -> f32 {
        unsafe {
            GameEvent_Tutorial_TA_GetStepLoc(self.addr(), steps, total_dist, b_increment, out_current_step)
        }
    }
    fn get_debug_location_in_extent(&self, extent: Vector) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            let mut extent = extent;
            let extent: *mut Vector = &mut extent as *mut Vector;
            GameEvent_Tutorial_TA_GetDebugLocationInExtent(self.addr(), extent, result_ptr);
            result
        }
    }
    fn init_debug_setup(&self, car: CarWrapper) {
        unsafe {
            GameEvent_Tutorial_TA_InitDebugSetup(self.addr(), car.addr());
        }
    }
    fn skip_tutorial(&self) {
        unsafe {
            GameEvent_Tutorial_TA_SkipTutorial(self.addr());
        }
    }
    fn update_bot_count(&self) {
        unsafe {
            GameEvent_Tutorial_TA_UpdateBotCount(self.addr());
        }
    }
    fn init_mutators(&self) {
        unsafe {
            GameEvent_Tutorial_TA_InitMutators(self.addr());
        }
    }
    fn is_primary_player(&self, car: CarWrapper) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_IsPrimaryPlayer(self.addr(), car.addr())
        }
    }
    fn can_award_points(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_CanAwardPoints(self.addr())
        }
    }
    fn show_reset_round_message(&self) {
        unsafe {
            GameEvent_Tutorial_TA_ShowResetRoundMessage(self.addr());
        }
    }
    fn destroyed(&self) {
        unsafe {
            GameEvent_Tutorial_TA_Destroyed(self.addr());
        }
    }
    fn end_tutorial(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_EndTutorial(self.addr())
        }
    }
    fn commit_redo_round(&self) {
        unsafe {
            GameEvent_Tutorial_TA_CommitRedoRound(self.addr());
        }
    }
    fn redo_round(&self) {
        unsafe {
            GameEvent_Tutorial_TA_RedoRound(self.addr());
        }
    }
    fn can_redo_round(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_CanRedoRound(self.addr())
        }
    }
    fn start_new_round(&self) {
        unsafe {
            GameEvent_Tutorial_TA_StartNewRound(self.addr());
        }
    }
    fn save_local_player_stats(&self) {
        unsafe {
            GameEvent_Tutorial_TA_SaveLocalPlayerStats(self.addr());
        }
    }
    fn get_winning_team(&self) -> TeamWrapper {
        unsafe {
            TeamWrapper::new(GameEvent_Tutorial_TA_GetWinningTeam(self.addr()))
        }
    }
    fn cleanup_round_actors(&self) {
        unsafe {
            GameEvent_Tutorial_TA_CleanupRoundActors(self.addr());
        }
    }
    fn can_que_save_replay(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_CanQueSaveReplay(self.addr())
        }
    }
    fn reset_balls(&self) {
        unsafe {
            GameEvent_Tutorial_TA_ResetBalls(self.addr());
        }
    }
    fn get_score(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_GetScore(self.addr())
        }
    }
    fn start_round(&self) {
        unsafe {
            GameEvent_Tutorial_TA_StartRound(self.addr());
        }
    }
    fn get_game_event_rounds2(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_GetGameEventRounds2(self.addr())
        }
    }
    fn get_total_rounds(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_GetTotalRounds(self.addr())
        }
    }
    fn reset_round_time(&self) {
        unsafe {
            GameEvent_Tutorial_TA_ResetRoundTime(self.addr());
        }
    }
    fn on_player_restarted(&self, player_car: CarWrapper) {
        unsafe {
            GameEvent_Tutorial_TA_OnPlayerRestarted(self.addr(), player_car.addr());
        }
    }
    fn get_training_type(&self) -> u8 {
        unsafe {
            GameEvent_Tutorial_TA_GetTrainingType(self.addr())
        }
    }
    fn end_game(&self) {
        unsafe {
            GameEvent_Tutorial_TA_EndGame(self.addr());
        }
    }
    fn get_random_location_in_extent(&self, extent: Vector) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            let mut extent = extent;
            let extent: *mut Vector = &mut extent as *mut Vector;
            GameEvent_Tutorial_TA_GetRandomLocationInExtent(self.addr(), extent, result_ptr);
            result
        }
    }
    fn chance(&self, chances: i32) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_Chance(self.addr(), chances)
        }
    }
    fn get_opposite_team_num(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_GetOppositeTeamNum(self.addr())
        }
    }
    fn get_direction_to_goal(&self, goal_num: i32) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            GameEvent_Tutorial_TA_GetDirectionToGoal(self.addr(), goal_num, result_ptr);
            result
        }
    }
    fn is_game_event_complete(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_IsGameEventComplete(self.addr())
        }
    }
    fn clamp_point_to_extent(&self, extent_center: Vector, point: Vector, extent: Vector) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            let mut extent_center = extent_center;
            let extent_center: *mut Vector = &mut extent_center as *mut Vector;
            let mut point = point;
            let point: *mut Vector = &mut point as *mut Vector;
            let mut extent = extent;
            let extent: *mut Vector = &mut extent as *mut Vector;
            GameEvent_Tutorial_TA_ClampPointToExtent(self.addr(), extent_center, point, extent, result_ptr);
            result
        }
    }
    fn predict_initial_velocity(&self, start_loc: Vector, end_loc: Vector, z: f32) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            let mut start_loc = start_loc;
            let start_loc: *mut Vector = &mut start_loc as *mut Vector;
            let mut end_loc = end_loc;
            let end_loc: *mut Vector = &mut end_loc as *mut Vector;
            GameEvent_Tutorial_TA_PredictInitialVelocity(self.addr(), start_loc, end_loc, z, result_ptr);
            result
        }
    }
    fn get_game_ball(&self) -> BallWrapper {
        unsafe {
            BallWrapper::new(GameEvent_Tutorial_TA_GetGameBall(self.addr()))
        }
    }
    fn get_game_pawn(&self) -> CarWrapper {
        unsafe {
            CarWrapper::new(GameEvent_Tutorial_TA_GetGamePawn(self.addr()))
        }
    }
    fn reset_game_event(&self) {
        unsafe {
            GameEvent_Tutorial_TA_ResetGameEvent(self.addr());
        }
    }
    fn check_for_reset(&self) {
        unsafe {
            GameEvent_Tutorial_TA_CheckForReset(self.addr());
        }
    }
    fn get_goal_view_width(&self, goal: GoalWrapper, viewer_loc: Vector) -> f32 {
        unsafe {
            let mut viewer_loc = viewer_loc;
            let viewer_loc: *mut Vector = &mut viewer_loc as *mut Vector;
            GameEvent_Tutorial_TA_GetGoalViewWidth(self.addr(), goal.addr(), viewer_loc)
        }
    }
    fn is_ball_moving_towards_goal(&self, goal: GoalWrapper, ball: BallWrapper, min_velocity_for_destroy: f32, in_goal_depth: f32) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_IsBallMovingTowardsGoal(self.addr(), goal.addr(), ball.addr(), min_velocity_for_destroy, in_goal_depth)
        }
    }
    fn set_goal_depth2(&self) {
        unsafe {
            GameEvent_Tutorial_TA_SetGoalDepth2(self.addr());
        }
    }
    fn get_shuffled_spawn_index(&self) -> i32 {
        unsafe {
            GameEvent_Tutorial_TA_GetShuffledSpawnIndex(self.addr())
        }
    }
    fn destroy_cannon(&self) {
        unsafe {
            GameEvent_Tutorial_TA_DestroyCannon(self.addr());
        }
    }
    fn set_cannon_orientation(&self, new_location: Vector, new_rotation: Rotator) {
        unsafe {
            let mut new_location = new_location;
            let new_location: *mut Vector = &mut new_location as *mut Vector;
            let mut new_rotation = new_rotation;
            let new_rotation: *mut Rotator = &mut new_rotation as *mut Rotator;
            GameEvent_Tutorial_TA_SetCannonOrientation(self.addr(), new_location, new_rotation);
        }
    }
    fn spawn_ball(&self, spawn_loc: Vector, b_wake: bool, b_spawn_cannon: bool, ball_arch: RLString) -> BallWrapper {
        unsafe {
            let mut spawn_loc = spawn_loc;
            let spawn_loc: *mut Vector = &mut spawn_loc as *mut Vector;
            let mut ball_arch = ball_arch;
            let ball_arch: *mut RLString = &mut ball_arch as *mut RLString;
            BallWrapper::new(GameEvent_Tutorial_TA_SpawnBall(self.addr(), spawn_loc, b_wake, b_spawn_cannon, ball_arch))
        }
    }
    fn init_ball_effects(&self) {
        unsafe {
            GameEvent_Tutorial_TA_InitBallEffects(self.addr());
        }
    }
    fn init_ball_velocity(&self) {
        unsafe {
            GameEvent_Tutorial_TA_InitBallVelocity(self.addr());
        }
    }
    fn get_random_goal_aim_location(&self, in_team_num: i32, ball_loc: Vector) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            let mut ball_loc = ball_loc;
            let ball_loc: *mut Vector = &mut ball_loc as *mut Vector;
            GameEvent_Tutorial_TA_GetRandomGoalAimLocation(self.addr(), in_team_num, ball_loc, result_ptr);
            result
        }
    }
    fn get_goal_extent(&self, goal: GoalWrapper) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            GameEvent_Tutorial_TA_GetGoalExtent(self.addr(), goal.addr(), result_ptr);
            result
        }
    }
    fn set_ball_velocity(&self, initial_velocity: Vector, ball: BallWrapper) {
        unsafe {
            let mut initial_velocity = initial_velocity;
            let initial_velocity: *mut Vector = &mut initial_velocity as *mut Vector;
            GameEvent_Tutorial_TA_SetBallVelocity(self.addr(), initial_velocity, ball.addr());
        }
    }
    fn init_game_setup(&self, car: CarWrapper) {
        unsafe {
            GameEvent_Tutorial_TA_InitGameSetup(self.addr(), car.addr());
        }
    }
    fn should_allow_super_boost(&self) -> bool {
        unsafe {
            GameEvent_Tutorial_TA_ShouldAllowSuperBoost(self.addr())
        }
    }
    fn on_vehicle_setup(&self, car: CarWrapper) {
        unsafe {
            GameEvent_Tutorial_TA_OnVehicleSetup(self.addr(), car.addr());
        }
    }
    fn handle_vehicle_setup(&self, car: CarWrapper) {
        unsafe {
            GameEvent_Tutorial_TA_HandleVehicleSetup(self.addr(), car.addr());
        }
    }
    fn init_crowd_manager(&self) {
        unsafe {
            GameEvent_Tutorial_TA_InitCrowdManager(self.addr());
        }
    }
    fn handle_score_updated(&self, team: TeamWrapper) {
        unsafe {
            GameEvent_Tutorial_TA_HandleScoreUpdated(self.addr(), team.addr());
        }
    }
    fn set_difficulty2(&self, in_difficulty: i32) {
        unsafe {
            GameEvent_Tutorial_TA_SetDifficulty2(self.addr(), in_difficulty);
        }
    }
    fn update_stats(&self) {
        unsafe {
            GameEvent_Tutorial_TA_UpdateStats(self.addr());
        }
    }
    fn add_local_player(&self, player: PlayerControllerWrapper) {
        unsafe {
            GameEvent_Tutorial_TA_AddLocalPlayer(self.addr(), player.addr());
        }
    }
    fn handle_player_reset_training(&self, game_event: GameEventWrapper) {
        unsafe {
            GameEvent_Tutorial_TA_HandlePlayerResetTraining(self.addr(), game_event.addr());
        }
    }
    fn on_init(&self) {
        unsafe {
            GameEvent_Tutorial_TA_OnInit(self.addr());
        }
    }
    fn event_tutorial_tip_changed(&self, game_event: TutorialWrapper, new_tip: RLString) {
        unsafe {
            let mut new_tip = new_tip;
            let new_tip: *mut RLString = &mut new_tip as *mut RLString;
            GameEvent_Tutorial_TA_EventTutorialTipChanged(self.addr(), game_event.addr(), new_tip);
        }
    }

}

extern "C" {
    fn GameEvent_Tutorial_TA_Get_TotalFieldExtent(obj: usize, result: *mut Vector);
    fn TutorialWrapper_SetTotalFieldExtent(obj: usize, new_val: *mut Vector);
    fn GameEvent_Tutorial_TA_Get_TeamNum(obj: usize) -> i32;
    fn TutorialWrapper_SetTeamNum(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_BallGoalNum(obj: usize) -> i32;
    fn TutorialWrapper_SetBallGoalNum(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_bOnlyScoreInBallGoalNum(obj: usize) -> bool;
    fn TutorialWrapper_SetbOnlyScoreInBallGoalNum(obj: usize, new_val: bool);
    fn GameEvent_Tutorial_TA_Get_bRedoRound(obj: usize) -> bool;
    fn TutorialWrapper_SetbRedoRound(obj: usize, new_val: bool);
    fn GameEvent_Tutorial_TA_Get_bAllowSuperBoost(obj: usize) -> bool;
    fn TutorialWrapper_SetbAllowSuperBoost(obj: usize, new_val: bool);
    fn GameEvent_Tutorial_TA_Get_bDisplayedRedoPenaltyMessage(obj: usize) -> bool;
    fn TutorialWrapper_SetbDisplayedRedoPenaltyMessage(obj: usize, new_val: bool);
    fn GameEvent_Tutorial_TA_Get_bShowBoostMeter(obj: usize) -> bool;
    fn TutorialWrapper_SetbShowBoostMeter(obj: usize, new_val: bool);
    fn GameEvent_Tutorial_TA_Get_Difficulty(obj: usize) -> u8;
    fn TutorialWrapper_SetDifficulty(obj: usize, new_val: u8);
    fn GameEvent_Tutorial_TA_Get_DebugRotationType(obj: usize) -> u8;
    fn TutorialWrapper_SetDebugRotationType(obj: usize, new_val: u8);
    fn GameEvent_Tutorial_TA_Get_GoalDepth(obj: usize) -> f32;
    fn TutorialWrapper_SetGoalDepth(obj: usize, new_val: f32);
    fn GameEvent_Tutorial_TA_Get_GameEventRounds(obj: usize) -> i32;
    fn TutorialWrapper_SetGameEventRounds(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_EventStartTime(obj: usize) -> f32;
    fn TutorialWrapper_SetEventStartTime(obj: usize, new_val: f32);
    fn GameEvent_Tutorial_TA_Get_BallInitialVelocity(obj: usize, result: *mut Vector);
    fn TutorialWrapper_SetBallInitialVelocity(obj: usize, new_val: *mut Vector);
    fn GameEvent_Tutorial_TA_Get_SpawnIndexTypeOverride(obj: usize) -> i32;
    fn TutorialWrapper_SetSpawnIndexTypeOverride(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_WaveIndex(obj: usize) -> i32;
    fn TutorialWrapper_SetWaveIndex(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_WaveSpawnCount(obj: usize) -> i32;
    fn TutorialWrapper_SetWaveSpawnCount(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_RandomSpawnIndex(obj: usize) -> i32;
    fn TutorialWrapper_SetRandomSpawnIndex(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_StartMessageArchetype(obj: usize, result: *mut RLString);
    fn GameEvent_Tutorial_TA_Get_BallSpawnLocation(obj: usize, result: *mut Vector);
    fn TutorialWrapper_SetBallSpawnLocation(obj: usize, new_val: *mut Vector);
    fn GameEvent_Tutorial_TA_Get_PointsScoredThisRound(obj: usize) -> i32;
    fn TutorialWrapper_SetPointsScoredThisRound(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_BallSpawnCount(obj: usize) -> i32;
    fn TutorialWrapper_SetBallSpawnCount(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_BallBounceScale(obj: usize) -> f32;
    fn TutorialWrapper_SetBallBounceScale(obj: usize, new_val: f32);
    fn GameEvent_Tutorial_TA_Get_CurrentDebugStepX(obj: usize) -> i32;
    fn TutorialWrapper_SetCurrentDebugStepX(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_CurrentDebugStepY(obj: usize) -> i32;
    fn TutorialWrapper_SetCurrentDebugStepY(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_CurrentDebugStepZ(obj: usize) -> i32;
    fn TutorialWrapper_SetCurrentDebugStepZ(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_RedoCount(obj: usize) -> i32;
    fn TutorialWrapper_SetRedoCount(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_Get_RedoTotal(obj: usize) -> i32;
    fn TutorialWrapper_SetRedoTotal(obj: usize, new_val: i32);
    fn GameEvent_Tutorial_TA_InitIntro(obj: usize);
    fn GameEvent_Tutorial_TA_OnLoadingMovieClosed(obj: usize);
    fn GameEvent_Tutorial_TA_StartTimers(obj: usize);
    fn GameEvent_Tutorial_TA_UpdateMVP(obj: usize);
    fn GameEvent_Tutorial_TA_AllowDynamicCrowd(obj: usize) -> bool;
    fn GameEvent_Tutorial_TA_SetTutorialTip(obj: usize, NewTip: *mut RLString);
    fn GameEvent_Tutorial_TA_SetShowBoostMeter(obj: usize, bShow: bool);
    fn GameEvent_Tutorial_TA_GetStepLoc(obj: usize, Steps: i32, TotalDist: f32, bIncrement: bool, Out_CurrentStep: i32) -> f32;
    fn GameEvent_Tutorial_TA_GetDebugLocationInExtent(obj: usize, Extent: *mut Vector, result: *mut Vector);
    fn GameEvent_Tutorial_TA_InitDebugSetup(obj: usize, Car: usize);
    fn GameEvent_Tutorial_TA_SkipTutorial(obj: usize);
    fn GameEvent_Tutorial_TA_UpdateBotCount(obj: usize);
    fn GameEvent_Tutorial_TA_InitMutators(obj: usize);
    fn GameEvent_Tutorial_TA_IsPrimaryPlayer(obj: usize, Car: usize) -> bool;
    fn GameEvent_Tutorial_TA_CanAwardPoints(obj: usize) -> bool;
    fn GameEvent_Tutorial_TA_ShowResetRoundMessage(obj: usize);
    fn GameEvent_Tutorial_TA_Destroyed(obj: usize);
    fn GameEvent_Tutorial_TA_EndTutorial(obj: usize) -> bool;
    fn GameEvent_Tutorial_TA_CommitRedoRound(obj: usize);
    fn GameEvent_Tutorial_TA_RedoRound(obj: usize);
    fn GameEvent_Tutorial_TA_CanRedoRound(obj: usize) -> bool;
    fn GameEvent_Tutorial_TA_StartNewRound(obj: usize);
    fn GameEvent_Tutorial_TA_SaveLocalPlayerStats(obj: usize);
    fn GameEvent_Tutorial_TA_GetWinningTeam(obj: usize) -> usize;
    fn GameEvent_Tutorial_TA_CleanupRoundActors(obj: usize);
    fn GameEvent_Tutorial_TA_CanQueSaveReplay(obj: usize) -> bool;
    fn GameEvent_Tutorial_TA_ResetBalls(obj: usize);
    fn GameEvent_Tutorial_TA_GetScore(obj: usize) -> i32;
    fn GameEvent_Tutorial_TA_StartRound(obj: usize);
    fn GameEvent_Tutorial_TA_GetGameEventRounds2(obj: usize) -> i32;
    fn GameEvent_Tutorial_TA_GetTotalRounds(obj: usize) -> i32;
    fn GameEvent_Tutorial_TA_ResetRoundTime(obj: usize);
    fn GameEvent_Tutorial_TA_OnPlayerRestarted(obj: usize, PlayerCar: usize);
    fn GameEvent_Tutorial_TA_GetTrainingType(obj: usize) -> u8;
    fn GameEvent_Tutorial_TA_EndGame(obj: usize);
    fn GameEvent_Tutorial_TA_GetRandomLocationInExtent(obj: usize, Extent: *mut Vector, result: *mut Vector);
    fn GameEvent_Tutorial_TA_Chance(obj: usize, Chances: i32) -> bool;
    fn GameEvent_Tutorial_TA_GetOppositeTeamNum(obj: usize) -> i32;
    fn GameEvent_Tutorial_TA_GetDirectionToGoal(obj: usize, GoalNum: i32, result: *mut Vector);
    fn GameEvent_Tutorial_TA_IsGameEventComplete(obj: usize) -> bool;
    fn GameEvent_Tutorial_TA_ClampPointToExtent(obj: usize, ExtentCenter: *mut Vector, Point: *mut Vector, Extent: *mut Vector, result: *mut Vector);
    fn GameEvent_Tutorial_TA_PredictInitialVelocity(obj: usize, StartLoc: *mut Vector, EndLoc: *mut Vector, Z: f32, result: *mut Vector);
    fn GameEvent_Tutorial_TA_GetGameBall(obj: usize) -> usize;
    fn GameEvent_Tutorial_TA_GetGamePawn(obj: usize) -> usize;
    fn GameEvent_Tutorial_TA_ResetGameEvent(obj: usize);
    fn GameEvent_Tutorial_TA_CheckForReset(obj: usize);
    fn GameEvent_Tutorial_TA_GetGoalViewWidth(obj: usize, Goal: usize, ViewerLoc: *mut Vector) -> f32;
    fn GameEvent_Tutorial_TA_IsBallMovingTowardsGoal(obj: usize, Goal: usize, Ball: usize, MinVelocityForDestroy: f32, InGoalDepth: f32) -> bool;
    fn GameEvent_Tutorial_TA_SetGoalDepth2(obj: usize);
    fn GameEvent_Tutorial_TA_GetShuffledSpawnIndex(obj: usize) -> i32;
    fn GameEvent_Tutorial_TA_DestroyCannon(obj: usize);
    fn GameEvent_Tutorial_TA_SetCannonOrientation(obj: usize, NewLocation: *mut Vector, NewRotation: *mut Rotator);
    fn GameEvent_Tutorial_TA_SpawnBall(obj: usize, SpawnLoc: *mut Vector, bWake: bool, bSpawnCannon: bool, BallArch: *mut RLString) -> usize;
    fn GameEvent_Tutorial_TA_InitBallEffects(obj: usize);
    fn GameEvent_Tutorial_TA_InitBallVelocity(obj: usize);
    fn GameEvent_Tutorial_TA_GetRandomGoalAimLocation(obj: usize, InTeamNum: i32, BallLoc: *mut Vector, result: *mut Vector);
    fn GameEvent_Tutorial_TA_GetGoalExtent(obj: usize, Goal: usize, result: *mut Vector);
    fn GameEvent_Tutorial_TA_SetBallVelocity(obj: usize, InitialVelocity: *mut Vector, Ball: usize);
    fn GameEvent_Tutorial_TA_InitGameSetup(obj: usize, Car: usize);
    fn GameEvent_Tutorial_TA_ShouldAllowSuperBoost(obj: usize) -> bool;
    fn GameEvent_Tutorial_TA_OnVehicleSetup(obj: usize, Car: usize);
    fn GameEvent_Tutorial_TA_HandleVehicleSetup(obj: usize, Car: usize);
    fn GameEvent_Tutorial_TA_InitCrowdManager(obj: usize);
    fn GameEvent_Tutorial_TA_HandleScoreUpdated(obj: usize, Team: usize);
    fn GameEvent_Tutorial_TA_SetDifficulty2(obj: usize, InDifficulty: i32);
    fn GameEvent_Tutorial_TA_UpdateStats(obj: usize);
    fn GameEvent_Tutorial_TA_AddLocalPlayer(obj: usize, Player: usize);
    fn GameEvent_Tutorial_TA_HandlePlayerResetTraining(obj: usize, GameEvent: usize);
    fn GameEvent_Tutorial_TA_OnInit(obj: usize);
    fn GameEvent_Tutorial_TA_EventTutorialTipChanged(obj: usize, GameEvent: usize, NewTip: *mut RLString);

}