use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct ServerWrapper(pub usize);
impl_object!(ServerWrapper);

impl Server for ServerWrapper {}
impl TeamGameEvent for ServerWrapper {}
impl GameEvent for ServerWrapper {}
impl Actor for ServerWrapper {}

pub trait Server : TeamGameEvent {
    fn get_ball(&self) -> Option<BallWrapper> {
        unsafe { BallWrapper::try_new(Server_GetBall(self.addr())) }
    }

    fn spawn_car(&self, car_body: i32, name: &str) {
        let c_name = CString::new(name).unwrap();
        let c_name: *const c_char = c_name.as_ptr();
        unsafe { Server_SpawnCar(self.addr(), car_body, c_name) }
    }

    fn spawn_bot(&self, car_body: i32, name: &str) {
        let c_name = CString::new(name).unwrap();
        let c_name: *const c_char = c_name.as_ptr();
        unsafe { Server_SpawnBot(self.addr(), car_body, c_name) }
    }

    fn spawn_ball(&self, position: Vector, wake: bool, spawn_cannon: bool) -> Option<BallWrapper> {
        let mut position = position;
        let position = &mut position as *mut Vector;
        unsafe { BallWrapper::try_new(Server_SpawnBall(self.addr(), position, wake, spawn_cannon)) }
    }

    fn has_authority(&self) -> bool {
        unsafe { Server_HasAuthority(self.addr()) }
    }

    fn set_game_speed(&self, game_speed: f32) {
        unsafe { Server_SetGameSpeed(self.addr(), game_speed); }
    }

    fn get_game_speed(&self) -> f32 {
        unsafe { Server_GetGameSpeed(self.addr()) }
    }

    fn set_seconds_elapsed(&self, seconds_elapsed: f32) {
        unsafe { Server_SetSecondsElapsed(self.addr(), seconds_elapsed); }
    }

    fn get_seconds_elapsed(&self) -> f32 {
        unsafe { Server_GetSecondsElapsed(self.addr()) }
    }

    fn get_game_car(&self) -> Option<CarWrapper> {
        unsafe { CarWrapper::try_new(Server_GetGameCar(self.addr())) }
    }

    fn is_ball_moving_towards_goal(&self, goal_no: i32, ball: BallWrapper) -> bool {
        unsafe { Server_IsBallMovingTowardsGoal(self.addr(), goal_no, ball.addr()) }
    }

    fn is_in_goal(&self, vec: Vector) -> bool {
        let mut vec = vec;
        let vec = &mut vec as *mut Vector;
        unsafe { Server_IsInGoal(self.addr(), vec) }
    }

    fn disable_goal_reset(&self) {
        unsafe { Server_DisableGoalReset(self.addr()) }
    }

    fn enable_goal_reset(&self) {
        unsafe { Server_EnableGoalReset(self.addr()) }
    }

