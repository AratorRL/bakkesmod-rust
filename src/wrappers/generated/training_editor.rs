use crate::wrappers::*;
use super::*;

pub struct TrainingEditorWrapper(pub usize);
impl_object!(TrainingEditorWrapper);

impl TrainingEditor for TrainingEditorWrapper {}
impl GameEditor for TrainingEditorWrapper {}
impl Server for TrainingEditorWrapper {}
impl TeamGameEvent for TrainingEditorWrapper {}
impl GameEvent for TrainingEditorWrapper {}
impl Actor for TrainingEditorWrapper {}

pub trait TrainingEditor : GameEditor {
    fn get_min_round_time(&self) -> f32 {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_MinRoundTime(self.addr())
        }
    }
    fn get_max_round_time(&self) -> f32 {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_MaxRoundTime(self.addr())
        }
    }
    fn get_b_no_editor(&self) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_bNoEditor(self.addr())
        }
    }
    fn get_b_displayed_redo_penalty_message(&self) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_bDisplayedRedoPenaltyMessage(self.addr())
        }
    }
    fn get_b_unsaved_changes(&self) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_bUnsavedChanges(self.addr())
        }
    }
    fn get_points_scored_this_round(&self) -> i32 {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_PointsScoredThisRound(self.addr())
        }
    }
    fn get_shot_attempt(&self) -> i32 {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_ShotAttempt(self.addr())
        }
    }
    fn get_goalie_score(&self) -> i32 {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_GoalieScore(self.addr())
        }
    }
    fn get_play_test_type(&self) -> u8 {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_PlayTestType(self.addr())
        }
    }
    fn get_goal_mesh_blockers(&self) -> RLArray<ActorWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            GameEvent_TrainingEditor_TA_Get_GoalMeshBlockers(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_goal_mesh_blocker_archetype(&self) -> Option<ActorWrapper> {
        unsafe {
            ActorWrapper::try_new(GameEvent_TrainingEditor_TA_Get_GoalMeshBlockerArchetype(self.addr()))
        }
    }
    fn get_training_data(&self) -> Option<GameEditorSaveDataWrapper> {
        unsafe {
            GameEditorSaveDataWrapper::try_new(GameEvent_TrainingEditor_TA_Get_TrainingData(self.addr()))
        }
    }
    fn get_save_delay_time(&self) -> f32 {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_SaveDelayTime(self.addr())
        }
    }
    fn get_save_cooldown(&self) -> f32 {
        unsafe {
            GameEvent_TrainingEditor_TA_Get_SaveCooldown(self.addr())
        }
    }
    fn get_training_file_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            GameEvent_TrainingEditor_TA_Get_TrainingFileName(self.addr(), result_ptr);
            result
        }
    }
    fn on_loading_movie_closed(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_OnLoadingMovieClosed(self.addr());
        }
    }
    fn tag_history_changes(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_TagHistoryChanges(self.addr());
        }
    }
    fn mark_as_dirty(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_MarkAsDirty(self.addr());
        }
    }
    fn force_tag_history_changes(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_ForceTagHistoryChanges(self.addr());
        }
    }
    fn set_traced_crosshair_actor(&self, pc: PlayerControllerWrapper, new_actor: ActorWrapper) {
        unsafe {
            GameEvent_TrainingEditor_TA_SetTracedCrosshairActor(self.addr(), pc.addr(), new_actor.addr());
        }
    }
    fn allow_dynamic_crowd(&self) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_AllowDynamicCrowd(self.addr())
        }
    }
    fn broadcast_go_message(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_BroadcastGoMessage(self.addr());
        }
    }
    fn get_total_rounds(&self) -> i32 {
        unsafe {
            GameEvent_TrainingEditor_TA_GetTotalRounds(self.addr())
        }
    }
    fn get_difficulty(&self) -> u8 {
        unsafe {
            GameEvent_TrainingEditor_TA_GetDifficulty(self.addr())
        }
    }
    fn get_training_type(&self) -> u8 {
        unsafe {
            GameEvent_TrainingEditor_TA_GetTrainingType(self.addr())
        }
    }
    fn destroy_ball(&self, ball: BallWrapper) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_DestroyBall(self.addr(), ball.addr())
        }
    }
    fn save(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_Save(self.addr());
        }
    }
    fn on_training_mode_loaded(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_OnTrainingModeLoaded(self.addr());
        }
    }
    fn duplicate_round(&self, index: i32) {
        unsafe {
            GameEvent_TrainingEditor_TA_DuplicateRound(self.addr(), index);
        }
    }
    fn reorder_round(&self, from_index: i32, to_index: i32) {
        unsafe {
            GameEvent_TrainingEditor_TA_ReorderRound(self.addr(), from_index, to_index);
        }
    }
    fn set_round_time_limit(&self, new_round_time: f32) {
        unsafe {
            GameEvent_TrainingEditor_TA_SetRoundTimeLimit(self.addr(), new_round_time);
        }
    }
    fn handle_next_game(&self) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_HandleNextGame(self.addr())
        }
    }
    fn reset_balls(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_ResetBalls(self.addr());
        }
    }
    fn load(&self, save_name: RLString, pc: PlayerControllerWrapper) {
        unsafe {
            let mut save_name = save_name;
            let save_name: *mut RLString = &mut save_name as *mut RLString;
            GameEvent_TrainingEditor_TA_Load(self.addr(), save_name, pc.addr());
        }
    }
    fn destroy_goal_mesh_blockers(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_DestroyGoalMeshBlockers(self.addr());
        }
    }
    fn update_goal_mesh_blocker(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_UpdateGoalMeshBlocker(self.addr());
        }
    }
    fn get_score(&self) -> i32 {
        unsafe {
            GameEvent_TrainingEditor_TA_GetScore(self.addr())
        }
    }
    fn remove_points_from_score(&self, points_to_remove: i32, team_index: i32) {
        unsafe {
            GameEvent_TrainingEditor_TA_RemovePointsFromScore(self.addr(), points_to_remove, team_index);
        }
    }
    fn show_scorer_goal_message(&self) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_ShowScorerGoalMessage(self.addr())
        }
    }
    fn show_reset_round_message(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_ShowResetRoundMessage(self.addr());
        }
    }
    fn show_penalty_message(&self) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_ShowPenaltyMessage(self.addr())
        }
    }
    fn get_player_team_number(&self) -> i32 {
        unsafe {
            GameEvent_TrainingEditor_TA_GetPlayerTeamNumber(self.addr())
        }
    }
    fn get_ball_goal_score_number(&self) -> i32 {
        unsafe {
            GameEvent_TrainingEditor_TA_GetBallGoalScoreNumber(self.addr())
        }
    }
    fn remove_all_points_from_score(&self, team_index: i32) {
        unsafe {
            GameEvent_TrainingEditor_TA_RemoveAllPointsFromScore(self.addr(), team_index);
        }
    }
    fn increment_round(&self, b_loop: bool) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_IncrementRound(self.addr(), b_loop)
        }
    }
    fn adjust_to_floor_location(&self, trace_start: Vector, collsion_extent: Vector) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            let mut trace_start = trace_start;
            let trace_start: *mut Vector = &mut trace_start as *mut Vector;
            let mut collsion_extent = collsion_extent;
            let collsion_extent: *mut Vector = &mut collsion_extent as *mut Vector;
            GameEvent_TrainingEditor_TA_AdjustToFloorLocation(self.addr(), trace_start, collsion_extent, result_ptr);
            result
        }
    }
    fn spawn_archetype_at_and_adjust_to_floor(&self, archetype: ActorWrapper, spawn_location: Vector, spawn_rotation: Rotator) -> Option<ActorWrapper> {
        unsafe {
            let mut spawn_location = spawn_location;
            let spawn_location: *mut Vector = &mut spawn_location as *mut Vector;
            let mut spawn_rotation = spawn_rotation;
            let spawn_rotation: *mut Rotator = &mut spawn_rotation as *mut Rotator;
            ActorWrapper::try_new(GameEvent_TrainingEditor_TA_SpawnArchetypeAtAndAdjustToFloor(self.addr(), archetype.addr(), spawn_location, spawn_rotation))
        }
    }
    fn spawn_ball_and_start_point_at(&self, ball_spawn_location: Vector, ball_spawn_rotation: Rotator, start_point_spawn_location: Vector, start_point_spawn_rotation: Rotator) {
        unsafe {
            let mut ball_spawn_location = ball_spawn_location;
            let ball_spawn_location: *mut Vector = &mut ball_spawn_location as *mut Vector;
            let mut ball_spawn_rotation = ball_spawn_rotation;
            let ball_spawn_rotation: *mut Rotator = &mut ball_spawn_rotation as *mut Rotator;
            let mut start_point_spawn_location = start_point_spawn_location;
            let start_point_spawn_location: *mut Vector = &mut start_point_spawn_location as *mut Vector;
            let mut start_point_spawn_rotation = start_point_spawn_rotation;
            let start_point_spawn_rotation: *mut Rotator = &mut start_point_spawn_rotation as *mut Rotator;
            GameEvent_TrainingEditor_TA_SpawnBallAndStartPointAt(self.addr(), ball_spawn_location, ball_spawn_rotation, start_point_spawn_location, start_point_spawn_rotation);
        }
    }
    fn setup_default_round(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_SetupDefaultRound(self.addr());
        }
    }
    fn switch_to_round_number(&self, round_number: i32, backup_current_round: bool) {
        unsafe {
            GameEvent_TrainingEditor_TA_SwitchToRoundNumber(self.addr(), round_number, backup_current_round);
        }
    }
    fn delete_round(&self, round_to_delete: i32) {
        unsafe {
            GameEvent_TrainingEditor_TA_DeleteRound(self.addr(), round_to_delete);
        }
    }
    fn restart_play_test(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_RestartPlayTest(self.addr());
        }
    }
    fn end_play_test(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_EndPlayTest(self.addr());
        }
    }
    fn start_play_test(&self, in_play_test_type: u8) {
        unsafe {
            GameEvent_TrainingEditor_TA_StartPlayTest(self.addr(), in_play_test_type);
        }
    }
    fn can_play_test_round_number(&self, round_number: i32) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_CanPlayTestRoundNumber(self.addr(), round_number)
        }
    }
    fn set_unsaved_changes(&self, b_in_unsaved_changes: bool) {
        unsafe {
            GameEvent_TrainingEditor_TA_SetUnsavedChanges(self.addr(), b_in_unsaved_changes);
        }
    }
    fn update_active_round_data(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_UpdateActiveRoundData(self.addr());
        }
    }
    fn start_new_round(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_StartNewRound(self.addr());
        }
    }
    fn is_valid_round_index(&self, array_index: i32) -> bool {
        unsafe {
            GameEvent_TrainingEditor_TA_IsValidRoundIndex(self.addr(), array_index)
        }
    }
    fn add_local_player(&self, player: PlayerControllerWrapper) {
        unsafe {
            GameEvent_TrainingEditor_TA_AddLocalPlayer(self.addr(), player.addr());
        }
    }
    fn on_init(&self) {
        unsafe {
            GameEvent_TrainingEditor_TA_OnInit(self.addr());
        }
    }
    fn event_save_result(&self, b_success: bool) {
        unsafe {
            GameEvent_TrainingEditor_TA_EventSaveResult(self.addr(), b_success);
        }
    }
    fn event_unsaved_changes(&self, b_out_unsaved_changes: bool) {
        unsafe {
            GameEvent_TrainingEditor_TA_EventUnsavedChanges(self.addr(), b_out_unsaved_changes);
        }
    }
    fn event_playtest_started(&self, gam_event: TrainingEditorWrapper) {
        unsafe {
            GameEvent_TrainingEditor_TA_EventPlaytestStarted(self.addr(), gam_event.addr());
        }
    }
    fn event_round_changed(&self, gam_event: TrainingEditorWrapper) {
        unsafe {
            GameEvent_TrainingEditor_TA_EventRoundChanged(self.addr(), gam_event.addr());
        }
    }
    fn event_round_time_changed(&self, game_event: TrainingEditorWrapper) {
        unsafe {
            GameEvent_TrainingEditor_TA_EventRoundTimeChanged(self.addr(), game_event.addr());
        }
    }

}

