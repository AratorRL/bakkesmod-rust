use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct CameraWrapper(pub usize);
impl_object!(CameraWrapper);

impl Camera for CameraWrapper {}
impl CameraX for CameraWrapper {}
impl BaseCamera for CameraWrapper {}
impl Actor for CameraWrapper {}

pub trait Camera : CameraX {
	fn get_location(&self) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Camera_GetLocation(self.addr(), result_ptr); }
        result
    }

	fn set_location(&self, location: Vector) {
        let location = &location as *const Vector;
        unsafe { Camera_SetLocation(self.addr(), location); }
    }

	fn get_rotation(&self) -> Rotator {
        let mut result = Rotator::new();
        let result_ptr = &mut result as *mut Rotator;
        unsafe { Camera_GetRotation(self.addr(), result_ptr); }
        result
    }

	fn set_rotation(&self, rotation: Rotator) {
        let rotation = &rotation as *const Rotator;
        unsafe { Camera_SetRotation(self.addr(), rotation); }
    }

	fn get_camera_settings(&self) -> ProfileCameraSettings {
        let mut result = ProfileCameraSettings::new();
        let result_ptr = &result as *const ProfileCameraSettings;
        unsafe { Camera_GetCameraSettings(self.addr(), result_ptr); }
        result
    }

	fn set_camera_settings(&self, settings: ProfileCameraSettings) {
        let settings = &settings as *const ProfileCameraSettings;
        unsafe { Camera_SetCameraSettings(self.addr(), settings); }
    }

	fn is_camera_shake_on(&self) -> bool {
        unsafe { Camera_IsCameraShakeOn(self.addr()) }
    }

	fn get_pov(&self) -> POV {
        let mut result = POV::new();
        let result_ptr = &result as *const POV;
        unsafe { Camera_GetPOV(self.addr(), result_ptr); }
        result
    }

	fn set_pov(&self, pov: POV) {
        let pov = &pov as *const POV;
        unsafe { Camera_SetPOV(self.addr(), pov); }
    }

	fn set_fov(&self, fov: f32) {
        unsafe { Camera_SetFOV_custom(self.addr(), fov); }
    }

	fn get_fov(&self) -> f32 {
        unsafe { Camera_GetFOV(self.addr()) }
    }

	fn set_locked_fov(&self, lock: bool) {
        unsafe { Camera_SetLockedFOV(self.addr(), lock); }
    }

	fn get_camera_as_actor(&self) -> Option<ActorWrapper> {
        unsafe { ActorWrapper::try_new(Camera_GetCameraAsActor(self.addr())) }
    }

	fn get_camera_state(&self) -> String {
        let result: *const c_char = 0 as *const c_char;
        let result_ptr: *const *const c_char = &result as *const *const c_char;

        unsafe { 
            Camera_GetCameraState(self.addr(), result_ptr);
            let result = *result_ptr;
            let c_result = CStr::from_ptr(result);
            match c_result.to_str() {
                Ok(s) => String::from(s),
                Err(_) => String::new()
            }
        }
    }

	fn set_camera_state(&self, state_name: &str) {
        let c_string = CString::new(state_name).unwrap();
        let c_string: *const c_char = c_string.as_ptr();

        unsafe { Camera_SetCameraState(self.addr(), c_string); }
    }

	fn linterp(&self, start: Vector, end: Vector, elapsed: f32, speed: f32) -> Vector {
        let start = &start as *const Vector;
        let end = &end as *const Vector;
        let mut result = Vector::new();
        let result_ptr = &result as *const Vector;
        unsafe { Camera_linterp(self.addr(), start, end, elapsed, speed, result_ptr); }
        result
    }

	fn get_focus_actor(&self) -> String {
        let result: *const c_char = 0 as *const c_char;
        let result_ptr: *const *const c_char = &result as *const *const c_char;

        unsafe { 
            Camera_GetFocusActor(self.addr(), result_ptr);
            let result = *result_ptr;
            let c_result = CStr::from_ptr(result);
            match c_result.to_str() {
                Ok(s) => String::from(s),
                Err(_) => String::new()
            }
        }
    }

	fn set_focus_actor(&self, actor_name: &str) -> bool {
        let c_string = CString::new(actor_name).unwrap();
        let c_string: *const c_char = c_string.as_ptr();

        unsafe { Camera_SetFocusActor(self.addr(), c_string) }
    }

	fn set_fly_cam_ball_target_mode(&self) -> bool {
        unsafe { Camera_SetFlyCamBallTargetMode(self.addr()) }
    }



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
	fn Camera_GetLocation(obj: usize, result: *const Vector);
	fn Camera_SetLocation(obj: usize, location: *const Vector);
	fn Camera_GetRotation(obj: usize, result: *const Rotator);
	fn Camera_SetRotation(obj: usize, rotation: *const Rotator);
	fn Camera_GetCameraSettings(obj: usize, result: *const ProfileCameraSettings);
	fn Camera_SetCameraSettings(obj: usize, settings: *const ProfileCameraSettings);
	fn Camera_IsCameraShakeOn(obj: usize) -> bool;
	fn Camera_GetPOV(obj: usize, result: *const POV);
	fn Camera_SetPOV(obj: usize, pov: *const POV);
	fn Camera_SetFOV_custom(obj: usize, fov: f32);
	fn Camera_GetFOV(obj: usize) -> f32;
	fn Camera_SetLockedFOV(obj: usize, lock: bool);
	fn Camera_GetCameraAsActor(obj: usize) -> usize;
	fn Camera_GetCameraState(obj: usize, result: *const *const c_char);
	fn Camera_SetCameraState(obj: usize, state_name: *const c_char);
	fn Camera_linterp(obj: usize, start: *const Vector, end: *const Vector, elapsed: f32, speed: f32, result: *const Vector);
	fn Camera_GetFocusActor(obj: usize, result: *const *const c_char);
	fn Camera_SetFocusActor(obj: usize, actorName: *const c_char) -> bool;
	fn Camera_SetFlyCamBallTargetMode(obj: usize) -> bool;

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