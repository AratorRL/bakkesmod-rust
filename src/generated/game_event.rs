use crate::wrappers::*;
use crate::generated::*;

pub struct GameEventWrapper(pub usize);
impl_object!(GameEventWrapper);

impl GameEvent for GameEventWrapper {}
impl Actor for GameEventWrapper {}

pub trait GameEvent : Actor {
	fn get_car_archetype(&self) -> CarWrapper {
		unsafe {
			CarWrapper::new(GameEvent_TA_Get_CarArchetype(self.addr()))
		}
	}
	fn get_countdown_time(&self) -> i32 {
		unsafe {
			GameEvent_TA_Get_CountdownTime(self.addr())
		}
	}
	fn get_finish_time(&self) -> i32 {
		unsafe {
			GameEvent_TA_Get_FinishTime(self.addr())
		}
	}
	fn get_b_multiplayer(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bMultiplayer(self.addr())
		}
	}
	fn get_b_countdown_messages_disabled(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bCountdownMessagesDisabled(self.addr())
		}
	}
	fn get_b_fill_with_ai(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bFillWithAI(self.addr())
		}
	}
	fn get_b_allow_queue_save_replay(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bAllowQueueSaveReplay(self.addr())
		}
	}
	fn get_b_allow_ready_up(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bAllowReadyUp(self.addr())
		}
	}
	fn get_b_restarting_match(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bRestartingMatch(self.addr())
		}
	}
	fn get_b_randomized_bot_loadouts(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bRandomizedBotLoadouts(self.addr())
		}
	}
	fn get_b_has_leave_match_penalty(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bHasLeaveMatchPenalty(self.addr())
		}
	}
	fn get_b_can_vote_to_forfeit(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bCanVoteToForfeit(self.addr())
		}
	}
	fn get_b_disable_aim_assist(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bDisableAimAssist(self.addr())
		}
	}
	fn get_b_award_achievements(&self) -> bool {
		unsafe {
			GameEvent_TA_Get_bAwardAchievements(self.addr())
		}
	}
	fn get_min_players(&self) -> i32 {
		unsafe {
			GameEvent_TA_Get_MinPlayers(self.addr())
		}
	}
	fn get_max_players(&self) -> i32 {
		unsafe {
			GameEvent_TA_Get_MaxPlayers(self.addr())
		}
	}
	fn get_spawn_points(&self) -> RLArray<ActorWrapper> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			GameEvent_TA_Get_SpawnPoints(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_bot_skill(&self) -> f32 {
		unsafe {
			GameEvent_TA_Get_BotSkill(self.addr())
		}
	}
	fn get_match_time_dilation(&self) -> f32 {
		unsafe {
			GameEvent_TA_Get_MatchTimeDilation(self.addr())
		}
	}
	fn get_activator(&self) -> PlayerControllerWrapper {
		unsafe {
			PlayerControllerWrapper::new(GameEvent_TA_Get_Activator(self.addr()))
		}
	}
	fn get_activator_car(&self) -> CarWrapper {
		unsafe {
			CarWrapper::new(GameEvent_TA_Get_ActivatorCar(self.addr()))
		}
	}
	fn get_pr_is(&self) -> RLArray<PriWrapper> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			GameEvent_TA_Get_PRIs(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_cars(&self) -> RLArray<CarWrapper> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			GameEvent_TA_Get_Cars(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_local_players(&self) -> RLArray<PlayerControllerWrapper> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			GameEvent_TA_Get_LocalPlayers(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_start_point_index(&self) -> i32 {
		unsafe {
			GameEvent_TA_Get_StartPointIndex(self.addr())
		}
	}
	fn get_replicated_state_index(&self) -> u8 {
		unsafe {
			GameEvent_TA_Get_ReplicatedStateIndex(self.addr())
		}
	}
	fn get_game_state_time_remaining(&self) -> i32 {
		unsafe {
			GameEvent_TA_Get_GameStateTimeRemaining(self.addr())
		}
	}
	fn get_replicated_game_state_time_remaining(&self) -> i32 {
		unsafe {
			GameEvent_TA_Get_ReplicatedGameStateTimeRemaining(self.addr())
		}
	}
	fn get_forfeit_initiator_i_ds(&self) -> RLArray<UniqueNetId> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			GameEvent_TA_Get_ForfeitInitiatorIDs(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_banned_players(&self) -> RLArray<UniqueNetId> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			GameEvent_TA_Get_BannedPlayers(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_game_owner(&self) -> PriWrapper {
		unsafe {
			PriWrapper::new(GameEvent_TA_Get_GameOwner(self.addr()))
		}
	}
	fn get_rich_presence_string(&self) -> RLString {
		unsafe {
			let mut result = RLString::new();
			let result_ptr: *mut RLString = &mut result as *mut RLString;
			GameEvent_TA_Get_RichPresenceString(self.addr(), result_ptr);
			result
		}
	}
	fn get_replicated_round_count_down_number(&self) -> i32 {
		unsafe {
			GameEvent_TA_Get_ReplicatedRoundCountDownNumber(self.addr())
		}
	}
	fn init_count_down(&self) {
		unsafe {
			GameEvent_TA_InitCountDown(self.addr());
		}
	}
	fn start_countdown_timer(&self) {
		unsafe {
			GameEvent_TA_StartCountdownTimer(self.addr());
		}
	}
	fn allow_ready_up(&self) {
		unsafe {
			GameEvent_TA_AllowReadyUp(self.addr());
		}
	}
	fn find_player_pri(&self, unique_id: UniqueNetId) -> PriWrapper {
		unsafe {
			let mut unique_id = unique_id;
			let unique_id: *mut UniqueNetId = &mut unique_id as *mut UniqueNetId;
			PriWrapper::new(GameEvent_TA_FindPlayerPRI(self.addr(), unique_id))
		}
	}
	fn handle_player_removed(&self, game_event: GameEventWrapper, pri: PriWrapper) {
		unsafe {
			GameEvent_TA_HandlePlayerRemoved(self.addr(), game_event.addr(), pri.addr());
		}
	}
	fn update_game_owner(&self) {
		unsafe {
			GameEvent_TA_UpdateGameOwner(self.addr());
		}
	}
	fn set_game_owner2(&self, new_owner: PriWrapper) {
		unsafe {
			GameEvent_TA_SetGameOwner2(self.addr(), new_owner.addr());
		}
	}
	fn pylon_change_notify_func(&self) {
		unsafe {
			GameEvent_TA___Pylon__ChangeNotifyFunc(self.addr());
		}
	}
	fn player_reset_training(&self) {
		unsafe {
			GameEvent_TA_PlayerResetTraining(self.addr());
		}
	}
	fn suppress_modal_dialogs(&self) -> bool {
		unsafe {
			GameEvent_TA_SuppressModalDialogs(self.addr())
		}
	}
	fn should_show_ball_indicator(&self) -> bool {
		unsafe {
			GameEvent_TA_ShouldShowBallIndicator(self.addr())
		}
	}
	fn check_initiated_forfeit(&self, pri: PriWrapper) {
		unsafe {
			GameEvent_TA_CheckInitiatedForfeit(self.addr(), pri.addr());
		}
	}
	fn find_pc_for_unique_id(&self, player_id: UniqueNetId) -> PlayerControllerWrapper {
		unsafe {
			let mut player_id = player_id;
			let player_id: *mut UniqueNetId = &mut player_id as *mut UniqueNetId;
			PlayerControllerWrapper::new(GameEvent_TA_FindPCForUniqueID(self.addr(), player_id))
		}
	}
	fn allow_split_screen_player(&self) -> bool {
		unsafe {
			GameEvent_TA_AllowSplitScreenPlayer(self.addr())
		}
	}
	fn add_player_chat_message(&self, player_id: UniqueNetId, chat_channel: u8, team: TeamInfoWrapper, message: RLString) {
		unsafe {
			let mut player_id = player_id;
			let player_id: *mut UniqueNetId = &mut player_id as *mut UniqueNetId;
			let mut message = message;
			let message: *mut RLString = &mut message as *mut RLString;
			GameEvent_TA_AddPlayerChatMessage(self.addr(), player_id, chat_channel, team.addr(), message);
		}
	}
	fn conditional_start_spectator_match(&self) {
		unsafe {
			GameEvent_TA_ConditionalStartSpectatorMatch(self.addr());
		}
	}
	fn is_playing_training(&self) -> bool {
		unsafe {
			GameEvent_TA_IsPlayingTraining(self.addr())
		}
	}
	fn is_playing_lan(&self) -> bool {
		unsafe {
			GameEvent_TA_IsPlayingLan(self.addr())
		}
	}
	fn is_playing_offline(&self) -> bool {
		unsafe {
			GameEvent_TA_IsPlayingOffline(self.addr())
		}
	}
	fn is_playing_private(&self) -> bool {
		unsafe {
			GameEvent_TA_IsPlayingPrivate(self.addr())
		}
	}
	fn is_playing_public(&self) -> bool {
		unsafe {
			GameEvent_TA_IsPlayingPublic(self.addr())
		}
	}
	fn is_online_multiplayer(&self) -> bool {
		unsafe {
			GameEvent_TA_IsOnlineMultiplayer(self.addr())
		}
	}
	fn create_match_type(&self, options: RLString) {
		unsafe {
			let mut options = options;
			let options: *mut RLString = &mut options as *mut RLString;
			GameEvent_TA_CreateMatchType(self.addr(), options);
		}
	}
	fn all_players_selected_team(&self) -> bool {
		unsafe {
			GameEvent_TA_AllPlayersSelectedTeam(self.addr())
		}
	}
	fn can_que_save_replay(&self) -> bool {
		unsafe {
			GameEvent_TA_CanQueSaveReplay(self.addr())
		}
	}
	fn force_match_start(&self) {
		unsafe {
			GameEvent_TA_ForceMatchStart(self.addr());
		}
	}
	fn conditional_start_match(&self) {
		unsafe {
			GameEvent_TA_ConditionalStartMatch(self.addr());
		}
	}
	fn save_local_player_stats(&self) {
		unsafe {
			GameEvent_TA_SaveLocalPlayerStats(self.addr());
		}
	}
	fn can_use_ball_cam(&self) -> bool {
		unsafe {
			GameEvent_TA_CanUseBallCam(self.addr())
		}
	}
	fn handle_next_game(&self) -> bool {
		unsafe {
			GameEvent_TA_HandleNextGame(self.addr())
		}
	}
	fn set_max_players2(&self, in_max_players: i32) {
		unsafe {
			GameEvent_TA_SetMaxPlayers2(self.addr(), in_max_players);
		}
	}
	fn set_restarting_match(&self, b_restart: bool) {
		unsafe {
			GameEvent_TA_SetRestartingMatch(self.addr(), b_restart);
		}
	}
	fn should_be_full_screen(&self) -> bool {
		unsafe {
			GameEvent_TA_ShouldBeFullScreen(self.addr())
		}
	}
	fn is_finished(&self) -> bool {
		unsafe {
			GameEvent_TA_IsFinished(self.addr())
		}
	}
	fn on_all_players_ready(&self) {
		unsafe {
			GameEvent_TA_OnAllPlayersReady(self.addr());
		}
	}
	fn check_players_ready(&self) {
		unsafe {
			GameEvent_TA_CheckPlayersReady(self.addr());
		}
	}
	fn set_allow_ready_up(&self, b_allow: bool) {
		unsafe {
			GameEvent_TA_SetAllowReadyUp(self.addr(), b_allow);
		}
	}
	fn auto_ready_players(&self) {
		unsafe {
			GameEvent_TA_AutoReadyPlayers(self.addr());
		}
	}
	fn should_auto_ready_up(&self, pri: PriWrapper) -> bool {
		unsafe {
			GameEvent_TA_ShouldAutoReadyUp(self.addr(), pri.addr())
		}
	}
	fn send_go_message(&self, player: PlayerControllerWrapper) {
		unsafe {
			GameEvent_TA_SendGoMessage(self.addr(), player.addr());
		}
	}
	fn send_countdown_message(&self, seconds: i32, player: PlayerControllerWrapper) {
		unsafe {
			GameEvent_TA_SendCountdownMessage(self.addr(), seconds, player.addr());
		}
	}
	fn broadcast_countdown_message(&self, seconds: i32) {
		unsafe {
			GameEvent_TA_BroadcastCountdownMessage(self.addr(), seconds);
		}
	}
	fn broadcast_go_message(&self) {
		unsafe {
			GameEvent_TA_BroadcastGoMessage(self.addr());
		}
	}
	fn allow_shutdown(&self) -> bool {
		unsafe {
			GameEvent_TA_AllowShutdown(self.addr())
		}
	}
	fn get_real_delta_time(&self, elapsed_time: f32) -> f32 {
		unsafe {
			GameEvent_TA_GetRealDeltaTime(self.addr(), elapsed_time)
		}
	}
	fn set_time_dilation(&self, new_time_dilation: f32) {
		unsafe {
			GameEvent_TA_SetTimeDilation(self.addr(), new_time_dilation);
		}
	}
	fn replace_bots_with_awaiting_players(&self) {
		unsafe {
			GameEvent_TA_ReplaceBotsWithAwaitingPlayers(self.addr());
		}
	}
	fn remove_car(&self, car: CarWrapper) {
		unsafe {
			GameEvent_TA_RemoveCar(self.addr(), car.addr());
		}
	}
	fn add_car(&self, car: CarWrapper) {
		unsafe {
			GameEvent_TA_AddCar(self.addr(), car.addr());
		}
	}
	fn set_bot_skill2(&self, new_skill: f32) {
		unsafe {
			GameEvent_TA_SetBotSkill2(self.addr(), new_skill);
		}
	}
	fn get_local_primary_player(&self) -> PlayerControllerWrapper {
		unsafe {
			PlayerControllerWrapper::new(GameEvent_TA_GetLocalPrimaryPlayer(self.addr()))
		}
	}
	fn has_player_named(&self, player_name: RLString) -> bool {
		unsafe {
			let mut player_name = player_name;
			let player_name: *mut RLString = &mut player_name as *mut RLString;
			GameEvent_TA_HasPlayerNamed(self.addr(), player_name)
		}
	}
	fn randomize_bots(&self) {
		unsafe {
			GameEvent_TA_RandomizeBots(self.addr());
		}
	}
	fn move_to_ground(&self, mover: ActorWrapper, height_check: f32) -> bool {
		unsafe {
			GameEvent_TA_MoveToGround(self.addr(), mover.addr(), height_check)
		}
	}
	fn set_all_driving(&self, b_driving: bool) {
		unsafe {
			GameEvent_TA_SetAllDriving(self.addr(), b_driving);
		}
	}
	fn on_finished(&self) {
		unsafe {
			GameEvent_TA_OnFinished(self.addr());
		}
	}
	fn start_count_down(&self) {
		unsafe {
			GameEvent_TA_StartCountDown(self.addr());
		}
	}
	fn start_initial_count_down(&self) {
		unsafe {
			GameEvent_TA_StartInitialCountDown(self.addr());
		}
	}
	fn on_game_state_time_lapsed(&self) {
		unsafe {
			GameEvent_TA_OnGameStateTimeLapsed(self.addr());
		}
	}
	fn on_game_state_time_updated(&self) {
		unsafe {
			GameEvent_TA_OnGameStateTimeUpdated(self.addr());
		}
	}
	fn update_game_state_time(&self) {
		unsafe {
			GameEvent_TA_UpdateGameStateTime(self.addr());
		}
	}
	fn set_game_state_time_remaining2(&self, state_time: i32, b_from_replication: bool) {
		unsafe {
			GameEvent_TA_SetGameStateTimeRemaining2(self.addr(), state_time, b_from_replication);
		}
	}
	fn set_game_state_time(&self, state_time: i32) {
		unsafe {
			GameEvent_TA_SetGameStateTime(self.addr(), state_time);
		}
	}
	fn on_player_restarted(&self, player_car: CarWrapper) {
		unsafe {
			GameEvent_TA_OnPlayerRestarted(self.addr(), player_car.addr());
		}
	}
	fn teleport_car(&self, player_car: CarWrapper) {
		unsafe {
			GameEvent_TA_TeleportCar(self.addr(), player_car.addr());
		}
	}
	fn on_car_spawned(&self, new_car: CarWrapper) {
		unsafe {
			GameEvent_TA_OnCarSpawned(self.addr(), new_car.addr());
		}
	}
	fn spot_is_encroached(&self, spot: Vector) -> bool {
		unsafe {
			let mut spot = spot;
			let spot: *mut Vector = &mut spot as *mut Vector;
			GameEvent_TA_SpotIsEncroached(self.addr(), spot)
		}
	}
	fn randomize_spawn_points(&self) {
		unsafe {
			GameEvent_TA_RandomizeSpawnPoints(self.addr());
		}
	}
	fn restart_players(&self) {
		unsafe {
			GameEvent_TA_RestartPlayers(self.addr());
		}
	}
	fn remove_local_player(&self, player: PlayerControllerWrapper) {
		unsafe {
			GameEvent_TA_RemoveLocalPlayer(self.addr(), player.addr());
		}
	}
	fn add_local_player(&self, player: PlayerControllerWrapper) {
		unsafe {
			GameEvent_TA_AddLocalPlayer(self.addr(), player.addr());
		}
	}
	fn add_pri(&self, pri: PriWrapper) {
		unsafe {
			GameEvent_TA_AddPRI(self.addr(), pri.addr());
		}
	}
	fn add_forfeit_initiator(&self, player_id: UniqueNetId) {
		unsafe {
			let mut player_id = player_id;
			let player_id: *mut UniqueNetId = &mut player_id as *mut UniqueNetId;
			GameEvent_TA_AddForfeitInitiator(self.addr(), player_id);
		}
	}
	fn ban_player_id(&self, player_id: UniqueNetId) {
		unsafe {
			let mut player_id = player_id;
			let player_id: *mut UniqueNetId = &mut player_id as *mut UniqueNetId;
			GameEvent_TA_BanPlayerID(self.addr(), player_id);
		}
	}
	fn get_max_humans(&self) -> i32 {
		unsafe {
			GameEvent_TA_GetMaxHumans(self.addr())
		}
	}
	fn get_num_humans(&self) -> i32 {
		unsafe {
			GameEvent_TA_GetNumHumans(self.addr())
		}
	}
	fn find_bot_replacement(&self, pri: PriWrapper) -> bool {
		unsafe {
			GameEvent_TA_FindBotReplacement(self.addr(), pri.addr())
		}
	}
	fn update_bot_count(&self) {
		unsafe {
			GameEvent_TA_UpdateBotCount(self.addr());
		}
	}
	fn timer_update_bot_count(&self) {
		unsafe {
			GameEvent_TA_TimerUpdateBotCount(self.addr());
		}
	}
	fn init_bot_skill(&self) {
		unsafe {
			GameEvent_TA_InitBotSkill(self.addr());
		}
	}
	fn init_mutators(&self) {
		unsafe {
			GameEvent_TA_InitMutators(self.addr());
		}
	}
	fn init(&self, in_activator: PlayerControllerWrapper) {
		unsafe {
			GameEvent_TA_Init(self.addr(), in_activator.addr());
		}
	}
	fn on_game_state_changed(&self) {
		unsafe {
			GameEvent_TA_OnGameStateChanged(self.addr());
		}
	}
	fn on_can_vote_forfeit_changed(&self) {
		unsafe {
			GameEvent_TA_OnCanVoteForfeitChanged(self.addr());
		}
	}
	fn update_can_vote_to_forfeit(&self) {
		unsafe {
			GameEvent_TA_UpdateCanVoteToForfeit(self.addr());
		}
	}
	fn should_allow_vote_to_forfeit(&self) -> bool {
		unsafe {
			GameEvent_TA_ShouldAllowVoteToForfeit(self.addr())
		}
	}
	fn on_penalty_changed(&self) {
		unsafe {
			GameEvent_TA_OnPenaltyChanged(self.addr());
		}
	}
	fn update_leave_match_penalty(&self) {
		unsafe {
			GameEvent_TA_UpdateLeaveMatchPenalty(self.addr());
		}
	}
	fn get_playlist(&self) -> GameSettingPlaylistWrapper {
		unsafe {
			GameSettingPlaylistWrapper::new(GameEvent_TA_GetPlaylist(self.addr()))
		}
	}
	fn should_have_leave_match_penalty(&self) -> bool {
		unsafe {
			GameEvent_TA_ShouldHaveLeaveMatchPenalty(self.addr())
		}
	}
	fn on_match_settings_changed(&self) {
		unsafe {
			GameEvent_TA_OnMatchSettingsChanged(self.addr());
		}
	}
	fn clear_game_score_from_custom_settings(&self) {
		unsafe {
			GameEvent_TA_ClearGameScoreFromCustomSettings(self.addr());
		}
	}
	fn event_player_reset_training(&self, game_event: GameEventWrapper) {
		unsafe {
			GameEvent_TA_EventPlayerResetTraining(self.addr(), game_event.addr());
		}
	}

}

extern "C" {
	fn GameEvent_TA_Get_CarArchetype(obj: usize) -> usize;
	fn GameEventWrapper_SetCarArchetype(obj: usize, new_val: usize);
	fn GameEvent_TA_Get_CountdownTime(obj: usize) -> i32;
	fn GameEventWrapper_SetCountdownTime(obj: usize, new_val: i32);
	fn GameEvent_TA_Get_FinishTime(obj: usize) -> i32;
	fn GameEventWrapper_SetFinishTime(obj: usize, new_val: i32);
	fn GameEvent_TA_Get_bMultiplayer(obj: usize) -> bool;
	fn GameEventWrapper_SetbMultiplayer(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bCountdownMessagesDisabled(obj: usize) -> bool;
	fn GameEventWrapper_SetbCountdownMessagesDisabled(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bFillWithAI(obj: usize) -> bool;
	fn GameEventWrapper_SetbFillWithAI(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bAllowQueueSaveReplay(obj: usize) -> bool;
	fn GameEventWrapper_SetbAllowQueueSaveReplay(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bAllowReadyUp(obj: usize) -> bool;
	fn GameEventWrapper_SetbAllowReadyUp(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bRestartingMatch(obj: usize) -> bool;
	fn GameEventWrapper_SetbRestartingMatch(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bRandomizedBotLoadouts(obj: usize) -> bool;
	fn GameEventWrapper_SetbRandomizedBotLoadouts(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bHasLeaveMatchPenalty(obj: usize) -> bool;
	fn GameEventWrapper_SetbHasLeaveMatchPenalty(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bCanVoteToForfeit(obj: usize) -> bool;
	fn GameEventWrapper_SetbCanVoteToForfeit(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bDisableAimAssist(obj: usize) -> bool;
	fn GameEventWrapper_SetbDisableAimAssist(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_bAwardAchievements(obj: usize) -> bool;
	fn GameEventWrapper_SetbAwardAchievements(obj: usize, new_val: bool);
	fn GameEvent_TA_Get_MinPlayers(obj: usize) -> i32;
	fn GameEventWrapper_SetMinPlayers(obj: usize, new_val: i32);
	fn GameEvent_TA_Get_MaxPlayers(obj: usize) -> i32;
	fn GameEventWrapper_SetMaxPlayers(obj: usize, new_val: i32);
	fn GameEvent_TA_Get_SpawnPoints(obj: usize, result: *mut RLArrayRaw);
	fn GameEvent_TA_Get_BotSkill(obj: usize) -> f32;
	fn GameEventWrapper_SetBotSkill(obj: usize, new_val: f32);
	fn GameEvent_TA_Get_MatchTimeDilation(obj: usize) -> f32;
	fn GameEventWrapper_SetMatchTimeDilation(obj: usize, new_val: f32);
	fn GameEvent_TA_Get_Activator(obj: usize) -> usize;
	fn GameEventWrapper_SetActivator(obj: usize, new_val: usize);
	fn GameEvent_TA_Get_ActivatorCar(obj: usize) -> usize;
	fn GameEventWrapper_SetActivatorCar(obj: usize, new_val: usize);
	fn GameEvent_TA_Get_PRIs(obj: usize, result: *mut RLArrayRaw);
	fn GameEvent_TA_Get_Cars(obj: usize, result: *mut RLArrayRaw);
	fn GameEvent_TA_Get_LocalPlayers(obj: usize, result: *mut RLArrayRaw);
	fn GameEvent_TA_Get_StartPointIndex(obj: usize) -> i32;
	fn GameEventWrapper_SetStartPointIndex(obj: usize, new_val: i32);
	fn GameEvent_TA_Get_ReplicatedStateIndex(obj: usize) -> u8;
	fn GameEventWrapper_SetReplicatedStateIndex(obj: usize, new_val: u8);
	fn GameEvent_TA_Get_GameStateTimeRemaining(obj: usize) -> i32;
	fn GameEventWrapper_SetGameStateTimeRemaining(obj: usize, new_val: i32);
	fn GameEvent_TA_Get_ReplicatedGameStateTimeRemaining(obj: usize) -> i32;
	fn GameEventWrapper_SetReplicatedGameStateTimeRemaining(obj: usize, new_val: i32);
	fn GameEvent_TA_Get_ForfeitInitiatorIDs(obj: usize, result: *mut RLArrayRaw);
	fn GameEvent_TA_Get_BannedPlayers(obj: usize, result: *mut RLArrayRaw);
	fn GameEvent_TA_Get_GameOwner(obj: usize) -> usize;
	fn GameEventWrapper_SetGameOwner(obj: usize, new_val: usize);
	fn GameEvent_TA_Get_RichPresenceString(obj: usize, result: *mut RLString);
	fn GameEvent_TA_Get_ReplicatedRoundCountDownNumber(obj: usize) -> i32;
	fn GameEventWrapper_SetReplicatedRoundCountDownNumber(obj: usize, new_val: i32);
	fn GameEvent_TA_InitCountDown(obj: usize);
	fn GameEvent_TA_StartCountdownTimer(obj: usize);
	fn GameEvent_TA_AllowReadyUp(obj: usize);
	fn GameEvent_TA_FindPlayerPRI(obj: usize, UniqueId: *mut UniqueNetId) -> usize;
	fn GameEvent_TA_HandlePlayerRemoved(obj: usize, GameEvent: usize, PRI: usize);
	fn GameEvent_TA_UpdateGameOwner(obj: usize);
	fn GameEvent_TA_SetGameOwner2(obj: usize, NewOwner: usize);
	fn GameEvent_TA___Pylon__ChangeNotifyFunc(obj: usize);
	fn GameEvent_TA_PlayerResetTraining(obj: usize);
	fn GameEvent_TA_SuppressModalDialogs(obj: usize) -> bool;
	fn GameEvent_TA_ShouldShowBallIndicator(obj: usize) -> bool;
	fn GameEvent_TA_CheckInitiatedForfeit(obj: usize, PRI: usize);
	fn GameEvent_TA_FindPCForUniqueID(obj: usize, PlayerID: *mut UniqueNetId) -> usize;
	fn GameEvent_TA_AllowSplitScreenPlayer(obj: usize) -> bool;
	fn GameEvent_TA_AddPlayerChatMessage(obj: usize, PlayerID: *mut UniqueNetId, ChatChannel: u8, Team: usize, Message: *mut RLString);
	fn GameEvent_TA_ConditionalStartSpectatorMatch(obj: usize);
	fn GameEvent_TA_IsPlayingTraining(obj: usize) -> bool;
	fn GameEvent_TA_IsPlayingLan(obj: usize) -> bool;
	fn GameEvent_TA_IsPlayingOffline(obj: usize) -> bool;
	fn GameEvent_TA_IsPlayingPrivate(obj: usize) -> bool;
	fn GameEvent_TA_IsPlayingPublic(obj: usize) -> bool;
	fn GameEvent_TA_IsOnlineMultiplayer(obj: usize) -> bool;
	fn GameEvent_TA_CreateMatchType(obj: usize, Options: *mut RLString);
	fn GameEvent_TA_AllPlayersSelectedTeam(obj: usize) -> bool;
	fn GameEvent_TA_CanQueSaveReplay(obj: usize) -> bool;
	fn GameEvent_TA_ForceMatchStart(obj: usize);
	fn GameEvent_TA_ConditionalStartMatch(obj: usize);
	fn GameEvent_TA_SaveLocalPlayerStats(obj: usize);
	fn GameEvent_TA_CanUseBallCam(obj: usize) -> bool;
	fn GameEvent_TA_HandleNextGame(obj: usize) -> bool;
	fn GameEvent_TA_SetMaxPlayers2(obj: usize, InMaxPlayers: i32);
	fn GameEvent_TA_SetRestartingMatch(obj: usize, bRestart: bool);
	fn GameEvent_TA_ShouldBeFullScreen(obj: usize) -> bool;
	fn GameEvent_TA_IsFinished(obj: usize) -> bool;
	fn GameEvent_TA_OnAllPlayersReady(obj: usize);
	fn GameEvent_TA_CheckPlayersReady(obj: usize);
	fn GameEvent_TA_SetAllowReadyUp(obj: usize, bAllow: bool);
	fn GameEvent_TA_AutoReadyPlayers(obj: usize);
	fn GameEvent_TA_ShouldAutoReadyUp(obj: usize, PRI: usize) -> bool;
	fn GameEvent_TA_SendGoMessage(obj: usize, Player: usize);
	fn GameEvent_TA_SendCountdownMessage(obj: usize, Seconds: i32, Player: usize);
	fn GameEvent_TA_BroadcastCountdownMessage(obj: usize, Seconds: i32);
	fn GameEvent_TA_BroadcastGoMessage(obj: usize);
	fn GameEvent_TA_AllowShutdown(obj: usize) -> bool;
	fn GameEvent_TA_GetRealDeltaTime(obj: usize, ElapsedTime: f32) -> f32;
	fn GameEvent_TA_SetTimeDilation(obj: usize, NewTimeDilation: f32);
	fn GameEvent_TA_ReplaceBotsWithAwaitingPlayers(obj: usize);
	fn GameEvent_TA_RemoveCar(obj: usize, Car: usize);
	fn GameEvent_TA_AddCar(obj: usize, Car: usize);
	fn GameEvent_TA_SetBotSkill2(obj: usize, NewSkill: f32);
	fn GameEvent_TA_GetLocalPrimaryPlayer(obj: usize) -> usize;
	fn GameEvent_TA_HasPlayerNamed(obj: usize, PlayerName: *mut RLString) -> bool;
	fn GameEvent_TA_RandomizeBots(obj: usize);
	fn GameEvent_TA_MoveToGround(obj: usize, Mover: usize, HeightCheck: f32) -> bool;
	fn GameEvent_TA_SetAllDriving(obj: usize, bDriving: bool);
	fn GameEvent_TA_OnFinished(obj: usize);
	fn GameEvent_TA_StartCountDown(obj: usize);
	fn GameEvent_TA_StartInitialCountDown(obj: usize);
	fn GameEvent_TA_OnGameStateTimeLapsed(obj: usize);
	fn GameEvent_TA_OnGameStateTimeUpdated(obj: usize);
	fn GameEvent_TA_UpdateGameStateTime(obj: usize);
	fn GameEvent_TA_SetGameStateTimeRemaining2(obj: usize, StateTime: i32, bFromReplication: bool);
	fn GameEvent_TA_SetGameStateTime(obj: usize, StateTime: i32);
	fn GameEvent_TA_OnPlayerRestarted(obj: usize, PlayerCar: usize);
	fn GameEvent_TA_TeleportCar(obj: usize, PlayerCar: usize);
	fn GameEvent_TA_OnCarSpawned(obj: usize, NewCar: usize);
	fn GameEvent_TA_SpotIsEncroached(obj: usize, Spot: *mut Vector) -> bool;
	fn GameEvent_TA_RandomizeSpawnPoints(obj: usize);
	fn GameEvent_TA_RestartPlayers(obj: usize);
	fn GameEvent_TA_RemoveLocalPlayer(obj: usize, Player: usize);
	fn GameEvent_TA_AddLocalPlayer(obj: usize, Player: usize);
	fn GameEvent_TA_AddPRI(obj: usize, PRI: usize);
	fn GameEvent_TA_AddForfeitInitiator(obj: usize, PlayerID: *mut UniqueNetId);
	fn GameEvent_TA_BanPlayerID(obj: usize, PlayerID: *mut UniqueNetId);
	fn GameEvent_TA_GetMaxHumans(obj: usize) -> i32;
	fn GameEvent_TA_GetNumHumans(obj: usize) -> i32;
	fn GameEvent_TA_FindBotReplacement(obj: usize, PRI: usize) -> bool;
	fn GameEvent_TA_UpdateBotCount(obj: usize);
	fn GameEvent_TA_TimerUpdateBotCount(obj: usize);
	fn GameEvent_TA_InitBotSkill(obj: usize);
	fn GameEvent_TA_InitMutators(obj: usize);
	fn GameEvent_TA_Init(obj: usize, InActivator: usize);
	fn GameEvent_TA_OnGameStateChanged(obj: usize);
	fn GameEvent_TA_OnCanVoteForfeitChanged(obj: usize);
	fn GameEvent_TA_UpdateCanVoteToForfeit(obj: usize);
	fn GameEvent_TA_ShouldAllowVoteToForfeit(obj: usize) -> bool;
	fn GameEvent_TA_OnPenaltyChanged(obj: usize);
	fn GameEvent_TA_UpdateLeaveMatchPenalty(obj: usize);
	fn GameEvent_TA_GetPlaylist(obj: usize) -> usize;
	fn GameEvent_TA_ShouldHaveLeaveMatchPenalty(obj: usize) -> bool;
	fn GameEvent_TA_OnMatchSettingsChanged(obj: usize);
	fn GameEvent_TA_ClearGameScoreFromCustomSettings(obj: usize);
	fn GameEvent_TA_EventPlayerResetTraining(obj: usize, GameEvent: usize);

}