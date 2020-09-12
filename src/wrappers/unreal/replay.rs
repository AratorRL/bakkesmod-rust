use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct ReplayWrapper(pub usize);
impl_object!(ReplayWrapper);

impl Replay for ReplayWrapper {}

pub trait Replay : Object {
    fn get_replay_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Replay_TA_Get_ReplayName(self.addr(), result_ptr);
            result
        }
    }
    fn get_engine_version(&self) -> i32 {
        unsafe {
            Replay_TA_Get_EngineVersion(self.addr())
        }
    }
    fn get_licensee_version(&self) -> i32 {
        unsafe {
            Replay_TA_Get_LicenseeVersion(self.addr())
        }
    }
    fn get_net_version(&self) -> i32 {
        unsafe {
            Replay_TA_Get_NetVersion(self.addr())
        }
    }
    fn get_replay_version(&self) -> i32 {
        unsafe {
            Replay_TA_Get_ReplayVersion(self.addr())
        }
    }
    fn get_replay_last_save_version(&self) -> i32 {
        unsafe {
            Replay_TA_Get_ReplayLastSaveVersion(self.addr())
        }
    }
    fn get_game_version(&self) -> i32 {
        unsafe {
            Replay_TA_Get_GameVersion(self.addr())
        }
    }
    fn get_build_id(&self) -> i32 {
        unsafe {
            Replay_TA_Get_BuildID(self.addr())
        }
    }
    fn get_changelist(&self) -> i32 {
        unsafe {
            Replay_TA_Get_Changelist(self.addr())
        }
    }
    fn get_build_version(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Replay_TA_Get_BuildVersion(self.addr(), result_ptr);
            result
        }
    }
    fn get_reserve_megabytes(&self) -> i32 {
        unsafe {
            Replay_TA_Get_ReserveMegabytes(self.addr())
        }
    }
    fn get_record_fps(&self) -> f32 {
        unsafe {
            Replay_TA_Get_RecordFPS(self.addr())
        }
    }
    fn get_keyframe_delay(&self) -> f32 {
        unsafe {
            Replay_TA_Get_KeyframeDelay(self.addr())
        }
    }
    fn get_max_channels(&self) -> i32 {
        unsafe {
            Replay_TA_Get_MaxChannels(self.addr())
        }
    }
    fn get_max_replay_size_mb(&self) -> i32 {
        unsafe {
            Replay_TA_Get_MaxReplaySizeMB(self.addr())
        }
    }
    fn get_id(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Replay_TA_Get_Id(self.addr(), result_ptr);
            result
        }
    }
    fn get_date(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Replay_TA_Get_Date(self.addr(), result_ptr);
            result
        }
    }
    fn get_num_frames(&self) -> i32 {
        unsafe {
            Replay_TA_Get_NumFrames(self.addr())
        }
    }
    fn get_player_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Replay_TA_Get_PlayerName(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_file_corrupted(&self) -> bool {
        unsafe {
            Replay_TA_Get_bFileCorrupted(self.addr())
        }
    }
    fn get_b_force_keyframe(&self) -> bool {
        unsafe {
            Replay_TA_Get_bForceKeyframe(self.addr())
        }
    }
    fn get_b_loaded_net_packages(&self) -> bool {
        unsafe {
            Replay_TA_Get_bLoadedNetPackages(self.addr())
        }
    }
    fn get_b_debug(&self) -> bool {
        unsafe {
            Replay_TA_Get_bDebug(self.addr())
        }
    }
    fn get_replay_state(&self) -> u8 {
        unsafe {
            Replay_TA_Get_ReplayState(self.addr())
        }
    }
    fn get_current_frame(&self) -> i32 {
        unsafe {
            Replay_TA_Get_CurrentFrame(self.addr())
        }
    }
    fn get_next_keyframe(&self) -> i32 {
        unsafe {
            Replay_TA_Get_NextKeyframe(self.addr())
        }
    }
    fn get_current_time(&self) -> f32 {
        unsafe {
            Replay_TA_Get_CurrentTime(self.addr())
        }
    }
    fn get_accumulated_delta_time(&self) -> f32 {
        unsafe {
            Replay_TA_Get_AccumulatedDeltaTime(self.addr())
        }
    }
    fn get_time_to_skip_to(&self) -> f32 {
        unsafe {
            Replay_TA_Get_TimeToSkipTo(self.addr())
        }
    }
    fn get_frame_to_skip_to(&self) -> i32 {
        unsafe {
            Replay_TA_Get_FrameToSkipTo(self.addr())
        }
    }
    fn get_players_only_ticks(&self) -> i32 {
        unsafe {
            Replay_TA_Get_PlayersOnlyTicks(self.addr())
        }
    }
    fn get_playback_time(&self) -> f32 {
        unsafe {
            Replay_TA_GetPlaybackTime(self.addr())
        }
    }
    fn is_from_before_game_version(&self, before_game_version: u8) -> bool {
        unsafe {
            Replay_TA_IsFromBeforeGameVersion(self.addr(), before_game_version)
        }
    }
    fn is_from_before_replay_version(&self, before_replay_version: u8) -> bool {
        unsafe {
            Replay_TA_IsFromBeforeReplayVersion(self.addr(), before_replay_version)
        }
    }
    fn set_replay_name(&self, new_name: RLString) {
        unsafe {
            let mut new_name = new_name;
            let new_name: *mut RLString = &mut new_name as *mut RLString;
            Replay_TA_SetReplayName(self.addr(), new_name);
        }
    }
    fn remove_timeline_keyframe(&self, keyframe_index: i32) {
        unsafe {
            Replay_TA_RemoveTimelineKeyframe(self.addr(), keyframe_index);
        }
    }
    fn create_copy(&self, start_time: f32) -> Option<ReplayWrapper> {
        unsafe {
            ReplayWrapper::try_new(Replay_TA_CreateCopy(self.addr(), start_time))
        }
    }
    fn import_replay(&self, path: RLString) {
        unsafe {
            let mut path = path;
            let path: *mut RLString = &mut path as *mut RLString;
            Replay_TA_ImportReplay(self.addr(), path);
        }
    }
    fn export_replay(&self, path: RLString) {
        unsafe {
            let mut path = path;
            let path: *mut RLString = &mut path as *mut RLString;
            Replay_TA_ExportReplay(self.addr(), path);
        }
    }
    fn skip_to_frame(&self, frame: i32, b_flush: bool) {
        unsafe {
            Replay_TA_SkipToFrame(self.addr(), frame, b_flush);
        }
    }
    fn skip_to_time(&self, time: f32, b_flush: bool) {
        unsafe {
            Replay_TA_SkipToTime(self.addr(), time, b_flush);
        }
    }
    fn get_replay_time_seconds(&self) -> f32 {
        unsafe {
            Replay_TA_GetReplayTimeSeconds(self.addr())
        }
    }
    fn stop_playback(&self) {
        unsafe {
            Replay_TA_StopPlayback(self.addr());
        }
    }
    fn start_playback_at_frame(&self, start_frame: i32) {
        unsafe {
            Replay_TA_StartPlaybackAtFrame(self.addr(), start_frame);
        }
    }
    fn start_playback_at_time_seconds(&self, start_time: f32) {
        unsafe {
            Replay_TA_StartPlaybackAtTimeSeconds(self.addr(), start_time);
        }
    }
    fn stop_record(&self) {
        unsafe {
            Replay_TA_StopRecord(self.addr());
        }
    }
    fn start_record(&self) {
        unsafe {
            Replay_TA_StartRecord(self.addr());
        }
    }
    fn tick(&self, delta_time: f32) {
        unsafe {
            Replay_TA_Tick(self.addr(), delta_time);
        }
    }
    fn event_played_frame(&self, replay: ReplayWrapper) {
        unsafe {
            Replay_TA_EventPlayedFrame(self.addr(), replay.addr());
        }
    }
    fn event_post_time_skip(&self, replay: ReplayWrapper) {
        unsafe {
            Replay_TA_EventPostTimeSkip(self.addr(), replay.addr());
        }
    }
    fn event_pre_time_skip(&self, replay: ReplayWrapper) {
        unsafe {
            Replay_TA_EventPreTimeSkip(self.addr(), replay.addr());
        }
    }
    fn event_spawned(&self, replay: ReplayWrapper, a: ActorWrapper) {
        unsafe {
            Replay_TA_EventSpawned(self.addr(), replay.addr(), a.addr());
        }
    }
    fn event_playback_stopped(&self, replay: ReplayWrapper) {
        unsafe {
            Replay_TA_EventPlaybackStopped(self.addr(), replay.addr());
        }
    }

}

