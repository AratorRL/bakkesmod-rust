use crate::wrappers::*;
use super::*;

pub struct EngineTAWrapper(pub usize);
impl_object!(EngineTAWrapper);

impl EngineTA for EngineTAWrapper {}

pub trait EngineTA : Object {
    fn get_b_enable_client_prediction(&self) -> bool {
        unsafe {
            EngineShare_TA_Get_bEnableClientPrediction(self.addr())
        }
    }
    fn get_b_client_physics_update(&self) -> bool {
        unsafe {
            EngineShare_TA_Get_bClientPhysicsUpdate(self.addr())
        }
    }
    fn get_b_disable_client_corrections(&self) -> bool {
        unsafe {
            EngineShare_TA_Get_bDisableClientCorrections(self.addr())
        }
    }
    fn get_b_debug_client_corrections(&self) -> bool {
        unsafe {
            EngineShare_TA_Get_bDebugClientCorrections(self.addr())
        }
    }
    fn get_b_force_client_correction(&self) -> bool {
        unsafe {
            EngineShare_TA_Get_bForceClientCorrection(self.addr())
        }
    }
    fn get_physics_framerate(&self) -> f32 {
        unsafe {
            EngineShare_TA_Get_PhysicsFramerate(self.addr())
        }
    }
    fn get_max_physics_substeps(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_MaxPhysicsSubsteps(self.addr())
        }
    }
    fn get_max_uploaded_client_frames(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_MaxUploadedClientFrames(self.addr())
        }
    }
    fn get_max_client_replay_frames(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_MaxClientReplayFrames(self.addr())
        }
    }
    fn get_physics_frame(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_PhysicsFrame(self.addr())
        }
    }
    fn get_render_alpha(&self) -> f32 {
        unsafe {
            EngineShare_TA_Get_RenderAlpha(self.addr())
        }
    }
    fn get_replicated_physics_frame(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_ReplicatedPhysicsFrame(self.addr())
        }
    }
    fn get_dirty_physics_frame(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_DirtyPhysicsFrame(self.addr())
        }
    }
    fn get_force_correction_frames(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_ForceCorrectionFrames(self.addr())
        }
    }
    fn get_tick_notify_index(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_TickNotifyIndex(self.addr())
        }
    }
    fn get_shell_archetype_path(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            EngineShare_TA_Get_ShellArchetypePath(self.addr(), result_ptr);
            result
        }
    }
    fn get_last_bug_report_time(&self) -> f32 {
        unsafe {
            EngineShare_TA_Get_LastBugReportTime(self.addr())
        }
    }
    fn get_debug_client_correction_start_time(&self) -> f32 {
        unsafe {
            EngineShare_TA_Get_DebugClientCorrectionStartTime(self.addr())
        }
    }
    fn get_debug_client_correction_count(&self) -> i32 {
        unsafe {
            EngineShare_TA_Get_DebugClientCorrectionCount(self.addr())
        }
    }
    fn get_stat_graphs(&self) -> Option<StatGraphSystemWrapper> {
        unsafe {
            StatGraphSystemWrapper::try_new(EngineShare_TA_Get_StatGraphs(self.addr()))
        }
    }
    fn get_last_physics_delta_time_scale(&self) -> f32 {
        unsafe {
            EngineShare_TA_Get_LastPhysicsDeltaTimeScale(self.addr())
        }
    }
    fn debug_client_corrections(&self) {
        unsafe {
            EngineShare_TA_DebugClientCorrections(self.addr());
        }
    }
    fn get_bullet_fixed_delta_time(&self) -> f32 {
        unsafe {
            EngineShare_TA_GetBulletFixedDeltaTime(self.addr())
        }
    }
    fn run_physics_step(&self, bullet_scene_index: i32, delta_time: f32) {
        unsafe {
            EngineShare_TA_RunPhysicsStep(self.addr(), bullet_scene_index, delta_time);
        }
    }
    fn update_replicated_physics_frame(&self, server_frame: i32) {
        unsafe {
            EngineShare_TA_UpdateReplicatedPhysicsFrame(self.addr(), server_frame);
        }
    }
    fn debug_dedicated_server(&self, for_how_long: f32) {
        unsafe {
            EngineShare_TA_DebugDedicatedServer(self.addr(), for_how_long);
        }
    }
    fn get_physics_time(&self) -> f32 {
        unsafe {
            EngineShare_TA_GetPhysicsTime(self.addr())
        }
    }
    fn event_pre_async_tick(&self, delta_time: f32) {
        unsafe {
            EngineShare_TA_EventPreAsyncTick(self.addr(), delta_time);
        }
    }

}

