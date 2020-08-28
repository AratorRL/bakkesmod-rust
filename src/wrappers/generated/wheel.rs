use crate::wrappers::*;
use super::*;

pub struct WheelWrapper(pub usize);
impl_object!(WheelWrapper);

impl Wheel for WheelWrapper {}

pub trait Wheel : Object {
    fn get_steer_factor(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_SteerFactor(self.addr())
        }
    }
    fn get_wheel_radius(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_WheelRadius(self.addr())
        }
    }
    fn get_suspension_stiffness(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_SuspensionStiffness(self.addr())
        }
    }
    fn get_suspension_damping_compression(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_SuspensionDampingCompression(self.addr())
        }
    }
    fn get_suspension_damping_relaxation(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_SuspensionDampingRelaxation(self.addr())
        }
    }
    fn get_suspension_travel(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_SuspensionTravel(self.addr())
        }
    }
    fn get_suspension_max_raise(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_SuspensionMaxRaise(self.addr())
        }
    }
    fn get_contact_force_distance(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_ContactForceDistance(self.addr())
        }
    }
    fn get_spin_speed_decay_rate(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_SpinSpeedDecayRate(self.addr())
        }
    }
    fn get_bone_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Wheel_TA_Get_BoneOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_preset_rest_position(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Wheel_TA_Get_PresetRestPosition(self.addr(), result_ptr);
            result
        }
    }
    fn get_local_suspension_ray_start(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Wheel_TA_Get_LocalSuspensionRayStart(self.addr(), result_ptr);
            result
        }
    }
    fn get_local_rest_position(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Wheel_TA_Get_LocalRestPosition(self.addr(), result_ptr);
            result
        }
    }
    fn get_vehicle_sim(&self) -> Option<VehicleSimWrapper> {
        unsafe {
            VehicleSimWrapper::try_new(Wheel_TA_Get_VehicleSim(self.addr()))
        }
    }
    fn get_wheel_index(&self) -> i32 {
        unsafe {
            Wheel_TA_Get_WheelIndex(self.addr())
        }
    }
    fn get_contact(&self) -> WheelContactData {
        unsafe {
            let mut result = WheelContactData::new();
            let result_ptr: *mut WheelContactData = &mut result as *mut WheelContactData;
            Wheel_TA_Get_Contact(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_draw_debug(&self) -> bool {
        unsafe {
            Wheel_TA_Get_bDrawDebug(self.addr())
        }
    }
    fn get_b_had_contact(&self) -> bool {
        unsafe {
            Wheel_TA_Get_bHadContact(self.addr())
        }
    }
    fn get_friction_curve_input(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_FrictionCurveInput(self.addr())
        }
    }
    fn get_aerial_throttle_to_velocity_factor(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_AerialThrottleToVelocityFactor(self.addr())
        }
    }
    fn get_aerial_acceleration_factor(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_AerialAccelerationFactor(self.addr())
        }
    }
    fn get_spin_speed(&self) -> f32 {
        unsafe {
            Wheel_TA_Get_SpinSpeed(self.addr())
        }
    }
    fn get_ref_wheel_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Wheel_TA_GetRefWheelLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_suspension_distance(&self) -> f32 {
        unsafe {
            Wheel_TA_GetSuspensionDistance(self.addr())
        }
    }
    fn get_linear_velocity(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Wheel_TA_GetLinearVelocity(self.addr(), result_ptr);
            result
        }
    }
    fn event_contact_changed(&self, wheel: WheelWrapper) {
        unsafe {
            Wheel_TA_EventContactChanged(self.addr(), wheel.addr());
        }
    }

}