extern "C" {
    fn Replay_TA_Get_ReplayName(obj: usize, result: *mut RLString);
    fn Replay_TA_Get_EngineVersion(obj: usize) -> i32;
    fn ReplayWrapper_SetEngineVersion(obj: usize, new_val: i32);
    fn Replay_TA_Get_LicenseeVersion(obj: usize) -> i32;
    fn ReplayWrapper_SetLicenseeVersion(obj: usize, new_val: i32);
    fn Replay_TA_Get_NetVersion(obj: usize) -> i32;
    fn ReplayWrapper_SetNetVersion(obj: usize, new_val: i32);
    fn Replay_TA_Get_ReplayVersion(obj: usize) -> i32;
    fn ReplayWrapper_SetReplayVersion(obj: usize, new_val: i32);
    fn Replay_TA_Get_ReplayLastSaveVersion(obj: usize) -> i32;
    fn ReplayWrapper_SetReplayLastSaveVersion(obj: usize, new_val: i32);
    fn Replay_TA_Get_GameVersion(obj: usize) -> i32;
    fn ReplayWrapper_SetGameVersion(obj: usize, new_val: i32);
    fn Replay_TA_Get_BuildID(obj: usize) -> i32;
    fn ReplayWrapper_SetBuildID(obj: usize, new_val: i32);
    fn Replay_TA_Get_Changelist(obj: usize) -> i32;
    fn ReplayWrapper_SetChangelist(obj: usize, new_val: i32);
    fn Replay_TA_Get_BuildVersion(obj: usize, result: *mut RLString);
    fn Replay_TA_Get_ReserveMegabytes(obj: usize) -> i32;
    fn ReplayWrapper_SetReserveMegabytes(obj: usize, new_val: i32);
    fn Replay_TA_Get_RecordFPS(obj: usize) -> f32;
    fn ReplayWrapper_SetRecordFPS(obj: usize, new_val: f32);
    fn Replay_TA_Get_KeyframeDelay(obj: usize) -> f32;
    fn ReplayWrapper_SetKeyframeDelay(obj: usize, new_val: f32);
    fn Replay_TA_Get_MaxChannels(obj: usize) -> i32;
    fn ReplayWrapper_SetMaxChannels(obj: usize, new_val: i32);
    fn Replay_TA_Get_MaxReplaySizeMB(obj: usize) -> i32;
    fn ReplayWrapper_SetMaxReplaySizeMB(obj: usize, new_val: i32);
    fn Replay_TA_Get_Id(obj: usize, result: *mut RLString);
    fn Replay_TA_Get_Date(obj: usize, result: *mut RLString);
    fn Replay_TA_Get_NumFrames(obj: usize) -> i32;
    fn ReplayWrapper_SetNumFrames(obj: usize, new_val: i32);
    fn Replay_TA_Get_PlayerName(obj: usize, result: *mut RLString);
    fn Replay_TA_Get_bFileCorrupted(obj: usize) -> bool;
    fn ReplayWrapper_SetbFileCorrupted(obj: usize, new_val: bool);
    fn Replay_TA_Get_bForceKeyframe(obj: usize) -> bool;
    fn ReplayWrapper_SetbForceKeyframe(obj: usize, new_val: bool);
    fn Replay_TA_Get_bLoadedNetPackages(obj: usize) -> bool;
    fn ReplayWrapper_SetbLoadedNetPackages(obj: usize, new_val: bool);
    fn Replay_TA_Get_bDebug(obj: usize) -> bool;
    fn ReplayWrapper_SetbDebug(obj: usize, new_val: bool);
    fn Replay_TA_Get_ReplayState(obj: usize) -> u8;
    fn ReplayWrapper_SetReplayState(obj: usize, new_val: u8);
    fn Replay_TA_Get_CurrentFrame(obj: usize) -> i32;
    fn ReplayWrapper_SetCurrentFrame(obj: usize, new_val: i32);
    fn Replay_TA_Get_NextKeyframe(obj: usize) -> i32;
    fn ReplayWrapper_SetNextKeyframe(obj: usize, new_val: i32);
    fn Replay_TA_Get_CurrentTime(obj: usize) -> f32;
    fn ReplayWrapper_SetCurrentTime(obj: usize, new_val: f32);
    fn Replay_TA_Get_AccumulatedDeltaTime(obj: usize) -> f32;
    fn ReplayWrapper_SetAccumulatedDeltaTime(obj: usize, new_val: f32);
    fn Replay_TA_Get_TimeToSkipTo(obj: usize) -> f32;
    fn ReplayWrapper_SetTimeToSkipTo(obj: usize, new_val: f32);
    fn Replay_TA_Get_FrameToSkipTo(obj: usize) -> i32;
    fn ReplayWrapper_SetFrameToSkipTo(obj: usize, new_val: i32);
    fn Replay_TA_Get_PlayersOnlyTicks(obj: usize) -> i32;
    fn ReplayWrapper_SetPlayersOnlyTicks(obj: usize, new_val: i32);
    fn Replay_TA_GetPlaybackTime(obj: usize) -> f32;
    fn Replay_TA_IsFromBeforeGameVersion(obj: usize, BeforeGameVersion: u8) -> bool;
    fn Replay_TA_IsFromBeforeReplayVersion(obj: usize, BeforeReplayVersion: u8) -> bool;
    fn Replay_TA_SetReplayName(obj: usize, NewName: *mut RLString);
    fn Replay_TA_RemoveTimelineKeyframe(obj: usize, KeyframeIndex: i32);
    fn Replay_TA_CreateCopy(obj: usize, StartTime: f32) -> usize;
    fn Replay_TA_ImportReplay(obj: usize, Path: *mut RLString);
    fn Replay_TA_ExportReplay(obj: usize, Path: *mut RLString);
    fn Replay_TA_SkipToFrame(obj: usize, frame: i32, bFlush: bool);
    fn Replay_TA_SkipToTime(obj: usize, Time: f32, bFlush: bool);
    fn Replay_TA_GetReplayTimeSeconds(obj: usize) -> f32;
    fn Replay_TA_StopPlayback(obj: usize);
    fn Replay_TA_StartPlaybackAtFrame(obj: usize, StartFrame: i32);
    fn Replay_TA_StartPlaybackAtTimeSeconds(obj: usize, StartTime: f32);
    fn Replay_TA_StopRecord(obj: usize);
    fn Replay_TA_StartRecord(obj: usize);
    fn Replay_TA_Tick(obj: usize, DeltaTime: f32);
    fn Replay_TA_EventPlayedFrame(obj: usize, Replay: usize);
    fn Replay_TA_EventPostTimeSkip(obj: usize, Replay: usize);
    fn Replay_TA_EventPreTimeSkip(obj: usize, Replay: usize);
    fn Replay_TA_EventSpawned(obj: usize, Replay: usize, A: usize);
    fn Replay_TA_EventPlaybackStopped(obj: usize, Replay: usize);

}