use crate::wrappers::*;
use crate::generated::*;

pub struct AirControlComponentWrapper(pub usize);
impl_object!(AirControlComponentWrapper);

impl AirControlComponent for AirControlComponentWrapper {}
impl CarComponent for AirControlComponentWrapper {}
impl Actor for AirControlComponentWrapper {}

pub trait AirControlComponent : CarComponent {
    fn get_air_torque(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            CarComponent_AirControl_TA_Get_AirTorque(self.addr(), result_ptr);
            result
        }
    }
    fn get_air_damping(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            CarComponent_AirControl_TA_Get_AirDamping(self.addr(), result_ptr);
            result
        }
    }
    fn get_throttle_force(&self) -> f32 {
        unsafe {
            CarComponent_AirControl_TA_Get_ThrottleForce(self.addr())
        }
    }
    fn get_air_control_sensitivity(&self) -> f32 {
        unsafe {
            CarComponent_AirControl_TA_Get_AirControlSensitivity(self.addr())
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            CarComponent_AirControl_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn on_created(&self) {
        unsafe {
            CarComponent_AirControl_TA_OnCreated(self.addr());
        }
    }

}

extern "C" {
    fn CarComponent_AirControl_TA_Get_AirTorque(obj: usize, result: *mut Rotator);
    fn AirControlComponentWrapper_SetAirTorque(obj: usize, new_val: *mut Rotator);
    fn CarComponent_AirControl_TA_Get_AirDamping(obj: usize, result: *mut Rotator);
    fn AirControlComponentWrapper_SetAirDamping(obj: usize, new_val: *mut Rotator);
    fn CarComponent_AirControl_TA_Get_ThrottleForce(obj: usize) -> f32;
    fn AirControlComponentWrapper_SetThrottleForce(obj: usize, new_val: f32);
    fn CarComponent_AirControl_TA_Get_AirControlSensitivity(obj: usize) -> f32;
    fn AirControlComponentWrapper_SetAirControlSensitivity(obj: usize, new_val: f32);
    fn CarComponent_AirControl_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn CarComponent_AirControl_TA_OnCreated(obj: usize);

}