extern "C" {
    fn EngineShare_TA_Get_bEnableClientPrediction(obj: usize) -> bool;
    fn EngineTAWrapper_SetbEnableClientPrediction(obj: usize, new_val: bool);
    fn EngineShare_TA_Get_bClientPhysicsUpdate(obj: usize) -> bool;
    fn EngineTAWrapper_SetbClientPhysicsUpdate(obj: usize, new_val: bool);
    fn EngineShare_TA_Get_bDisableClientCorrections(obj: usize) -> bool;
    fn EngineTAWrapper_SetbDisableClientCorrections(obj: usize, new_val: bool);
    fn EngineShare_TA_Get_bDebugClientCorrections(obj: usize) -> bool;
    fn EngineTAWrapper_SetbDebugClientCorrections(obj: usize, new_val: bool);
    fn EngineShare_TA_Get_bForceClientCorrection(obj: usize) -> bool;
    fn EngineTAWrapper_SetbForceClientCorrection(obj: usize, new_val: bool);
    fn EngineShare_TA_Get_PhysicsFramerate(obj: usize) -> f32;
    fn EngineTAWrapper_SetPhysicsFramerate(obj: usize, new_val: f32);
    fn EngineShare_TA_Get_MaxPhysicsSubsteps(obj: usize) -> i32;
    fn EngineTAWrapper_SetMaxPhysicsSubsteps(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_MaxUploadedClientFrames(obj: usize) -> i32;
    fn EngineTAWrapper_SetMaxUploadedClientFrames(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_MaxClientReplayFrames(obj: usize) -> i32;
    fn EngineTAWrapper_SetMaxClientReplayFrames(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_PhysicsFrame(obj: usize) -> i32;
    fn EngineTAWrapper_SetPhysicsFrame(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_RenderAlpha(obj: usize) -> f32;
    fn EngineTAWrapper_SetRenderAlpha(obj: usize, new_val: f32);
    fn EngineShare_TA_Get_ReplicatedPhysicsFrame(obj: usize) -> i32;
    fn EngineTAWrapper_SetReplicatedPhysicsFrame(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_DirtyPhysicsFrame(obj: usize) -> i32;
    fn EngineTAWrapper_SetDirtyPhysicsFrame(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_ForceCorrectionFrames(obj: usize) -> i32;
    fn EngineTAWrapper_SetForceCorrectionFrames(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_TickNotifyIndex(obj: usize) -> i32;
    fn EngineTAWrapper_SetTickNotifyIndex(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_ShellArchetypePath(obj: usize, result: *mut RLString);
    fn EngineShare_TA_Get_LastBugReportTime(obj: usize) -> f32;
    fn EngineTAWrapper_SetLastBugReportTime(obj: usize, new_val: f32);
    fn EngineShare_TA_Get_DebugClientCorrectionStartTime(obj: usize) -> f32;
    fn EngineTAWrapper_SetDebugClientCorrectionStartTime(obj: usize, new_val: f32);
    fn EngineShare_TA_Get_DebugClientCorrectionCount(obj: usize) -> i32;
    fn EngineTAWrapper_SetDebugClientCorrectionCount(obj: usize, new_val: i32);
    fn EngineShare_TA_Get_StatGraphs(obj: usize) -> usize;
    fn EngineTAWrapper_SetStatGraphs(obj: usize, new_val: usize);
    fn EngineShare_TA_Get_LastPhysicsDeltaTimeScale(obj: usize) -> f32;
    fn EngineTAWrapper_SetLastPhysicsDeltaTimeScale(obj: usize, new_val: f32);
    fn EngineShare_TA_DebugClientCorrections(obj: usize);
    fn EngineShare_TA_GetBulletFixedDeltaTime(obj: usize) -> f32;
    fn EngineShare_TA_RunPhysicsStep(obj: usize, BulletSceneIndex: i32, DeltaTime: f32);
    fn EngineShare_TA_UpdateReplicatedPhysicsFrame(obj: usize, ServerFrame: i32);
    fn EngineShare_TA_DebugDedicatedServer(obj: usize, ForHowLong: f32);
    fn EngineShare_TA_GetPhysicsTime(obj: usize) -> f32;
    fn EngineShare_TA_EventPreAsyncTick(obj: usize, DeltaTime: f32);

}