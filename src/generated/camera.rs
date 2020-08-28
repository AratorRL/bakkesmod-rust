use crate::wrappers::*;
use crate::generated::*;

pub struct CameraWrapper(pub usize);
impl_object!(CameraWrapper);

impl Camera for CameraWrapper {}
impl CameraX for CameraWrapper {}
impl BaseCamera for CameraWrapper {}
impl Actor for CameraWrapper {}

pub trait Camera : CameraX {
	fn get_swivel_fast_speed(&self) -> f32 {
		unsafe {
			Camera_TA_Get_SwivelFastSpeed(self.addr())
		}
	}
	fn get_swivel_die_rate(&self) -> f32 {
		unsafe {
			Camera_TA_Get_SwivelDieRate(self.addr())
		}
	}
	fn get_camera_preset_settings(&self) -> RLArray<ProfileCameraSettings> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			Camera_TA_Get_CameraPresetSettings(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_horizontal_splitscreen_height_offset(&self) -> f32 {
		unsafe {
			Camera_TA_Get_HorizontalSplitscreenHeightOffset(self.addr())
		}
	}
	fn get_horizontal_splitscreen_fov_offset(&self) -> f32 {
		unsafe {
			Camera_TA_Get_HorizontalSplitscreenFOVOffset(self.addr())
		}
	}
	fn get_vertical_splitscreen_fov_offset(&self) -> f32 {
		unsafe {
			Camera_TA_Get_VerticalSplitscreenFOVOffset(self.addr())
		}
	}
	fn get_clip_rate(&self) -> f32 {
		unsafe {
			Camera_TA_Get_ClipRate(self.addr())
		}
	}
	fn get_current_swivel(&self) -> Rotator {
		unsafe {
			let mut result = Rotator::new();
			let result_ptr: *mut Rotator = &mut result as *mut Rotator;
			Camera_TA_Get_CurrentSwivel(self.addr(), result_ptr);
			result
		}
	}
	fn get_demolisher(&self) -> Option<RBActorWrapper> {
		unsafe {
			RBActorWrapper::try_new(Camera_TA_Get_Demolisher(self.addr()))
		}
	}
	fn get_b_demolished(&self) -> bool {
		unsafe {
			Camera_TA_Get_bDemolished(self.addr())
		}
	}
	fn clip_to_field(&self, camera_location_z: f32) -> f32 {
		unsafe {
			Camera_TA_ClipToField(self.addr(), camera_location_z)
		}
	}
	fn demolished(&self, in_demolisher: RBActorWrapper) {
		unsafe {
			Camera_TA_Demolished(self.addr(), in_demolisher.addr());
		}
	}
	fn get_desired_swivel(&self, look_up: f32, look_right: f32) -> Rotator {
		unsafe {
			let mut result = Rotator::new();
			let result_ptr: *mut Rotator = &mut result as *mut Rotator;
			Camera_TA_GetDesiredSwivel(self.addr(), look_up, look_right, result_ptr);
			result
		}
	}
	fn update_swivel(&self, delta_time: f32) {
		unsafe {
			Camera_TA_UpdateSwivel(self.addr(), delta_time);
		}
	}
	fn get_default_fov_offset(&self) -> f32 {
		unsafe {
			Camera_TA_GetDefaultFOVOffset(self.addr())
		}
	}
	fn get_default_view_height_offset(&self) -> f32 {
		unsafe {
			Camera_TA_GetDefaultViewHeightOffset(self.addr())
		}
	}
	fn update_fov(&self) {
		unsafe {
			Camera_TA_UpdateFOV(self.addr());
		}
	}
	fn event_camera_target_changed(&self, camera: CameraWrapper, target: ActorWrapper) {
		unsafe {
			Camera_TA_EventCameraTargetChanged(self.addr(), camera.addr(), target.addr());
		}
	}

}

extern "C" {
	fn Camera_TA_Get_SwivelFastSpeed(obj: usize) -> f32;
	fn CameraWrapper_SetSwivelFastSpeed(obj: usize, new_val: f32);
	fn Camera_TA_Get_SwivelDieRate(obj: usize) -> f32;
	fn CameraWrapper_SetSwivelDieRate(obj: usize, new_val: f32);
	fn Camera_TA_Get_CameraPresetSettings(obj: usize, result: *mut RLArrayRaw);
	fn Camera_TA_Get_HorizontalSplitscreenHeightOffset(obj: usize) -> f32;
	fn CameraWrapper_SetHorizontalSplitscreenHeightOffset(obj: usize, new_val: f32);
	fn Camera_TA_Get_HorizontalSplitscreenFOVOffset(obj: usize) -> f32;
	fn CameraWrapper_SetHorizontalSplitscreenFOVOffset(obj: usize, new_val: f32);
	fn Camera_TA_Get_VerticalSplitscreenFOVOffset(obj: usize) -> f32;
	fn CameraWrapper_SetVerticalSplitscreenFOVOffset(obj: usize, new_val: f32);
	fn Camera_TA_Get_ClipRate(obj: usize) -> f32;
	fn CameraWrapper_SetClipRate(obj: usize, new_val: f32);
	fn Camera_TA_Get_CurrentSwivel(obj: usize, result: *mut Rotator);
	fn CameraWrapper_SetCurrentSwivel(obj: usize, new_val: *mut Rotator);
	fn Camera_TA_Get_Demolisher(obj: usize) -> usize;
	fn CameraWrapper_SetDemolisher(obj: usize, new_val: usize);
	fn Camera_TA_Get_bDemolished(obj: usize) -> bool;
	fn CameraWrapper_SetbDemolished(obj: usize, new_val: bool);
	fn Camera_TA_ClipToField(obj: usize, CameraLocationZ: f32) -> f32;
	fn Camera_TA_Demolished(obj: usize, InDemolisher: usize);
	fn Camera_TA_GetDesiredSwivel(obj: usize, LookUp: f32, LookRight: f32, result: *mut Rotator);
	fn Camera_TA_UpdateSwivel(obj: usize, DeltaTime: f32);
	fn Camera_TA_GetDefaultFOVOffset(obj: usize) -> f32;
	fn Camera_TA_GetDefaultViewHeightOffset(obj: usize) -> f32;
	fn Camera_TA_UpdateFOV(obj: usize);
	fn Camera_TA_EventCameraTargetChanged(obj: usize, Camera: usize, Target: usize);

}