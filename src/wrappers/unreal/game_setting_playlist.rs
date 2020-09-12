use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct GameSettingPlaylistWrapper(pub usize);
impl_object!(GameSettingPlaylistWrapper);

impl GameSettingPlaylist for GameSettingPlaylistWrapper {}

pub trait GameSettingPlaylist : Object {
    fn get_title(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            GameSettingPlaylist_X_Get_Title(self.addr(), result_ptr);
            result
        }
    }
    fn get_description(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            GameSettingPlaylist_X_Get_Description(self.addr(), result_ptr);
            result
        }
    }
    fn get_player_count(&self) -> i32 {
        unsafe {
            GameSettingPlaylist_X_Get_PlayerCount(self.addr())
        }
    }
    fn get_b_standard(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bStandard(self.addr())
        }
    }
    fn get_b_ranked(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bRanked(self.addr())
        }
    }
    fn get_b_solo(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bSolo(self.addr())
        }
    }
    fn get_b_new(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bNew(self.addr())
        }
    }
    fn get_b_apply_quit_penalty(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bApplyQuitPenalty(self.addr())
        }
    }
    fn get_b_allow_forfeit(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bAllowForfeit(self.addr())
        }
    }
    fn get_b_disable_ranked_reconnect(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bDisableRankedReconnect(self.addr())
        }
    }
    fn get_b_ignore_assign_teams(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bIgnoreAssignTeams(self.addr())
        }
    }
    fn get_b_kick_on_migrate(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bKickOnMigrate(self.addr())
        }
    }
    fn get_b_allow_clubs(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bAllowClubs(self.addr())
        }
    }
    fn get_b_players_vs_bots(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_Get_bPlayersVSBots(self.addr())
        }
    }
    fn get_playlist_id(&self) -> i32 {
        unsafe {
            GameSettingPlaylist_X_Get_PlaylistId(self.addr())
        }
    }
    fn get_server_command(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            GameSettingPlaylist_X_Get_ServerCommand(self.addr(), result_ptr);
            result
        }
    }
    fn is_lan_match(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_IsLanMatch(self.addr())
        }
    }
    fn is_private_match(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_IsPrivateMatch(self.addr())
        }
    }
    fn should_update_skills(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_ShouldUpdateSkills(self.addr())
        }
    }
    fn is_valid_id(&self, in_playlist_id: i32) -> bool {
        unsafe {
            GameSettingPlaylist_X_IsValidID(self.addr(), in_playlist_id)
        }
    }
    fn is_valid(&self) -> bool {
        unsafe {
            GameSettingPlaylist_X_IsValid(self.addr())
        }
    }

}

extern "C" {
    fn GameSettingPlaylist_X_Get_Title(obj: usize, result: *mut RLString);
    fn GameSettingPlaylist_X_Get_Description(obj: usize, result: *mut RLString);
    fn GameSettingPlaylist_X_Get_PlayerCount(obj: usize) -> i32;
    fn GameSettingPlaylistWrapper_SetPlayerCount(obj: usize, new_val: i32);
    fn GameSettingPlaylist_X_Get_bStandard(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbStandard(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bRanked(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbRanked(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bSolo(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbSolo(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bNew(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbNew(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bApplyQuitPenalty(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbApplyQuitPenalty(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bAllowForfeit(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbAllowForfeit(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bDisableRankedReconnect(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbDisableRankedReconnect(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bIgnoreAssignTeams(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbIgnoreAssignTeams(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bKickOnMigrate(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbKickOnMigrate(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bAllowClubs(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbAllowClubs(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_bPlayersVSBots(obj: usize) -> bool;
    fn GameSettingPlaylistWrapper_SetbPlayersVSBots(obj: usize, new_val: bool);
    fn GameSettingPlaylist_X_Get_PlaylistId(obj: usize) -> i32;
    fn GameSettingPlaylistWrapper_SetPlaylistId(obj: usize, new_val: i32);
    fn GameSettingPlaylist_X_Get_ServerCommand(obj: usize, result: *mut RLString);
    fn GameSettingPlaylist_X_IsLanMatch(obj: usize) -> bool;
    fn GameSettingPlaylist_X_IsPrivateMatch(obj: usize) -> bool;
    fn GameSettingPlaylist_X_ShouldUpdateSkills(obj: usize) -> bool;
    fn GameSettingPlaylist_X_IsValidID(obj: usize, InPlaylistID: i32) -> bool;
    fn GameSettingPlaylist_X_IsValid(obj: usize) -> bool;

}