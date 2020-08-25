use crate::wrappers::*;
use crate::generated::*;

pub struct DodgeComponentWrapper(pub usize);
impl_object!(DodgeComponentWrapper);

impl DodgeComponent for DodgeComponentWrapper {}
impl CarComponent for DodgeComponentWrapper {}
impl Actor for DodgeComponentWrapper {}

pub trait DodgeComponent : CarComponent {
	fn get_dodge_input_threshold(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_DodgeInputThreshold(self.addr())
		}
	}
	fn get_side_dodge_impulse(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_SideDodgeImpulse(self.addr())
		}
	}
	fn get_side_dodge_impulse_max_speed_scale(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_SideDodgeImpulseMaxSpeedScale(self.addr())
		}
	}
	fn get_forward_dodge_impulse(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_ForwardDodgeImpulse(self.addr())
		}
	}
	fn get_forward_dodge_impulse_max_speed_scale(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_ForwardDodgeImpulseMaxSpeedScale(self.addr())
		}
	}
	fn get_backward_dodge_impulse(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_BackwardDodgeImpulse(self.addr())
		}
	}
	fn get_backward_dodge_impulse_max_speed_scale(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_BackwardDodgeImpulseMaxSpeedScale(self.addr())
		}
	}
	fn get_side_dodge_torque(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_SideDodgeTorque(self.addr())
		}
	}
	fn get_forward_dodge_torque(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_ForwardDodgeTorque(self.addr())
		}
	}
	fn get_dodge_torque_time(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_DodgeTorqueTime(self.addr())
		}
	}
	fn get_min_dodge_torque_time(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_MinDodgeTorqueTime(self.addr())
		}
	}
	fn get_dodge_z_damping(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_DodgeZDamping(self.addr())
		}
	}
	fn get_dodge_z_damping_delay(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_DodgeZDampingDelay(self.addr())
		}
	}
	fn get_dodge_z_damping_up_time(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_DodgeZDampingUpTime(self.addr())
		}
	}
	fn get_dodge_impulse_scale(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_DodgeImpulseScale(self.addr())
		}
	}
	fn get_dodge_torque_scale(&self) -> f32 {
		unsafe {
			CarComponent_Dodge_TA_Get_DodgeTorqueScale(self.addr())
		}
	}
	fn get_dodge_torque(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			CarComponent_Dodge_TA_Get_DodgeTorque(self.addr(), result_ptr);
			result
		}
	}
	fn get_dodge_direction(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			CarComponent_Dodge_TA_Get_DodgeDirection(self.addr(), result_ptr);
			result
		}
	}
	fn set_dodge_settings(&self) {
		unsafe {
			CarComponent_Dodge_TA_SetDodgeSettings(self.addr());
		}
	}
	fn apply_torque_forces(&self, active_time: f32) {
		unsafe {
			CarComponent_Dodge_TA_ApplyTorqueForces(self.addr(), active_time);
		}
	}
	fn apply_dodge_impulse(&self) {
		unsafe {
			CarComponent_Dodge_TA_ApplyDodgeImpulse(self.addr());
		}
	}
	fn get_dodge_impulse(&self, dodge_dir: Vector) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			let mut dodge_dir = dodge_dir;
			let dodge_dir: *mut Vector = &mut dodge_dir as *mut Vector;
			CarComponent_Dodge_TA_GetDodgeImpulse(self.addr(), dodge_dir, result_ptr);
			result
		}
	}
	fn apply_forces(&self, active_time: f32) {
		unsafe {
			CarComponent_Dodge_TA_ApplyForces(self.addr(), active_time);
		}
	}
	fn can_activate(&self) -> bool {
		unsafe {
			CarComponent_Dodge_TA_CanActivate(self.addr())
		}
	}
	fn on_created(&self) {
		unsafe {
			CarComponent_Dodge_TA_OnCreated(self.addr());
		}
	}

}

extern "C" {
	fn CarComponent_Dodge_TA_Get_DodgeInputThreshold(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetDodgeInputThreshold(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_SideDodgeImpulse(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetSideDodgeImpulse(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_SideDodgeImpulseMaxSpeedScale(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetSideDodgeImpulseMaxSpeedScale(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_ForwardDodgeImpulse(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetForwardDodgeImpulse(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_ForwardDodgeImpulseMaxSpeedScale(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetForwardDodgeImpulseMaxSpeedScale(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_BackwardDodgeImpulse(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetBackwardDodgeImpulse(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_BackwardDodgeImpulseMaxSpeedScale(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetBackwardDodgeImpulseMaxSpeedScale(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_SideDodgeTorque(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetSideDodgeTorque(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_ForwardDodgeTorque(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetForwardDodgeTorque(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_DodgeTorqueTime(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetDodgeTorqueTime(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_MinDodgeTorqueTime(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetMinDodgeTorqueTime(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_DodgeZDamping(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetDodgeZDamping(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_DodgeZDampingDelay(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetDodgeZDampingDelay(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_DodgeZDampingUpTime(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetDodgeZDampingUpTime(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_DodgeImpulseScale(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetDodgeImpulseScale(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_DodgeTorqueScale(obj: usize) -> f32;
	fn DodgeComponentWrapper_SetDodgeTorqueScale(obj: usize, new_val: f32);
	fn CarComponent_Dodge_TA_Get_DodgeTorque(obj: usize, result: *mut Vector);
	fn DodgeComponentWrapper_SetDodgeTorque(obj: usize, new_val: *mut Vector);
	fn CarComponent_Dodge_TA_Get_DodgeDirection(obj: usize, result: *mut Vector);
	fn DodgeComponentWrapper_SetDodgeDirection(obj: usize, new_val: *mut Vector);
	fn CarComponent_Dodge_TA_SetDodgeSettings(obj: usize);
	fn CarComponent_Dodge_TA_ApplyTorqueForces(obj: usize, ActiveTime: f32);
	fn CarComponent_Dodge_TA_ApplyDodgeImpulse(obj: usize);
	fn CarComponent_Dodge_TA_GetDodgeImpulse(obj: usize, DodgeDir: *mut Vector, result: *mut Vector);
	fn CarComponent_Dodge_TA_ApplyForces(obj: usize, ActiveTime: f32);
	fn CarComponent_Dodge_TA_CanActivate(obj: usize) -> bool;
	fn CarComponent_Dodge_TA_OnCreated(obj: usize);

}