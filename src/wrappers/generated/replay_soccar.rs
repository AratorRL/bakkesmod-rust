use crate::wrappers::*;
use super::*;

pub struct ReplaySoccarWrapper(pub usize);
impl_object!(ReplaySoccarWrapper);

impl ReplaySoccar for ReplaySoccarWrapper {}
impl Replay for ReplaySoccarWrapper {}

pub trait ReplaySoccar : Replay {
    fn get_team_size(&self) -> i32 {
        unsafe {
            Replay_Soccar_TA_Get_TeamSize(self.addr())
        }
    }
    fn get_unfair_team_size(&self) -> i32 {
        unsafe {
            Replay_Soccar_TA_Get_UnfairTeamSize(self.addr())
        }
    }
    fn get_b_unfair_bots(&self) -> bool {
        unsafe {
            Replay_Soccar_TA_Get_bUnfairBots(self.addr())
        }
    }
    fn get_primary_player_team(&self) -> i32 {
        unsafe {
            Replay_Soccar_TA_Get_PrimaryPlayerTeam(self.addr())
        }
    }
    fn get_team0_score(&self) -> i32 {
        unsafe {
            Replay_Soccar_TA_Get_Team0Score(self.addr())
        }
    }
    fn get_team1_score(&self) -> i32 {
        unsafe {
            Replay_Soccar_TA_Get_Team1Score(self.addr())
        }
    }
    fn remove_timeline_keyframe(&self, keyframe_index: i32) {
        unsafe {
            Replay_Soccar_TA_RemoveTimelineKeyframe(self.addr(), keyframe_index);
        }
    }
    fn record_user_event(&self) {
        unsafe {
            Replay_Soccar_TA_RecordUserEvent(self.addr());
        }
    }
    fn add_player(&self, pri: PriWrapper) {
        unsafe {
            Replay_Soccar_TA_AddPlayer(self.addr(), pri.addr());
        }
    }

}

extern "C" {
    fn Replay_Soccar_TA_Get_TeamSize(obj: usize) -> i32;
    fn ReplaySoccarWrapper_SetTeamSize(obj: usize, new_val: i32);
    fn Replay_Soccar_TA_Get_UnfairTeamSize(obj: usize) -> i32;
    fn ReplaySoccarWrapper_SetUnfairTeamSize(obj: usize, new_val: i32);
    fn Replay_Soccar_TA_Get_bUnfairBots(obj: usize) -> bool;
    fn ReplaySoccarWrapper_SetbUnfairBots(obj: usize, new_val: bool);
    fn Replay_Soccar_TA_Get_PrimaryPlayerTeam(obj: usize) -> i32;
    fn ReplaySoccarWrapper_SetPrimaryPlayerTeam(obj: usize, new_val: i32);
    fn Replay_Soccar_TA_Get_Team0Score(obj: usize) -> i32;
    fn ReplaySoccarWrapper_SetTeam0Score(obj: usize, new_val: i32);
    fn Replay_Soccar_TA_Get_Team1Score(obj: usize) -> i32;
    fn ReplaySoccarWrapper_SetTeam1Score(obj: usize, new_val: i32);
    fn Replay_Soccar_TA_RemoveTimelineKeyframe(obj: usize, KeyframeIndex: i32);
    fn Replay_Soccar_TA_RecordUserEvent(obj: usize);
    fn Replay_Soccar_TA_AddPlayer(obj: usize, PRI: usize);

}