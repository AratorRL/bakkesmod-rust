use crate::wrappers::*;
use super::*;

pub struct PlayerReplicationInfoWrapper(pub usize);
impl_object!(PlayerReplicationInfoWrapper);

impl PlayerReplicationInfo for PlayerReplicationInfoWrapper {}
impl Actor for PlayerReplicationInfoWrapper {}

pub trait PlayerReplicationInfo : Actor {
    fn get_score(&self) -> i32 {
        unsafe {
            PlayerReplicationInfo_Get_Score(self.addr())
        }
    }
    fn get_deaths(&self) -> i32 {
        unsafe {
            PlayerReplicationInfo_Get_Deaths(self.addr())
        }
    }
    fn get_ping(&self) -> u8 {
        unsafe {
            PlayerReplicationInfo_Get_Ping(self.addr())
        }
    }
    fn get_tts_speaker(&self) -> u8 {
        unsafe {
            PlayerReplicationInfo_Get_TTSSpeaker(self.addr())
        }
    }
    fn get_num_lives(&self) -> i32 {
        unsafe {
            PlayerReplicationInfo_Get_NumLives(self.addr())
        }
    }
    fn get_player_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            PlayerReplicationInfo_Get_PlayerName(self.addr(), result_ptr);
            result
        }
    }
    fn get_old_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            PlayerReplicationInfo_Get_OldName(self.addr(), result_ptr);
            result
        }
    }
    fn get_player_id(&self) -> i32 {
        unsafe {
            PlayerReplicationInfo_Get_PlayerID(self.addr())
        }
    }
    fn get_team(&self) -> Option<TeamInfoWrapper> {
        unsafe {
            TeamInfoWrapper::try_new(PlayerReplicationInfo_Get_Team(self.addr()))
        }
    }
    fn get_b_admin(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bAdmin(self.addr())
        }
    }
    fn get_b_is_spectator(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bIsSpectator(self.addr())
        }
    }
    fn get_b_only_spectator(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bOnlySpectator(self.addr())
        }
    }
    fn get_b_waiting_player(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bWaitingPlayer(self.addr())
        }
    }
    fn get_b_ready_to_play(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bReadyToPlay(self.addr())
        }
    }
    fn get_b_out_of_lives(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bOutOfLives(self.addr())
        }
    }
    fn get_b_bot(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bBot(self.addr())
        }
    }
    fn get_b_is_inactive(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bIsInactive(self.addr())
        }
    }
    fn get_b_from_previous_level(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bFromPreviousLevel(self.addr())
        }
    }
    fn get_b_timed_out(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bTimedOut(self.addr())
        }
    }
    fn get_b_unregistered(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_Get_bUnregistered(self.addr())
        }
    }
    fn get_start_time(&self) -> i32 {
        unsafe {
            PlayerReplicationInfo_Get_StartTime(self.addr())
        }
    }
    fn get_string_spectating(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            PlayerReplicationInfo_Get_StringSpectating(self.addr(), result_ptr);
            result
        }
    }
    fn get_string_unknown(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            PlayerReplicationInfo_Get_StringUnknown(self.addr(), result_ptr);
            result
        }
    }
    fn get_kills(&self) -> i32 {
        unsafe {
            PlayerReplicationInfo_Get_Kills(self.addr())
        }
    }
    fn get_exact_ping(&self) -> f32 {
        unsafe {
            PlayerReplicationInfo_Get_ExactPing(self.addr())
        }
    }
    fn get_saved_network_address(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            PlayerReplicationInfo_Get_SavedNetworkAddress(self.addr(), result_ptr);
            result
        }
    }
    fn get_unique_id(&self) -> UniqueNetId {
        unsafe {
            let mut result = UniqueNetId::new();
            let result_ptr: *mut UniqueNetId = &mut result as *mut UniqueNetId;
            PlayerReplicationInfo_Get_UniqueId(self.addr(), result_ptr);
            result
        }
    }
    fn unregister_player_from_session(&self) {
        unsafe {
            PlayerReplicationInfo_UnregisterPlayerFromSession(self.addr());
        }
    }
    fn register_player_with_session(&self) {
        unsafe {
            PlayerReplicationInfo_RegisterPlayerWithSession(self.addr());
        }
    }
    fn is_invalid_name(&self) -> bool {
        unsafe {
            PlayerReplicationInfo_IsInvalidName(self.addr())
        }
    }
    fn get_team_num(&self) -> u8 {
        unsafe {
            PlayerReplicationInfo_GetTeamNum(self.addr())
        }
    }
    fn set_unique_id2(&self, player_unique_id: UniqueNetId) {
        unsafe {
            let mut player_unique_id = player_unique_id;
            let player_unique_id: *mut UniqueNetId = &mut player_unique_id as *mut UniqueNetId;
            PlayerReplicationInfo_SetUniqueId2(self.addr(), player_unique_id);
        }
    }
    fn seamless_travel_to(&self, new_pri: PlayerReplicationInfoWrapper) {
        unsafe {
            PlayerReplicationInfo_SeamlessTravelTo(self.addr(), new_pri.addr());
        }
    }
    fn increment_deaths(&self, amt: i32) {
        unsafe {
            PlayerReplicationInfo_IncrementDeaths(self.addr(), amt);
        }
    }
    fn copy_properties(&self, pri: PlayerReplicationInfoWrapper) {
        unsafe {
            PlayerReplicationInfo_CopyProperties(self.addr(), pri.addr());
        }
    }
    fn override_with(&self, pri: PlayerReplicationInfoWrapper) {
        unsafe {
            PlayerReplicationInfo_OverrideWith(self.addr(), pri.addr());
        }
    }
    fn duplicate(&self) -> Option<PlayerReplicationInfoWrapper> {
        unsafe {
            PlayerReplicationInfoWrapper::try_new(PlayerReplicationInfo_Duplicate(self.addr()))
        }
    }
    fn set_waiting_player(&self, b: bool) {
        unsafe {
            PlayerReplicationInfo_SetWaitingPlayer(self.addr(), b);
        }
    }
    fn reset(&self) {
        unsafe {
            PlayerReplicationInfo_Reset(self.addr());
        }
    }
    fn unregister(&self) {
        unsafe {
            PlayerReplicationInfo_Unregister(self.addr());
        }
    }
    fn update_ping(&self, time_stamp: f32) {
        unsafe {
            PlayerReplicationInfo_UpdatePing(self.addr(), time_stamp);
        }
    }
    fn remote_user_data_replicated(&self) {
        unsafe {
            PlayerReplicationInfo_RemoteUserDataReplicated(self.addr());
        }
    }
    fn set_player_team(&self, new_team: TeamInfoWrapper) {
        unsafe {
            PlayerReplicationInfo_SetPlayerTeam(self.addr(), new_team.addr());
        }
    }

}

