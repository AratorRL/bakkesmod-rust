use crate::wrappers::*;
use crate::generated::*;

pub struct ReplayDirectorWrapper(pub usize);
impl_object!(ReplayDirectorWrapper);

impl ReplayDirector for ReplayDirectorWrapper {}
impl Actor for ReplayDirectorWrapper {}

pub trait ReplayDirector : Actor {
	fn get_slomo_pre_score_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_SlomoPreScoreTime(self.addr())
		}
	}
	fn get_slomo_post_score_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_SlomoPostScoreTime(self.addr())
		}
	}
	fn get_slomo_defend_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_SlomoDefendTime(self.addr())
		}
	}
	fn get_slomo_defend_distance(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_SlomoDefendDistance(self.addr())
		}
	}
	fn get_slomo_time_dilation(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_SlomoTimeDilation(self.addr())
		}
	}
	fn get_min_replay_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_MinReplayTime(self.addr())
		}
	}
	fn get_max_replay_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_MaxReplayTime(self.addr())
		}
	}
	fn get_replay_padding(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_ReplayPadding(self.addr())
		}
	}
	fn get_highlight_replay_duration(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_HighlightReplayDuration(self.addr())
		}
	}
	fn get_time_before_highlight_replay(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_TimeBeforeHighlightReplay(self.addr())
		}
	}
	fn get_replay(&self) -> ReplaySoccarWrapper {
		unsafe {
			ReplaySoccarWrapper::new(ReplayDirector_TA_Get_Replay(self.addr()))
		}
	}
	fn get_focus_car(&self) -> ActorWrapper {
		unsafe {
			ActorWrapper::new(ReplayDirector_TA_Get_FocusCar(self.addr()))
		}
	}
	fn get_focus_car_change_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_FocusCarChangeTime(self.addr())
		}
	}
	fn get_focus_ball(&self) -> ActorWrapper {
		unsafe {
			ActorWrapper::new(ReplayDirector_TA_Get_FocusBall(self.addr()))
		}
	}
	fn get_score_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_ScoreTime(self.addr())
		}
	}
	fn get_score_hit_index(&self) -> i32 {
		unsafe {
			ReplayDirector_TA_Get_ScoreHitIndex(self.addr())
		}
	}
	fn get_scored_goal(&self) -> GoalWrapper {
		unsafe {
			GoalWrapper::new(ReplayDirector_TA_Get_ScoredGoal(self.addr()))
		}
	}
	fn get_b_slomo(&self) -> bool {
		unsafe {
			ReplayDirector_TA_Get_bSlomo(self.addr())
		}
	}
	fn get_b_slomo_for_defender(&self) -> bool {
		unsafe {
			ReplayDirector_TA_Get_bSlomoForDefender(self.addr())
		}
	}
	fn get_b_auto_save(&self) -> bool {
		unsafe {
			ReplayDirector_TA_Get_bAutoSave(self.addr())
		}
	}
	fn get_focus_hit_index(&self) -> i32 {
		unsafe {
			ReplayDirector_TA_Get_FocusHitIndex(self.addr())
		}
	}
	fn get_focus_car_idx(&self) -> i32 {
		unsafe {
			ReplayDirector_TA_Get_FocusCarIdx(self.addr())
		}
	}
	fn get_replay_start_game_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_ReplayStartGameTime(self.addr())
		}
	}
	fn get_ball_spawn_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_Get_BallSpawnTime(self.addr())
		}
	}
	fn get_soccar_game(&self) -> ServerWrapper {
		unsafe {
			ServerWrapper::new(ReplayDirector_TA_Get_SoccarGame(self.addr()))
		}
	}
	fn get_scored_on_team(&self) -> u8 {
		unsafe {
			ReplayDirector_TA_Get_ScoredOnTeam(self.addr())
		}
	}
	fn get_force_cut_to_focus_actors(&self) -> i32 {
		unsafe {
			ReplayDirector_TA_Get_ForceCutToFocusActors(self.addr())
		}
	}
	fn handle_replay_finished(&self, in_replay: ReplayWrapper) {
		unsafe {
			ReplayDirector_TA_HandleReplayFinished(self.addr(), in_replay.addr());
		}
	}
	fn should_slomo(&self) -> bool {
		unsafe {
			ReplayDirector_TA_ShouldSlomo(self.addr())
		}
	}
	fn update_slomo(&self) {
		unsafe {
			ReplayDirector_TA_UpdateSlomo(self.addr());
		}
	}
	fn update_focus_actors(&self) {
		unsafe {
			ReplayDirector_TA_UpdateFocusActors(self.addr());
		}
	}
	fn play_random_highlight(&self) {
		unsafe {
			ReplayDirector_TA_PlayRandomHighlight(self.addr());
		}
	}
	fn get_next_highlight_frame(&self) -> i32 {
		unsafe {
			ReplayDirector_TA_GetNextHighlightFrame(self.addr())
		}
	}
	fn set_auto_save(&self) {
		unsafe {
			ReplayDirector_TA_SetAutoSave(self.addr());
		}
	}
	fn save_user_keyframe(&self) {
		unsafe {
			ReplayDirector_TA_SaveUserKeyframe(self.addr());
		}
	}
	fn build_focus_cars(&self) {
		unsafe {
			ReplayDirector_TA_BuildFocusCars(self.addr());
		}
	}
	fn set_slomo(&self, b_new_slomo: bool) {
		unsafe {
			ReplayDirector_TA_SetSlomo(self.addr(), b_new_slomo);
		}
	}
	fn get_replay_time_seconds(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_GetReplayTimeSeconds(self.addr())
		}
	}
	fn set_focus_actors(&self, new_car: ActorWrapper, new_ball: ActorWrapper) {
		unsafe {
			ReplayDirector_TA_SetFocusActors(self.addr(), new_car.addr(), new_ball.addr());
		}
	}
	fn get_replay_start_time(&self) -> f32 {
		unsafe {
			ReplayDirector_TA_GetReplayStartTime(self.addr())
		}
	}
	fn set_slomo_for_defender(&self, ball: BallWrapper, defending_team: i32) {
		unsafe {
			ReplayDirector_TA_SetSlomoForDefender(self.addr(), ball.addr(), defending_team);
		}
	}
	fn on_score_data_changed(&self) {
		unsafe {
			ReplayDirector_TA_OnScoreDataChanged(self.addr());
		}
	}
	fn handle_score_updated(&self, team: TeamWrapper) {
		unsafe {
			ReplayDirector_TA_HandleScoreUpdated(self.addr(), team.addr());
		}
	}
	fn handle_all_teams_created(&self, team_game: TeamGameEventWrapper) {
		unsafe {
			ReplayDirector_TA_HandleAllTeamsCreated(self.addr(), team_game.addr());
		}
	}
	fn record_players(&self) {
		unsafe {
			ReplayDirector_TA_RecordPlayers(self.addr());
		}
	}
	fn handle_game_state_changed(&self, g: GameEventWrapper) {
		unsafe {
			ReplayDirector_TA_HandleGameStateChanged(self.addr(), g.addr());
		}
	}
	fn on_soccar_game_set(&self) {
		unsafe {
			ReplayDirector_TA_OnSoccarGameSet(self.addr());
		}
	}
	fn set_game_event(&self, in_game_event: ServerWrapper) {
		unsafe {
			ReplayDirector_TA_SetGameEvent(self.addr(), in_game_event.addr());
		}
	}
	fn event_focus_car_changed(&self, new_focus_car: ActorWrapper) {
		unsafe {
			ReplayDirector_TA_EventFocusCarChanged(self.addr(), new_focus_car.addr());
		}
	}
	fn event_auto_save_changed(&self, director: ReplayDirectorWrapper) {
		unsafe {
			ReplayDirector_TA_EventAutoSaveChanged(self.addr(), director.addr());
		}
	}
	fn event_score_data_changed(&self, director: ReplayDirectorWrapper) {
		unsafe {
			ReplayDirector_TA_EventScoreDataChanged(self.addr(), director.addr());
		}
	}
	fn event_replay_finished(&self, director: ReplayDirectorWrapper) {
		unsafe {
			ReplayDirector_TA_EventReplayFinished(self.addr(), director.addr());
		}
	}

}

