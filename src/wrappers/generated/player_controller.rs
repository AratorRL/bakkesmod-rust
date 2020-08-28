use crate::wrappers::*;
use super::*;

pub struct PlayerControllerWrapper(pub usize);
impl_object!(PlayerControllerWrapper);

impl PlayerController for PlayerControllerWrapper {}
impl Actor for PlayerControllerWrapper {}

pub trait PlayerController : Actor {
    fn get_car(&self) -> Option<CarWrapper> {
        unsafe {
            CarWrapper::try_new(PlayerController_TA_Get_Car(self.addr()))
        }
    }
    fn get_pri(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(PlayerController_TA_Get_PRI(self.addr()))
        }
    }
    fn get_vehicle_input(&self) -> VehicleInputs {
        unsafe {
            let mut result = VehicleInputs::new();
            let result_ptr: *mut VehicleInputs = &mut result as *mut VehicleInputs;
            PlayerController_TA_Get_VehicleInput(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_received_server_shutdown_message(&self) -> bool {
        unsafe {
            PlayerController_TA_Get_bReceivedServerShutdownMessage(self.addr())
        }
    }
    fn get_b_use_debug_inputs(&self) -> bool {
        unsafe {
            PlayerController_TA_Get_bUseDebugInputs(self.addr())
        }
    }
    fn get_b_jump_pressed(&self) -> bool {
        unsafe {
            PlayerController_TA_Get_bJumpPressed(self.addr())
        }
    }
    fn get_b_boost_pressed(&self) -> bool {
        unsafe {
            PlayerController_TA_Get_bBoostPressed(self.addr())
        }
    }
    fn get_b_handbrake_pressed(&self) -> bool {
        unsafe {
            PlayerController_TA_Get_bHandbrakePressed(self.addr())
        }
    }
    fn get_b_has_pitched_back(&self) -> bool {
        unsafe {
            PlayerController_TA_Get_bHasPitchedBack(self.addr())
        }
    }
    fn get_b_allow_asymmetrical_mute(&self) -> bool {
        unsafe {
            PlayerController_TA_Get_bAllowAsymmetricalMute(self.addr())
        }
    }
    fn get_b_reset_camera(&self) -> bool {
        unsafe {
            PlayerController_TA_Get_bResetCamera(self.addr())
        }
    }
    fn get_login_url(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            PlayerController_TA_Get_LoginURL(self.addr(), result_ptr);
            result
        }
    }
    fn get_voice_filter(&self) -> u8 {
        unsafe {
            PlayerController_TA_Get_VoiceFilter(self.addr())
        }
    }
    fn get_chat_filter(&self) -> u8 {
        unsafe {
            PlayerController_TA_Get_ChatFilter(self.addr())
        }
    }
    fn get_follow_target(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(PlayerController_TA_Get_FollowTarget(self.addr()))
        }
    }
    fn get_spectator_camera_archetype(&self) -> Option<BaseCameraWrapper> {
        unsafe {
            BaseCameraWrapper::try_new(PlayerController_TA_Get_SpectatorCameraArchetype(self.addr()))
        }
    }
    fn get_editor_camera_archetype(&self) -> Option<BaseCameraWrapper> {
        unsafe {
            BaseCameraWrapper::try_new(PlayerController_TA_Get_EditorCameraArchetype(self.addr()))
        }
    }
    fn get_move_actor_grab_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            PlayerController_TA_Get_MoveActorGrabOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_move_actor_grab_increment(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_MoveActorGrabIncrement(self.addr())
        }
    }
    fn get_min_move_actor_grab_distance(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_MinMoveActorGrabDistance(self.addr())
        }
    }
    fn get_mouse_increment_speed(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_MouseIncrementSpeed(self.addr())
        }
    }
    fn get_ball_velocity_increment_amount(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_BallVelocityIncrementAmount(self.addr())
        }
    }
    fn get_ball_velocity_increment_fire_count(&self) -> i32 {
        unsafe {
            PlayerController_TA_Get_BallVelocityIncrementFireCount(self.addr())
        }
    }
    fn get_ball_velocity_increment_fire_count_max(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_BallVelocityIncrementFireCountMax(self.addr())
        }
    }
    fn get_ball_velocity_increment_speed_default(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_BallVelocityIncrementSpeedDefault(self.addr())
        }
    }
    fn get_ball_velocity_increment_speed_max(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_BallVelocityIncrementSpeedMax(self.addr())
        }
    }
    fn get_crosshair_trace_distance(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_CrosshairTraceDistance(self.addr())
        }
    }
    fn get_traced_crosshair_actor(&self) -> Option<ActorWrapper> {
        unsafe {
            ActorWrapper::try_new(PlayerController_TA_Get_TracedCrosshairActor(self.addr()))
        }
    }
    fn get_rotate_actor_camera_location_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            PlayerController_TA_Get_RotateActorCameraLocationOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_rotate_actor_camera_rotation_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            PlayerController_TA_Get_RotateActorCameraRotationOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_rotate_actor_camera_side(&self) -> i32 {
        unsafe {
            PlayerController_TA_Get_RotateActorCameraSide(self.addr())
        }
    }
    fn get_desired_camera_side(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_DesiredCameraSide(self.addr())
        }
    }
    fn get_pawn_type_changed_time(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_PawnTypeChangedTime(self.addr())
        }
    }
    fn get_selected_spawn_archetype(&self) -> i32 {
        unsafe {
            PlayerController_TA_Get_SelectedSpawnArchetype(self.addr())
        }
    }
    fn get_debug_inputs(&self) -> VehicleInputs {
        unsafe {
            let mut result = VehicleInputs::new();
            let result_ptr: *mut VehicleInputs = &mut result as *mut VehicleInputs;
            PlayerController_TA_Get_DebugInputs(self.addr(), result_ptr);
            result
        }
    }
    fn get_min_client_input_rate(&self) -> i32 {
        unsafe {
            PlayerController_TA_Get_MinClientInputRate(self.addr())
        }
    }
    fn get_median_client_input_rate(&self) -> i32 {
        unsafe {
            PlayerController_TA_Get_MedianClientInputRate(self.addr())
        }
    }
    fn get_max_client_input_rate(&self) -> i32 {
        unsafe {
            PlayerController_TA_Get_MaxClientInputRate(self.addr())
        }
    }
    fn get_configured_client_input_rate(&self) -> i32 {
        unsafe {
            PlayerController_TA_Get_ConfiguredClientInputRate(self.addr())
        }
    }
    fn get_time_since_last_move_packet(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_TimeSinceLastMovePacket(self.addr())
        }
    }
    fn get_time_last_replicated_move_packet(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_TimeLastReplicatedMovePacket(self.addr())
        }
    }
    fn get_mouse_x_dead_zone(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_MouseXDeadZone(self.addr())
        }
    }
    fn get_mouse_y_dead_zone(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_MouseYDeadZone(self.addr())
        }
    }
    fn get_mouse_x_dead_zone_air(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_MouseXDeadZoneAir(self.addr())
        }
    }
    fn get_mouse_y_dead_zone_air(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_MouseYDeadZoneAir(self.addr())
        }
    }
    fn get_last_inputs(&self) -> VehicleInputs {
        unsafe {
            let mut result = VehicleInputs::new();
            let result_ptr: *mut VehicleInputs = &mut result as *mut VehicleInputs;
            PlayerController_TA_Get_LastInputs(self.addr(), result_ptr);
            result
        }
    }
    fn get_pending_view_pri(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(PlayerController_TA_Get_PendingViewPRI(self.addr()))
        }
    }
    fn get_last_input_pitch_up(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_LastInputPitchUp(self.addr())
        }
    }
    fn get_last_input_pitch_down(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_LastInputPitchDown(self.addr())
        }
    }
    fn get_last_input_yaw_left(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_LastInputYawLeft(self.addr())
        }
    }
    fn get_last_input_yaw_right(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_LastInputYawRight(self.addr())
        }
    }
    fn get_last_input_pitch(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_LastInputPitch(self.addr())
        }
    }
    fn get_last_input_yaw(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_LastInputYaw(self.addr())
        }
    }
    fn get_mouse_input_max(&self) -> f32 {
        unsafe {
            PlayerController_TA_Get_MouseInputMax(self.addr())
        }
    }
    fn get_engine_share(&self) -> Option<EngineTAWrapper> {
        unsafe {
            EngineTAWrapper::try_new(PlayerController_TA_Get_EngineShare(self.addr()))
        }
    }
    fn handle_car_set(&self, in_pri: PriWrapper) {
        unsafe {
            PlayerController_TA_HandleCarSet(self.addr(), in_pri.addr());
        }
    }
    fn spawn_selected_archetype(&self) {
        unsafe {
            PlayerController_TA_SpawnSelectedArchetype(self.addr());
        }
    }
    fn remove_actor(&self) -> bool {
        unsafe {
            PlayerController_TA_RemoveActor(self.addr())
        }
    }
    fn toggle_grab_actor(&self) {
        unsafe {
            PlayerController_TA_ToggleGrabActor(self.addr());
        }
    }
    fn toggle_rotate_actor(&self) {
        unsafe {
            PlayerController_TA_ToggleRotateActor(self.addr());
        }
    }
    fn editor_release_actor(&self) {
        unsafe {
            PlayerController_TA_EditorReleaseActor(self.addr());
        }
    }
    fn editor_cycle_actor(&self) {
        unsafe {
            PlayerController_TA_EditorCycleActor(self.addr());
        }
    }
    fn duplicate_shot(&self) {
        unsafe {
            PlayerController_TA_DuplicateShot(self.addr());
        }
    }
    fn stop_editing(&self) {
        unsafe {
            PlayerController_TA_StopEditing(self.addr());
        }
    }
    fn editor_increase_power(&self) {
        unsafe {
            PlayerController_TA_EditorIncreasePower(self.addr());
        }
    }
    fn editor_decrease_power(&self) {
        unsafe {
            PlayerController_TA_EditorDecreasePower(self.addr());
        }
    }
    fn editor_increase_power_toggle_interim(&self) {
        unsafe {
            PlayerController_TA_EditorIncreasePowerToggleInterim(self.addr());
        }
    }
    fn editor_decrease_power_toggle_interim(&self) {
        unsafe {
            PlayerController_TA_EditorDecreasePowerToggleInterim(self.addr());
        }
    }
    fn editor_increase_power_toggle(&self, b_toggle: bool) {
        unsafe {
            PlayerController_TA_EditorIncreasePowerToggle(self.addr(), b_toggle);
        }
    }
    fn editor_decrease_power_toggle(&self, b_toggle: bool) {
        unsafe {
            PlayerController_TA_EditorDecreasePowerToggle(self.addr(), b_toggle);
        }
    }
    fn modify_editor_power(&self, direction: i32) {
        unsafe {
            PlayerController_TA_ModifyEditorPower(self.addr(), direction);
        }
    }
    fn toggle_camera_position(&self) {
        unsafe {
            PlayerController_TA_ToggleCameraPosition(self.addr());
        }
    }
    fn editor_undo(&self) {
        unsafe {
            PlayerController_TA_EditorUndo(self.addr());
        }
    }
    fn editor_redo(&self) {
        unsafe {
            PlayerController_TA_EditorRedo(self.addr());
        }
    }
    fn editor_increase_round_time(&self) {
        unsafe {
            PlayerController_TA_EditorIncreaseRoundTime(self.addr());
        }
    }
    fn editor_decrease_round_time(&self) {
        unsafe {
            PlayerController_TA_EditorDecreaseRoundTime(self.addr());
        }
    }
    fn editor_next_round(&self) {
        unsafe {
            PlayerController_TA_EditorNextRound(self.addr());
        }
    }
    fn editor_prev_round(&self) {
        unsafe {
            PlayerController_TA_EditorPrevRound(self.addr());
        }
    }
    fn update_crosshair(&self) {
        unsafe {
            PlayerController_TA_UpdateCrosshair(self.addr());
        }
    }
    fn net_client_input_rate(&self, rate: i32) {
        unsafe {
            PlayerController_TA_NetClientInputRate(self.addr(), rate);
        }
    }
    fn server_create_match_broadcast(&self, game_event: ServerWrapper) {
        unsafe {
            PlayerController_TA_ServerCreateMatchBroadcast(self.addr(), game_event.addr());
        }
    }
    fn clamp_move_actor_grab_offset(&self) {
        unsafe {
            PlayerController_TA_ClampMoveActorGrabOffset(self.addr());
        }
    }
    fn revert_to_default_camera_hud_input(&self) {
        unsafe {
            PlayerController_TA_RevertToDefaultCameraHUDInput(self.addr());
        }
    }
    fn switch_to_edit_pawn(&self) {
        unsafe {
            PlayerController_TA_SwitchToEditPawn(self.addr());
        }
    }
    fn toggle_editor_round(&self) {
        unsafe {
            PlayerController_TA_ToggleEditorRound(self.addr());
        }
    }
    fn toggle_between_car_and_edit_pawn(&self) {
        unsafe {
            PlayerController_TA_ToggleBetweenCarAndEditPawn(self.addr());
        }
    }
    fn interact(&self) {
        unsafe {
            PlayerController_TA_Interact(self.addr());
        }
    }
    fn stop_movement(&self, b_only_if_no_accel: bool) {
        unsafe {
            PlayerController_TA_StopMovement(self.addr(), b_only_if_no_accel);
        }
    }
    fn get_rotate_actor_camera_offset(&self, delta_time: f32, b_snap: bool) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            PlayerController_TA_GetRotateActorCameraOffset(self.addr(), delta_time, b_snap, result_ptr);
            result
        }
    }
    fn restore_editor_pawn_orientation(&self) {
        unsafe {
            PlayerController_TA_RestoreEditorPawnOrientation(self.addr());
        }
    }
    fn backup_editor_pawn_orientation(&self) {
        unsafe {
            PlayerController_TA_BackupEditorPawnOrientation(self.addr());
        }
    }
    fn update_rotated_actor_orientation(&self, delta_time: f32) {
        unsafe {
            PlayerController_TA_UpdateRotatedActorOrientation(self.addr(), delta_time);
        }
    }
    fn on_open_pause_menu(&self) {
        unsafe {
            PlayerController_TA_OnOpenPauseMenu(self.addr());
        }
    }
    fn reset_mouse_center(&self) {
        unsafe {
            PlayerController_TA_ResetMouseCenter(self.addr());
        }
    }
    fn calculate_mouse_axis(&self, center: f32, current_location: f32, deadzone: f32, max_dist: f32) -> f32 {
        unsafe {
            PlayerController_TA_CalculateMouseAxis(self.addr(), center, current_location, deadzone, max_dist)
        }
    }
    fn show_controller_applet(&self) {
        unsafe {
            PlayerController_TA_ShowControllerApplet(self.addr());
        }
    }
    fn show_account_picker(&self) {
        unsafe {
            PlayerController_TA_ShowAccountPicker(self.addr());
        }
    }
    fn que_save_replay(&self) {
        unsafe {
            PlayerController_TA_QueSaveReplay(self.addr());
        }
    }
    fn set_follow_target2(&self, in_target: PriWrapper) {
        unsafe {
            PlayerController_TA_SetFollowTarget2(self.addr(), in_target.addr());
        }
    }
    fn follow_player(&self, in_player: PriWrapper) {
        unsafe {
            PlayerController_TA_FollowPlayer(self.addr(), in_player.addr());
        }
    }
    fn client_splitscreen_join_response(&self, player_id: UniqueNetId, b_allow: bool, error: RLString) {
        unsafe {
            let mut player_id = player_id;
            let player_id: *mut UniqueNetId = &mut player_id as *mut UniqueNetId;
            let mut error = error;
            let error: *mut RLString = &mut error as *mut RLString;
            PlayerController_TA_ClientSplitscreenJoinResponse(self.addr(), player_id, b_allow, error);
        }
    }
    fn server_request_splitscreen_join(&self, player_id: UniqueNetId, player_name: RLString) {
        unsafe {
            let mut player_id = player_id;
            let player_id: *mut UniqueNetId = &mut player_id as *mut UniqueNetId;
            let mut player_name = player_name;
            let player_name: *mut RLString = &mut player_name as *mut RLString;
            PlayerController_TA_ServerRequestSplitscreenJoin(self.addr(), player_id, player_name);
        }
    }
    fn notify_goal_scored(&self, scored_on_team: i32) {
        unsafe {
            PlayerController_TA_NotifyGoalScored(self.addr(), scored_on_team);
        }
    }
    fn should_be_muted(&self, other: PlayerControllerWrapper) -> bool {
        unsafe {
            PlayerController_TA_ShouldBeMuted(self.addr(), other.addr())
        }
    }
    fn is_communication_filtered(&self, filter: u8, other: PlayerControllerWrapper, b_preset: bool) -> bool {
        unsafe {
            PlayerController_TA_IsCommunicationFiltered(self.addr(), filter, other.addr(), b_preset)
        }
    }
    fn refresh_muted_players(&self, b_force_refresh: bool) {
        unsafe {
            PlayerController_TA_RefreshMutedPlayers(self.addr(), b_force_refresh);
        }
    }
    fn gameplay_unmute_player(&self, player_net_id: UniqueNetId) {
        unsafe {
            let mut player_net_id = player_net_id;
            let player_net_id: *mut UniqueNetId = &mut player_net_id as *mut UniqueNetId;
            PlayerController_TA_GameplayUnmutePlayer(self.addr(), player_net_id);
        }
    }
    fn gameplay_mute_player(&self, player_net_id: UniqueNetId) {
        unsafe {
            let mut player_net_id = player_net_id;
            let player_net_id: *mut UniqueNetId = &mut player_net_id as *mut UniqueNetId;
            PlayerController_TA_GameplayMutePlayer(self.addr(), player_net_id);
        }
    }
    fn server_set_chat_filter(&self, in_voice_filter: u8) {
        unsafe {
            PlayerController_TA_ServerSetChatFilter(self.addr(), in_voice_filter);
        }
    }
    fn server_set_voice_filter(&self, in_voice_filter: u8) {
        unsafe {
            PlayerController_TA_ServerSetVoiceFilter(self.addr(), in_voice_filter);
        }
    }
    fn is_explictly_muted(&self, player_net_id: UniqueNetId) -> bool {
        unsafe {
            let mut player_net_id = player_net_id;
            let player_net_id: *mut UniqueNetId = &mut player_net_id as *mut UniqueNetId;
            PlayerController_TA_IsExplictlyMuted(self.addr(), player_net_id)
        }
    }
    fn debug_ai(&self) {
        unsafe {
            PlayerController_TA_DebugAI(self.addr());
        }
    }
    fn send_pending_rp_cs(&self) {
        unsafe {
            PlayerController_TA_SendPendingRPCs(self.addr());
        }
    }
    fn can_send_message(&self, b_quick_chat_message: bool) -> bool {
        unsafe {
            PlayerController_TA_CanSendMessage(self.addr(), b_quick_chat_message)
        }
    }
    fn client_notify_chat_banned(&self, chat_ban_expiration: i64) {
        unsafe {
            PlayerController_TA_ClientNotifyChatBanned(self.addr(), chat_ban_expiration);
        }
    }
    fn client_notify_chat_disabled(&self, time: f32) {
        unsafe {
            PlayerController_TA_ClientNotifyChatDisabled(self.addr(), time);
        }
    }
    fn chat_message_ta(&self, in_pri: PlayerReplicationInfoWrapper, message: RLString, chat_channel: u8, b_preset: bool) {
        unsafe {
            let mut message = message;
            let message: *mut RLString = &mut message as *mut RLString;
            PlayerController_TA_ChatMessage_TA(self.addr(), in_pri.addr(), message, chat_channel, b_preset);
        }
    }
    fn server_say_internal_ta(&self, message: RLString, chat_channel: u8, b_preset: bool) {
        unsafe {
            let mut message = message;
            let message: *mut RLString = &mut message as *mut RLString;
            PlayerController_TA_ServerSayInternal_TA(self.addr(), message, chat_channel, b_preset);
        }
    }
    fn server_say_ta(&self, message: RLString, chat_channel: u8, b_preset: bool) {
        unsafe {
            let mut message = message;
            let message: *mut RLString = &mut message as *mut RLString;
            PlayerController_TA_ServerSay_TA(self.addr(), message, chat_channel, b_preset);
        }
    }
    fn say_ta(&self, message: RLString, chat_channel: u8, recipient: UniqueNetId, b_preset: bool) {
        unsafe {
            let mut message = message;
            let message: *mut RLString = &mut message as *mut RLString;
            let mut recipient = recipient;
            let recipient: *mut UniqueNetId = &mut recipient as *mut UniqueNetId;
            PlayerController_TA_Say_TA(self.addr(), message, chat_channel, recipient, b_preset);
        }
    }
    fn can_chat_with(&self, other: PlayerControllerWrapper, b_preset: bool) -> bool {
        unsafe {
            PlayerController_TA_CanChatWith(self.addr(), other.addr(), b_preset)
        }
    }
    fn allow_text_message(&self, msg: RLString) -> bool {
        unsafe {
            let mut msg = msg;
            let msg: *mut RLString = &mut msg as *mut RLString;
            PlayerController_TA_AllowTextMessage(self.addr(), msg)
        }
    }
    fn push_to_talk_end(&self) {
        unsafe {
            PlayerController_TA_PushToTalkEnd(self.addr());
        }
    }
    fn push_to_talk(&self) {
        unsafe {
            PlayerController_TA_PushToTalk(self.addr());
        }
    }
    fn handle_join_game_migration_completed(&self, b_success: bool, fail_reason: RLString) {
        unsafe {
            let mut fail_reason = fail_reason;
            let fail_reason: *mut RLString = &mut fail_reason as *mut RLString;
            PlayerController_TA_HandleJoinGameMigrationCompleted(self.addr(), b_success, fail_reason);
        }
    }
    fn client_set_online_status(&self) {
        unsafe {
            PlayerController_TA_ClientSetOnlineStatus(self.addr());
        }
    }
    fn handle_game_data_selected(&self, playlist_id: i32, mutator_index: i32) {
        unsafe {
            PlayerController_TA_HandleGameDataSelected(self.addr(), playlist_id, mutator_index);
        }
    }
    fn get_game_event(&self) -> Option<GameEventWrapper> {
        unsafe {
            GameEventWrapper::try_new(PlayerController_TA_GetGameEvent(self.addr()))
        }
    }
    fn client_arbitrated_match_ended(&self) {
        unsafe {
            PlayerController_TA_ClientArbitratedMatchEnded(self.addr());
        }
    }
    fn banned_kick(&self) {
        unsafe {
            PlayerController_TA_BannedKick(self.addr());
        }
    }
    fn no_reservation_kick(&self) {
        unsafe {
            PlayerController_TA_NoReservationKick(self.addr());
        }
    }
    fn idle_kick(&self) {
        unsafe {
            PlayerController_TA_IdleKick(self.addr());
        }
    }
    fn server_report_server(&self) {
        unsafe {
            PlayerController_TA_ServerReportServer(self.addr());
        }
    }
    fn client_use_item(&self, use_location: Vector, use_rotation: Rotator) {
        unsafe {
            let mut use_location = use_location;
            let use_location: *mut Vector = &mut use_location as *mut Vector;
            let mut use_rotation = use_rotation;
            let use_rotation: *mut Rotator = &mut use_rotation as *mut Rotator;
            PlayerController_TA_ClientUseItem(self.addr(), use_location, use_rotation);
        }
    }
    fn server_teleport_car(&self, spawn_location: Vector, new_rotation: Rotator) {
        unsafe {
            let mut spawn_location = spawn_location;
            let spawn_location: *mut Vector = &mut spawn_location as *mut Vector;
            let mut new_rotation = new_rotation;
            let new_rotation: *mut Rotator = &mut new_rotation as *mut Rotator;
            PlayerController_TA_ServerTeleportCar(self.addr(), spawn_location, new_rotation);
        }
    }
    fn server_use_pickup(&self, target: RBActorWrapper) {
        unsafe {
            PlayerController_TA_ServerUsePickup(self.addr(), target.addr());
        }
    }
    fn use_pickup(&self) {
        unsafe {
            PlayerController_TA_UsePickup(self.addr());
        }
    }
    fn toggle_handbrake(&self, b_handbrake: bool) {
        unsafe {
            PlayerController_TA_ToggleHandbrake(self.addr(), b_handbrake);
        }
    }
    fn toggle_boost(&self, b_boost: bool) {
        unsafe {
            PlayerController_TA_ToggleBoost(self.addr(), b_boost);
        }
    }
    fn toggle_jump(&self, b_jump: bool) {
        unsafe {
            PlayerController_TA_ToggleJump(self.addr(), b_jump);
        }
    }
    fn target_select_left(&self) {
        unsafe {
            PlayerController_TA_TargetSelectLeft(self.addr());
        }
    }
    fn target_select_right(&self) {
        unsafe {
            PlayerController_TA_TargetSelectRight(self.addr());
        }
    }
    fn release_rear_camera(&self) {
        unsafe {
            PlayerController_TA_ReleaseRearCamera(self.addr());
        }
    }
    fn press_rear_camera(&self) {
        unsafe {
            PlayerController_TA_PressRearCamera(self.addr());
        }
    }
    fn release_secondary_camera(&self) {
        unsafe {
            PlayerController_TA_ReleaseSecondaryCamera(self.addr());
        }
    }
    fn press_secondary_camera(&self) {
        unsafe {
            PlayerController_TA_PressSecondaryCamera(self.addr());
        }
    }
    fn ready_up(&self) {
        unsafe {
            PlayerController_TA_ReadyUp(self.addr());
        }
    }
    fn spectate(&self) {
        unsafe {
            PlayerController_TA_Spectate(self.addr());
        }
    }
    fn change_team(&self, team_num: i32) {
        unsafe {
            PlayerController_TA_ChangeTeam(self.addr(), team_num);
        }
    }
    fn switch_team(&self) {
        unsafe {
            PlayerController_TA_SwitchTeam(self.addr());
        }
    }
    fn set_default_camera_mode(&self) {
        unsafe {
            PlayerController_TA_SetDefaultCameraMode(self.addr());
        }
    }
    fn reset_camera_mode(&self) {
        unsafe {
            PlayerController_TA_ResetCameraMode(self.addr());
        }
    }
    fn zero_move_input(&self) {
        unsafe {
            PlayerController_TA_ZeroMoveInput(self.addr());
        }
    }
    fn player_move(&self, delta_time: f32) {
        unsafe {
            PlayerController_TA_PlayerMove(self.addr(), delta_time);
        }
    }
    fn ignore_move_input(&self, b_new_move_input: bool) {
        unsafe {
            PlayerController_TA_IgnoreMoveInput(self.addr(), b_new_move_input);
        }
    }
    fn get_primary_player_controller(&self) -> Option<PlayerControllerWrapper> {
        unsafe {
            PlayerControllerWrapper::try_new(PlayerController_TA_GetPrimaryPlayerController(self.addr()))
        }
    }
    fn handle_add_boost_component(&self, boost: BoostWrapper) {
        unsafe {
            PlayerController_TA_HandleAddBoostComponent(self.addr(), boost.addr());
        }
    }
    fn replicate_loadout(&self) {
        unsafe {
            PlayerController_TA_ReplicateLoadout(self.addr());
        }
    }
    fn replicate_camera_rotation(&self) {
        unsafe {
            PlayerController_TA_ReplicateCameraRotation(self.addr());
        }
    }
    fn set_using_freecam(&self, b_freecam: bool) {
        unsafe {
            PlayerController_TA_SetUsingFreecam(self.addr(), b_freecam);
        }
    }
    fn set_using_behind_view(&self, b_behind_view: bool) {
        unsafe {
            PlayerController_TA_SetUsingBehindView(self.addr(), b_behind_view);
        }
    }
    fn set_using_secondary_camera(&self, b_secondary_camera: bool) {
        unsafe {
            PlayerController_TA_SetUsingSecondaryCamera(self.addr(), b_secondary_camera);
        }
    }
    fn replicate_camera_settings(&self) {
        unsafe {
            PlayerController_TA_ReplicateCameraSettings(self.addr());
        }
    }
    fn handle_controller_layout_changed(&self, in_pri: PriWrapper) {
        unsafe {
            PlayerController_TA_HandleControllerLayoutChanged(self.addr(), in_pri.addr());
        }
    }
    fn handle_pawn_type_changed(&self, in_pri: PriWrapper) {
        unsafe {
            PlayerController_TA_HandlePawnTypeChanged(self.addr(), in_pri.addr());
        }
    }
    fn handle_pending_view_car_set(&self, in_pri: PriWrapper) {
        unsafe {
            PlayerController_TA_HandlePendingViewCarSet(self.addr(), in_pri.addr());
        }
    }
    fn handle_replace_bot(&self, in_pri: PriWrapper) {
        unsafe {
            PlayerController_TA_HandleReplaceBot(self.addr(), in_pri.addr());
        }
    }
    fn handle_team_changed(&self, in_pri: PriXWrapper) {
        unsafe {
            PlayerController_TA_HandleTeamChanged(self.addr(), in_pri.addr());
        }
    }
    fn handle_match_ended(&self, game_event: ServerWrapper) {
        unsafe {
            PlayerController_TA_HandleMatchEnded(self.addr(), game_event.addr());
        }
    }
    fn handle_start_new_round(&self, game_event: ServerWrapper) {
        unsafe {
            PlayerController_TA_HandleStartNewRound(self.addr(), game_event.addr());
        }
    }
    fn handle_game_event_changed(&self, in_pri: PriWrapper) {
        unsafe {
            PlayerController_TA_HandleGameEventChanged(self.addr(), in_pri.addr());
        }
    }
    fn handle_pri_camera_changed(&self, in_pri: PriWrapper) {
        unsafe {
            PlayerController_TA_HandlePRICameraChanged(self.addr(), in_pri.addr());
        }
    }
    fn setup_light_bar(&self) {
        unsafe {
            PlayerController_TA_SetupLightBar(self.addr());
        }
    }
    fn handle_persistent_camera_set(&self, in_pri: PriWrapper) {
        unsafe {
            PlayerController_TA_HandlePersistentCameraSet(self.addr(), in_pri.addr());
        }
    }
    fn on_received_player_and_pri(&self) {
        unsafe {
            PlayerController_TA_OnReceivedPlayerAndPRI(self.addr());
        }
    }
    fn update_voice_filter(&self) {
        unsafe {
            PlayerController_TA_UpdateVoiceFilter(self.addr());
        }
    }
    fn replicate_level_session_id(&self) {
        unsafe {
            PlayerController_TA_ReplicateLevelSessionID(self.addr());
        }
    }
    fn server_init_input_buffer(&self, type_: u8) {
        unsafe {
            PlayerController_TA_ServerInitInputBuffer(self.addr(), type_);
        }
    }
    fn event_chat_message(&self, pc: PlayerControllerWrapper, message: RLString, b_preset: bool) {
        unsafe {
            let mut message = message;
            let message: *mut RLString = &mut message as *mut RLString;
            PlayerController_TA_EventChatMessage(self.addr(), pc.addr(), message, b_preset);
        }
    }
    fn event_select_ball_cam_target(&self, pc: PlayerControllerWrapper, direction: i32) {
        unsafe {
            PlayerController_TA_EventSelectBallCamTarget(self.addr(), pc.addr(), direction);
        }
    }
    fn event_training_editor_actor_modified(&self) {
        unsafe {
            PlayerController_TA_EventTrainingEditorActorModified(self.addr());
        }
    }
    fn event_mute_changed(&self, pc: PlayerControllerWrapper, player_id: UniqueNetId, b_muted: bool) {
        unsafe {
            let mut player_id = player_id;
            let player_id: *mut UniqueNetId = &mut player_id as *mut UniqueNetId;
            PlayerController_TA_EventMuteChanged(self.addr(), pc.addr(), player_id, b_muted);
        }
    }
    fn event_launch_controller_applet(&self) {
        unsafe {
            PlayerController_TA_EventLaunchControllerApplet(self.addr());
        }
    }
    fn event_launch_account_picker(&self, controller_id: i32) {
        unsafe {
            PlayerController_TA_EventLaunchAccountPicker(self.addr(), controller_id);
        }
    }

}

