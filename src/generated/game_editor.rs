use crate::wrappers::*;
use crate::generated::*;

pub struct GameEditorWrapper(pub usize);
impl_object!(GameEditorWrapper);

impl GameEditor for GameEditorWrapper {}
impl Server for GameEditorWrapper {}
impl TeamGameEvent for GameEditorWrapper {}
impl GameEvent for GameEditorWrapper {}
impl Actor for GameEditorWrapper {}

pub trait GameEditor : Server {
	fn get_active_round_number(&self) -> i32 {
		unsafe {
			GameEvent_GameEditor_TA_Get_ActiveRoundNumber(self.addr())
		}
	}
	fn get_max_rounds(&self) -> i32 {
		unsafe {
			GameEvent_GameEditor_TA_Get_MaxRounds(self.addr())
		}
	}
	fn get_history_position(&self) -> i32 {
		unsafe {
			GameEvent_GameEditor_TA_Get_HistoryPosition(self.addr())
		}
	}
	fn get_max_undo_history(&self) -> i32 {
		unsafe {
			GameEvent_GameEditor_TA_Get_MaxUndoHistory(self.addr())
		}
	}
	fn get_fx_actor_archetype(&self) -> Option<FXActorWrapper> {
		unsafe {
			FXActorWrapper::try_new(GameEvent_GameEditor_TA_Get_FXActorArchetype(self.addr()))
		}
	}
	fn get_fx_actor(&self) -> Option<FXActorWrapper> {
		unsafe {
			FXActorWrapper::try_new(GameEvent_GameEditor_TA_Get_FXActor(self.addr()))
		}
	}
	fn rotate_actor(&self, pc: PlayerControllerWrapper, b_snap_orientation: bool) {
		unsafe {
			GameEvent_GameEditor_TA_RotateActor(self.addr(), pc.addr(), b_snap_orientation);
		}
	}
	fn prev_round(&self) {
		unsafe {
			GameEvent_GameEditor_TA_PrevRound(self.addr());
		}
	}
	fn next_round(&self) {
		unsafe {
			GameEvent_GameEditor_TA_NextRound(self.addr());
		}
	}
	fn decrease_time(&self) {
		unsafe {
			GameEvent_GameEditor_TA_DecreaseTime(self.addr());
		}
	}
	fn increase_time(&self) {
		unsafe {
			GameEvent_GameEditor_TA_IncreaseTime(self.addr());
		}
	}
	fn stop_editing(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_StopEditing(self.addr(), pc.addr());
		}
	}
	fn start_editing(&self) {
		unsafe {
			GameEvent_GameEditor_TA_StartEditing(self.addr());
		}
	}
	fn cycle_actor(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_CycleActor(self.addr(), pc.addr());
		}
	}
	fn release_grabbed_actor(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_ReleaseGrabbedActor(self.addr(), pc.addr());
		}
	}
	fn release_rotate_actor(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_ReleaseRotateActor(self.addr(), pc.addr());
		}
	}
	fn toggle_rotate_actor(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_ToggleRotateActor(self.addr(), pc.addr());
		}
	}
	fn toggle_grab_actor(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_ToggleGrabActor(self.addr(), pc.addr());
		}
	}
	fn toggle_release_actor(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_ToggleReleaseActor(self.addr(), pc.addr());
		}
	}
	fn release_actor(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_ReleaseActor(self.addr(), pc.addr());
		}
	}
	fn grab_actor(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_GrabActor(self.addr(), pc.addr());
		}
	}
	fn can_que_save_replay(&self) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_CanQueSaveReplay(self.addr())
		}
	}
	fn should_update_crosshair(&self) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_ShouldUpdateCrosshair(self.addr())
		}
	}
	fn get_player_team_number(&self) -> i32 {
		unsafe {
			GameEvent_GameEditor_TA_GetPlayerTeamNumber(self.addr())
		}
	}
	fn can_add_history(&self) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_CanAddHistory(self.addr())
		}
	}
	fn toggle_editor_round(&self) {
		unsafe {
			GameEvent_GameEditor_TA_ToggleEditorRound(self.addr());
		}
	}
	fn commit_redo_round(&self) {
		unsafe {
			GameEvent_GameEditor_TA_CommitRedoRound(self.addr());
		}
	}
	fn reset_round(&self) {
		unsafe {
			GameEvent_GameEditor_TA_ResetRound(self.addr());
		}
	}
	fn save(&self) {
		unsafe {
			GameEvent_GameEditor_TA_Save(self.addr());
		}
	}
	fn redo(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_Redo(self.addr(), pc.addr());
		}
	}
	fn undo(&self, pc: PlayerControllerWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_Undo(self.addr(), pc.addr());
		}
	}
	fn get_other_history_type(&self, history_type: u8) -> u8 {
		unsafe {
			GameEvent_GameEditor_TA_GetOtherHistoryType(self.addr(), history_type)
		}
	}
	fn clamp_undo_history(&self) {
		unsafe {
			GameEvent_GameEditor_TA_ClampUndoHistory(self.addr());
		}
	}
	fn clear_redo_history(&self) {
		unsafe {
			GameEvent_GameEditor_TA_ClearRedoHistory(self.addr());
		}
	}
	fn clear_all_history(&self) {
		unsafe {
			GameEvent_GameEditor_TA_ClearAllHistory(self.addr());
		}
	}
	fn destroy_actor(&self, a: ActorWrapper, history_type: u8) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_DestroyActor(self.addr(), a.addr(), history_type)
		}
	}
	fn is_in_editor_mode(&self) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_IsInEditorMode(self.addr())
		}
	}
	fn remove_all_points_from_score(&self, team_index: i32) {
		unsafe {
			GameEvent_GameEditor_TA_RemoveAllPointsFromScore(self.addr(), team_index);
		}
	}
	fn remove_points_from_score(&self, points_to_remove: i32, team_index: i32) {
		unsafe {
			GameEvent_GameEditor_TA_RemovePointsFromScore(self.addr(), points_to_remove, team_index);
		}
	}
	fn delete_all_existing_actors_based_off_spawn_list(&self) {
		unsafe {
			GameEvent_GameEditor_TA_DeleteAllExistingActorsBasedOffSpawnList(self.addr());
		}
	}
	fn round_contains_a_switch(&self) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_RoundContainsASwitch(self.addr())
		}
	}
	fn enable_triggers(&self, b_enable: bool) {
		unsafe {
			GameEvent_GameEditor_TA_EnableTriggers(self.addr(), b_enable);
		}
	}
	fn hide_car_spawn_points(&self, b_hide: bool) {
		unsafe {
			GameEvent_GameEditor_TA_HideCarSpawnPoints(self.addr(), b_hide);
		}
	}
	fn reset_spawn_locations(&self) {
		unsafe {
			GameEvent_GameEditor_TA_ResetSpawnLocations(self.addr());
		}
	}
	fn on_spawned_archetype(&self, spawned_actor: ActorWrapper, history_type: u8) {
		unsafe {
			GameEvent_GameEditor_TA_OnSpawnedArchetype(self.addr(), spawned_actor.addr(), history_type);
		}
	}
	fn spawn_archetype(&self, controller: PlayerControllerWrapper, archetype_index: i32) {
		unsafe {
			GameEvent_GameEditor_TA_SpawnArchetype(self.addr(), controller.addr(), archetype_index);
		}
	}
	fn increment_selected_spawn_archetype_index(&self, direction: i32, index: i32) {
		unsafe {
			GameEvent_GameEditor_TA_IncrementSelectedSpawnArchetypeIndex(self.addr(), direction, index);
		}
	}
	fn can_change_team(&self) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_CanChangeTeam(self.addr())
		}
	}
	fn reset_balls_to_default_position(&self) {
		unsafe {
			GameEvent_GameEditor_TA_ResetBallsToDefaultPosition(self.addr());
		}
	}
	fn fire_balls(&self) {
		unsafe {
			GameEvent_GameEditor_TA_FireBalls(self.addr());
		}
	}
	fn interact(&self) {
		unsafe {
			GameEvent_GameEditor_TA_Interact(self.addr());
		}
	}
	fn should_reset_balls(&self) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_ShouldResetBalls(self.addr())
		}
	}
	fn on_vehicle_setup(&self, car: CarWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_OnVehicleSetup(self.addr(), car.addr());
		}
	}
	fn handle_vehicle_setup(&self, car: CarWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_HandleVehicleSetup(self.addr(), car.addr());
		}
	}
	fn on_player_restarted(&self, player_car: CarWrapper) {
		unsafe {
			GameEvent_GameEditor_TA_OnPlayerRestarted(self.addr(), player_car.addr());
		}
	}
	fn choose_team(&self, team_index: i32, player: PlayerControllerWrapper) -> bool {
		unsafe {
			GameEvent_GameEditor_TA_ChooseTeam(self.addr(), team_index, player.addr())
		}
	}
	fn init_fx(&self) {
		unsafe {
			GameEvent_GameEditor_TA_InitFX(self.addr());
		}
	}

}