    fn generate_shot(&self, start_pos: Vector, destination: Vector, speed: f32) -> Vector {
        let mut start_pos = start_pos;
        let start_pos = &mut start_pos as *mut Vector;
        let mut destination = destination;
        let destination = &mut destination as *mut Vector;
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Server_GenerateShot(self.addr(), start_pos, destination, speed, result_ptr) }
        result
    }

    fn generate_goal_aim_location(&self, goal_number: i32, current_ball_location: Vector) -> Vector {
        let mut current_ball_location = current_ball_location;
        let current_ball_location = &mut current_ball_location as *mut Vector;
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Server_GenerateGoalAimLocation(self.addr(), goal_number, current_ball_location, result_ptr)}
        result
    }

    fn get_goal_extent(&self, goal_number: i32) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Server_GetGoalExtent(self.addr(), goal_number, result_ptr)}
        result
    }

    fn get_goal_location(&self, goal_number: i32) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Server_GetGoalLocation(self.addr(), goal_number, result_ptr)}
        result
    }

    fn get_test_car_archetype(&self) -> Option<CarWrapper> {
        unsafe {
            CarWrapper::try_new(GameEvent_Soccar_TA_Get_TestCarArchetype(self.addr()))
        }
    }
    fn get_ball_archetype(&self) -> Option<BallWrapper> {
        unsafe {
            BallWrapper::try_new(GameEvent_Soccar_TA_Get_BallArchetype(self.addr()))
        }
    }
    fn get_ball_spawn_point(&self) -> Option<ActorWrapper> {
        unsafe {
            ActorWrapper::try_new(GameEvent_Soccar_TA_Get_BallSpawnPoint(self.addr()))
        }
    }
    fn get_series_length(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_SeriesLength(self.addr())
        }
    }
    fn get_game_time(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_GameTime(self.addr())
        }
    }
    fn get_warmup_time(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_WarmupTime(self.addr())
        }
    }
    fn get_max_score(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_MaxScore(self.addr())
        }
    }
    fn get_auto_balance_difference(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_AutoBalanceDifference(self.addr())
        }
    }
    fn get_score_slomo_time(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_ScoreSlomoTime(self.addr())
        }
    }
    fn get_game_time_remaining(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_GameTimeRemaining(self.addr())
        }
    }
    fn get_seconds_remaining(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_SecondsRemaining(self.addr())
        }
    }
    fn get_wait_time_remaining(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_WaitTimeRemaining(self.addr())
        }
    }
    fn get_total_game_time_played(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_TotalGameTimePlayed(self.addr())
        }
    }
    fn get_overtime_time_played(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_OvertimeTimePlayed(self.addr())
        }
    }
    fn get_b_round_active(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bRoundActive(self.addr())
        }
    }
    fn get_b_play_replays(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bPlayReplays(self.addr())
        }
    }
    fn get_b_ball_has_been_hit(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bBallHasBeenHit(self.addr())
        }
    }
    fn get_b_over_time(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bOverTime(self.addr())
        }
    }
    fn get_b_unlimited_time(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bUnlimitedTime(self.addr())
        }
    }
    fn get_b_no_contest(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bNoContest(self.addr())
        }
    }
    fn get_b_disable_goal_delay(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bDisableGoalDelay(self.addr())
        }
    }
    fn get_b_show_no_scorer_goal_message(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bShowNoScorerGoalMessage(self.addr())
        }
    }
    fn get_b_match_ended(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bMatchEnded(self.addr())
        }
    }
    fn get_b_show_intro_scene(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bShowIntroScene(self.addr())
        }
    }
    fn get_b_club_match(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_Get_bClubMatch(self.addr())
        }
    }
    fn get_next_spawn_index(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_NextSpawnIndex(self.addr())
        }
    }
    fn get_replay_director_archetype(&self) -> Option<ReplayDirectorWrapper> {
        unsafe {
            ReplayDirectorWrapper::try_new(GameEvent_Soccar_TA_Get_ReplayDirectorArchetype(self.addr()))
        }
    }
    fn get_replay_director(&self) -> Option<ReplayDirectorWrapper> {
        unsafe {
            ReplayDirectorWrapper::try_new(GameEvent_Soccar_TA_Get_ReplayDirector(self.addr()))
        }
    }
    fn get_game_balls(&self) -> RLArray<BallWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            GameEvent_Soccar_TA_Get_GameBalls(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_total_game_balls(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_TotalGameBalls(self.addr())
        }
    }
    fn get_post_goal_time(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_PostGoalTime(self.addr())
        }
    }
    fn get_goals(&self) -> RLArray<GoalWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            GameEvent_Soccar_TA_Get_Goals(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_seconds_remaining_countdown(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_SecondsRemainingCountdown(self.addr())
        }
    }
    fn get_field_center(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            GameEvent_Soccar_TA_Get_FieldCenter(self.addr(), result_ptr);
            result
        }
    }
    fn get_game_winner(&self) -> Option<TeamWrapper> {
        unsafe {
            TeamWrapper::try_new(GameEvent_Soccar_TA_Get_GameWinner(self.addr()))
        }
    }
    fn get_match_winner(&self) -> Option<TeamWrapper> {
        unsafe {
            TeamWrapper::try_new(GameEvent_Soccar_TA_Get_MatchWinner(self.addr()))
        }
    }
    fn get_replicated_scored_on_team(&self) -> u8 {
        unsafe {
            GameEvent_Soccar_TA_Get_ReplicatedScoredOnTeam(self.addr())
        }
    }
    fn get_replicated_server_performance_state(&self) -> u8 {
        unsafe {
            GameEvent_Soccar_TA_Get_ReplicatedServerPerformanceState(self.addr())
        }
    }
    fn get_mvp(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(GameEvent_Soccar_TA_Get_MVP(self.addr()))
        }
    }
    fn get_fastest_goal_player(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(GameEvent_Soccar_TA_Get_FastestGoalPlayer(self.addr()))
        }
    }
    fn get_slowest_goal_player(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(GameEvent_Soccar_TA_Get_SlowestGoalPlayer(self.addr()))
        }
    }
    fn get_furthest_goal_player(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(GameEvent_Soccar_TA_Get_FurthestGoalPlayer(self.addr()))
        }
    }
    fn get_fastest_goal_speed(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_FastestGoalSpeed(self.addr())
        }
    }
    fn get_slowest_goal_speed(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_SlowestGoalSpeed(self.addr())
        }
    }
    fn get_furthest_goal(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_FurthestGoal(self.addr())
        }
    }
    fn get_round_num(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_Get_RoundNum(self.addr())
        }
    }
    fn get_assist_max_time(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_AssistMaxTime(self.addr())
        }
    }
    fn get_ball_has_been_hit_start_delay(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_BallHasBeenHitStartDelay(self.addr())
        }
    }
    fn get_podium_delay(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_PodiumDelay(self.addr())
        }
    }
    fn get_podium_time(&self) -> f32 {
        unsafe {
            GameEvent_Soccar_TA_Get_PodiumTime(self.addr())
        }
    }
    fn get_pauser(&self) -> Option<PlayerControllerWrapper> {
        unsafe {
            PlayerControllerWrapper::try_new(GameEvent_Soccar_TA_Get_Pauser(self.addr()))
        }
    }
    fn check_start(&self) {
        unsafe {
            GameEvent_Soccar_TA_CheckStart(self.addr());
        }
    }
    fn get_player_car_count(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_GetPlayerCarCount(self.addr())
        }
    }
    fn replicate_skill_tiers(&self) {
        unsafe {
            GameEvent_Soccar_TA_ReplicateSkillTiers(self.addr());
        }
    }
    fn can_spawn_bots(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_CanSpawnBots(self.addr())
        }
    }
    fn start_round(&self) {
        unsafe {
            GameEvent_Soccar_TA_StartRound(self.addr());
        }
    }
    fn end_round(&self) {
        unsafe {
            GameEvent_Soccar_TA_EndRound(self.addr());
        }
    }
    fn set_ball_event_listeners(&self, ball: BallWrapper, b_listen: bool) {
        unsafe {
            GameEvent_Soccar_TA_SetBallEventListeners(self.addr(), ball.addr(), b_listen);
        }
    }
    fn can_award_points(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_CanAwardPoints(self.addr())
        }
    }
    fn handle_car_touch(&self, ball: BallWrapper, hit_car: CarWrapper, hit_type: u8) {
        unsafe {
            GameEvent_Soccar_TA_HandleCarTouch(self.addr(), ball.addr(), hit_car.addr(), hit_type);
        }
    }
    fn set_ball_has_been_hit(&self) {
        unsafe {
            GameEvent_Soccar_TA_SetBallHasBeenHit(self.addr());
        }
    }
    fn determine_score_touch_index(&self, ball: BallWrapper, goal: GoalWrapper) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_DetermineScoreTouchIndex(self.addr(), ball.addr(), goal.addr())
        }
    }
    fn determine_assist_touch_index(&self, ball: BallWrapper, score_idx: i32) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_DetermineAssistTouchIndex(self.addr(), ball.addr(), score_idx)
        }
    }
    fn update_total_game_time_played(&self, delta_time: f32) {
        unsafe {
            GameEvent_Soccar_TA_UpdateTotalGameTimePlayed(self.addr(), delta_time);
        }
    }
    fn update_game_time(&self, delta_time: f32) {
        unsafe {
            GameEvent_Soccar_TA_UpdateGameTime(self.addr(), delta_time);
        }
    }
    fn can_update_game_time(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_CanUpdateGameTime(self.addr())
        }
    }
    fn start_replay(&self) {
        unsafe {
            GameEvent_Soccar_TA_StartReplay(self.addr());
        }
    }
    fn handle_replay_finished(&self, in_replay: ReplayDirectorWrapper) {
        unsafe {
            GameEvent_Soccar_TA_HandleReplayFinished(self.addr(), in_replay.addr());
        }
    }
    fn goto_podium_spotlight(&self) {
        unsafe {
            GameEvent_Soccar_TA_GotoPodiumSpotlight(self.addr());
        }
    }
    fn update_spotlight(&self) {
        unsafe {
            GameEvent_Soccar_TA_UpdateSpotlight(self.addr());
        }
    }
    fn spawn_podium_cars(&self) {
        unsafe {
            GameEvent_Soccar_TA_SpawnPodiumCars(self.addr());
        }
    }
    fn can_enable_car_podium_movement(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_CanEnableCarPodiumMovement(self.addr())
        }
    }
    fn finish_event(&self) {
        unsafe {
            GameEvent_Soccar_TA_FinishEvent(self.addr());
        }
    }
    fn replicated_server_performance_state_change_notify_func(&self) {
        unsafe {
            GameEvent_Soccar_TA___ReplicatedServerPerformanceState__ChangeNotifyFunc(self.addr());
        }
    }
    fn b_club_match_change_notify_func(&self) {
        unsafe {
            GameEvent_Soccar_TA___bClubMatch__ChangeNotifyFunc(self.addr());
        }
    }
    fn b_show_intro_scene_change_notify_func(&self) {
        unsafe {
            GameEvent_Soccar_TA___bShowIntroScene__ChangeNotifyFunc(self.addr());
        }
    }
    fn wait_time_remaining_change_notify_func(&self) {
        unsafe {
            GameEvent_Soccar_TA___WaitTimeRemaining__ChangeNotifyFunc(self.addr());
        }
    }
    fn check_join_in_progress(&self, pri: PriWrapper) {
        unsafe {
            GameEvent_Soccar_TA_CheckJoinInProgress(self.addr(), pri.addr());
        }
    }
    fn allow_dynamic_crowd(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_AllowDynamicCrowd(self.addr())
        }
    }
    fn add_ball_trajectory(&self, in_ball: BallWrapper) {
        unsafe {
            GameEvent_Soccar_TA_AddBallTrajectory(self.addr(), in_ball.addr());
        }
    }
    fn show_scorer_goal_message(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_ShowScorerGoalMessage(self.addr())
        }
    }
    fn can_use_ball_cam(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_CanUseBallCam(self.addr())
        }
    }
    fn disable_stat_xp(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_DisableStatXP(self.addr())
        }
    }
    fn force_match_start(&self) {
        unsafe {
            GameEvent_Soccar_TA_ForceMatchStart(self.addr());
        }
    }
    fn remove_local_player(&self, player: PlayerControllerWrapper) {
        unsafe {
            GameEvent_Soccar_TA_RemoveLocalPlayer(self.addr(), player.addr());
        }
    }
    fn add_local_player(&self, player: PlayerControllerWrapper) {
        unsafe {
            GameEvent_Soccar_TA_AddLocalPlayer(self.addr(), player.addr());
        }
    }
    fn destroy_goal_indicators(&self, player: PlayerControllerWrapper) {
        unsafe {
            GameEvent_Soccar_TA_DestroyGoalIndicators(self.addr(), player.addr());
        }
    }
    fn create_goal_indicators(&self, player: PlayerControllerWrapper) {
        unsafe {
            GameEvent_Soccar_TA_CreateGoalIndicators(self.addr(), player.addr());
        }
    }
    fn begin_highlights_replay(&self) {
        unsafe {
            GameEvent_Soccar_TA_BeginHighlightsReplay(self.addr());
        }
    }
    fn should_count_up(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_ShouldCountUp(self.addr())
        }
    }
    fn should_allow_vote_to_forfeit(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_ShouldAllowVoteToForfeit(self.addr())
        }
    }
    fn should_have_leave_match_penalty(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_ShouldHaveLeaveMatchPenalty(self.addr())
        }
    }
    fn set_paused(&self, in_pauser: PlayerControllerWrapper, b_in_paused: bool) {
        unsafe {
            GameEvent_Soccar_TA_SetPaused(self.addr(), in_pauser.addr(), b_in_paused);
        }
    }
    fn should_countdown_resume_from_pause(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_ShouldCountdownResumeFromPause(self.addr())
        }
    }
    fn set_score_and_time(&self, pc: PlayerControllerWrapper, new_score_team0: i32, new_score_team1: i32, in_game_time_remaining: i32, b_in_overtime: bool, b_restart_round: bool) {
        unsafe {
            GameEvent_Soccar_TA_SetScoreAndTime(self.addr(), pc.addr(), new_score_team0, new_score_team1, in_game_time_remaining, b_in_overtime, b_restart_round);
        }
    }
    fn save_local_player_stats(&self) {
        unsafe {
            GameEvent_Soccar_TA_SaveLocalPlayerStats(self.addr());
        }
    }
    fn should_play_replay(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_ShouldPlayReplay(self.addr())
        }
    }
    fn should_record_replay(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_ShouldRecordReplay(self.addr())
        }
    }
    fn on_ball_has_been_hit(&self) {
        unsafe {
            GameEvent_Soccar_TA_OnBallHasBeenHit(self.addr());
        }
    }

    fn get_total_score(&self) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_GetTotalScore(self.addr())
        }
    }
    fn handle_car_set(&self, in_pri: PriWrapper) {
        unsafe {
            GameEvent_Soccar_TA_HandleCarSet(self.addr(), in_pri.addr());
        }
    }
    fn remove_pri(&self, pri: PriWrapper) {
        unsafe {
            GameEvent_Soccar_TA_RemovePRI(self.addr(), pri.addr());
        }
    }
    fn add_pri(&self, pri: PriWrapper) {
        unsafe {
            GameEvent_Soccar_TA_AddPRI(self.addr(), pri.addr());
        }
    }
    fn on_game_winner_set(&self) {
        unsafe {
            GameEvent_Soccar_TA_OnGameWinnerSet(self.addr());
        }
    }
    fn mvp_sort(&self, a: PriWrapper, b: PriWrapper) -> i32 {
        unsafe {
            GameEvent_Soccar_TA_MVPSort(self.addr(), a.addr(), b.addr())
        }
    }
    fn handle_hit_goal(&self, ball: BallWrapper, goal: GoalWrapper) {
        unsafe {
            GameEvent_Soccar_TA_HandleHitGoal(self.addr(), ball.addr(), goal.addr());
        }
    }
    fn clear_replicated_scored_on_team(&self) {
        unsafe {
            GameEvent_Soccar_TA_ClearReplicatedScoredOnTeam(self.addr());
        }
    }
    fn trigger_score_changed_event(&self) {
        unsafe {
            GameEvent_Soccar_TA_TriggerScoreChangedEvent(self.addr());
        }
    }
    fn handle_score_updated(&self, team: TeamWrapper) {
        unsafe {
            GameEvent_Soccar_TA_HandleScoreUpdated(self.addr(), team.addr());
        }
    }
    fn on_all_teams_created(&self) {
        unsafe {
            GameEvent_Soccar_TA_OnAllTeamsCreated(self.addr());
        }
    }
    fn trigger_goal_score_event(&self, team_scored_on: i32, scorer: CarWrapper) {
        unsafe {
            GameEvent_Soccar_TA_TriggerGoalScoreEvent(self.addr(), team_scored_on, scorer.addr());
        }
    }
    fn set_total_game_balls2(&self, total_balls: i32) {
        unsafe {
            GameEvent_Soccar_TA_SetTotalGameBalls2(self.addr(), total_balls);
        }
    }
    fn record_recent_players(&self) {
        unsafe {
            GameEvent_Soccar_TA_RecordRecentPlayers(self.addr());
        }
    }
    fn update_stats(&self) {
        unsafe {
            GameEvent_Soccar_TA_UpdateStats(self.addr());
        }
    }
    fn notify_kismet_of_current_time(&self) {
        unsafe {
            GameEvent_Soccar_TA_NotifyKismetOfCurrentTime(self.addr());
        }
    }
    fn enough_time_passed_to_forfeit(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_EnoughTimePassedToForfeit(self.addr())
        }
    }
    fn on_game_time_updated(&self) {
        unsafe {
            GameEvent_Soccar_TA_OnGameTimeUpdated(self.addr());
        }
    }
    fn on_overtime_updated(&self) {
        unsafe {
            GameEvent_Soccar_TA_OnOvertimeUpdated(self.addr());
        }
    }
    fn force_overtime(&self) {
        unsafe {
            GameEvent_Soccar_TA_ForceOvertime(self.addr());
        }
    }
    fn start_overtime(&self) {
        unsafe {
            GameEvent_Soccar_TA_StartOvertime(self.addr());
        }
    }
    fn on_my_half(&self, test_location: Vector, team_num: u8) -> bool {
        unsafe {
            let mut test_location = test_location;
            let test_location: *mut Vector = &mut test_location as *mut Vector;
            GameEvent_Soccar_TA_OnMyHalf(self.addr(), test_location, team_num)
        }
    }
    fn get_winning_team(&self) -> Option<TeamWrapper> {
        unsafe {
            TeamWrapper::try_new(GameEvent_Soccar_TA_GetWinningTeam(self.addr()))
        }
    }
    fn on_ball_spawned(&self, new_ball: BallWrapper) {
        unsafe {
            GameEvent_Soccar_TA_OnBallSpawned(self.addr(), new_ball.addr());
        }
    }
    fn reset_balls(&self) {
        unsafe {
            GameEvent_Soccar_TA_ResetBalls(self.addr());
        }
    }
    fn freeze_pawns(&self) {
        unsafe {
            GameEvent_Soccar_TA_FreezePawns(self.addr());
        }
    }
    fn destroy_balls(&self) {
        unsafe {
            GameEvent_Soccar_TA_DestroyBalls(self.addr());
        }
    }
    fn remove_game_ball(&self, ball: BallWrapper) {
        unsafe {
            GameEvent_Soccar_TA_RemoveGameBall(self.addr(), ball.addr());
        }
    }
    fn add_game_ball(&self, ball: BallWrapper) {
        unsafe {
            GameEvent_Soccar_TA_AddGameBall(self.addr(), ball.addr());
        }
    }
    fn start_new_round(&self) {
        unsafe {
            GameEvent_Soccar_TA_StartNewRound(self.addr());
        }
    }
    fn check_for_auto_balance(&self) {
        unsafe {
            GameEvent_Soccar_TA_CheckForAutoBalance(self.addr());
        }
    }
    fn has_winner(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_HasWinner(self.addr())
        }
    }
    fn submit_match(&self) {
        unsafe {
            GameEvent_Soccar_TA_SubmitMatch(self.addr());
        }
    }
    fn submit_match_complete(&self) {
        unsafe {
            GameEvent_Soccar_TA_SubmitMatchComplete(self.addr());
        }
    }
    fn on_match_ended(&self) {
        unsafe {
            GameEvent_Soccar_TA_OnMatchEnded(self.addr());
        }
    }
    fn should_do_podium_spotlight(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_ShouldDoPodiumSpotlight(self.addr())
        }
    }
    fn end_game(&self) {
        unsafe {
            GameEvent_Soccar_TA_EndGame(self.addr());
        }
    }
    fn update_team_scores(&self) {
        unsafe {
            GameEvent_Soccar_TA_UpdateTeamScores(self.addr());
        }
    }
    fn start_new_game(&self) {
        unsafe {
            GameEvent_Soccar_TA_StartNewGame(self.addr());
        }
    }
    fn reset_game(&self) {
        unsafe {
            GameEvent_Soccar_TA_ResetGame(self.addr());
        }
    }
    fn clear_replicated_stat_event(&self) {
        unsafe {
            GameEvent_Soccar_TA_ClearReplicatedStatEvent(self.addr());
        }
    }
    fn init_bot_detection(&self) {
        unsafe {
            GameEvent_Soccar_TA_InitBotDetection(self.addr());
        }
    }
    fn init_crowd_manager(&self) {
        unsafe {
            GameEvent_Soccar_TA_InitCrowdManager(self.addr());
        }
    }
    fn init_field(&self) {
        unsafe {
            GameEvent_Soccar_TA_InitField(self.addr());
        }
    }
    fn init_game_observer(&self) {
        unsafe {
            GameEvent_Soccar_TA_InitGameObserver(self.addr());
        }
    }
    fn on_init(&self) {
        unsafe {
            GameEvent_Soccar_TA_OnInit(self.addr());
        }
    }
    fn init_mutators(&self) {
        unsafe {
            GameEvent_Soccar_TA_InitMutators(self.addr());
        }
    }
    fn on_club_match(&self) {
        unsafe {
            GameEvent_Soccar_TA_OnClubMatch(self.addr());
        }
    }
    fn can_init_club_match(&self) -> bool {
        unsafe {
            GameEvent_Soccar_TA_CanInitClubMatch(self.addr())
        }
    }
    fn assign_custom_team_settings(&self) {
        unsafe {
            GameEvent_Soccar_TA_AssignCustomTeamSettings(self.addr());
        }
    }
    fn init_game(&self, options: RLString) {
        unsafe {
            let mut options = options;
            let options: *mut RLString = &mut options as *mut RLString;
            GameEvent_Soccar_TA_InitGame(self.addr(), options);
        }
    }
    fn event_game_winner_set(&self, game_event: ServerWrapper) {
        unsafe {
            GameEvent_Soccar_TA_EventGameWinnerSet(self.addr(), game_event.addr());
        }
    }
    fn event_goal_scored(&self, game_event: ServerWrapper, ball: BallWrapper, goal: GoalWrapper, score_index: i32, assist_idx: i32) {
        unsafe {
            GameEvent_Soccar_TA_EventGoalScored(self.addr(), game_event.addr(), ball.addr(), goal.addr(), score_index, assist_idx);
        }
    }

}