extern "C" {
    fn PlayerReplicationInfo_Get_Score(obj: usize) -> i32;
    fn PlayerReplicationInfoWrapper_SetScore(obj: usize, new_val: i32);
    fn PlayerReplicationInfo_Get_Deaths(obj: usize) -> i32;
    fn PlayerReplicationInfoWrapper_SetDeaths(obj: usize, new_val: i32);
    fn PlayerReplicationInfo_Get_Ping(obj: usize) -> u8;
    fn PlayerReplicationInfoWrapper_SetPing(obj: usize, new_val: u8);
    fn PlayerReplicationInfo_Get_TTSSpeaker(obj: usize) -> u8;
    fn PlayerReplicationInfoWrapper_SetTTSSpeaker(obj: usize, new_val: u8);
    fn PlayerReplicationInfo_Get_NumLives(obj: usize) -> i32;
    fn PlayerReplicationInfoWrapper_SetNumLives(obj: usize, new_val: i32);
    fn PlayerReplicationInfo_Get_PlayerName(obj: usize, result: *mut RLString);
    fn PlayerReplicationInfo_Get_OldName(obj: usize, result: *mut RLString);
    fn PlayerReplicationInfo_Get_PlayerID(obj: usize) -> i32;
    fn PlayerReplicationInfoWrapper_SetPlayerID(obj: usize, new_val: i32);
    fn PlayerReplicationInfo_Get_Team(obj: usize) -> usize;
    fn PlayerReplicationInfoWrapper_SetTeam(obj: usize, new_val: usize);
    fn PlayerReplicationInfo_Get_bAdmin(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbAdmin(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bIsSpectator(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbIsSpectator(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bOnlySpectator(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbOnlySpectator(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bWaitingPlayer(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbWaitingPlayer(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bReadyToPlay(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbReadyToPlay(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bOutOfLives(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbOutOfLives(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bBot(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbBot(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bIsInactive(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbIsInactive(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bFromPreviousLevel(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbFromPreviousLevel(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bTimedOut(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbTimedOut(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_bUnregistered(obj: usize) -> bool;
    fn PlayerReplicationInfoWrapper_SetbUnregistered(obj: usize, new_val: bool);
    fn PlayerReplicationInfo_Get_StartTime(obj: usize) -> i32;
    fn PlayerReplicationInfoWrapper_SetStartTime(obj: usize, new_val: i32);
    fn PlayerReplicationInfo_Get_StringSpectating(obj: usize, result: *mut RLString);
    fn PlayerReplicationInfo_Get_StringUnknown(obj: usize, result: *mut RLString);
    fn PlayerReplicationInfo_Get_Kills(obj: usize) -> i32;
    fn PlayerReplicationInfoWrapper_SetKills(obj: usize, new_val: i32);
    fn PlayerReplicationInfo_Get_ExactPing(obj: usize) -> f32;
    fn PlayerReplicationInfoWrapper_SetExactPing(obj: usize, new_val: f32);
    fn PlayerReplicationInfo_Get_SavedNetworkAddress(obj: usize, result: *mut RLString);
    fn PlayerReplicationInfo_Get_UniqueId(obj: usize, result: *mut UniqueNetId);
    fn PlayerReplicationInfoWrapper_SetUniqueId(obj: usize, new_val: *mut UniqueNetId);
    fn PlayerReplicationInfo_UnregisterPlayerFromSession(obj: usize);
    fn PlayerReplicationInfo_RegisterPlayerWithSession(obj: usize);
    fn PlayerReplicationInfo_IsInvalidName(obj: usize) -> bool;
    fn PlayerReplicationInfo_GetTeamNum(obj: usize) -> u8;
    fn PlayerReplicationInfo_SetUniqueId2(obj: usize, PlayerUniqueId: *mut UniqueNetId);
    fn PlayerReplicationInfo_SeamlessTravelTo(obj: usize, NewPRI: usize);
    fn PlayerReplicationInfo_IncrementDeaths(obj: usize, Amt: i32);
    fn PlayerReplicationInfo_CopyProperties(obj: usize, PRI: usize);
    fn PlayerReplicationInfo_OverrideWith(obj: usize, PRI: usize);
    fn PlayerReplicationInfo_Duplicate(obj: usize) -> usize;
    fn PlayerReplicationInfo_SetWaitingPlayer(obj: usize, B: bool);
    fn PlayerReplicationInfo_Reset(obj: usize);
    fn PlayerReplicationInfo_Unregister(obj: usize);
    fn PlayerReplicationInfo_UpdatePing(obj: usize, TimeStamp: f32);
    fn PlayerReplicationInfo_RemoteUserDataReplicated(obj: usize);
    fn PlayerReplicationInfo_SetPlayerTeam(obj: usize, NewTeam: usize);

}