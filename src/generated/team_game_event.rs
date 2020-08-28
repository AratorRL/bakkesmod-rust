use crate::wrappers::*;
use crate::generated::*;

pub struct TeamGameEventWrapper(pub usize);
impl_object!(TeamGameEventWrapper);

impl TeamGameEvent for TeamGameEventWrapper {}
impl GameEvent for TeamGameEventWrapper {}
impl Actor for TeamGameEventWrapper {}

pub trait TeamGameEvent : GameEvent {
    fn get_team_archetypes(&self) -> RLArray<TeamWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            GameEvent_Team_TA_Get_TeamArchetypes(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_teams(&self) -> RLArray<TeamWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            GameEvent_Team_TA_Get_Teams(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_max_team_size(&self) -> i32 {
        unsafe {
            GameEvent_Team_TA_Get_MaxTeamSize(self.addr())
        }
    }
    fn get_num_bots(&self) -> i32 {
        unsafe {
            GameEvent_Team_TA_Get_NumBots(self.addr())
        }
    }
    fn get_b_mute_opposite_teams(&self) -> bool {
        unsafe {
            GameEvent_Team_TA_Get_bMuteOppositeTeams(self.addr())
        }
    }
    fn get_b_disable_muting_other_team(&self) -> bool {
        unsafe {
            GameEvent_Team_TA_Get_bDisableMutingOtherTeam(self.addr())
        }
    }
    fn get_b_forfeit(&self) -> bool {
        unsafe {
            GameEvent_Team_TA_Get_bForfeit(self.addr())
        }
    }
    fn get_b_unfair_teams(&self) -> bool {
        unsafe {
            GameEvent_Team_TA_Get_bUnfairTeams(self.addr())
        }
    }
    fn get_b_always_auto_select_team(&self) -> bool {
        unsafe {
            GameEvent_Team_TA_Get_bAlwaysAutoSelectTeam(self.addr())
        }
    }
    fn start_rematch_vote(&self) {
        unsafe {
            GameEvent_Team_TA_StartRematchVote(self.addr());
        }
    }
    fn check_rematch_vote(&self) {
        unsafe {
            GameEvent_Team_TA_CheckRematchVote(self.addr());
        }
    }
    fn update_player_shortcuts(&self) {
        unsafe {
            GameEvent_Team_TA_UpdatePlayerShortcuts(self.addr());
        }
    }
    fn clear_temporary_spawn_spots(&self) {
        unsafe {
            GameEvent_Team_TA_ClearTemporarySpawnSpots(self.addr());
        }
    }
    fn choose_team(&self, team_index: i32, player: PlayerControllerWrapper) -> bool {
        unsafe {
            GameEvent_Team_TA_ChooseTeam(self.addr(), team_index, player.addr())
        }
    }
    fn can_change_team(&self) -> bool {
        unsafe {
            GameEvent_Team_TA_CanChangeTeam(self.addr())
        }
    }
    fn on_penalty_changed(&self) {
        unsafe {
            GameEvent_Team_TA_OnPenaltyChanged(self.addr());
        }
    }
    fn on_team_forfeited(&self, team: TeamWrapper) {
        unsafe {
            GameEvent_Team_TA_OnTeamForfeited(self.addr(), team.addr());
        }
    }
    fn handle_team_forfeit(&self, team: TeamWrapper) {
        unsafe {
            GameEvent_Team_TA_HandleTeamForfeit(self.addr(), team.addr());
        }
    }
    fn mute_opposite_teams(&self, b_mute: bool) {
        unsafe {
            GameEvent_Team_TA_MuteOppositeTeams(self.addr(), b_mute);
        }
    }
    fn update_bot_count(&self) {
        unsafe {
            GameEvent_Team_TA_UpdateBotCount(self.addr());
        }
    }
    fn find_bot_replacement(&self, pri: PriWrapper) -> bool {
        unsafe {
            GameEvent_Team_TA_FindBotReplacement(self.addr(), pri.addr())
        }
    }
    fn end_game(&self) {
        unsafe {
            GameEvent_Team_TA_EndGame(self.addr());
        }
    }
    fn force_no_contest(&self) {
        unsafe {
            GameEvent_Team_TA_ForceNoContest(self.addr());
        }
    }
    fn add_temporary_spawn_spot(&self, team: TeamWrapper, car: CarWrapper) {
        unsafe {
            GameEvent_Team_TA_AddTemporarySpawnSpot(self.addr(), team.addr(), car.addr());
        }
    }
    fn handle_selected_loadout(&self, player_pri: PriWrapper) {
        unsafe {
            GameEvent_Team_TA_HandleSelectedLoadout(self.addr(), player_pri.addr());
        }
    }
    fn destroy_teams(&self) {
        unsafe {
            GameEvent_Team_TA_DestroyTeams(self.addr());
        }
    }
    fn on_all_teams_created(&self) {
        unsafe {
            GameEvent_Team_TA_OnAllTeamsCreated(self.addr());
        }
    }
    fn assign_custom_team_settings(&self) {
        unsafe {
            GameEvent_Team_TA_AssignCustomTeamSettings(self.addr());
        }
    }
    fn on_match_settings_changed(&self) {
        unsafe {
            GameEvent_Team_TA_OnMatchSettingsChanged(self.addr());
        }
    }
    fn all_teams_have_humans(&self) -> bool {
        unsafe {
            GameEvent_Team_TA_AllTeamsHaveHumans(self.addr())
        }
    }
    fn all_teams_created(&self) -> bool {
        unsafe {
            GameEvent_Team_TA_AllTeamsCreated(self.addr())
        }
    }
    fn set_team(&self, team_num: i32, new_team: TeamWrapper) {
        unsafe {
            GameEvent_Team_TA_SetTeam(self.addr(), team_num, new_team.addr());
        }
    }
    fn create_teams(&self) {
        unsafe {
            GameEvent_Team_TA_CreateTeams(self.addr());
        }
    }
    fn preload_bots(&self) {
        unsafe {
            GameEvent_Team_TA_PreloadBots(self.addr());
        }
    }
    fn on_init(&self) {
        unsafe {
            GameEvent_Team_TA_OnInit(self.addr());
        }
    }
    fn set_max_team_size2(&self, max_size: i32) {
        unsafe {
            GameEvent_Team_TA_SetMaxTeamSize2(self.addr(), max_size);
        }
    }
    fn update_max_team_size(&self) {
        unsafe {
            GameEvent_Team_TA_UpdateMaxTeamSize(self.addr());
        }
    }
    fn set_unfair_teams(&self, b_unfair: bool) {
        unsafe {
            GameEvent_Team_TA_SetUnfairTeams(self.addr(), b_unfair);
        }
    }
    fn init_bot_skill(&self) {
        unsafe {
            GameEvent_Team_TA_InitBotSkill(self.addr());
        }
    }

}

extern "C" {
    fn GameEvent_Team_TA_Get_TeamArchetypes(obj: usize, result: *mut RLArrayRaw);
    fn GameEvent_Team_TA_Get_Teams(obj: usize, result: *mut RLArrayRaw);
    fn GameEvent_Team_TA_Get_MaxTeamSize(obj: usize) -> i32;
    fn TeamGameEventWrapper_SetMaxTeamSize(obj: usize, new_val: i32);
    fn GameEvent_Team_TA_Get_NumBots(obj: usize) -> i32;
    fn TeamGameEventWrapper_SetNumBots(obj: usize, new_val: i32);
    fn GameEvent_Team_TA_Get_bMuteOppositeTeams(obj: usize) -> bool;
    fn TeamGameEventWrapper_SetbMuteOppositeTeams(obj: usize, new_val: bool);
    fn GameEvent_Team_TA_Get_bDisableMutingOtherTeam(obj: usize) -> bool;
    fn TeamGameEventWrapper_SetbDisableMutingOtherTeam(obj: usize, new_val: bool);
    fn GameEvent_Team_TA_Get_bForfeit(obj: usize) -> bool;
    fn TeamGameEventWrapper_SetbForfeit(obj: usize, new_val: bool);
    fn GameEvent_Team_TA_Get_bUnfairTeams(obj: usize) -> bool;
    fn TeamGameEventWrapper_SetbUnfairTeams(obj: usize, new_val: bool);
    fn GameEvent_Team_TA_Get_bAlwaysAutoSelectTeam(obj: usize) -> bool;
    fn TeamGameEventWrapper_SetbAlwaysAutoSelectTeam(obj: usize, new_val: bool);
    fn GameEvent_Team_TA_StartRematchVote(obj: usize);
    fn GameEvent_Team_TA_CheckRematchVote(obj: usize);
    fn GameEvent_Team_TA_UpdatePlayerShortcuts(obj: usize);
    fn GameEvent_Team_TA_ClearTemporarySpawnSpots(obj: usize);
    fn GameEvent_Team_TA_ChooseTeam(obj: usize, TeamIndex: i32, Player: usize) -> bool;
    fn GameEvent_Team_TA_CanChangeTeam(obj: usize) -> bool;
    fn GameEvent_Team_TA_OnPenaltyChanged(obj: usize);
    fn GameEvent_Team_TA_OnTeamForfeited(obj: usize, Team: usize);
    fn GameEvent_Team_TA_HandleTeamForfeit(obj: usize, Team: usize);
    fn GameEvent_Team_TA_MuteOppositeTeams(obj: usize, bMute: bool);
    fn GameEvent_Team_TA_UpdateBotCount(obj: usize);
    fn GameEvent_Team_TA_FindBotReplacement(obj: usize, PRI: usize) -> bool;
    fn GameEvent_Team_TA_EndGame(obj: usize);
    fn GameEvent_Team_TA_ForceNoContest(obj: usize);
    fn GameEvent_Team_TA_AddTemporarySpawnSpot(obj: usize, Team: usize, Car: usize);
    fn GameEvent_Team_TA_HandleSelectedLoadout(obj: usize, PlayerPRI: usize);
    fn GameEvent_Team_TA_DestroyTeams(obj: usize);
    fn GameEvent_Team_TA_OnAllTeamsCreated(obj: usize);
    fn GameEvent_Team_TA_AssignCustomTeamSettings(obj: usize);
    fn GameEvent_Team_TA_OnMatchSettingsChanged(obj: usize);
    fn GameEvent_Team_TA_AllTeamsHaveHumans(obj: usize) -> bool;
    fn GameEvent_Team_TA_AllTeamsCreated(obj: usize) -> bool;
    fn GameEvent_Team_TA_SetTeam(obj: usize, TeamNum: i32, NewTeam: usize);
    fn GameEvent_Team_TA_CreateTeams(obj: usize);
    fn GameEvent_Team_TA_PreloadBots(obj: usize);
    fn GameEvent_Team_TA_OnInit(obj: usize);
    fn GameEvent_Team_TA_SetMaxTeamSize2(obj: usize, MaxSize: i32);
    fn GameEvent_Team_TA_UpdateMaxTeamSize(obj: usize);
    fn GameEvent_Team_TA_SetUnfairTeams(obj: usize, bUnfair: bool);
    fn GameEvent_Team_TA_InitBotSkill(obj: usize);

}