extern "C" {
    fn PlayerController_TA_Get_Car(obj: usize) -> usize;
    fn PlayerControllerWrapper_SetCar(obj: usize, new_val: usize);
    fn PlayerController_TA_Get_PRI(obj: usize) -> usize;
    fn PlayerControllerWrapper_SetPRI(obj: usize, new_val: usize);
    fn PlayerController_TA_Get_VehicleInput(obj: usize, result: *mut VehicleInputs);
    fn PlayerControllerWrapper_SetVehicleInput(obj: usize, new_val: *mut VehicleInputs);
    fn PlayerController_TA_Get_bReceivedServerShutdownMessage(obj: usize) -> bool;
    fn PlayerControllerWrapper_SetbReceivedServerShutdownMessage(obj: usize, new_val: bool);
    fn PlayerController_TA_Get_bUseDebugInputs(obj: usize) -> bool;
    fn PlayerControllerWrapper_SetbUseDebugInputs(obj: usize, new_val: bool);
    fn PlayerController_TA_Get_bJumpPressed(obj: usize) -> bool;
    fn PlayerControllerWrapper_SetbJumpPressed(obj: usize, new_val: bool);
    fn PlayerController_TA_Get_bBoostPressed(obj: usize) -> bool;
    fn PlayerControllerWrapper_SetbBoostPressed(obj: usize, new_val: bool);
    fn PlayerController_TA_Get_bHandbrakePressed(obj: usize) -> bool;
    fn PlayerControllerWrapper_SetbHandbrakePressed(obj: usize, new_val: bool);
    fn PlayerController_TA_Get_bHasPitchedBack(obj: usize) -> bool;
    fn PlayerControllerWrapper_SetbHasPitchedBack(obj: usize, new_val: bool);
    fn PlayerController_TA_Get_bAllowAsymmetricalMute(obj: usize) -> bool;
    fn PlayerControllerWrapper_SetbAllowAsymmetricalMute(obj: usize, new_val: bool);
    fn PlayerController_TA_Get_bResetCamera(obj: usize) -> bool;
    fn PlayerControllerWrapper_SetbResetCamera(obj: usize, new_val: bool);
    fn PlayerController_TA_Get_LoginURL(obj: usize, result: *mut RLString);
    fn PlayerController_TA_Get_VoiceFilter(obj: usize) -> u8;
    fn PlayerControllerWrapper_SetVoiceFilter(obj: usize, new_val: u8);
    fn PlayerController_TA_Get_ChatFilter(obj: usize) -> u8;
    fn PlayerControllerWrapper_SetChatFilter(obj: usize, new_val: u8);
    fn PlayerController_TA_Get_FollowTarget(obj: usize) -> usize;
    fn PlayerControllerWrapper_SetFollowTarget(obj: usize, new_val: usize);
    fn PlayerController_TA_Get_SpectatorCameraArchetype(obj: usize) -> usize;
    fn PlayerControllerWrapper_SetSpectatorCameraArchetype(obj: usize, new_val: usize);
    fn PlayerController_TA_Get_EditorCameraArchetype(obj: usize) -> usize;
    fn PlayerControllerWrapper_SetEditorCameraArchetype(obj: usize, new_val: usize);
    fn PlayerController_TA_Get_MoveActorGrabOffset(obj: usize, result: *mut Vector);
    fn PlayerControllerWrapper_SetMoveActorGrabOffset(obj: usize, new_val: *mut Vector);
    fn PlayerController_TA_Get_MoveActorGrabIncrement(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetMoveActorGrabIncrement(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_MinMoveActorGrabDistance(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetMinMoveActorGrabDistance(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_MouseIncrementSpeed(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetMouseIncrementSpeed(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_BallVelocityIncrementAmount(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetBallVelocityIncrementAmount(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_BallVelocityIncrementFireCount(obj: usize) -> i32;
    fn PlayerControllerWrapper_SetBallVelocityIncrementFireCount(obj: usize, new_val: i32);
    fn PlayerController_TA_Get_BallVelocityIncrementFireCountMax(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetBallVelocityIncrementFireCountMax(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_BallVelocityIncrementSpeedDefault(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetBallVelocityIncrementSpeedDefault(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_BallVelocityIncrementSpeedMax(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetBallVelocityIncrementSpeedMax(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_CrosshairTraceDistance(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetCrosshairTraceDistance(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_TracedCrosshairActor(obj: usize) -> usize;
    fn PlayerControllerWrapper_SetTracedCrosshairActor(obj: usize, new_val: usize);
    fn PlayerController_TA_Get_RotateActorCameraLocationOffset(obj: usize, result: *mut Vector);
    fn PlayerControllerWrapper_SetRotateActorCameraLocationOffset(obj: usize, new_val: *mut Vector);
    fn PlayerController_TA_Get_RotateActorCameraRotationOffset(obj: usize, result: *mut Vector);
    fn PlayerControllerWrapper_SetRotateActorCameraRotationOffset(obj: usize, new_val: *mut Vector);
    fn PlayerController_TA_Get_RotateActorCameraSide(obj: usize) -> i32;
    fn PlayerControllerWrapper_SetRotateActorCameraSide(obj: usize, new_val: i32);
    fn PlayerController_TA_Get_DesiredCameraSide(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetDesiredCameraSide(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_PawnTypeChangedTime(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetPawnTypeChangedTime(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_SelectedSpawnArchetype(obj: usize) -> i32;
    fn PlayerControllerWrapper_SetSelectedSpawnArchetype(obj: usize, new_val: i32);
    fn PlayerController_TA_Get_DebugInputs(obj: usize, result: *mut VehicleInputs);
    fn PlayerControllerWrapper_SetDebugInputs(obj: usize, new_val: *mut VehicleInputs);
    fn PlayerController_TA_Get_MinClientInputRate(obj: usize) -> i32;
    fn PlayerControllerWrapper_SetMinClientInputRate(obj: usize, new_val: i32);
    fn PlayerController_TA_Get_MedianClientInputRate(obj: usize) -> i32;
    fn PlayerControllerWrapper_SetMedianClientInputRate(obj: usize, new_val: i32);
    fn PlayerController_TA_Get_MaxClientInputRate(obj: usize) -> i32;
    fn PlayerControllerWrapper_SetMaxClientInputRate(obj: usize, new_val: i32);
    fn PlayerController_TA_Get_ConfiguredClientInputRate(obj: usize) -> i32;
    fn PlayerControllerWrapper_SetConfiguredClientInputRate(obj: usize, new_val: i32);
    fn PlayerController_TA_Get_TimeSinceLastMovePacket(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetTimeSinceLastMovePacket(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_TimeLastReplicatedMovePacket(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetTimeLastReplicatedMovePacket(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_MouseXDeadZone(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetMouseXDeadZone(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_MouseYDeadZone(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetMouseYDeadZone(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_MouseXDeadZoneAir(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetMouseXDeadZoneAir(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_MouseYDeadZoneAir(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetMouseYDeadZoneAir(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_LastInputs(obj: usize, result: *mut VehicleInputs);
    fn PlayerControllerWrapper_SetLastInputs(obj: usize, new_val: *mut VehicleInputs);
    fn PlayerController_TA_Get_PendingViewPRI(obj: usize) -> usize;
    fn PlayerControllerWrapper_SetPendingViewPRI(obj: usize, new_val: usize);
    fn PlayerController_TA_Get_LastInputPitchUp(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetLastInputPitchUp(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_LastInputPitchDown(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetLastInputPitchDown(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_LastInputYawLeft(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetLastInputYawLeft(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_LastInputYawRight(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetLastInputYawRight(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_LastInputPitch(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetLastInputPitch(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_LastInputYaw(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetLastInputYaw(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_MouseInputMax(obj: usize) -> f32;
    fn PlayerControllerWrapper_SetMouseInputMax(obj: usize, new_val: f32);
    fn PlayerController_TA_Get_EngineShare(obj: usize) -> usize;
    fn PlayerControllerWrapper_SetEngineShare(obj: usize, new_val: usize);
    fn PlayerController_TA_HandleCarSet(obj: usize, InPRI: usize);
    fn PlayerController_TA_SpawnSelectedArchetype(obj: usize);
    fn PlayerController_TA_RemoveActor(obj: usize) -> bool;
    fn PlayerController_TA_ToggleGrabActor(obj: usize);
    fn PlayerController_TA_ToggleRotateActor(obj: usize);
    fn PlayerController_TA_EditorReleaseActor(obj: usize);
    fn PlayerController_TA_EditorCycleActor(obj: usize);
    fn PlayerController_TA_DuplicateShot(obj: usize);
    fn PlayerController_TA_StopEditing(obj: usize);
    fn PlayerController_TA_EditorIncreasePower(obj: usize);
    fn PlayerController_TA_EditorDecreasePower(obj: usize);
    fn PlayerController_TA_EditorIncreasePowerToggleInterim(obj: usize);
    fn PlayerController_TA_EditorDecreasePowerToggleInterim(obj: usize);
    fn PlayerController_TA_EditorIncreasePowerToggle(obj: usize, bToggle: bool);
    fn PlayerController_TA_EditorDecreasePowerToggle(obj: usize, bToggle: bool);
    fn PlayerController_TA_ModifyEditorPower(obj: usize, Direction: i32);
    fn PlayerController_TA_ToggleCameraPosition(obj: usize);
    fn PlayerController_TA_EditorUndo(obj: usize);
    fn PlayerController_TA_EditorRedo(obj: usize);
    fn PlayerController_TA_EditorIncreaseRoundTime(obj: usize);
    fn PlayerController_TA_EditorDecreaseRoundTime(obj: usize);
    fn PlayerController_TA_EditorNextRound(obj: usize);
    fn PlayerController_TA_EditorPrevRound(obj: usize);
    fn PlayerController_TA_UpdateCrosshair(obj: usize);
    fn PlayerController_TA_NetClientInputRate(obj: usize, Rate: i32);
    fn PlayerController_TA_ServerCreateMatchBroadcast(obj: usize, GameEvent: usize);
    fn PlayerController_TA_ClampMoveActorGrabOffset(obj: usize);
    fn PlayerController_TA_RevertToDefaultCameraHUDInput(obj: usize);
    fn PlayerController_TA_SwitchToEditPawn(obj: usize);
    fn PlayerController_TA_ToggleEditorRound(obj: usize);
    fn PlayerController_TA_ToggleBetweenCarAndEditPawn(obj: usize);
    fn PlayerController_TA_Interact(obj: usize);
    fn PlayerController_TA_StopMovement(obj: usize, bOnlyIfNoAccel: bool);
    fn PlayerController_TA_GetRotateActorCameraOffset(obj: usize, DeltaTime: f32, bSnap: bool, result: *mut Vector);
    fn PlayerController_TA_RestoreEditorPawnOrientation(obj: usize);
    fn PlayerController_TA_BackupEditorPawnOrientation(obj: usize);
    fn PlayerController_TA_UpdateRotatedActorOrientation(obj: usize, DeltaTime: f32);
    fn PlayerController_TA_OnOpenPauseMenu(obj: usize);
    fn PlayerController_TA_ResetMouseCenter(obj: usize);
    fn PlayerController_TA_CalculateMouseAxis(obj: usize, Center: f32, CurrentLocation: f32, Deadzone: f32, MaxDist: f32) -> f32;
    fn PlayerController_TA_ShowControllerApplet(obj: usize);
    fn PlayerController_TA_ShowAccountPicker(obj: usize);
    fn PlayerController_TA_QueSaveReplay(obj: usize);
    fn PlayerController_TA_SetFollowTarget2(obj: usize, InTarget: usize);
    fn PlayerController_TA_FollowPlayer(obj: usize, InPlayer: usize);
    fn PlayerController_TA_ClientSplitscreenJoinResponse(obj: usize, PlayerID: *mut UniqueNetId, bAllow: bool, Error: *mut RLString);
    fn PlayerController_TA_ServerRequestSplitscreenJoin(obj: usize, PlayerID: *mut UniqueNetId, PlayerName: *mut RLString);
    fn PlayerController_TA_NotifyGoalScored(obj: usize, ScoredOnTeam: i32);
    fn PlayerController_TA_ShouldBeMuted(obj: usize, Other: usize) -> bool;
    fn PlayerController_TA_IsCommunicationFiltered(obj: usize, Filter: u8, Other: usize, bPreset: bool) -> bool;
    fn PlayerController_TA_RefreshMutedPlayers(obj: usize, bForceRefresh: bool);
    fn PlayerController_TA_GameplayUnmutePlayer(obj: usize, PlayerNetId: *mut UniqueNetId);
    fn PlayerController_TA_GameplayMutePlayer(obj: usize, PlayerNetId: *mut UniqueNetId);
    fn PlayerController_TA_ServerSetChatFilter(obj: usize, InVoiceFilter: u8);
    fn PlayerController_TA_ServerSetVoiceFilter(obj: usize, InVoiceFilter: u8);
    fn PlayerController_TA_IsExplictlyMuted(obj: usize, PlayerNetId: *mut UniqueNetId) -> bool;
    fn PlayerController_TA_DebugAI(obj: usize);
    fn PlayerController_TA_SendPendingRPCs(obj: usize);
    fn PlayerController_TA_CanSendMessage(obj: usize, bQuickChatMessage: bool) -> bool;
    fn PlayerController_TA_ClientNotifyChatBanned(obj: usize, ChatBanExpiration: i64);
    fn PlayerController_TA_ClientNotifyChatDisabled(obj: usize, Time: f32);
    fn PlayerController_TA_ChatMessage_TA(obj: usize, InPRI: usize, Message: *mut RLString, ChatChannel: u8, bPreset: bool);
    fn PlayerController_TA_ServerSayInternal_TA(obj: usize, Message: *mut RLString, ChatChannel: u8, bPreset: bool);
    fn PlayerController_TA_ServerSay_TA(obj: usize, Message: *mut RLString, ChatChannel: u8, bPreset: bool);
    fn PlayerController_TA_Say_TA(obj: usize, Message: *mut RLString, ChatChannel: u8, Recipient: *mut UniqueNetId, bPreset: bool);
    fn PlayerController_TA_CanChatWith(obj: usize, Other: usize, bPreset: bool) -> bool;
    fn PlayerController_TA_AllowTextMessage(obj: usize, msg: *mut RLString) -> bool;
    fn PlayerController_TA_PushToTalkEnd(obj: usize);
    fn PlayerController_TA_PushToTalk(obj: usize);
    fn PlayerController_TA_HandleJoinGameMigrationCompleted(obj: usize, bSuccess: bool, FailReason: *mut RLString);
    fn PlayerController_TA_ClientSetOnlineStatus(obj: usize);
    fn PlayerController_TA_HandleGameDataSelected(obj: usize, PlaylistId: i32, MutatorIndex: i32);
    fn PlayerController_TA_GetGameEvent(obj: usize) -> usize;
    fn PlayerController_TA_ClientArbitratedMatchEnded(obj: usize);
    fn PlayerController_TA_BannedKick(obj: usize);
    fn PlayerController_TA_NoReservationKick(obj: usize);
    fn PlayerController_TA_IdleKick(obj: usize);
    fn PlayerController_TA_ServerReportServer(obj: usize);
    fn PlayerController_TA_ClientUseItem(obj: usize, UseLocation: *mut Vector, UseRotation: *mut Rotator);
    fn PlayerController_TA_ServerTeleportCar(obj: usize, SpawnLocation: *mut Vector, NewRotation: *mut Rotator);
    fn PlayerController_TA_ServerUsePickup(obj: usize, Target: usize);
    fn PlayerController_TA_UsePickup(obj: usize);
    fn PlayerController_TA_ToggleHandbrake(obj: usize, bHandbrake: bool);
    fn PlayerController_TA_ToggleBoost(obj: usize, bBoost: bool);
    fn PlayerController_TA_ToggleJump(obj: usize, bJump: bool);
    fn PlayerController_TA_TargetSelectLeft(obj: usize);
    fn PlayerController_TA_TargetSelectRight(obj: usize);
    fn PlayerController_TA_ReleaseRearCamera(obj: usize);
    fn PlayerController_TA_PressRearCamera(obj: usize);
    fn PlayerController_TA_ReleaseSecondaryCamera(obj: usize);
    fn PlayerController_TA_PressSecondaryCamera(obj: usize);
    fn PlayerController_TA_ReadyUp(obj: usize);
    fn PlayerController_TA_Spectate(obj: usize);
    fn PlayerController_TA_ChangeTeam(obj: usize, TeamNum: i32);
    fn PlayerController_TA_SwitchTeam(obj: usize);
    fn PlayerController_TA_SetDefaultCameraMode(obj: usize);
    fn PlayerController_TA_ResetCameraMode(obj: usize);
    fn PlayerController_TA_ZeroMoveInput(obj: usize);
    fn PlayerController_TA_PlayerMove(obj: usize, DeltaTime: f32);
    fn PlayerController_TA_IgnoreMoveInput(obj: usize, bNewMoveInput: bool);
    fn PlayerController_TA_GetPrimaryPlayerController(obj: usize) -> usize;
    fn PlayerController_TA_HandleAddBoostComponent(obj: usize, Boost: usize);
    fn PlayerController_TA_ReplicateLoadout(obj: usize);
    fn PlayerController_TA_ReplicateCameraRotation(obj: usize);
    fn PlayerController_TA_SetUsingFreecam(obj: usize, bFreecam: bool);
    fn PlayerController_TA_SetUsingBehindView(obj: usize, bBehindView: bool);
    fn PlayerController_TA_SetUsingSecondaryCamera(obj: usize, bSecondaryCamera: bool);
    fn PlayerController_TA_ReplicateCameraSettings(obj: usize);
    fn PlayerController_TA_HandleControllerLayoutChanged(obj: usize, InPRI: usize);
    fn PlayerController_TA_HandlePawnTypeChanged(obj: usize, InPRI: usize);
    fn PlayerController_TA_HandlePendingViewCarSet(obj: usize, InPRI: usize);
    fn PlayerController_TA_HandleReplaceBot(obj: usize, InPRI: usize);
    fn PlayerController_TA_HandleTeamChanged(obj: usize, InPRI: usize);
    fn PlayerController_TA_HandleMatchEnded(obj: usize, GameEvent: usize);
    fn PlayerController_TA_HandleStartNewRound(obj: usize, GameEvent: usize);
    fn PlayerController_TA_HandleGameEventChanged(obj: usize, InPRI: usize);
    fn PlayerController_TA_HandlePRICameraChanged(obj: usize, InPRI: usize);
    fn PlayerController_TA_SetupLightBar(obj: usize);
    fn PlayerController_TA_HandlePersistentCameraSet(obj: usize, InPRI: usize);
    fn PlayerController_TA_OnReceivedPlayerAndPRI(obj: usize);
    fn PlayerController_TA_UpdateVoiceFilter(obj: usize);
    fn PlayerController_TA_ReplicateLevelSessionID(obj: usize);
    fn PlayerController_TA_ServerInitInputBuffer(obj: usize, Type: u8);
    fn PlayerController_TA_EventChatMessage(obj: usize, PC: usize, Message: *mut RLString, bPreset: bool);
    fn PlayerController_TA_EventSelectBallCamTarget(obj: usize, PC: usize, Direction: i32);
    fn PlayerController_TA_EventTrainingEditorActorModified(obj: usize);
    fn PlayerController_TA_EventMuteChanged(obj: usize, PC: usize, PlayerID: *mut UniqueNetId, bMuted: bool);
    fn PlayerController_TA_EventLaunchControllerApplet(obj: usize);
    fn PlayerController_TA_EventLaunchAccountPicker(obj: usize, ControllerId: i32);

}