extern "C" {
    fn Server_GetBall(obj: usize) -> usize;
    fn Server_SpawnCar(obj: usize, carBody: i32, name: *const c_char);
    fn Server_SpawnBot(obj: usize, carBody: i32, name: *const c_char);
    fn Server_SpawnBall(obj: usize, position: *mut Vector, wake: bool, spawnCannon: bool) -> usize;
    fn Server_HasAuthority(obj: usize) -> bool;
    fn Server_SetGameSpeed(obj: usize, GameSpeed: f32);
    fn Server_GetGameSpeed(obj: usize) -> f32;
    fn Server_SetSecondsElapsed(obj: usize, SecondsElapsed: f32);
    fn Server_GetSecondsElapsed(obj: usize) -> f32;
    fn Server_GetGameCar(obj: usize) -> usize;
    fn Server_IsBallMovingTowardsGoal(obj: usize, goalNo: i32, bw: usize) -> bool;
    fn Server_IsInGoal(obj: usize, vec: *mut Vector) -> bool;
    fn Server_DisableGoalReset(obj: usize);
    fn Server_EnableGoalReset(obj: usize);
    fn Server_GenerateShot(obj: usize, startPos: *mut Vector, destination: *mut Vector, speed: f32, result: *mut Vector);
    fn Server_GenerateGoalAimLocation(obj: usize, goalNumber: i32, currentBallLocation: *mut Vector, result: *mut Vector);
    fn Server_GetGoalExtent(obj: usize, goalNumber: i32, result: *mut Vector);
    fn Server_GetGoalLocation(obj: usize, goalNumber: i32, result: *mut Vector);

    fn GameEvent_Soccar_TA_Get_TestCarArchetype(obj: usize) -> usize;
    fn ServerWrapper_SetTestCarArchetype(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_BallArchetype(obj: usize) -> usize;
    fn ServerWrapper_SetBallArchetype(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_BallSpawnPoint(obj: usize) -> usize;
    fn ServerWrapper_SetBallSpawnPoint(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_SeriesLength(obj: usize) -> i32;
    fn ServerWrapper_SetSeriesLength(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_GameTime(obj: usize) -> i32;
    fn ServerWrapper_SetGameTime(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_WarmupTime(obj: usize) -> i32;
    fn ServerWrapper_SetWarmupTime(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_MaxScore(obj: usize) -> i32;
    fn ServerWrapper_SetMaxScore(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_AutoBalanceDifference(obj: usize) -> i32;
    fn ServerWrapper_SetAutoBalanceDifference(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_ScoreSlomoTime(obj: usize) -> f32;
    fn ServerWrapper_SetScoreSlomoTime(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_GameTimeRemaining(obj: usize) -> f32;
    fn ServerWrapper_SetGameTimeRemaining(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_SecondsRemaining(obj: usize) -> i32;
    fn ServerWrapper_SetSecondsRemaining(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_WaitTimeRemaining(obj: usize) -> i32;
    fn ServerWrapper_SetWaitTimeRemaining(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_TotalGameTimePlayed(obj: usize) -> f32;
    fn ServerWrapper_SetTotalGameTimePlayed(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_OvertimeTimePlayed(obj: usize) -> f32;
    fn ServerWrapper_SetOvertimeTimePlayed(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_bRoundActive(obj: usize) -> bool;
    fn ServerWrapper_SetbRoundActive(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bPlayReplays(obj: usize) -> bool;
    fn ServerWrapper_SetbPlayReplays(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bBallHasBeenHit(obj: usize) -> bool;
    fn ServerWrapper_SetbBallHasBeenHit(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bOverTime(obj: usize) -> bool;
    fn ServerWrapper_SetbOverTime(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bUnlimitedTime(obj: usize) -> bool;
    fn ServerWrapper_SetbUnlimitedTime(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bNoContest(obj: usize) -> bool;
    fn ServerWrapper_SetbNoContest(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bDisableGoalDelay(obj: usize) -> bool;
    fn ServerWrapper_SetbDisableGoalDelay(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bShowNoScorerGoalMessage(obj: usize) -> bool;
    fn ServerWrapper_SetbShowNoScorerGoalMessage(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bMatchEnded(obj: usize) -> bool;
    fn ServerWrapper_SetbMatchEnded(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bShowIntroScene(obj: usize) -> bool;
    fn ServerWrapper_SetbShowIntroScene(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_bClubMatch(obj: usize) -> bool;
    fn ServerWrapper_SetbClubMatch(obj: usize, new_val: bool);
    fn GameEvent_Soccar_TA_Get_NextSpawnIndex(obj: usize) -> i32;
    fn ServerWrapper_SetNextSpawnIndex(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_ReplayDirectorArchetype(obj: usize) -> usize;
    fn ServerWrapper_SetReplayDirectorArchetype(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_ReplayDirector(obj: usize) -> usize;
    fn ServerWrapper_SetReplayDirector(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_GameBalls(obj: usize, result: *mut RLArrayRaw);
    fn GameEvent_Soccar_TA_Get_TotalGameBalls(obj: usize) -> i32;
    fn ServerWrapper_SetTotalGameBalls(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_PostGoalTime(obj: usize) -> f32;
    fn ServerWrapper_SetPostGoalTime(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_Goals(obj: usize, result: *mut RLArrayRaw);
    fn GameEvent_Soccar_TA_Get_SecondsRemainingCountdown(obj: usize) -> i32;
    fn ServerWrapper_SetSecondsRemainingCountdown(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_FieldCenter(obj: usize, result: *mut Vector);
    fn ServerWrapper_SetFieldCenter(obj: usize, new_val: *mut Vector);
    fn GameEvent_Soccar_TA_Get_GameWinner(obj: usize) -> usize;
    fn ServerWrapper_SetGameWinner(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_MatchWinner(obj: usize) -> usize;
    fn ServerWrapper_SetMatchWinner(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_ReplicatedScoredOnTeam(obj: usize) -> u8;
    fn ServerWrapper_SetReplicatedScoredOnTeam(obj: usize, new_val: u8);
    fn GameEvent_Soccar_TA_Get_ReplicatedServerPerformanceState(obj: usize) -> u8;
    fn ServerWrapper_SetReplicatedServerPerformanceState(obj: usize, new_val: u8);
    fn GameEvent_Soccar_TA_Get_MVP(obj: usize) -> usize;
    fn ServerWrapper_SetMVP(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_FastestGoalPlayer(obj: usize) -> usize;
    fn ServerWrapper_SetFastestGoalPlayer(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_SlowestGoalPlayer(obj: usize) -> usize;
    fn ServerWrapper_SetSlowestGoalPlayer(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_FurthestGoalPlayer(obj: usize) -> usize;
    fn ServerWrapper_SetFurthestGoalPlayer(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_Get_FastestGoalSpeed(obj: usize) -> f32;
    fn ServerWrapper_SetFastestGoalSpeed(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_SlowestGoalSpeed(obj: usize) -> f32;
    fn ServerWrapper_SetSlowestGoalSpeed(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_FurthestGoal(obj: usize) -> f32;
    fn ServerWrapper_SetFurthestGoal(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_RoundNum(obj: usize) -> i32;
    fn ServerWrapper_SetRoundNum(obj: usize, new_val: i32);
    fn GameEvent_Soccar_TA_Get_AssistMaxTime(obj: usize) -> f32;
    fn ServerWrapper_SetAssistMaxTime(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_BallHasBeenHitStartDelay(obj: usize) -> f32;
    fn ServerWrapper_SetBallHasBeenHitStartDelay(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_PodiumDelay(obj: usize) -> f32;
    fn ServerWrapper_SetPodiumDelay(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_PodiumTime(obj: usize) -> f32;
    fn ServerWrapper_SetPodiumTime(obj: usize, new_val: f32);
    fn GameEvent_Soccar_TA_Get_Pauser(obj: usize) -> usize;
    fn ServerWrapper_SetPauser(obj: usize, new_val: usize);
    fn GameEvent_Soccar_TA_CheckStart(obj: usize);
    fn GameEvent_Soccar_TA_GetPlayerCarCount(obj: usize) -> i32;
    fn GameEvent_Soccar_TA_ReplicateSkillTiers(obj: usize);
    fn GameEvent_Soccar_TA_CanSpawnBots(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_StartRound(obj: usize);
    fn GameEvent_Soccar_TA_EndRound(obj: usize);
    fn GameEvent_Soccar_TA_SetBallEventListeners(obj: usize, Ball: usize, bListen: bool);
    fn GameEvent_Soccar_TA_CanAwardPoints(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_HandleCarTouch(obj: usize, Ball: usize, HitCar: usize, HitType: u8);
    fn GameEvent_Soccar_TA_SetBallHasBeenHit(obj: usize);
    fn GameEvent_Soccar_TA_DetermineScoreTouchIndex(obj: usize, Ball: usize, Goal: usize) -> i32;
    fn GameEvent_Soccar_TA_DetermineAssistTouchIndex(obj: usize, Ball: usize, ScoreIdx: i32) -> i32;
    fn GameEvent_Soccar_TA_UpdateTotalGameTimePlayed(obj: usize, DeltaTime: f32);
    fn GameEvent_Soccar_TA_UpdateGameTime(obj: usize, DeltaTime: f32);
    fn GameEvent_Soccar_TA_CanUpdateGameTime(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_StartReplay(obj: usize);
    fn GameEvent_Soccar_TA_HandleReplayFinished(obj: usize, InReplay: usize);
    fn GameEvent_Soccar_TA_GotoPodiumSpotlight(obj: usize);
    fn GameEvent_Soccar_TA_UpdateSpotlight(obj: usize);
    fn GameEvent_Soccar_TA_SpawnPodiumCars(obj: usize);
    fn GameEvent_Soccar_TA_CanEnableCarPodiumMovement(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_FinishEvent(obj: usize);
    fn GameEvent_Soccar_TA___ReplicatedServerPerformanceState__ChangeNotifyFunc(obj: usize);
    fn GameEvent_Soccar_TA___bClubMatch__ChangeNotifyFunc(obj: usize);
    fn GameEvent_Soccar_TA___bShowIntroScene__ChangeNotifyFunc(obj: usize);
    fn GameEvent_Soccar_TA___WaitTimeRemaining__ChangeNotifyFunc(obj: usize);
    fn GameEvent_Soccar_TA_CheckJoinInProgress(obj: usize, PRI: usize);
    fn GameEvent_Soccar_TA_AllowDynamicCrowd(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_AddBallTrajectory(obj: usize, InBall: usize);
    fn GameEvent_Soccar_TA_ShowScorerGoalMessage(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_CanUseBallCam(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_DisableStatXP(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_ForceMatchStart(obj: usize);
    fn GameEvent_Soccar_TA_RemoveLocalPlayer(obj: usize, Player: usize);
    fn GameEvent_Soccar_TA_AddLocalPlayer(obj: usize, Player: usize);
    fn GameEvent_Soccar_TA_DestroyGoalIndicators(obj: usize, Player: usize);
    fn GameEvent_Soccar_TA_CreateGoalIndicators(obj: usize, Player: usize);
    fn GameEvent_Soccar_TA_BeginHighlightsReplay(obj: usize);
    fn GameEvent_Soccar_TA_ShouldCountUp(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_ShouldAllowVoteToForfeit(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_ShouldHaveLeaveMatchPenalty(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_SetPaused(obj: usize, InPauser: usize, bInPaused: bool);
    fn GameEvent_Soccar_TA_ShouldCountdownResumeFromPause(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_SetScoreAndTime(obj: usize, PC: usize, NewScoreTeam0: i32, NewScoreTeam1: i32, InGameTimeRemaining: i32, bInOvertime: bool, bRestartRound: bool);
    fn GameEvent_Soccar_TA_SaveLocalPlayerStats(obj: usize);
    fn GameEvent_Soccar_TA_ShouldPlayReplay(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_ShouldRecordReplay(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_OnBallHasBeenHit(obj: usize);
    fn GameEvent_Soccar_TA_SpawnBall(obj: usize, SpawnLoc: *mut Vector, bWake: bool, bSpawnCannon: bool, BallArch: *mut RLString) -> usize;
    fn GameEvent_Soccar_TA_GetTotalScore(obj: usize) -> i32;
    fn GameEvent_Soccar_TA_HandleCarSet(obj: usize, InPRI: usize);
    fn GameEvent_Soccar_TA_RemovePRI(obj: usize, PRI: usize);
    fn GameEvent_Soccar_TA_AddPRI(obj: usize, PRI: usize);
    fn GameEvent_Soccar_TA_OnGameWinnerSet(obj: usize);
    fn GameEvent_Soccar_TA_MVPSort(obj: usize, A: usize, B: usize) -> i32;
    fn GameEvent_Soccar_TA_HandleHitGoal(obj: usize, Ball: usize, Goal: usize);
    fn GameEvent_Soccar_TA_ClearReplicatedScoredOnTeam(obj: usize);
    fn GameEvent_Soccar_TA_TriggerScoreChangedEvent(obj: usize);
    fn GameEvent_Soccar_TA_HandleScoreUpdated(obj: usize, Team: usize);
    fn GameEvent_Soccar_TA_OnAllTeamsCreated(obj: usize);
    fn GameEvent_Soccar_TA_TriggerGoalScoreEvent(obj: usize, TeamScoredOn: i32, Scorer: usize);
    fn GameEvent_Soccar_TA_SetTotalGameBalls2(obj: usize, TotalBalls: i32);
    fn GameEvent_Soccar_TA_RecordRecentPlayers(obj: usize);
    fn GameEvent_Soccar_TA_UpdateStats(obj: usize);
    fn GameEvent_Soccar_TA_NotifyKismetOfCurrentTime(obj: usize);
    fn GameEvent_Soccar_TA_EnoughTimePassedToForfeit(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_OnGameTimeUpdated(obj: usize);
    fn GameEvent_Soccar_TA_OnOvertimeUpdated(obj: usize);
    fn GameEvent_Soccar_TA_ForceOvertime(obj: usize);
    fn GameEvent_Soccar_TA_StartOvertime(obj: usize);
    fn GameEvent_Soccar_TA_OnMyHalf(obj: usize, TestLocation: *mut Vector, TeamNum: u8) -> bool;
    fn GameEvent_Soccar_TA_GetWinningTeam(obj: usize) -> usize;
    fn GameEvent_Soccar_TA_OnBallSpawned(obj: usize, NewBall: usize);
    fn GameEvent_Soccar_TA_ResetBalls(obj: usize);
    fn GameEvent_Soccar_TA_FreezePawns(obj: usize);
    fn GameEvent_Soccar_TA_DestroyBalls(obj: usize);
    fn GameEvent_Soccar_TA_RemoveGameBall(obj: usize, Ball: usize);
    fn GameEvent_Soccar_TA_AddGameBall(obj: usize, Ball: usize);
    fn GameEvent_Soccar_TA_StartNewRound(obj: usize);
    fn GameEvent_Soccar_TA_CheckForAutoBalance(obj: usize);
    fn GameEvent_Soccar_TA_HasWinner(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_SubmitMatch(obj: usize);
    fn GameEvent_Soccar_TA_SubmitMatchComplete(obj: usize);
    fn GameEvent_Soccar_TA_OnMatchEnded(obj: usize);
    fn GameEvent_Soccar_TA_ShouldDoPodiumSpotlight(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_EndGame(obj: usize);
    fn GameEvent_Soccar_TA_UpdateTeamScores(obj: usize);
    fn GameEvent_Soccar_TA_StartNewGame(obj: usize);
    fn GameEvent_Soccar_TA_ResetGame(obj: usize);
    fn GameEvent_Soccar_TA_ClearReplicatedStatEvent(obj: usize);
    fn GameEvent_Soccar_TA_InitBotDetection(obj: usize);
    fn GameEvent_Soccar_TA_InitCrowdManager(obj: usize);
    fn GameEvent_Soccar_TA_InitField(obj: usize);
    fn GameEvent_Soccar_TA_InitGameObserver(obj: usize);
    fn GameEvent_Soccar_TA_OnInit(obj: usize);
    fn GameEvent_Soccar_TA_InitMutators(obj: usize);
    fn GameEvent_Soccar_TA_OnClubMatch(obj: usize);
    fn GameEvent_Soccar_TA_CanInitClubMatch(obj: usize) -> bool;
    fn GameEvent_Soccar_TA_AssignCustomTeamSettings(obj: usize);
    fn GameEvent_Soccar_TA_InitGame(obj: usize, Options: *mut RLString);
    fn GameEvent_Soccar_TA_EventGameWinnerSet(obj: usize, GameEvent: usize);
    fn GameEvent_Soccar_TA_EventGoalScored(obj: usize, GameEvent: usize, Ball: usize, Goal: usize, ScoreIndex: i32, AssistIdx: i32);

}