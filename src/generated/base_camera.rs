use crate::wrappers::*;
use crate::generated::*;

pub struct BaseCameraWrapper(pub usize);
impl_object!(BaseCameraWrapper);

impl BaseCamera for BaseCameraWrapper {}
impl Actor for BaseCameraWrapper {}

pub trait BaseCamera : Actor {
    fn get_default_fov(&self) -> f32 {
        unsafe {
            Camera_Get_DefaultFOV(self.addr())
        }
    }
    fn get_b_locked_fov(&self) -> bool {
        unsafe {
            Camera_Get_bLockedFOV(self.addr())
        }
    }
    fn get_b_constrain_aspect_ratio(&self) -> bool {
        unsafe {
            Camera_Get_bConstrainAspectRatio(self.addr())
        }
    }
    fn get_b_enable_fading(&self) -> bool {
        unsafe {
            Camera_Get_bEnableFading(self.addr())
        }
    }
    fn get_b_fade_audio(&self) -> bool {
        unsafe {
            Camera_Get_bFadeAudio(self.addr())
        }
    }
    fn get_b_force_disable_temporal_aa(&self) -> bool {
        unsafe {
            Camera_Get_bForceDisableTemporalAA(self.addr())
        }
    }
    fn get_b_enable_color_scaling(&self) -> bool {
        unsafe {
            Camera_Get_bEnableColorScaling(self.addr())
        }
    }
    fn get_b_enable_color_scale_interp(&self) -> bool {
        unsafe {
            Camera_Get_bEnableColorScaleInterp(self.addr())
        }
    }
    fn get_b_use_client_side_camera_updates(&self) -> bool {
        unsafe {
            Camera_Get_bUseClientSideCameraUpdates(self.addr())
        }
    }
    fn get_b_debug_client_side_camera(&self) -> bool {
        unsafe {
            Camera_Get_bDebugClientSideCamera(self.addr())
        }
    }
    fn get_b_should_send_client_side_camera_update(&self) -> bool {
        unsafe {
            Camera_Get_bShouldSendClientSideCameraUpdate(self.addr())
        }
    }
    fn get_locked_fov(&self) -> f32 {
        unsafe {
            Camera_Get_LockedFOV(self.addr())
        }
    }
    fn get_constrained_aspect_ratio(&self) -> f32 {
        unsafe {
            Camera_Get_ConstrainedAspectRatio(self.addr())
        }
    }
    fn get_default_aspect_ratio(&self) -> f32 {
        unsafe {
            Camera_Get_DefaultAspectRatio(self.addr())
        }
    }
    fn get_off_axis_yaw_angle(&self) -> f32 {
        unsafe {
            Camera_Get_OffAxisYawAngle(self.addr())
        }
    }
    fn get_off_axis_pitch_angle(&self) -> f32 {
        unsafe {
            Camera_Get_OffAxisPitchAngle(self.addr())
        }
    }
    fn get_fade_color(&self) -> Color {
        unsafe {
            let mut result = Color::new();
            let result_ptr: *mut Color = &mut result as *mut Color;
            Camera_Get_FadeColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_fade_amount(&self) -> f32 {
        unsafe {
            Camera_Get_FadeAmount(self.addr())
        }
    }
    fn get_cam_override_post_process_alpha(&self) -> f32 {
        unsafe {
            Camera_Get_CamOverridePostProcessAlpha(self.addr())
        }
    }
    fn get_color_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Camera_Get_ColorScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_desired_color_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Camera_Get_DesiredColorScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_original_color_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Camera_Get_OriginalColorScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_color_scale_interp_duration(&self) -> f32 {
        unsafe {
            Camera_Get_ColorScaleInterpDuration(self.addr())
        }
    }
    fn get_color_scale_interp_start_time(&self) -> f32 {
        unsafe {
            Camera_Get_ColorScaleInterpStartTime(self.addr())
        }
    }
    fn get_view_target(&self) -> TViewTarget {
        unsafe {
            let mut result = TViewTarget::new();
            let result_ptr: *mut TViewTarget = &mut result as *mut TViewTarget;
            Camera_Get_ViewTarget(self.addr(), result_ptr);
            result
        }
    }
    fn get_pending_view_target(&self) -> TViewTarget {
        unsafe {
            let mut result = TViewTarget::new();
            let result_ptr: *mut TViewTarget = &mut result as *mut TViewTarget;
            Camera_Get_PendingViewTarget(self.addr(), result_ptr);
            result
        }
    }
    fn get_blend_time_to_go(&self) -> f32 {
        unsafe {
            Camera_Get_BlendTimeToGo(self.addr())
        }
    }
    fn get_free_cam_distance(&self) -> f32 {
        unsafe {
            Camera_Get_FreeCamDistance(self.addr())
        }
    }
    fn get_free_cam_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Camera_Get_FreeCamOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_fade_time(&self) -> f32 {
        unsafe {
            Camera_Get_FadeTime(self.addr())
        }
    }
    fn get_fade_time_remaining(&self) -> f32 {
        unsafe {
            Camera_Get_FadeTimeRemaining(self.addr())
        }
    }
    fn stop_all_camera_anims(&self, b_immediate: bool) {
        unsafe {
            Camera_StopAllCameraAnims(self.addr(), b_immediate);
        }
    }
    fn clear_all_camera_shakes(&self) {
        unsafe {
            Camera_ClearAllCameraShakes(self.addr());
        }
    }
    fn calc_radial_shake_scale(&self, cam: BaseCameraWrapper, epicenter: Vector, inner_radius: f32, outer_radius: f32, falloff: f32) -> f32 {
        unsafe {
            let mut epicenter = epicenter;
            let epicenter: *mut Vector = &mut epicenter as *mut Vector;
            Camera_CalcRadialShakeScale(self.addr(), cam.addr(), epicenter, inner_radius, outer_radius, falloff)
        }
    }
    fn clear_camera_lens_effects(&self) {
        unsafe {
            Camera_ClearCameraLensEffects(self.addr());
        }
    }
    fn apply_audio_fade(&self) {
        unsafe {
            Camera_ApplyAudioFade(self.addr());
        }
    }
    fn update_fade(&self, delta_time: f32) {
        unsafe {
            Camera_UpdateFade(self.addr(), delta_time);
        }
    }
    fn do_update_camera(&self, delta_time: f32) {
        unsafe {
            Camera_DoUpdateCamera(self.addr(), delta_time);
        }
    }
    fn set_desired_color_scale2(&self, new_color_scale: Vector, interp_time: f32) {
        unsafe {
            let mut new_color_scale = new_color_scale;
            let new_color_scale: *mut Vector = &mut new_color_scale as *mut Vector;
            Camera_SetDesiredColorScale2(self.addr(), new_color_scale, interp_time);
        }
    }
    fn get_camera_rotation(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            Camera_GetCameraRotation(self.addr(), result_ptr);
            result
        }
    }
    fn set_fov(&self, new_fov: f32) {
        unsafe {
            Camera_SetFOV(self.addr(), new_fov);
        }
    }
    fn get_fov_angle(&self) -> f32 {
        unsafe {
            Camera_GetFOVAngle(self.addr())
        }
    }
    fn post_begin_play(&self) {
        unsafe {
            Camera_PostBeginPlay(self.addr());
        }
    }

}

extern "C" {
    fn Camera_Get_DefaultFOV(obj: usize) -> f32;
    fn BaseCameraWrapper_SetDefaultFOV(obj: usize, new_val: f32);
    fn Camera_Get_bLockedFOV(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbLockedFOV(obj: usize, new_val: bool);
    fn Camera_Get_bConstrainAspectRatio(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbConstrainAspectRatio(obj: usize, new_val: bool);
    fn Camera_Get_bEnableFading(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbEnableFading(obj: usize, new_val: bool);
    fn Camera_Get_bFadeAudio(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbFadeAudio(obj: usize, new_val: bool);
    fn Camera_Get_bForceDisableTemporalAA(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbForceDisableTemporalAA(obj: usize, new_val: bool);
    fn Camera_Get_bEnableColorScaling(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbEnableColorScaling(obj: usize, new_val: bool);
    fn Camera_Get_bEnableColorScaleInterp(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbEnableColorScaleInterp(obj: usize, new_val: bool);
    fn Camera_Get_bUseClientSideCameraUpdates(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbUseClientSideCameraUpdates(obj: usize, new_val: bool);
    fn Camera_Get_bDebugClientSideCamera(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbDebugClientSideCamera(obj: usize, new_val: bool);
    fn Camera_Get_bShouldSendClientSideCameraUpdate(obj: usize) -> bool;
    fn BaseCameraWrapper_SetbShouldSendClientSideCameraUpdate(obj: usize, new_val: bool);
    fn Camera_Get_LockedFOV(obj: usize) -> f32;
    fn BaseCameraWrapper_SetLockedFOV(obj: usize, new_val: f32);
    fn Camera_Get_ConstrainedAspectRatio(obj: usize) -> f32;
    fn BaseCameraWrapper_SetConstrainedAspectRatio(obj: usize, new_val: f32);
    fn Camera_Get_DefaultAspectRatio(obj: usize) -> f32;
    fn BaseCameraWrapper_SetDefaultAspectRatio(obj: usize, new_val: f32);
    fn Camera_Get_OffAxisYawAngle(obj: usize) -> f32;
    fn BaseCameraWrapper_SetOffAxisYawAngle(obj: usize, new_val: f32);
    fn Camera_Get_OffAxisPitchAngle(obj: usize) -> f32;
    fn BaseCameraWrapper_SetOffAxisPitchAngle(obj: usize, new_val: f32);
    fn Camera_Get_FadeColor(obj: usize, result: *mut Color);
    fn BaseCameraWrapper_SetFadeColor(obj: usize, new_val: *mut Color);
    fn Camera_Get_FadeAmount(obj: usize) -> f32;
    fn BaseCameraWrapper_SetFadeAmount(obj: usize, new_val: f32);
    fn Camera_Get_CamOverridePostProcessAlpha(obj: usize) -> f32;
    fn BaseCameraWrapper_SetCamOverridePostProcessAlpha(obj: usize, new_val: f32);
    fn Camera_Get_ColorScale(obj: usize, result: *mut Vector);
    fn BaseCameraWrapper_SetColorScale(obj: usize, new_val: *mut Vector);
    fn Camera_Get_DesiredColorScale(obj: usize, result: *mut Vector);
    fn BaseCameraWrapper_SetDesiredColorScale(obj: usize, new_val: *mut Vector);
    fn Camera_Get_OriginalColorScale(obj: usize, result: *mut Vector);
    fn BaseCameraWrapper_SetOriginalColorScale(obj: usize, new_val: *mut Vector);
    fn Camera_Get_ColorScaleInterpDuration(obj: usize) -> f32;
    fn BaseCameraWrapper_SetColorScaleInterpDuration(obj: usize, new_val: f32);
    fn Camera_Get_ColorScaleInterpStartTime(obj: usize) -> f32;
    fn BaseCameraWrapper_SetColorScaleInterpStartTime(obj: usize, new_val: f32);
    fn Camera_Get_ViewTarget(obj: usize, result: *mut TViewTarget);
    fn BaseCameraWrapper_SetViewTarget(obj: usize, new_val: *mut TViewTarget);
    fn Camera_Get_PendingViewTarget(obj: usize, result: *mut TViewTarget);
    fn BaseCameraWrapper_SetPendingViewTarget(obj: usize, new_val: *mut TViewTarget);
    fn Camera_Get_BlendTimeToGo(obj: usize) -> f32;
    fn BaseCameraWrapper_SetBlendTimeToGo(obj: usize, new_val: f32);
    fn Camera_Get_FreeCamDistance(obj: usize) -> f32;
    fn BaseCameraWrapper_SetFreeCamDistance(obj: usize, new_val: f32);
    fn Camera_Get_FreeCamOffset(obj: usize, result: *mut Vector);
    fn BaseCameraWrapper_SetFreeCamOffset(obj: usize, new_val: *mut Vector);
    fn Camera_Get_FadeTime(obj: usize) -> f32;
    fn BaseCameraWrapper_SetFadeTime(obj: usize, new_val: f32);
    fn Camera_Get_FadeTimeRemaining(obj: usize) -> f32;
    fn BaseCameraWrapper_SetFadeTimeRemaining(obj: usize, new_val: f32);
    fn Camera_StopAllCameraAnims(obj: usize, bImmediate: bool);
    fn Camera_ClearAllCameraShakes(obj: usize);
    fn Camera_CalcRadialShakeScale(obj: usize, Cam: usize, Epicenter: *mut Vector, InnerRadius: f32, OuterRadius: f32, Falloff: f32) -> f32;
    fn Camera_ClearCameraLensEffects(obj: usize);
    fn Camera_ApplyAudioFade(obj: usize);
    fn Camera_UpdateFade(obj: usize, DeltaTime: f32);
    fn Camera_DoUpdateCamera(obj: usize, DeltaTime: f32);
    fn Camera_SetDesiredColorScale2(obj: usize, NewColorScale: *mut Vector, InterpTime: f32);
    fn Camera_GetCameraRotation(obj: usize, result: *mut Rotator);
    fn Camera_SetFOV(obj: usize, NewFOV: f32);
    fn Camera_GetFOVAngle(obj: usize) -> f32;
    fn Camera_PostBeginPlay(obj: usize);

}