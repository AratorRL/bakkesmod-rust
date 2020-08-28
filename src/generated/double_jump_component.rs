use crate::wrappers::*;
use crate::generated::*;

pub struct DoubleJumpComponentWrapper(pub usize);
impl_object!(DoubleJumpComponentWrapper);

impl DoubleJumpComponent for DoubleJumpComponentWrapper {}
impl CarComponent for DoubleJumpComponentWrapper {}
impl Actor for DoubleJumpComponentWrapper {}

pub trait DoubleJumpComponent : CarComponent {
    fn get_impulse_scale(&self) -> f32 {
        unsafe {
            CarComponent_DoubleJump_TA_Get_ImpulseScale(self.addr())
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            CarComponent_DoubleJump_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn on_created(&self) {
        unsafe {
            CarComponent_DoubleJump_TA_OnCreated(self.addr());
        }
    }

}

extern "C" {
    fn DoubleJumpComponentWrapper_SetJumpImpulse(obj: usize, new_val: f32);
    fn CarComponent_DoubleJump_TA_Get_ImpulseScale(obj: usize) -> f32;
    fn DoubleJumpComponentWrapper_SetImpulseScale(obj: usize, new_val: f32);
    fn CarComponent_DoubleJump_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn CarComponent_DoubleJump_TA_OnCreated(obj: usize);

}