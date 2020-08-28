use crate::wrappers::*;
use crate::generated::*;

pub struct JumpComponentWrapper(pub usize);
impl_object!(JumpComponentWrapper);

impl JumpComponent for JumpComponentWrapper {}
impl CarComponent for JumpComponentWrapper {}
impl Actor for JumpComponentWrapper {}

pub trait JumpComponent : CarComponent {
    fn get_min_jump_time(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_MinJumpTime(self.addr())
        }
    }
    fn get_jump_impulse(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_JumpImpulse(self.addr())
        }
    }
    fn get_jump_force(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_JumpForce(self.addr())
        }
    }
    fn get_jump_force_time(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_JumpForceTime(self.addr())
        }
    }
    fn get_podium_jump_force_time(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_PodiumJumpForceTime(self.addr())
        }
    }
    fn get_jump_impulse_speed(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_JumpImpulseSpeed(self.addr())
        }
    }
    fn get_jump_accel(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_JumpAccel(self.addr())
        }
    }
    fn get_max_jump_height(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_MaxJumpHeight(self.addr())
        }
    }
    fn get_max_jump_height_time(&self) -> f32 {
        unsafe {
            CarComponent_Jump_TA_Get_MaxJumpHeightTime(self.addr())
        }
    }
    fn get_b_deactivate(&self) -> bool {
        unsafe {
            CarComponent_Jump_TA_Get_bDeactivate(self.addr())
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            CarComponent_Jump_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn cache_jump_data(&self) {
        unsafe {
            CarComponent_Jump_TA_CacheJumpData(self.addr());
        }
    }
    fn on_created(&self) {
        unsafe {
            CarComponent_Jump_TA_OnCreated(self.addr());
        }
    }

}

extern "C" {
    fn CarComponent_Jump_TA_Get_MinJumpTime(obj: usize) -> f32;
    fn JumpComponentWrapper_SetMinJumpTime(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_JumpImpulse(obj: usize) -> f32;
    fn JumpComponentWrapper_SetJumpImpulse(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_JumpForce(obj: usize) -> f32;
    fn JumpComponentWrapper_SetJumpForce(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_JumpForceTime(obj: usize) -> f32;
    fn JumpComponentWrapper_SetJumpForceTime(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_PodiumJumpForceTime(obj: usize) -> f32;
    fn JumpComponentWrapper_SetPodiumJumpForceTime(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_JumpImpulseSpeed(obj: usize) -> f32;
    fn JumpComponentWrapper_SetJumpImpulseSpeed(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_JumpAccel(obj: usize) -> f32;
    fn JumpComponentWrapper_SetJumpAccel(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_MaxJumpHeight(obj: usize) -> f32;
    fn JumpComponentWrapper_SetMaxJumpHeight(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_MaxJumpHeightTime(obj: usize) -> f32;
    fn JumpComponentWrapper_SetMaxJumpHeightTime(obj: usize, new_val: f32);
    fn CarComponent_Jump_TA_Get_bDeactivate(obj: usize) -> bool;
    fn JumpComponentWrapper_SetbDeactivate(obj: usize, new_val: bool);
    fn CarComponent_Jump_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn CarComponent_Jump_TA_CacheJumpData(obj: usize);
    fn CarComponent_Jump_TA_OnCreated(obj: usize);

}