use crate::wrappers::*;
use crate::generated::*;

pub struct TeamWrapper(pub usize);
impl_object!(TeamWrapper);

impl Team for TeamWrapper {}
impl TeamInfo for TeamWrapper {}
impl Actor for TeamWrapper {}

pub trait Team : TeamInfo {
    fn get_font_color(&self) -> LinearColor {
        unsafe {
            let mut result = LinearColor::new();
            let result_ptr: *mut LinearColor = &mut result as *mut LinearColor;
            Team_TA_Get_FontColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_color_blind_font_color(&self) -> LinearColor {
        unsafe {
            let mut result = LinearColor::new();
            let result_ptr: *mut LinearColor = &mut result as *mut LinearColor;
            Team_TA_Get_ColorBlindFontColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_team_controller_color(&self) -> Color {
        unsafe {
            let mut result = Color::new();
            let result_ptr: *mut Color = &mut result as *mut Color;
            Team_TA_Get_TeamControllerColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_team_score_strobe_color(&self) -> Color {
        unsafe {
            let mut result = Color::new();
            let result_ptr: *mut Color = &mut result as *mut Color;
            Team_TA_Get_TeamScoreStrobeColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_default_color_list(&self) -> RLArray<LinearColor> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            Team_TA_Get_DefaultColorList(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_color_blind_color_list(&self) -> RLArray<LinearColor> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            Team_TA_Get_ColorBlindColorList(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_current_color_list(&self) -> RLArray<LinearColor> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            Team_TA_Get_CurrentColorList(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_game_event(&self) -> Option<TeamGameEventWrapper> {
        unsafe {
            TeamGameEventWrapper::try_new(Team_TA_Get_GameEvent(self.addr()))
        }
    }
    fn get_members(&self) -> RLArray<PriWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            Team_TA_Get_Members(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_custom_team_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Team_TA_Get_CustomTeamName(self.addr(), result_ptr);
            result
        }
    }
    fn get_sanitized_team_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Team_TA_Get_SanitizedTeamName(self.addr(), result_ptr);
            result
        }
    }
    fn get_club_id(&self) -> i64 {
        unsafe {
            Team_TA_Get_ClubID(self.addr())
        }
    }
    fn get_b_forfeit(&self) -> bool {
        unsafe {
            Team_TA_Get_bForfeit(self.addr())
        }
    }
    fn on_club_colors_changed(&self) {
        unsafe {
            Team_TA_OnClubColorsChanged(self.addr());
        }
    }
    fn forfeit(&self) {
        unsafe {
            Team_TA_Forfeit(self.addr());
        }
    }
    fn enable_all_members_start_vote_to_forfeit(&self) {
        unsafe {
            Team_TA_EnableAllMembersStartVoteToForfeit(self.addr());
        }
    }
    fn enable_all_members_start_vote_to_forfeit_if_necessary(&self) {
        unsafe {
            Team_TA_EnableAllMembersStartVoteToForfeitIfNecessary(self.addr());
        }
    }
    fn notify_kismet_team_color_changed(&self) {
        unsafe {
            Team_TA_NotifyKismetTeamColorChanged(self.addr());
        }
    }
    fn update_colors(&self) {
        unsafe {
            Team_TA_UpdateColors(self.addr());
        }
    }
    fn set_logo(&self, logo_id: i32, b_swap_colors: bool) {
        unsafe {
            Team_TA_SetLogo(self.addr(), logo_id, b_swap_colors);
        }
    }
    fn handle_team_name_sanitized(&self, original: RLString, sanitized: RLString) {
        unsafe {
            let mut original = original;
            let original: *mut RLString = &mut original as *mut RLString;
            let mut sanitized = sanitized;
            let sanitized: *mut RLString = &mut sanitized as *mut RLString;
            Team_TA_HandleTeamNameSanitized(self.addr(), original, sanitized);
        }
    }
    fn set_club_id2(&self, in_club_id: i64) {
        unsafe {
            Team_TA_SetClubID2(self.addr(), in_club_id);
        }
    }
    fn set_custom_team_name(&self, new_name: RLString) {
        unsafe {
            let mut new_name = new_name;
            let new_name: *mut RLString = &mut new_name as *mut RLString;
            Team_TA_SetCustomTeamName(self.addr(), new_name);
        }
    }
    fn set_default_colors(&self) {
        unsafe {
            Team_TA_SetDefaultColors(self.addr());
        }
    }
    fn is_single_party(&self) -> bool {
        unsafe {
            Team_TA_IsSingleParty(self.addr())
        }
    }
    fn get_team_member_named(&self, player_name: RLString) -> Option<PriWrapper> {
        unsafe {
            let mut player_name = player_name;
            let player_name: *mut RLString = &mut player_name as *mut RLString;
            PriWrapper::try_new(Team_TA_GetTeamMemberNamed(self.addr(), player_name))
        }
    }
    fn get_num_bots(&self) -> i32 {
        unsafe {
            Team_TA_GetNumBots(self.addr())
        }
    }
    fn get_num_humans(&self) -> i32 {
        unsafe {
            Team_TA_GetNumHumans(self.addr())
        }
    }
    fn on_score_updated(&self) {
        unsafe {
            Team_TA_OnScoreUpdated(self.addr());
        }
    }
    fn reset_score(&self) {
        unsafe {
            Team_TA_ResetScore(self.addr());
        }
    }
    fn remove_points(&self, points: i32) {
        unsafe {
            Team_TA_RemovePoints(self.addr(), points);
        }
    }
    fn set_score(&self, points: i32) {
        unsafe {
            Team_TA_SetScore(self.addr(), points);
        }
    }
    fn score_point(&self, additional_score: i32) {
        unsafe {
            Team_TA_ScorePoint(self.addr(), additional_score);
        }
    }
    fn mute_other_team(&self, other_team: TeamWrapper, b_mute: bool) {
        unsafe {
            Team_TA_MuteOtherTeam(self.addr(), other_team.addr(), b_mute);
        }
    }
    fn clear_temporary_spawn_spots(&self) {
        unsafe {
            Team_TA_ClearTemporarySpawnSpots(self.addr());
        }
    }
    fn expire_temporary_spawn_spots(&self) {
        unsafe {
            Team_TA_ExpireTemporarySpawnSpots(self.addr());
        }
    }
    fn add_temporary_spawn_spot(&self, at_actor: ActorWrapper) {
        unsafe {
            Team_TA_AddTemporarySpawnSpot(self.addr(), at_actor.addr());
        }
    }
    fn on_game_event_set(&self) {
        unsafe {
            Team_TA_OnGameEventSet(self.addr());
        }
    }
    fn set_game_event2(&self, in_game_event: TeamGameEventWrapper) {
        unsafe {
            Team_TA_SetGameEvent2(self.addr(), in_game_event.addr());
        }
    }

}

extern "C" {
    fn Team_TA_Get_FontColor(obj: usize, result: *mut LinearColor);
    fn TeamWrapper_SetFontColor(obj: usize, new_val: *mut LinearColor);
    fn Team_TA_Get_ColorBlindFontColor(obj: usize, result: *mut LinearColor);
    fn TeamWrapper_SetColorBlindFontColor(obj: usize, new_val: *mut LinearColor);
    fn Team_TA_Get_TeamControllerColor(obj: usize, result: *mut Color);
    fn TeamWrapper_SetTeamControllerColor(obj: usize, new_val: *mut Color);
    fn Team_TA_Get_TeamScoreStrobeColor(obj: usize, result: *mut Color);
    fn TeamWrapper_SetTeamScoreStrobeColor(obj: usize, new_val: *mut Color);
    fn Team_TA_Get_DefaultColorList(obj: usize, result: *mut RLArrayRaw);
    fn Team_TA_Get_ColorBlindColorList(obj: usize, result: *mut RLArrayRaw);
    fn Team_TA_Get_CurrentColorList(obj: usize, result: *mut RLArrayRaw);
    fn Team_TA_Get_GameEvent(obj: usize) -> usize;
    fn TeamWrapper_SetGameEvent(obj: usize, new_val: usize);
    fn Team_TA_Get_Members(obj: usize, result: *mut RLArrayRaw);
    fn Team_TA_Get_CustomTeamName(obj: usize, result: *mut RLString);
    fn Team_TA_Get_SanitizedTeamName(obj: usize, result: *mut RLString);
    fn Team_TA_Get_ClubID(obj: usize) -> i64;
    fn TeamWrapper_SetClubID(obj: usize, new_val: i64);
    fn Team_TA_Get_bForfeit(obj: usize) -> bool;
    fn TeamWrapper_SetbForfeit(obj: usize, new_val: bool);
    fn Team_TA_OnClubColorsChanged(obj: usize);
    fn Team_TA_Forfeit(obj: usize);
    fn Team_TA_EnableAllMembersStartVoteToForfeit(obj: usize);
    fn Team_TA_EnableAllMembersStartVoteToForfeitIfNecessary(obj: usize);
    fn Team_TA_NotifyKismetTeamColorChanged(obj: usize);
    fn Team_TA_UpdateColors(obj: usize);
    fn Team_TA_SetLogo(obj: usize, LogoID: i32, bSwapColors: bool);
    fn Team_TA_HandleTeamNameSanitized(obj: usize, Original: *mut RLString, Sanitized: *mut RLString);
    fn Team_TA_SetClubID2(obj: usize, InClubID: i64);
    fn Team_TA_SetCustomTeamName(obj: usize, NewName: *mut RLString);
    fn Team_TA_SetDefaultColors(obj: usize);
    fn Team_TA_IsSingleParty(obj: usize) -> bool;
    fn Team_TA_GetTeamMemberNamed(obj: usize, PlayerName: *mut RLString) -> usize;
    fn Team_TA_GetNumBots(obj: usize) -> i32;
    fn Team_TA_GetNumHumans(obj: usize) -> i32;
    fn Team_TA_OnScoreUpdated(obj: usize);
    fn Team_TA_ResetScore(obj: usize);
    fn Team_TA_RemovePoints(obj: usize, Points: i32);
    fn Team_TA_SetScore(obj: usize, Points: i32);
    fn Team_TA_ScorePoint(obj: usize, AdditionalScore: i32);
    fn Team_TA_MuteOtherTeam(obj: usize, OtherTeam: usize, bMute: bool);
    fn Team_TA_ClearTemporarySpawnSpots(obj: usize);
    fn Team_TA_ExpireTemporarySpawnSpots(obj: usize);
    fn Team_TA_AddTemporarySpawnSpot(obj: usize, AtActor: usize);
    fn Team_TA_OnGameEventSet(obj: usize);
    fn Team_TA_SetGameEvent2(obj: usize, InGameEvent: usize);

}