extern "C" {
	fn GameEvent_GameEditor_TA_Get_ActiveRoundNumber(obj: usize) -> i32;
	fn GameEditorWrapper_SetActiveRoundNumber(obj: usize, new_val: i32);
	fn GameEvent_GameEditor_TA_Get_MaxRounds(obj: usize) -> i32;
	fn GameEditorWrapper_SetMaxRounds(obj: usize, new_val: i32);
	fn GameEvent_GameEditor_TA_Get_HistoryPosition(obj: usize) -> i32;
	fn GameEditorWrapper_SetHistoryPosition(obj: usize, new_val: i32);
	fn GameEvent_GameEditor_TA_Get_MaxUndoHistory(obj: usize) -> i32;
	fn GameEditorWrapper_SetMaxUndoHistory(obj: usize, new_val: i32);
	fn GameEvent_GameEditor_TA_Get_FXActorArchetype(obj: usize) -> usize;
	fn GameEditorWrapper_SetFXActorArchetype(obj: usize, new_val: usize);
	fn GameEvent_GameEditor_TA_Get_FXActor(obj: usize) -> usize;
	fn GameEditorWrapper_SetFXActor(obj: usize, new_val: usize);
	fn GameEvent_GameEditor_TA_RotateActor(obj: usize, PC: usize, bSnapOrientation: bool);
	fn GameEvent_GameEditor_TA_PrevRound(obj: usize);
	fn GameEvent_GameEditor_TA_NextRound(obj: usize);
	fn GameEvent_GameEditor_TA_DecreaseTime(obj: usize);
	fn GameEvent_GameEditor_TA_IncreaseTime(obj: usize);
	fn GameEvent_GameEditor_TA_StopEditing(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_StartEditing(obj: usize);
	fn GameEvent_GameEditor_TA_CycleActor(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_ReleaseGrabbedActor(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_ReleaseRotateActor(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_ToggleRotateActor(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_ToggleGrabActor(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_ToggleReleaseActor(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_ReleaseActor(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_GrabActor(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_CanQueSaveReplay(obj: usize) -> bool;
	fn GameEvent_GameEditor_TA_ShouldUpdateCrosshair(obj: usize) -> bool;
	fn GameEvent_GameEditor_TA_GetPlayerTeamNumber(obj: usize) -> i32;
	fn GameEvent_GameEditor_TA_CanAddHistory(obj: usize) -> bool;
	fn GameEvent_GameEditor_TA_ToggleEditorRound(obj: usize);
	fn GameEvent_GameEditor_TA_CommitRedoRound(obj: usize);
	fn GameEvent_GameEditor_TA_ResetRound(obj: usize);
	fn GameEvent_GameEditor_TA_Save(obj: usize);
	fn GameEvent_GameEditor_TA_Redo(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_Undo(obj: usize, PC: usize);
	fn GameEvent_GameEditor_TA_GetOtherHistoryType(obj: usize, HistoryType: u8) -> u8;
	fn GameEvent_GameEditor_TA_ClampUndoHistory(obj: usize);
	fn GameEvent_GameEditor_TA_ClearRedoHistory(obj: usize);
	fn GameEvent_GameEditor_TA_ClearAllHistory(obj: usize);
	fn GameEvent_GameEditor_TA_DestroyActor(obj: usize, A: usize, HistoryType: u8) -> bool;
	fn GameEvent_GameEditor_TA_IsInEditorMode(obj: usize) -> bool;
	fn GameEvent_GameEditor_TA_RemoveAllPointsFromScore(obj: usize, TeamIndex: i32);
	fn GameEvent_GameEditor_TA_RemovePointsFromScore(obj: usize, PointsToRemove: i32, TeamIndex: i32);
	fn GameEvent_GameEditor_TA_DeleteAllExistingActorsBasedOffSpawnList(obj: usize);
	fn GameEvent_GameEditor_TA_RoundContainsASwitch(obj: usize) -> bool;
	fn GameEvent_GameEditor_TA_EnableTriggers(obj: usize, bEnable: bool);
	fn GameEvent_GameEditor_TA_HideCarSpawnPoints(obj: usize, bHide: bool);
	fn GameEvent_GameEditor_TA_ResetSpawnLocations(obj: usize);
	fn GameEvent_GameEditor_TA_OnSpawnedArchetype(obj: usize, SpawnedActor: usize, HistoryType: u8);
	fn GameEvent_GameEditor_TA_SpawnArchetype(obj: usize, Controller: usize, ArchetypeIndex: i32);
	fn GameEvent_GameEditor_TA_IncrementSelectedSpawnArchetypeIndex(obj: usize, Direction: i32, Index: i32);
	fn GameEvent_GameEditor_TA_CanChangeTeam(obj: usize) -> bool;
	fn GameEvent_GameEditor_TA_ResetBallsToDefaultPosition(obj: usize);
	fn GameEvent_GameEditor_TA_FireBalls(obj: usize);
	fn GameEvent_GameEditor_TA_Interact(obj: usize);
	fn GameEvent_GameEditor_TA_ShouldResetBalls(obj: usize) -> bool;
	fn GameEvent_GameEditor_TA_OnVehicleSetup(obj: usize, Car: usize);
	fn GameEvent_GameEditor_TA_HandleVehicleSetup(obj: usize, Car: usize);
	fn GameEvent_GameEditor_TA_OnPlayerRestarted(obj: usize, PlayerCar: usize);
	fn GameEvent_GameEditor_TA_ChooseTeam(obj: usize, TeamIndex: i32, Player: usize) -> bool;
	fn GameEvent_GameEditor_TA_InitFX(obj: usize);

}