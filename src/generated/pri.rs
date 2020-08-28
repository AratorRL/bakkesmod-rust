use crate::wrappers::*;
use crate::generated::*;

pub struct PriWrapper(pub usize);
impl_object!(PriWrapper);

impl Pri for PriWrapper {}
impl PriX for PriWrapper {}
impl PlayerReplicationInfo for PriWrapper {}
impl Actor for PriWrapper {}

pub trait Pri : PriX {
    fn get_match_score(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchScore(self.addr())
        }
    }
    fn get_match_goals(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchGoals(self.addr())
        }
    }
    fn get_match_own_goals(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchOwnGoals(self.addr())
        }
    }
    fn get_match_assists(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchAssists(self.addr())
        }
    }
    fn get_match_saves(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchSaves(self.addr())
        }
    }
    fn get_match_shots(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchShots(self.addr())
        }
    }
    fn get_match_demolishes(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchDemolishes(self.addr())
        }
    }
    fn get_match_bonus_xp(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchBonusXP(self.addr())
        }
    }
    fn get_match_breakout_damage(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MatchBreakoutDamage(self.addr())
        }
    }
    fn get_b_match_mvp(&self) -> bool {
        unsafe {
            PRI_TA_Get_bMatchMVP(self.addr())
        }
    }
    fn get_b_match_admin(&self) -> bool {
        unsafe {
            PRI_TA_Get_bMatchAdmin(self.addr())
        }
    }
    fn get_b_loadout_set(&self) -> bool {
        unsafe {
            PRI_TA_Get_bLoadoutSet(self.addr())
        }
    }
    fn get_b_online_loadout_set(&self) -> bool {
        unsafe {
            PRI_TA_Get_bOnlineLoadoutSet(self.addr())
        }
    }
    fn get_b_loadouts_set(&self) -> bool {
        unsafe {
            PRI_TA_Get_bLoadoutsSet(self.addr())
        }
    }
    fn get_b_online_loadouts_set(&self) -> bool {
        unsafe {
            PRI_TA_Get_bOnlineLoadoutsSet(self.addr())
        }
    }
    fn get_b_team_paint_set(&self) -> bool {
        unsafe {
            PRI_TA_Get_bTeamPaintSet(self.addr())
        }
    }
    fn get_b_ready(&self) -> bool {
        unsafe {
            PRI_TA_Get_bReady(self.addr())
        }
    }
    fn get_b_using_secondary_camera(&self) -> bool {
        unsafe {
            PRI_TA_Get_bUsingSecondaryCamera(self.addr())
        }
    }
    fn get_b_using_behind_view(&self) -> bool {
        unsafe {
            PRI_TA_Get_bUsingBehindView(self.addr())
        }
    }
    fn get_b_using_freecam(&self) -> bool {
        unsafe {
            PRI_TA_Get_bUsingFreecam(self.addr())
        }
    }
    fn get_b_is_in_split_screen(&self) -> bool {
        unsafe {
            PRI_TA_Get_bIsInSplitScreen(self.addr())
        }
    }
    fn get_b_start_vote_to_forfeit_disabled(&self) -> bool {
        unsafe {
            PRI_TA_Get_bStartVoteToForfeitDisabled(self.addr())
        }
    }
    fn get_b_using_items(&self) -> bool {
        unsafe {
            PRI_TA_Get_bUsingItems(self.addr())
        }
    }
    fn get_player_history_valid(&self) -> bool {
        unsafe {
            PRI_TA_Get_PlayerHistoryValid(self.addr())
        }
    }
    fn get_game_event(&self) -> Option<GameEventWrapper> {
        unsafe {
            GameEventWrapper::try_new(PRI_TA_Get_GameEvent(self.addr()))
        }
    }
    fn get_replicated_game_event(&self) -> Option<GameEventWrapper> {
        unsafe {
            GameEventWrapper::try_new(PRI_TA_Get_ReplicatedGameEvent(self.addr()))
        }
    }
    fn get_car(&self) -> Option<CarWrapper> {
        unsafe {
            CarWrapper::try_new(PRI_TA_Get_Car(self.addr()))
        }
    }
    fn get_waiting_start_time(&self) -> i32 {
        unsafe {
            PRI_TA_Get_WaitingStartTime(self.addr())
        }
    }
    fn get_total_game_time_played(&self) -> f32 {
        unsafe {
            PRI_TA_Get_TotalGameTimePlayed(self.addr())
        }
    }
    fn get_camera_settings(&self) -> ProfileCameraSettings {
        unsafe {
            let mut result = ProfileCameraSettings::new();
            let result_ptr: *mut ProfileCameraSettings = &mut result as *mut ProfileCameraSettings;
            PRI_TA_Get_CameraSettings(self.addr(), result_ptr);
            result
        }
    }
    fn get_camera_pitch(&self) -> u8 {
        unsafe {
            PRI_TA_Get_CameraPitch(self.addr())
        }
    }
    fn get_camera_yaw(&self) -> u8 {
        unsafe {
            PRI_TA_Get_CameraYaw(self.addr())
        }
    }
    fn get_pawn_type(&self) -> u8 {
        unsafe {
            PRI_TA_Get_PawnType(self.addr())
        }
    }
    fn get_replicated_worst_net_quality_beyond_latency(&self) -> u8 {
        unsafe {
            PRI_TA_Get_ReplicatedWorstNetQualityBeyondLatency(self.addr())
        }
    }
    fn get_party_leader(&self) -> UniqueNetId {
        unsafe {
            let mut result = UniqueNetId::new();
            let result_ptr: *mut UniqueNetId = &mut result as *mut UniqueNetId;
            PRI_TA_Get_PartyLeader(self.addr(), result_ptr);
            result
        }
    }
    fn get_dodge_input_threshold(&self) -> f32 {
        unsafe {
            PRI_TA_Get_DodgeInputThreshold(self.addr())
        }
    }
    fn get_steering_sensitivity(&self) -> f32 {
        unsafe {
            PRI_TA_Get_SteeringSensitivity(self.addr())
        }
    }
    fn get_air_control_sensitivity(&self) -> f32 {
        unsafe {
            PRI_TA_Get_AirControlSensitivity(self.addr())
        }
    }
    fn get_time_till_item(&self) -> i32 {
        unsafe {
            PRI_TA_Get_TimeTillItem(self.addr())
        }
    }
    fn get_max_time_till_item(&self) -> i32 {
        unsafe {
            PRI_TA_Get_MaxTimeTillItem(self.addr())
        }
    }
    fn get_boost_pickups(&self) -> i32 {
        unsafe {
            PRI_TA_Get_BoostPickups(self.addr())
        }
    }
    fn get_ball_touches(&self) -> i32 {
        unsafe {
            PRI_TA_Get_BallTouches(self.addr())
        }
    }
    fn get_car_touches(&self) -> i32 {
        unsafe {
            PRI_TA_Get_CarTouches(self.addr())
        }
    }
    fn get_replacing_bot_pri(&self) -> Option<PriWrapper> {
        unsafe {
            PriWrapper::try_new(PRI_TA_Get_ReplacingBotPRI(self.addr()))
        }
    }
    fn get_club_id(&self) -> i64 {
        unsafe {
            PRI_TA_Get_ClubID(self.addr())
        }
    }
    fn get_public_ip(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            PRI_TA_Get_PublicIP(self.addr(), result_ptr);
            result
        }
    }
    fn get_spectator_shortcut(&self) -> i32 {
        unsafe {
            PRI_TA_Get_SpectatorShortcut(self.addr())
        }
    }
    fn club_id_change_notify_func(&self) {
        unsafe {
            PRI_TA___ClubID__ChangeNotifyFunc(self.addr());
        }
    }
    fn prei_change_notify_func(&self) {
        unsafe {
            PRI_TA___PREI__ChangeNotifyFunc(self.addr());
        }
    }
    fn replicated_worst_net_quality_beyond_latency_change_notify_func(&self) {
        unsafe {
            PRI_TA___ReplicatedWorstNetQualityBeyondLatency__ChangeNotifyFunc(self.addr());
        }
    }
    fn on_spectator_shortcut_changed(&self) {
        unsafe {
            PRI_TA_OnSpectatorShortcutChanged(self.addr());
        }
    }
    fn set_spectator_shortcut2(&self, in_shortcut: i32) {
        unsafe {
            PRI_TA_SetSpectatorShortcut2(self.addr(), in_shortcut);
        }
    }
    fn server_set_public_ip(&self, ip: RLString) {
        unsafe {
            let mut ip = ip;
            let ip: *mut RLString = &mut ip as *mut RLString;
            PRI_TA_ServerSetPublicIP(self.addr(), ip);
        }
    }
    fn on_unique_id_changed(&self) {
        unsafe {
            PRI_TA_OnUniqueIdChanged(self.addr());
        }
    }
    fn update_player_avatar_border(&self) {
        unsafe {
            PRI_TA_UpdatePlayerAvatarBorder(self.addr());
        }
    }
    fn update_player_banner(&self) {
        unsafe {
            PRI_TA_UpdatePlayerBanner(self.addr());
        }
    }
    fn client_achievement_progression(&self, achievement_id: i32, achievement_type: u8, progress: f32, max_progress: f32) {
        unsafe {
            PRI_TA_ClientAchievementProgression(self.addr(), achievement_id, achievement_type, progress, max_progress);
        }
    }
    fn client_unlock_achievement(&self, achievement_id: i32, achievement_type: u8) {
        unsafe {
            PRI_TA_ClientUnlockAchievement(self.addr(), achievement_id, achievement_type);
        }
    }
    fn server_set_player_activated_fx(&self, fx: FXActorWrapper) {
        unsafe {
            PRI_TA_ServerSetPlayerActivatedFX(self.addr(), fx.addr());
        }
    }
    fn update_car_local_player(&self) {
        unsafe {
            PRI_TA_UpdateCarLocalPlayer(self.addr());
        }
    }
    fn on_replacing_bot_pri_changed(&self) {
        unsafe {
            PRI_TA_OnReplacingBotPRIChanged(self.addr());
        }
    }
    fn on_team_changed(&self) {
        unsafe {
            PRI_TA_OnTeamChanged(self.addr());
        }
    }
    fn clear_bot_replacement(&self) {
        unsafe {
            PRI_TA_ClearBotReplacement(self.addr());
        }
    }
    fn report_cheater(&self, reason: RLString) {
        unsafe {
            let mut reason = reason;
            let reason: *mut RLString = &mut reason as *mut RLString;
            PRI_TA_ReportCheater(self.addr(), reason);
        }
    }
    fn validate_loadout_team_paints(&self) -> bool {
        unsafe {
            PRI_TA_ValidateLoadoutTeamPaints(self.addr())
        }
    }
    fn validate_loadout_slots(&self) -> bool {
        unsafe {
            PRI_TA_ValidateLoadoutSlots(self.addr())
        }
    }
    fn validate_loadout_dlc(&self) {
        unsafe {
            PRI_TA_ValidateLoadoutDLC(self.addr());
        }
    }
    fn on_start_vote_to_forfeit_disabled_changed(&self) {
        unsafe {
            PRI_TA_OnStartVoteToForfeitDisabledChanged(self.addr());
        }
    }
    fn set_start_vote_to_forfeit_disabled(&self, b_disabled: bool) {
        unsafe {
            PRI_TA_SetStartVoteToForfeitDisabled(self.addr(), b_disabled);
        }
    }
    fn server_vote_to_forfeit(&self) {
        unsafe {
            PRI_TA_ServerVoteToForfeit(self.addr());
        }
    }
    fn set_user_car_preferences(&self, new_dodge_threshold: f32, new_steering_sensitivity: f32, new_air_control_sensitivity: f32) {
        unsafe {
            PRI_TA_SetUserCarPreferences(self.addr(), new_dodge_threshold, new_steering_sensitivity, new_air_control_sensitivity);
        }
    }
    fn server_set_user_car_preferences(&self, new_dodge_threshold: f32, new_steering_sensitivity: f32, new_air_control_sensitivity: f32) {
        unsafe {
            PRI_TA_ServerSetUserCarPreferences(self.addr(), new_dodge_threshold, new_steering_sensitivity, new_air_control_sensitivity);
        }
    }
    fn validate_user_int(&self, reason: RLString, new_value: i32, min: i32, max: i32) -> i32 {
        unsafe {
            let mut reason = reason;
            let reason: *mut RLString = &mut reason as *mut RLString;
            PRI_TA_ValidateUserInt(self.addr(), reason, new_value, min, max)
        }
    }
    fn validate_user_float(&self, reason: RLString, new_value: f32, min: f32, max: f32) -> f32 {
        unsafe {
            let mut reason = reason;
            let reason: *mut RLString = &mut reason as *mut RLString;
            PRI_TA_ValidateUserFloat(self.addr(), reason, new_value, min, max)
        }
    }
    fn on_pawn_type_changed(&self) {
        unsafe {
            PRI_TA_OnPawnTypeChanged(self.addr());
        }
    }
    fn set_waiting_player(&self, b: bool) {
        unsafe {
            PRI_TA_SetWaitingPlayer(self.addr(), b);
        }
    }
    fn set_pawn_type2(&self, new_pawn_type: u8) {
        unsafe {
            PRI_TA_SetPawnType2(self.addr(), new_pawn_type);
        }
    }
    fn is_player(&self) -> bool {
        unsafe {
            PRI_TA_IsPlayer(self.addr())
        }
    }
    fn is_editor(&self) -> bool {
        unsafe {
            PRI_TA_IsEditor(self.addr())
        }
    }
    fn is_spectator(&self) -> bool {
        unsafe {
            PRI_TA_IsSpectator(self.addr())
        }
    }
    fn server_spectate(&self) {
        unsafe {
            PRI_TA_ServerSpectate(self.addr());
        }
    }
    fn client_change_team_failed(&self, team_num: i32) {
        unsafe {
            PRI_TA_ClientChangeTeamFailed(self.addr(), team_num);
        }
    }
    fn server_change_team(&self, team_num: i32) {
        unsafe {
            PRI_TA_ServerChangeTeam(self.addr(), team_num);
        }
    }
    fn replicate_podium_titles(&self) {
        unsafe {
            PRI_TA_ReplicatePodiumTitles(self.addr());
        }
    }
    fn on_match_admin(&self) {
        unsafe {
            PRI_TA_OnMatchAdmin(self.addr());
        }
    }
    fn on_mvp_change(&self) {
        unsafe {
            PRI_TA_OnMVPChange(self.addr());
        }
    }
    fn notify_won_mvp(&self) {
        unsafe {
            PRI_TA_NotifyWonMVP(self.addr());
        }
    }
    fn is_invalid_name(&self) -> bool {
        unsafe {
            PRI_TA_IsInvalidName(self.addr())
        }
    }
    fn is_in_warm_up_mode(&self) -> bool {
        unsafe {
            PRI_TA_IsInWarmUpMode(self.addr())
        }
    }
    fn set_ready(&self, b_new_ready: bool) {
        unsafe {
            PRI_TA_SetReady(self.addr(), b_new_ready);
        }
    }
    fn server_match_admin_set_paused(&self, b_pause: bool) {
        unsafe {
            PRI_TA_ServerMatchAdminSetPaused(self.addr(), b_pause);
        }
    }
    fn server_match_admin_set_score_and_time(&self, new_score_team0: i32, new_score_team1: i32, seconds_remaining: i32, b_over_time: bool, b_restart_round: bool) {
        unsafe {
            PRI_TA_ServerMatchAdminSetScoreAndTime(self.addr(), new_score_team0, new_score_team1, seconds_remaining, b_over_time, b_restart_round);
        }
    }
    fn set_match_admin(&self, b_is_match_admin: bool) {
        unsafe {
            PRI_TA_SetMatchAdmin(self.addr(), b_is_match_admin);
        }
    }
    fn server_ready_up(&self) {
        unsafe {
            PRI_TA_ServerReadyUp(self.addr());
        }
    }
    fn should_broad_cast_welcome_message(&self, b_exiting: bool) -> bool {
        unsafe {
            PRI_TA_ShouldBroadCastWelcomeMessage(self.addr(), b_exiting)
        }
    }
    fn update_online_product_stats(&self) {
        unsafe {
            PRI_TA_UpdateOnlineProductStats(self.addr());
        }
    }
    fn set_waiting_start_time2(&self) {
        unsafe {
            PRI_TA_SetWaitingStartTime2(self.addr());
        }
    }
    fn client_scored_goal(&self, ball_hit_location: Vector) {
        unsafe {
            let mut ball_hit_location = ball_hit_location;
            let ball_hit_location: *mut Vector = &mut ball_hit_location as *mut Vector;
            PRI_TA_ClientScoredGoal(self.addr(), ball_hit_location);
        }
    }
    fn on_scored_goal(&self, ball_hit_location: Vector) {
        unsafe {
            let mut ball_hit_location = ball_hit_location;
            let ball_hit_location: *mut Vector = &mut ball_hit_location as *mut Vector;
            PRI_TA_OnScoredGoal(self.addr(), ball_hit_location);
        }
    }
    fn on_rep_steering_sensitivity(&self) {
        unsafe {
            PRI_TA_OnRep_SteeringSensitivity(self.addr());
        }
    }
    fn on_rep_client_score_point(&self) {
        unsafe {
            PRI_TA_OnRep_ClientScorePoint(self.addr());
        }
    }
    fn reset_score(&self) {
        unsafe {
            PRI_TA_ResetScore(self.addr());
        }
    }
    fn remove_points(&self, points: i32) {
        unsafe {
            PRI_TA_RemovePoints(self.addr(), points);
        }
    }
    fn score_point(&self, additional_score: i32) {
        unsafe {
            PRI_TA_ScorePoint(self.addr(), additional_score);
        }
    }
    fn get_match_xp(&self) -> i32 {
        unsafe {
            PRI_TA_GetMatchXP(self.addr())
        }
    }
    fn commit_stats(&self) {
        unsafe {
            PRI_TA_CommitStats(self.addr());
        }
    }
    fn update_from_loadout(&self) {
        unsafe {
            PRI_TA_UpdateFromLoadout(self.addr());
        }
    }
    fn update_user_car_preferences(&self, air_control_component: AirControlComponentWrapper) {
        unsafe {
            PRI_TA_UpdateUserCarPreferences(self.addr(), air_control_component.addr());
        }
    }
    fn handle_air_control(&self, air_control_comp: AirControlComponentWrapper) {
        unsafe {
            PRI_TA_HandleAirControl(self.addr(), air_control_comp.addr());
        }
    }
    fn set_car2(&self, new_car: CarWrapper) {
        unsafe {
            PRI_TA_SetCar2(self.addr(), new_car.addr());
        }
    }
    fn is_client_player_pri(&self) -> bool {
        unsafe {
            PRI_TA_IsClientPlayerPRI(self.addr())
        }
    }
    fn is_local_player_pri(&self) -> bool {
        unsafe {
            PRI_TA_IsLocalPlayerPRI(self.addr())
        }
    }
    fn unregister(&self) {
        unsafe {
            PRI_TA_Unregister(self.addr());
        }
    }
    fn add_local_player_to_game_event(&self) {
        unsafe {
            PRI_TA_AddLocalPlayerToGameEvent(self.addr());
        }
    }
    fn on_loadouts_set_internal(&self) {
        unsafe {
            PRI_TA_OnLoadoutsSetInternal(self.addr());
        }
    }
    fn are_loadouts_set(&self) -> bool {
        unsafe {
            PRI_TA_AreLoadoutsSet(self.addr())
        }
    }
    fn on_loadouts_online_set(&self) {
        unsafe {
            PRI_TA_OnLoadoutsOnlineSet(self.addr());
        }
    }
    fn on_loadouts_set(&self) {
        unsafe {
            PRI_TA_OnLoadoutsSet(self.addr());
        }
    }
    fn remove_certified_product_stat(&self, instance_id: i64) {
        unsafe {
            PRI_TA_RemoveCertifiedProductStat(self.addr(), instance_id);
        }
    }
    fn init_loadout_attributes_for_team(&self, pri: PriXWrapper) {
        unsafe {
            PRI_TA_InitLoadoutAttributesForTeam(self.addr(), pri.addr());
        }
    }
    fn should_validate_online_products(&self) -> bool {
        unsafe {
            PRI_TA_ShouldValidateOnlineProducts(self.addr())
        }
    }
    fn on_split_screen_status_changed(&self) {
        unsafe {
            PRI_TA_OnSplitScreenStatusChanged(self.addr());
        }
    }
    fn server_split_screen_status_changed(&self, b_in_split_screen: bool) {
        unsafe {
            PRI_TA_ServerSplitScreenStatusChanged(self.addr(), b_in_split_screen);
        }
    }
    fn update_split_screen_status(&self) {
        unsafe {
            PRI_TA_UpdateSplitScreenStatus(self.addr());
        }
    }
    fn on_party_leader_changed(&self) {
        unsafe {
            PRI_TA_OnPartyLeaderChanged(self.addr());
        }
    }
    fn set_party_leader2(&self, in_party_leader: UniqueNetId) {
        unsafe {
            let mut in_party_leader = in_party_leader;
            let in_party_leader: *mut UniqueNetId = &mut in_party_leader as *mut UniqueNetId;
            PRI_TA_SetPartyLeader2(self.addr(), in_party_leader);
        }
    }
    fn update_party_status(&self) {
        unsafe {
            PRI_TA_UpdatePartyStatus(self.addr());
        }
    }
    fn on_skill_tier_changed(&self) {
        unsafe {
            PRI_TA_OnSkillTierChanged(self.addr());
        }
    }
    fn replicate_skill_tier(&self, new_tier: i32) {
        unsafe {
            PRI_TA_ReplicateSkillTier(self.addr(), new_tier);
        }
    }
    fn on_title_changed(&self) {
        unsafe {
            PRI_TA_OnTitleChanged(self.addr());
        }
    }
    fn sync_player_title(&self) {
        unsafe {
            PRI_TA_SyncPlayerTitle(self.addr());
        }
    }
    fn update_title_from_loadout(&self) {
        unsafe {
            PRI_TA_UpdateTitleFromLoadout(self.addr());
        }
    }
    fn update_title(&self) {
        unsafe {
            PRI_TA_UpdateTitle(self.addr());
        }
    }
    fn get_new_friend_key(&self) {
        unsafe {
            PRI_TA_GetNewFriendKey(self.addr());
        }
    }
    fn on_rep_unique_id(&self) {
        unsafe {
            PRI_TA_OnRep_UniqueId(self.addr());
        }
    }
    fn event_owner_changed(&self, pri: PriWrapper) {
        unsafe {
            PRI_TA_EventOwnerChanged(self.addr(), pri.addr());
        }
    }
    fn event_spectator_shortcut_changed(&self, pri: PriWrapper) {
        unsafe {
            PRI_TA_EventSpectatorShortcutChanged(self.addr(), pri.addr());
        }
    }
    fn event_server_achievement_progression(&self, pri: PriWrapper, achievement_id: i32, achievement_type: u8, progress: f32, max_progress: f32) {
        unsafe {
            PRI_TA_EventServerAchievementProgression(self.addr(), pri.addr(), achievement_id, achievement_type, progress, max_progress);
        }
    }
    fn event_start_vote_to_forfeit_disabled_changed(&self, pri: PriWrapper) {
        unsafe {
            PRI_TA_EventStartVoteToForfeitDisabledChanged(self.addr(), pri.addr());
        }
    }
    fn event_car_pre_update(&self, pri: PriWrapper) {
        unsafe {
            PRI_TA_EventCarPreUpdate(self.addr(), pri.addr());
        }
    }

}