extern "C" {
    fn Wheel_TA_Get_SteerFactor(obj: usize) -> f32;
    fn WheelWrapper_SetSteerFactor(obj: usize, new_val: f32);
    fn Wheel_TA_Get_WheelRadius(obj: usize) -> f32;
    fn WheelWrapper_SetWheelRadius(obj: usize, new_val: f32);
    fn Wheel_TA_Get_SuspensionStiffness(obj: usize) -> f32;
    fn WheelWrapper_SetSuspensionStiffness(obj: usize, new_val: f32);
    fn Wheel_TA_Get_SuspensionDampingCompression(obj: usize) -> f32;
    fn WheelWrapper_SetSuspensionDampingCompression(obj: usize, new_val: f32);
    fn Wheel_TA_Get_SuspensionDampingRelaxation(obj: usize) -> f32;
    fn WheelWrapper_SetSuspensionDampingRelaxation(obj: usize, new_val: f32);
    fn Wheel_TA_Get_SuspensionTravel(obj: usize) -> f32;
    fn WheelWrapper_SetSuspensionTravel(obj: usize, new_val: f32);
    fn Wheel_TA_Get_SuspensionMaxRaise(obj: usize) -> f32;
    fn WheelWrapper_SetSuspensionMaxRaise(obj: usize, new_val: f32);
    fn Wheel_TA_Get_ContactForceDistance(obj: usize) -> f32;
    fn WheelWrapper_SetContactForceDistance(obj: usize, new_val: f32);
    fn Wheel_TA_Get_SpinSpeedDecayRate(obj: usize) -> f32;
    fn WheelWrapper_SetSpinSpeedDecayRate(obj: usize, new_val: f32);
    fn Wheel_TA_Get_BoneOffset(obj: usize, result: *mut Vector);
    fn WheelWrapper_SetBoneOffset(obj: usize, new_val: *mut Vector);
    fn Wheel_TA_Get_PresetRestPosition(obj: usize, result: *mut Vector);
    fn WheelWrapper_SetPresetRestPosition(obj: usize, new_val: *mut Vector);
    fn Wheel_TA_Get_LocalSuspensionRayStart(obj: usize, result: *mut Vector);
    fn WheelWrapper_SetLocalSuspensionRayStart(obj: usize, new_val: *mut Vector);
    fn Wheel_TA_Get_LocalRestPosition(obj: usize, result: *mut Vector);
    fn WheelWrapper_SetLocalRestPosition(obj: usize, new_val: *mut Vector);
    fn Wheel_TA_Get_VehicleSim(obj: usize) -> usize;
    fn WheelWrapper_SetVehicleSim(obj: usize, new_val: usize);
    fn Wheel_TA_Get_WheelIndex(obj: usize) -> i32;
    fn WheelWrapper_SetWheelIndex(obj: usize, new_val: i32);
    fn Wheel_TA_Get_Contact(obj: usize, result: *mut WheelContactData);
    fn WheelWrapper_SetContact(obj: usize, new_val: *mut WheelContactData);
    fn Wheel_TA_Get_bDrawDebug(obj: usize) -> bool;
    fn WheelWrapper_SetbDrawDebug(obj: usize, new_val: bool);
    fn Wheel_TA_Get_bHadContact(obj: usize) -> bool;
    fn WheelWrapper_SetbHadContact(obj: usize, new_val: bool);
    fn Wheel_TA_Get_FrictionCurveInput(obj: usize) -> f32;
    fn WheelWrapper_SetFrictionCurveInput(obj: usize, new_val: f32);
    fn Wheel_TA_Get_AerialThrottleToVelocityFactor(obj: usize) -> f32;
    fn WheelWrapper_SetAerialThrottleToVelocityFactor(obj: usize, new_val: f32);
    fn Wheel_TA_Get_AerialAccelerationFactor(obj: usize) -> f32;
    fn WheelWrapper_SetAerialAccelerationFactor(obj: usize, new_val: f32);
    fn Wheel_TA_Get_SpinSpeed(obj: usize) -> f32;
    fn WheelWrapper_SetSpinSpeed(obj: usize, new_val: f32);
    fn Wheel_TA_GetRefWheelLocation(obj: usize, result: *mut Vector);
    fn Wheel_TA_GetSuspensionDistance(obj: usize) -> f32;
    fn Wheel_TA_GetLinearVelocity(obj: usize, result: *mut Vector);
    fn Wheel_TA_EventContactChanged(obj: usize, Wheel: usize);

}