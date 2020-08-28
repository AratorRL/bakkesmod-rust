use crate::wrappers::*;
use super::*;

pub struct CameraXWrapper(pub usize);
impl_object!(CameraXWrapper);

impl CameraX for CameraXWrapper {}
impl BaseCamera for CameraXWrapper {}
impl Actor for CameraXWrapper {}

pub trait CameraX : BaseCamera {
    fn get_pc_delta_rotation(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            Camera_X_Get_PCDeltaRotation(self.addr(), result_ptr);
            result
        }
    }
    fn get_old_controller_rotation(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            Camera_X_Get_OldControllerRotation(self.addr(), result_ptr);
            result
        }
    }
    fn get_pc_delta_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Camera_X_Get_PCDeltaLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_old_controller_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Camera_X_Get_OldControllerLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_shake_location_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Camera_X_Get_ShakeLocationOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_shake_rotation_offset(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            Camera_X_Get_ShakeRotationOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_shake_fov_offset(&self) -> f32 {
        unsafe {
            Camera_X_Get_ShakeFOVOffset(self.addr())
        }
    }
    fn get_start_fade_color(&self) -> Color {
        unsafe {
            let mut result = Color::new();
            let result_ptr: *mut Color = &mut result as *mut Color;
            Camera_X_Get_StartFadeColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_end_fade_color(&self) -> Color {
        unsafe {
            let mut result = Color::new();
            let result_ptr: *mut Color = &mut result as *mut Color;
            Camera_X_Get_EndFadeColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_clip_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Camera_X_Get_ClipOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_disable_camera_shake(&self) -> bool {
        unsafe {
            Camera_X_Get_bDisableCameraShake(self.addr())
        }
    }
    fn get_b_snap_next_transition(&self) -> bool {
        unsafe {
            Camera_X_Get_bSnapNextTransition(self.addr())
        }
    }
    fn is_transitioning(&self) -> bool {
        unsafe {
            Camera_X_IsTransitioning(self.addr())
        }
    }
    fn snap_transition(&self) {
        unsafe {
            Camera_X_SnapTransition(self.addr());
        }
    }
    fn copy_fade(&self, other: CameraXWrapper) {
        unsafe {
            Camera_X_CopyFade(self.addr(), other.addr());
        }
    }
    fn update_fade(&self, delta_time: f32) {
        unsafe {
            Camera_X_UpdateFade(self.addr(), delta_time);
        }
    }
    fn remove_roll(&self, in_rot: Rotator) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            let mut in_rot = in_rot;
            let in_rot: *mut Rotator = &mut in_rot as *mut Rotator;
            Camera_X_RemoveRoll(self.addr(), in_rot, result_ptr);
            result
        }
    }
    fn update_camera_state(&self) {
        unsafe {
            Camera_X_UpdateCameraState(self.addr());
        }
    }
    fn instance_camera_states(&self) {
        unsafe {
            Camera_X_InstanceCameraStates(self.addr());
        }
    }
    fn on_loading_movie_closesd(&self) {
        unsafe {
            Camera_X_OnLoadingMovieClosesd(self.addr());
        }
    }

}

extern "C" {
    fn Camera_X_Get_PCDeltaRotation(obj: usize, result: *mut Rotator);
    fn CameraXWrapper_SetPCDeltaRotation(obj: usize, new_val: *mut Rotator);
    fn Camera_X_Get_OldControllerRotation(obj: usize, result: *mut Rotator);
    fn CameraXWrapper_SetOldControllerRotation(obj: usize, new_val: *mut Rotator);
    fn Camera_X_Get_PCDeltaLocation(obj: usize, result: *mut Vector);
    fn CameraXWrapper_SetPCDeltaLocation(obj: usize, new_val: *mut Vector);
    fn Camera_X_Get_OldControllerLocation(obj: usize, result: *mut Vector);
    fn CameraXWrapper_SetOldControllerLocation(obj: usize, new_val: *mut Vector);
    fn Camera_X_Get_ShakeLocationOffset(obj: usize, result: *mut Vector);
    fn CameraXWrapper_SetShakeLocationOffset(obj: usize, new_val: *mut Vector);
    fn Camera_X_Get_ShakeRotationOffset(obj: usize, result: *mut Rotator);
    fn CameraXWrapper_SetShakeRotationOffset(obj: usize, new_val: *mut Rotator);
    fn Camera_X_Get_ShakeFOVOffset(obj: usize) -> f32;
    fn CameraXWrapper_SetShakeFOVOffset(obj: usize, new_val: f32);
    fn Camera_X_Get_StartFadeColor(obj: usize, result: *mut Color);
    fn CameraXWrapper_SetStartFadeColor(obj: usize, new_val: *mut Color);
    fn Camera_X_Get_EndFadeColor(obj: usize, result: *mut Color);
    fn CameraXWrapper_SetEndFadeColor(obj: usize, new_val: *mut Color);
    fn Camera_X_Get_ClipOffset(obj: usize, result: *mut Vector);
    fn CameraXWrapper_SetClipOffset(obj: usize, new_val: *mut Vector);
    fn Camera_X_Get_bDisableCameraShake(obj: usize) -> bool;
    fn CameraXWrapper_SetbDisableCameraShake(obj: usize, new_val: bool);
    fn Camera_X_Get_bSnapNextTransition(obj: usize) -> bool;
    fn CameraXWrapper_SetbSnapNextTransition(obj: usize, new_val: bool);
    fn Camera_X_IsTransitioning(obj: usize) -> bool;
    fn Camera_X_SnapTransition(obj: usize);
    fn Camera_X_CopyFade(obj: usize, Other: usize);
    fn Camera_X_UpdateFade(obj: usize, DeltaTime: f32);
    fn Camera_X_RemoveRoll(obj: usize, InRot: *mut Rotator, result: *mut Rotator);
    fn Camera_X_UpdateCameraState(obj: usize);
    fn Camera_X_InstanceCameraStates(obj: usize);
    fn Camera_X_OnLoadingMovieClosesd(obj: usize);

}