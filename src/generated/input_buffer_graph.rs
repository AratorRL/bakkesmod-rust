use crate::wrappers::*;
use crate::generated::*;

pub struct InputBufferGraphWrapper(pub usize);
impl_object!(InputBufferGraphWrapper);

impl InputBufferGraph for InputBufferGraphWrapper {}
impl StatGraph for InputBufferGraphWrapper {}

pub trait InputBufferGraph : StatGraph {
	fn get_buffer(&self) -> SampleHistoryWrapper {
		unsafe {
			SampleHistoryWrapper::new(InputBufferGraph_TA_Get_Buffer(self.addr()))
		}
	}
	fn get_buffer_target(&self) -> SampleHistoryWrapper {
		unsafe {
			SampleHistoryWrapper::new(InputBufferGraph_TA_Get_BufferTarget(self.addr()))
		}
	}
	fn get_over_under_frames(&self) -> SampleHistoryWrapper {
		unsafe {
			SampleHistoryWrapper::new(InputBufferGraph_TA_Get_OverUnderFrames(self.addr()))
		}
	}
	fn get_physics_rate(&self) -> SampleHistoryWrapper {
		unsafe {
			SampleHistoryWrapper::new(InputBufferGraph_TA_Get_PhysicsRate(self.addr()))
		}
	}
	fn get_max_physics_rate(&self) -> f32 {
		unsafe {
			InputBufferGraph_TA_Get_MaxPhysicsRate(self.addr())
		}
	}
	fn get_min_physics_rate(&self) -> f32 {
		unsafe {
			InputBufferGraph_TA_Get_MinPhysicsRate(self.addr())
		}
	}
	fn create_buffer_history(&self, title: RLString) -> SampleHistoryWrapper {
		unsafe {
			let mut title = title;
			let title: *mut RLString = &mut title as *mut RLString;
			SampleHistoryWrapper::new(InputBufferGraph_TA_CreateBufferHistory(self.addr(), title))
		}
	}

}

extern "C" {
	fn InputBufferGraph_TA_Get_Buffer(obj: usize) -> usize;
	fn InputBufferGraphWrapper_SetBuffer(obj: usize, new_val: usize);
	fn InputBufferGraph_TA_Get_BufferTarget(obj: usize) -> usize;
	fn InputBufferGraphWrapper_SetBufferTarget(obj: usize, new_val: usize);
	fn InputBufferGraph_TA_Get_OverUnderFrames(obj: usize) -> usize;
	fn InputBufferGraphWrapper_SetOverUnderFrames(obj: usize, new_val: usize);
	fn InputBufferGraph_TA_Get_PhysicsRate(obj: usize) -> usize;
	fn InputBufferGraphWrapper_SetPhysicsRate(obj: usize, new_val: usize);
	fn InputBufferGraph_TA_Get_MaxPhysicsRate(obj: usize) -> f32;
	fn InputBufferGraphWrapper_SetMaxPhysicsRate(obj: usize, new_val: f32);
	fn InputBufferGraph_TA_Get_MinPhysicsRate(obj: usize) -> f32;
	fn InputBufferGraphWrapper_SetMinPhysicsRate(obj: usize, new_val: f32);
	fn InputBufferGraph_TA_CreateBufferHistory(obj: usize, Title: *mut RLString) -> usize;

}