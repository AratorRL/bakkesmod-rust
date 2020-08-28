use crate::wrappers::*;
use crate::generated::*;

pub struct PerfStatGraphWrapper(pub usize);
impl_object!(PerfStatGraphWrapper);

impl PerfStatGraph for PerfStatGraphWrapper {}
impl StatGraph for PerfStatGraphWrapper {}

pub trait PerfStatGraph : StatGraph {
	fn get_fps(&self) -> Option<SampleHistoryWrapper> {
		unsafe {
			SampleHistoryWrapper::try_new(PerfStatGraph_TA_Get_FPS(self.addr()))
		}
	}
	fn get_frame_time(&self) -> Option<SampleHistoryWrapper> {
		unsafe {
			SampleHistoryWrapper::try_new(PerfStatGraph_TA_Get_FrameTime(self.addr()))
		}
	}
	fn get_game_thread_time(&self) -> Option<SampleHistoryWrapper> {
		unsafe {
			SampleHistoryWrapper::try_new(PerfStatGraph_TA_Get_GameThreadTime(self.addr()))
		}
	}
	fn get_render_thread_time(&self) -> Option<SampleHistoryWrapper> {
		unsafe {
			SampleHistoryWrapper::try_new(PerfStatGraph_TA_Get_RenderThreadTime(self.addr()))
		}
	}
	fn get_gpu_frame_time(&self) -> Option<SampleHistoryWrapper> {
		unsafe {
			SampleHistoryWrapper::try_new(PerfStatGraph_TA_Get_GPUFrameTime(self.addr()))
		}
	}
	fn get_frame_time_histories(&self) -> RLArray<SampleHistoryWrapper> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			PerfStatGraph_TA_Get_FrameTimeHistories(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_max_fps(&self) -> f32 {
		unsafe {
			PerfStatGraph_TA_Get_MaxFPS(self.addr())
		}
	}
	fn get_target_fps(&self) -> f32 {
		unsafe {
			PerfStatGraph_TA_Get_TargetFPS(self.addr())
		}
	}
	fn create_frame_time_history(&self, title: RLString) -> Option<SampleHistoryWrapper> {
		unsafe {
			let mut title = title;
			let title: *mut RLString = &mut title as *mut RLString;
			SampleHistoryWrapper::try_new(PerfStatGraph_TA_CreateFrameTimeHistory(self.addr(), title))
		}
	}
	fn create_fps_history(&self, title: RLString) -> Option<SampleHistoryWrapper> {
		unsafe {
			let mut title = title;
			let title: *mut RLString = &mut title as *mut RLString;
			SampleHistoryWrapper::try_new(PerfStatGraph_TA_CreateFpsHistory(self.addr(), title))
		}
	}

}

extern "C" {
	fn PerfStatGraph_TA_Get_FPS(obj: usize) -> usize;
	fn PerfStatGraphWrapper_SetFPS(obj: usize, new_val: usize);
	fn PerfStatGraph_TA_Get_FrameTime(obj: usize) -> usize;
	fn PerfStatGraphWrapper_SetFrameTime(obj: usize, new_val: usize);
	fn PerfStatGraph_TA_Get_GameThreadTime(obj: usize) -> usize;
	fn PerfStatGraphWrapper_SetGameThreadTime(obj: usize, new_val: usize);
	fn PerfStatGraph_TA_Get_RenderThreadTime(obj: usize) -> usize;
	fn PerfStatGraphWrapper_SetRenderThreadTime(obj: usize, new_val: usize);
	fn PerfStatGraph_TA_Get_GPUFrameTime(obj: usize) -> usize;
	fn PerfStatGraphWrapper_SetGPUFrameTime(obj: usize, new_val: usize);
	fn PerfStatGraph_TA_Get_FrameTimeHistories(obj: usize, result: *mut RLArrayRaw);
	fn PerfStatGraph_TA_Get_MaxFPS(obj: usize) -> f32;
	fn PerfStatGraphWrapper_SetMaxFPS(obj: usize, new_val: f32);
	fn PerfStatGraph_TA_Get_TargetFPS(obj: usize) -> f32;
	fn PerfStatGraphWrapper_SetTargetFPS(obj: usize, new_val: f32);
	fn PerfStatGraph_TA_CreateFrameTimeHistory(obj: usize, Title: *mut RLString) -> usize;
	fn PerfStatGraph_TA_CreateFpsHistory(obj: usize, Title: *mut RLString) -> usize;

}