extern "C" {
	fn ReplayDirector_TA_Get_SlomoPreScoreTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetSlomoPreScoreTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_SlomoPostScoreTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetSlomoPostScoreTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_SlomoDefendTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetSlomoDefendTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_SlomoDefendDistance(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetSlomoDefendDistance(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_SlomoTimeDilation(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetSlomoTimeDilation(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_MinReplayTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetMinReplayTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_MaxReplayTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetMaxReplayTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_ReplayPadding(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetReplayPadding(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_HighlightReplayDuration(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetHighlightReplayDuration(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_TimeBeforeHighlightReplay(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetTimeBeforeHighlightReplay(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_Replay(obj: usize) -> usize;
	fn ReplayDirectorWrapper_SetReplay(obj: usize, new_val: usize);
	fn ReplayDirector_TA_Get_FocusCar(obj: usize) -> usize;
	fn ReplayDirectorWrapper_SetFocusCar(obj: usize, new_val: usize);
	fn ReplayDirector_TA_Get_FocusCarChangeTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetFocusCarChangeTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_FocusBall(obj: usize) -> usize;
	fn ReplayDirectorWrapper_SetFocusBall(obj: usize, new_val: usize);
	fn ReplayDirector_TA_Get_ScoreTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetScoreTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_ScoreHitIndex(obj: usize) -> i32;
	fn ReplayDirectorWrapper_SetScoreHitIndex(obj: usize, new_val: i32);
	fn ReplayDirector_TA_Get_ScoredGoal(obj: usize) -> usize;
	fn ReplayDirectorWrapper_SetScoredGoal(obj: usize, new_val: usize);
	fn ReplayDirector_TA_Get_bSlomo(obj: usize) -> bool;
	fn ReplayDirectorWrapper_SetbSlomo(obj: usize, new_val: bool);
	fn ReplayDirector_TA_Get_bSlomoForDefender(obj: usize) -> bool;
	fn ReplayDirectorWrapper_SetbSlomoForDefender(obj: usize, new_val: bool);
	fn ReplayDirector_TA_Get_bAutoSave(obj: usize) -> bool;
	fn ReplayDirectorWrapper_SetbAutoSave(obj: usize, new_val: bool);
	fn ReplayDirector_TA_Get_FocusHitIndex(obj: usize) -> i32;
	fn ReplayDirectorWrapper_SetFocusHitIndex(obj: usize, new_val: i32);
	fn ReplayDirector_TA_Get_FocusCarIdx(obj: usize) -> i32;
	fn ReplayDirectorWrapper_SetFocusCarIdx(obj: usize, new_val: i32);
	fn ReplayDirector_TA_Get_ReplayStartGameTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetReplayStartGameTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_BallSpawnTime(obj: usize) -> f32;
	fn ReplayDirectorWrapper_SetBallSpawnTime(obj: usize, new_val: f32);
	fn ReplayDirector_TA_Get_SoccarGame(obj: usize) -> usize;
	fn ReplayDirectorWrapper_SetSoccarGame(obj: usize, new_val: usize);
	fn ReplayDirector_TA_Get_ScoredOnTeam(obj: usize) -> u8;
	fn ReplayDirectorWrapper_SetScoredOnTeam(obj: usize, new_val: u8);
	fn ReplayDirector_TA_Get_ForceCutToFocusActors(obj: usize) -> i32;
	fn ReplayDirectorWrapper_SetForceCutToFocusActors(obj: usize, new_val: i32);
	fn ReplayDirector_TA_HandleReplayFinished(obj: usize, InReplay: usize);
	fn ReplayDirector_TA_ShouldSlomo(obj: usize) -> bool;
	fn ReplayDirector_TA_UpdateSlomo(obj: usize);
	fn ReplayDirector_TA_UpdateFocusActors(obj: usize);
	fn ReplayDirector_TA_PlayRandomHighlight(obj: usize);
	fn ReplayDirector_TA_GetNextHighlightFrame(obj: usize) -> i32;
	fn ReplayDirector_TA_SetAutoSave(obj: usize);
	fn ReplayDirector_TA_SaveUserKeyframe(obj: usize);
	fn ReplayDirector_TA_BuildFocusCars(obj: usize);
	fn ReplayDirector_TA_SetSlomo(obj: usize, bNewSlomo: bool);
	fn ReplayDirector_TA_GetReplayTimeSeconds(obj: usize) -> f32;
	fn ReplayDirector_TA_SetFocusActors(obj: usize, NewCar: usize, NewBall: usize);
	fn ReplayDirector_TA_GetReplayStartTime(obj: usize) -> f32;
	fn ReplayDirector_TA_SetSlomoForDefender(obj: usize, Ball: usize, DefendingTeam: i32);
	fn ReplayDirector_TA_OnScoreDataChanged(obj: usize);
	fn ReplayDirector_TA_HandleScoreUpdated(obj: usize, Team: usize);
	fn ReplayDirector_TA_HandleAllTeamsCreated(obj: usize, TeamGame: usize);
	fn ReplayDirector_TA_RecordPlayers(obj: usize);
	fn ReplayDirector_TA_HandleGameStateChanged(obj: usize, G: usize);
	fn ReplayDirector_TA_OnSoccarGameSet(obj: usize);
	fn ReplayDirector_TA_SetGameEvent(obj: usize, InGameEvent: usize);
	fn ReplayDirector_TA_EventFocusCarChanged(obj: usize, NewFocusCar: usize);
	fn ReplayDirector_TA_EventAutoSaveChanged(obj: usize, Director: usize);
	fn ReplayDirector_TA_EventScoreDataChanged(obj: usize, Director: usize);
	fn ReplayDirector_TA_EventReplayFinished(obj: usize, Director: usize);

}