extern "C" {
    fn GameEvent_TrainingEditor_TA_Get_MinRoundTime(obj: usize) -> f32;
    fn TrainingEditorWrapper_SetMinRoundTime(obj: usize, new_val: f32);
    fn GameEvent_TrainingEditor_TA_Get_MaxRoundTime(obj: usize) -> f32;
    fn TrainingEditorWrapper_SetMaxRoundTime(obj: usize, new_val: f32);
    fn GameEvent_TrainingEditor_TA_Get_bNoEditor(obj: usize) -> bool;
    fn TrainingEditorWrapper_SetbNoEditor(obj: usize, new_val: bool);
    fn GameEvent_TrainingEditor_TA_Get_bDisplayedRedoPenaltyMessage(obj: usize) -> bool;
    fn TrainingEditorWrapper_SetbDisplayedRedoPenaltyMessage(obj: usize, new_val: bool);
    fn GameEvent_TrainingEditor_TA_Get_bUnsavedChanges(obj: usize) -> bool;
    fn TrainingEditorWrapper_SetbUnsavedChanges(obj: usize, new_val: bool);
    fn GameEvent_TrainingEditor_TA_Get_PointsScoredThisRound(obj: usize) -> i32;
    fn TrainingEditorWrapper_SetPointsScoredThisRound(obj: usize, new_val: i32);
    fn GameEvent_TrainingEditor_TA_Get_ShotAttempt(obj: usize) -> i32;
    fn TrainingEditorWrapper_SetShotAttempt(obj: usize, new_val: i32);
    fn GameEvent_TrainingEditor_TA_Get_GoalieScore(obj: usize) -> i32;
    fn TrainingEditorWrapper_SetGoalieScore(obj: usize, new_val: i32);
    fn GameEvent_TrainingEditor_TA_Get_PlayTestType(obj: usize) -> u8;
    fn TrainingEditorWrapper_SetPlayTestType(obj: usize, new_val: u8);
    fn GameEvent_TrainingEditor_TA_Get_GoalMeshBlockers(obj: usize, result: *mut RLArrayRaw);
    fn GameEvent_TrainingEditor_TA_Get_GoalMeshBlockerArchetype(obj: usize) -> usize;
    fn TrainingEditorWrapper_SetGoalMeshBlockerArchetype(obj: usize, new_val: usize);
    fn GameEvent_TrainingEditor_TA_Get_TrainingData(obj: usize) -> usize;
    fn TrainingEditorWrapper_SetTrainingData(obj: usize, new_val: usize);
    fn GameEvent_TrainingEditor_TA_Get_SaveDelayTime(obj: usize) -> f32;
    fn TrainingEditorWrapper_SetSaveDelayTime(obj: usize, new_val: f32);
    fn GameEvent_TrainingEditor_TA_Get_SaveCooldown(obj: usize) -> f32;
    fn TrainingEditorWrapper_SetSaveCooldown(obj: usize, new_val: f32);
    fn GameEvent_TrainingEditor_TA_Get_TrainingFileName(obj: usize, result: *mut RLString);
    fn GameEvent_TrainingEditor_TA_OnLoadingMovieClosed(obj: usize);
    fn GameEvent_TrainingEditor_TA_TagHistoryChanges(obj: usize);
    fn GameEvent_TrainingEditor_TA_MarkAsDirty(obj: usize);
    fn GameEvent_TrainingEditor_TA_ForceTagHistoryChanges(obj: usize);
    fn GameEvent_TrainingEditor_TA_SetTracedCrosshairActor(obj: usize, PC: usize, NewActor: usize);
    fn GameEvent_TrainingEditor_TA_AllowDynamicCrowd(obj: usize) -> bool;
    fn GameEvent_TrainingEditor_TA_BroadcastGoMessage(obj: usize);
    fn GameEvent_TrainingEditor_TA_GetTotalRounds(obj: usize) -> i32;
    fn GameEvent_TrainingEditor_TA_GetDifficulty(obj: usize) -> u8;
    fn GameEvent_TrainingEditor_TA_GetTrainingType(obj: usize) -> u8;
    fn GameEvent_TrainingEditor_TA_DestroyBall(obj: usize, Ball: usize) -> bool;
    fn GameEvent_TrainingEditor_TA_Save(obj: usize);
    fn GameEvent_TrainingEditor_TA_OnTrainingModeLoaded(obj: usize);
    fn GameEvent_TrainingEditor_TA_DuplicateRound(obj: usize, Index: i32);
    fn GameEvent_TrainingEditor_TA_ReorderRound(obj: usize, FromIndex: i32, ToIndex: i32);
    fn GameEvent_TrainingEditor_TA_SetRoundTimeLimit(obj: usize, NewRoundTime: f32);
    fn GameEvent_TrainingEditor_TA_HandleNextGame(obj: usize) -> bool;
    fn GameEvent_TrainingEditor_TA_ResetBalls(obj: usize);
    fn GameEvent_TrainingEditor_TA_Load(obj: usize, SaveName: *mut RLString, PC: usize);
    fn GameEvent_TrainingEditor_TA_DestroyGoalMeshBlockers(obj: usize);
    fn GameEvent_TrainingEditor_TA_UpdateGoalMeshBlocker(obj: usize);
    fn GameEvent_TrainingEditor_TA_GetScore(obj: usize) -> i32;
    fn GameEvent_TrainingEditor_TA_RemovePointsFromScore(obj: usize, PointsToRemove: i32, TeamIndex: i32);
    fn GameEvent_TrainingEditor_TA_ShowScorerGoalMessage(obj: usize) -> bool;
    fn GameEvent_TrainingEditor_TA_ShowResetRoundMessage(obj: usize);
    fn GameEvent_TrainingEditor_TA_ShowPenaltyMessage(obj: usize) -> bool;
    fn GameEvent_TrainingEditor_TA_GetPlayerTeamNumber(obj: usize) -> i32;
    fn GameEvent_TrainingEditor_TA_GetBallGoalScoreNumber(obj: usize) -> i32;
    fn GameEvent_TrainingEditor_TA_RemoveAllPointsFromScore(obj: usize, TeamIndex: i32);
    fn GameEvent_TrainingEditor_TA_IncrementRound(obj: usize, bLoop: bool) -> bool;
    fn GameEvent_TrainingEditor_TA_AdjustToFloorLocation(obj: usize, TraceStart: *mut Vector, CollsionExtent: *mut Vector, result: *mut Vector);
    fn GameEvent_TrainingEditor_TA_SpawnArchetypeAtAndAdjustToFloor(obj: usize, Archetype: usize, SpawnLocation: *mut Vector, SpawnRotation: *mut Rotator) -> usize;
    fn GameEvent_TrainingEditor_TA_SpawnBallAndStartPointAt(obj: usize, BallSpawnLocation: *mut Vector, BallSpawnRotation: *mut Rotator, StartPointSpawnLocation: *mut Vector, StartPointSpawnRotation: *mut Rotator);
    fn GameEvent_TrainingEditor_TA_SetupDefaultRound(obj: usize);
    fn GameEvent_TrainingEditor_TA_SwitchToRoundNumber(obj: usize, RoundNumber: i32, BackupCurrentRound: bool);
    fn GameEvent_TrainingEditor_TA_DeleteRound(obj: usize, RoundToDelete: i32);
    fn GameEvent_TrainingEditor_TA_RestartPlayTest(obj: usize);
    fn GameEvent_TrainingEditor_TA_EndPlayTest(obj: usize);
    fn GameEvent_TrainingEditor_TA_StartPlayTest(obj: usize, InPlayTestType: u8);
    fn GameEvent_TrainingEditor_TA_CanPlayTestRoundNumber(obj: usize, RoundNumber: i32) -> bool;
    fn GameEvent_TrainingEditor_TA_SetUnsavedChanges(obj: usize, bInUnsavedChanges: bool);
    fn GameEvent_TrainingEditor_TA_UpdateActiveRoundData(obj: usize);
    fn GameEvent_TrainingEditor_TA_StartNewRound(obj: usize);
    fn GameEvent_TrainingEditor_TA_IsValidRoundIndex(obj: usize, ArrayIndex: i32) -> bool;
    fn GameEvent_TrainingEditor_TA_AddLocalPlayer(obj: usize, Player: usize);
    fn GameEvent_TrainingEditor_TA_OnInit(obj: usize);
    fn GameEvent_TrainingEditor_TA_EventSaveResult(obj: usize, bSuccess: bool);
    fn GameEvent_TrainingEditor_TA_EventUnsavedChanges(obj: usize, bOutUnsavedChanges: bool);
    fn GameEvent_TrainingEditor_TA_EventPlaytestStarted(obj: usize, GamEvent: usize);
    fn GameEvent_TrainingEditor_TA_EventRoundChanged(obj: usize, GamEvent: usize);
    fn GameEvent_TrainingEditor_TA_EventRoundTimeChanged(obj: usize, GameEvent: usize);

}