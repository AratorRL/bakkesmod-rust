use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct TornadoPickupWrapper(pub usize);
impl_object!(TornadoPickupWrapper);

impl TornadoPickup for TornadoPickupWrapper {}
impl RumblePickupComponent for TornadoPickupWrapper {}
impl CarComponent for TornadoPickupWrapper {}
impl Actor for TornadoPickupWrapper {}

pub trait TornadoPickup : RumblePickupComponent {
    fn get_height(&self) -> f32 {
        unsafe {
            SpecialPickup_Tornado_TA_Get_Height(self.addr())
        }
    }
    fn get_radius(&self) -> f32 {
        unsafe {
            SpecialPickup_Tornado_TA_Get_Radius(self.addr())
        }
    }
    fn get_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Tornado_TA_Get_Offset(self.addr(), result_ptr);
            result
        }
    }
    fn get_rotational_force(&self) -> f32 {
        unsafe {
            SpecialPickup_Tornado_TA_Get_RotationalForce(self.addr())
        }
    }
    fn get_torque(&self) -> f32 {
        unsafe {
            SpecialPickup_Tornado_TA_Get_Torque(self.addr())
        }
    }
    fn get_fx_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Tornado_TA_Get_FXScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_fx_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Tornado_TA_Get_FXOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_mesh_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Tornado_TA_Get_MeshOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_mesh_scale(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Tornado_TA_Get_MeshScale(self.addr(), result_ptr);
            result
        }
    }
    fn get_max_velocity_offset(&self) -> f32 {
        unsafe {
            SpecialPickup_Tornado_TA_Get_MaxVelocityOffset(self.addr())
        }
    }
    fn get_ball_multiplier(&self) -> f32 {
        unsafe {
            SpecialPickup_Tornado_TA_Get_BallMultiplier(self.addr())
        }
    }
    fn get_b_debug_vis(&self) -> bool {
        unsafe {
            SpecialPickup_Tornado_TA_Get_bDebugVis(self.addr())
        }
    }
    fn get_velocity_ease(&self) -> f32 {
        unsafe {
            SpecialPickup_Tornado_TA_Get_VelocityEase(self.addr())
        }
    }
    fn get_vel(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            SpecialPickup_Tornado_TA_Get_Vel(self.addr(), result_ptr);
            result
        }
    }
    fn get_affecting(&self) -> RLArray<RBActorWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            SpecialPickup_Tornado_TA_Get_Affecting(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            SpecialPickup_Tornado_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn play_car_sfx(&self, in_actor: RBActorWrapper) {
        unsafe {
            SpecialPickup_Tornado_TA_PlayCarSFX(self.addr(), in_actor.addr());
        }
    }
    fn play_ball_sfx(&self, in_actor: RBActorWrapper) {
        unsafe {
            SpecialPickup_Tornado_TA_PlayBallSFX(self.addr(), in_actor.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_Tornado_TA_Get_Height(obj: usize) -> f32;
    fn TornadoPickup_SetHeight(obj: usize, new_val: f32);
    fn SpecialPickup_Tornado_TA_Get_Radius(obj: usize) -> f32;
    fn TornadoPickup_SetRadius(obj: usize, new_val: f32);
    fn SpecialPickup_Tornado_TA_Get_Offset(obj: usize, result: *mut Vector);
    fn TornadoPickup_SetOffset(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Tornado_TA_Get_RotationalForce(obj: usize) -> f32;
    fn TornadoPickup_SetRotationalForce(obj: usize, new_val: f32);
    fn SpecialPickup_Tornado_TA_Get_Torque(obj: usize) -> f32;
    fn TornadoPickup_SetTorque(obj: usize, new_val: f32);
    fn SpecialPickup_Tornado_TA_Get_FXScale(obj: usize, result: *mut Vector);
    fn TornadoPickup_SetFXScale(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Tornado_TA_Get_FXOffset(obj: usize, result: *mut Vector);
    fn TornadoPickup_SetFXOffset(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Tornado_TA_Get_MeshOffset(obj: usize, result: *mut Vector);
    fn TornadoPickup_SetMeshOffset(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Tornado_TA_Get_MeshScale(obj: usize, result: *mut Vector);
    fn TornadoPickup_SetMeshScale(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Tornado_TA_Get_MaxVelocityOffset(obj: usize) -> f32;
    fn TornadoPickup_SetMaxVelocityOffset(obj: usize, new_val: f32);
    fn SpecialPickup_Tornado_TA_Get_BallMultiplier(obj: usize) -> f32;
    fn TornadoPickup_SetBallMultiplier(obj: usize, new_val: f32);
    fn SpecialPickup_Tornado_TA_Get_bDebugVis(obj: usize) -> bool;
    fn TornadoPickup_SetbDebugVis(obj: usize, new_val: bool);
    fn SpecialPickup_Tornado_TA_Get_VelocityEase(obj: usize) -> f32;
    fn TornadoPickup_SetVelocityEase(obj: usize, new_val: f32);
    fn SpecialPickup_Tornado_TA_Get_Vel(obj: usize, result: *mut Vector);
    fn TornadoPickup_SetVel(obj: usize, new_val: *mut Vector);
    fn SpecialPickup_Tornado_TA_Get_Affecting(obj: usize, result: *mut RLArrayRaw);
    fn SpecialPickup_Tornado_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn SpecialPickup_Tornado_TA_PlayCarSFX(obj: usize, InActor: usize);
    fn SpecialPickup_Tornado_TA_PlayBallSFX(obj: usize, InActor: usize);

}