extern "C" {
    fn PRI_TA_Get_MatchScore(obj: usize) -> i32;
    fn PriWrapper_SetMatchScore(obj: usize, new_val: i32);
    fn PRI_TA_Get_MatchGoals(obj: usize) -> i32;
    fn PriWrapper_SetMatchGoals(obj: usize, new_val: i32);
    fn PRI_TA_Get_MatchOwnGoals(obj: usize) -> i32;
    fn PriWrapper_SetMatchOwnGoals(obj: usize, new_val: i32);
    fn PRI_TA_Get_MatchAssists(obj: usize) -> i32;
    fn PriWrapper_SetMatchAssists(obj: usize, new_val: i32);
    fn PRI_TA_Get_MatchSaves(obj: usize) -> i32;
    fn PriWrapper_SetMatchSaves(obj: usize, new_val: i32);
    fn PRI_TA_Get_MatchShots(obj: usize) -> i32;
    fn PriWrapper_SetMatchShots(obj: usize, new_val: i32);
    fn PRI_TA_Get_MatchDemolishes(obj: usize) -> i32;
    fn PriWrapper_SetMatchDemolishes(obj: usize, new_val: i32);
    fn PRI_TA_Get_MatchBonusXP(obj: usize) -> i32;
    fn PriWrapper_SetMatchBonusXP(obj: usize, new_val: i32);
    fn PRI_TA_Get_MatchBreakoutDamage(obj: usize) -> i32;
    fn PriWrapper_SetMatchBreakoutDamage(obj: usize, new_val: i32);
    fn PRI_TA_Get_bMatchMVP(obj: usize) -> bool;
    fn PriWrapper_SetbMatchMVP(obj: usize, new_val: bool);
    fn PRI_TA_Get_bMatchAdmin(obj: usize) -> bool;
    fn PriWrapper_SetbMatchAdmin(obj: usize, new_val: bool);
    fn PRI_TA_Get_bLoadoutSet(obj: usize) -> bool;
    fn PriWrapper_SetbLoadoutSet(obj: usize, new_val: bool);
    fn PRI_TA_Get_bOnlineLoadoutSet(obj: usize) -> bool;
    fn PriWrapper_SetbOnlineLoadoutSet(obj: usize, new_val: bool);
    fn PRI_TA_Get_bLoadoutsSet(obj: usize) -> bool;
    fn PriWrapper_SetbLoadoutsSet(obj: usize, new_val: bool);
    fn PRI_TA_Get_bOnlineLoadoutsSet(obj: usize) -> bool;
    fn PriWrapper_SetbOnlineLoadoutsSet(obj: usize, new_val: bool);
    fn PRI_TA_Get_bTeamPaintSet(obj: usize) -> bool;
    fn PriWrapper_SetbTeamPaintSet(obj: usize, new_val: bool);
    fn PRI_TA_Get_bReady(obj: usize) -> bool;
    fn PriWrapper_SetbReady(obj: usize, new_val: bool);
    fn PRI_TA_Get_bUsingSecondaryCamera(obj: usize) -> bool;
    fn PriWrapper_SetbUsingSecondaryCamera(obj: usize, new_val: bool);
    fn PRI_TA_Get_bUsingBehindView(obj: usize) -> bool;
    fn PriWrapper_SetbUsingBehindView(obj: usize, new_val: bool);
    fn PRI_TA_Get_bUsingFreecam(obj: usize) -> bool;
    fn PriWrapper_SetbUsingFreecam(obj: usize, new_val: bool);
    fn PRI_TA_Get_bIsInSplitScreen(obj: usize) -> bool;
    fn PriWrapper_SetbIsInSplitScreen(obj: usize, new_val: bool);
    fn PRI_TA_Get_bStartVoteToForfeitDisabled(obj: usize) -> bool;
    fn PriWrapper_SetbStartVoteToForfeitDisabled(obj: usize, new_val: bool);
    fn PRI_TA_Get_bUsingItems(obj: usize) -> bool;
    fn PriWrapper_SetbUsingItems(obj: usize, new_val: bool);
    fn PRI_TA_Get_PlayerHistoryValid(obj: usize) -> bool;
    fn PriWrapper_SetPlayerHistoryValid(obj: usize, new_val: bool);
    fn PRI_TA_Get_GameEvent(obj: usize) -> usize;
    fn PriWrapper_SetGameEvent(obj: usize, new_val: usize);
    fn PRI_TA_Get_ReplicatedGameEvent(obj: usize) -> usize;
    fn PriWrapper_SetReplicatedGameEvent(obj: usize, new_val: usize);
    fn PRI_TA_Get_Car(obj: usize) -> usize;
    fn PriWrapper_SetCar(obj: usize, new_val: usize);
    fn PRI_TA_Get_WaitingStartTime(obj: usize) -> i32;
    fn PriWrapper_SetWaitingStartTime(obj: usize, new_val: i32);
    fn PRI_TA_Get_TotalGameTimePlayed(obj: usize) -> f32;
    fn PriWrapper_SetTotalGameTimePlayed(obj: usize, new_val: f32);
    fn PRI_TA_Get_CameraSettings(obj: usize, result: *mut ProfileCameraSettings);
    fn PriWrapper_SetCameraSettings(obj: usize, new_val: *mut ProfileCameraSettings);
    fn PRI_TA_Get_CameraPitch(obj: usize) -> u8;
    fn PriWrapper_SetCameraPitch(obj: usize, new_val: u8);
    fn PRI_TA_Get_CameraYaw(obj: usize) -> u8;
    fn PriWrapper_SetCameraYaw(obj: usize, new_val: u8);
    fn PRI_TA_Get_PawnType(obj: usize) -> u8;
    fn PriWrapper_SetPawnType(obj: usize, new_val: u8);
    fn PRI_TA_Get_ReplicatedWorstNetQualityBeyondLatency(obj: usize) -> u8;
    fn PriWrapper_SetReplicatedWorstNetQualityBeyondLatency(obj: usize, new_val: u8);
    fn PRI_TA_Get_PartyLeader(obj: usize, result: *mut UniqueNetId);
    fn PriWrapper_SetPartyLeader(obj: usize, new_val: *mut UniqueNetId);
    fn PRI_TA_Get_DodgeInputThreshold(obj: usize) -> f32;
    fn PriWrapper_SetDodgeInputThreshold(obj: usize, new_val: f32);
    fn PRI_TA_Get_SteeringSensitivity(obj: usize) -> f32;
    fn PriWrapper_SetSteeringSensitivity(obj: usize, new_val: f32);
    fn PRI_TA_Get_AirControlSensitivity(obj: usize) -> f32;
    fn PriWrapper_SetAirControlSensitivity(obj: usize, new_val: f32);
    fn PRI_TA_Get_TimeTillItem(obj: usize) -> i32;
    fn PriWrapper_SetTimeTillItem(obj: usize, new_val: i32);
    fn PRI_TA_Get_MaxTimeTillItem(obj: usize) -> i32;
    fn PriWrapper_SetMaxTimeTillItem(obj: usize, new_val: i32);
    fn PRI_TA_Get_BoostPickups(obj: usize) -> i32;
    fn PriWrapper_SetBoostPickups(obj: usize, new_val: i32);
    fn PRI_TA_Get_BallTouches(obj: usize) -> i32;
    fn PriWrapper_SetBallTouches(obj: usize, new_val: i32);
    fn PRI_TA_Get_CarTouches(obj: usize) -> i32;
    fn PriWrapper_SetCarTouches(obj: usize, new_val: i32);
    fn PRI_TA_Get_ReplacingBotPRI(obj: usize) -> usize;
    fn PriWrapper_SetReplacingBotPRI(obj: usize, new_val: usize);
    fn PRI_TA_Get_ClubID(obj: usize) -> i64;
    fn PriWrapper_SetClubID(obj: usize, new_val: i64);
    fn PRI_TA_Get_PublicIP(obj: usize, result: *mut RLString);
    fn PRI_TA_Get_SpectatorShortcut(obj: usize) -> i32;
    fn PriWrapper_SetSpectatorShortcut(obj: usize, new_val: i32);
    fn PRI_TA___ClubID__ChangeNotifyFunc(obj: usize);
    fn PRI_TA___PREI__ChangeNotifyFunc(obj: usize);
    fn PRI_TA___ReplicatedWorstNetQualityBeyondLatency__ChangeNotifyFunc(obj: usize);
    fn PRI_TA_OnSpectatorShortcutChanged(obj: usize);
    fn PRI_TA_SetSpectatorShortcut2(obj: usize, InShortcut: i32);
    fn PRI_TA_ServerSetPublicIP(obj: usize, IP: *mut RLString);
    fn PRI_TA_OnUniqueIdChanged(obj: usize);
    fn PRI_TA_UpdatePlayerAvatarBorder(obj: usize);
    fn PRI_TA_UpdatePlayerBanner(obj: usize);
    fn PRI_TA_ClientAchievementProgression(obj: usize, AchievementId: i32, AchievementType: u8, Progress: f32, MaxProgress: f32);
    fn PRI_TA_ClientUnlockAchievement(obj: usize, AchievementId: i32, AchievementType: u8);
    fn PRI_TA_ServerSetPlayerActivatedFX(obj: usize, FX: usize);
    fn PRI_TA_UpdateCarLocalPlayer(obj: usize);
    fn PRI_TA_OnReplacingBotPRIChanged(obj: usize);
    fn PRI_TA_OnTeamChanged(obj: usize);
    fn PRI_TA_ClearBotReplacement(obj: usize);
    fn PRI_TA_ReportCheater(obj: usize, Reason: *mut RLString);
    fn PRI_TA_ValidateLoadoutTeamPaints(obj: usize) -> bool;
    fn PRI_TA_ValidateLoadoutSlots(obj: usize) -> bool;
    fn PRI_TA_ValidateLoadoutDLC(obj: usize);
    fn PRI_TA_OnStartVoteToForfeitDisabledChanged(obj: usize);
    fn PRI_TA_SetStartVoteToForfeitDisabled(obj: usize, bDisabled: bool);
    fn PRI_TA_ServerVoteToForfeit(obj: usize);
    fn PRI_TA_SetUserCarPreferences(obj: usize, NewDodgeThreshold: f32, NewSteeringSensitivity: f32, NewAirControlSensitivity: f32);
    fn PRI_TA_ServerSetUserCarPreferences(obj: usize, NewDodgeThreshold: f32, NewSteeringSensitivity: f32, NewAirControlSensitivity: f32);
    fn PRI_TA_ValidateUserInt(obj: usize, Reason: *mut RLString, NewValue: i32, Min: i32, Max: i32) -> i32;
    fn PRI_TA_ValidateUserFloat(obj: usize, Reason: *mut RLString, NewValue: f32, Min: f32, Max: f32) -> f32;
    fn PRI_TA_OnPawnTypeChanged(obj: usize);
    fn PRI_TA_SetWaitingPlayer(obj: usize, B: bool);
    fn PRI_TA_SetPawnType2(obj: usize, NewPawnType: u8);
    fn PRI_TA_IsPlayer(obj: usize) -> bool;
    fn PRI_TA_IsEditor(obj: usize) -> bool;
    fn PRI_TA_IsSpectator(obj: usize) -> bool;
    fn PRI_TA_ServerSpectate(obj: usize);
    fn PRI_TA_ClientChangeTeamFailed(obj: usize, TeamNum: i32);
    fn PRI_TA_ServerChangeTeam(obj: usize, TeamNum: i32);
    fn PRI_TA_ReplicatePodiumTitles(obj: usize);
    fn PRI_TA_OnMatchAdmin(obj: usize);
    fn PRI_TA_OnMVPChange(obj: usize);
    fn PRI_TA_NotifyWonMVP(obj: usize);
    fn PRI_TA_IsInvalidName(obj: usize) -> bool;
    fn PRI_TA_IsInWarmUpMode(obj: usize) -> bool;
    fn PRI_TA_SetReady(obj: usize, bNewReady: bool);
    fn PRI_TA_ServerMatchAdminSetPaused(obj: usize, bPause: bool);
    fn PRI_TA_ServerMatchAdminSetScoreAndTime(obj: usize, NewScoreTeam0: i32, NewScoreTeam1: i32, SecondsRemaining: i32, bOverTime: bool, bRestartRound: bool);
    fn PRI_TA_SetMatchAdmin(obj: usize, bIsMatchAdmin: bool);
    fn PRI_TA_ServerReadyUp(obj: usize);
    fn PRI_TA_ShouldBroadCastWelcomeMessage(obj: usize, bExiting: bool) -> bool;
    fn PRI_TA_UpdateOnlineProductStats(obj: usize);
    fn PRI_TA_SetWaitingStartTime2(obj: usize);
    fn PRI_TA_ClientScoredGoal(obj: usize, BallHitLocation: *mut Vector);
    fn PRI_TA_OnScoredGoal(obj: usize, BallHitLocation: *mut Vector);
    fn PRI_TA_OnRep_SteeringSensitivity(obj: usize);
    fn PRI_TA_OnRep_ClientScorePoint(obj: usize);
    fn PRI_TA_ResetScore(obj: usize);
    fn PRI_TA_RemovePoints(obj: usize, Points: i32);
    fn PRI_TA_ScorePoint(obj: usize, AdditionalScore: i32);
    fn PRI_TA_GetMatchXP(obj: usize) -> i32;
    fn PRI_TA_CommitStats(obj: usize);
    fn PRI_TA_UpdateFromLoadout(obj: usize);
    fn PRI_TA_UpdateUserCarPreferences(obj: usize, AirControlComponent: usize);
    fn PRI_TA_HandleAirControl(obj: usize, AirControlComp: usize);
    fn PRI_TA_SetCar2(obj: usize, NewCar: usize);
    fn PRI_TA_IsClientPlayerPRI(obj: usize) -> bool;
    fn PRI_TA_IsLocalPlayerPRI(obj: usize) -> bool;
    fn PRI_TA_Unregister(obj: usize);
    fn PRI_TA_AddLocalPlayerToGameEvent(obj: usize);
    fn PRI_TA_OnLoadoutsSetInternal(obj: usize);
    fn PRI_TA_AreLoadoutsSet(obj: usize) -> bool;
    fn PRI_TA_OnLoadoutsOnlineSet(obj: usize);
    fn PRI_TA_OnLoadoutsSet(obj: usize);
    fn PRI_TA_RemoveCertifiedProductStat(obj: usize, InstanceID: i64);
    fn PRI_TA_InitLoadoutAttributesForTeam(obj: usize, PRI: usize);
    fn PRI_TA_ShouldValidateOnlineProducts(obj: usize) -> bool;
    fn PRI_TA_OnSplitScreenStatusChanged(obj: usize);
    fn PRI_TA_ServerSplitScreenStatusChanged(obj: usize, bInSplitScreen: bool);
    fn PRI_TA_UpdateSplitScreenStatus(obj: usize);
    fn PRI_TA_OnPartyLeaderChanged(obj: usize);
    fn PRI_TA_SetPartyLeader2(obj: usize, InPartyLeader: *mut UniqueNetId);
    fn PRI_TA_UpdatePartyStatus(obj: usize);
    fn PRI_TA_OnSkillTierChanged(obj: usize);
    fn PRI_TA_ReplicateSkillTier(obj: usize, NewTier: i32);
    fn PRI_TA_OnTitleChanged(obj: usize);
    fn PRI_TA_SyncPlayerTitle(obj: usize);
    fn PRI_TA_UpdateTitleFromLoadout(obj: usize);
    fn PRI_TA_UpdateTitle(obj: usize);
    fn PRI_TA_GetNewFriendKey(obj: usize);
    fn PRI_TA_OnRep_UniqueId(obj: usize);
    fn PRI_TA_EventOwnerChanged(obj: usize, PRI: usize);
    fn PRI_TA_EventSpectatorShortcutChanged(obj: usize, PRI: usize);
    fn PRI_TA_EventServerAchievementProgression(obj: usize, PRI: usize, AchievementId: i32, AchievementType: u8, Progress: f32, MaxProgress: f32);
    fn PRI_TA_EventStartVoteToForfeitDisabledChanged(obj: usize, PRI: usize);
    fn PRI_TA_EventCarPreUpdate(obj: usize, PRI: usize);

}