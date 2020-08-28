use crate::wrappers::*;
use crate::generated::*;

pub struct VehicleSimWrapper(pub usize);
impl_object!(VehicleSimWrapper);

impl VehicleSim for VehicleSimWrapper {}

pub trait VehicleSim : Object {
    fn get_wheels(&self) -> RLArray<WheelWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            VehicleSim_TA_Get_Wheels(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_drive_torque(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_DriveTorque(self.addr())
        }
    }
    fn get_brake_torque(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_BrakeTorque(self.addr())
        }
    }
    fn get_stop_threshold(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_StopThreshold(self.addr())
        }
    }
    fn get_idle_brake_factor(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_IdleBrakeFactor(self.addr())
        }
    }
    fn get_opposite_brake_factor(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_OppositeBrakeFactor(self.addr())
        }
    }
    fn get_b_use_ackermann_steering(&self) -> bool {
        unsafe {
            VehicleSim_TA_Get_bUseAckermannSteering(self.addr())
        }
    }
    fn get_b_was_attached(&self) -> bool {
        unsafe {
            VehicleSim_TA_Get_bWasAttached(self.addr())
        }
    }
    fn get_output_throttle(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_OutputThrottle(self.addr())
        }
    }
    fn get_output_steer(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_OutputSteer(self.addr())
        }
    }
    fn get_output_brake(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_OutputBrake(self.addr())
        }
    }
    fn get_output_handbrake(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_OutputHandbrake(self.addr())
        }
    }
    fn get_vehicle(&self) -> VehicleWrapper {
        unsafe {
            VehicleWrapper::new(VehicleSim_TA_Get_Vehicle(self.addr()))
        }
    }
    fn get_car(&self) -> CarWrapper {
        unsafe {
            CarWrapper::new(VehicleSim_TA_Get_Car(self.addr()))
        }
    }
    fn get_steering_sensitivity(&self) -> f32 {
        unsafe {
            VehicleSim_TA_Get_SteeringSensitivity(self.addr())
        }
    }

}

extern "C" {
    fn VehicleSim_TA_Get_Wheels(obj: usize, result: *mut RLArrayRaw);
    fn VehicleSim_TA_Get_DriveTorque(obj: usize) -> f32;
    fn VehicleSimWrapper_SetDriveTorque(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_BrakeTorque(obj: usize) -> f32;
    fn VehicleSimWrapper_SetBrakeTorque(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_StopThreshold(obj: usize) -> f32;
    fn VehicleSimWrapper_SetStopThreshold(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_IdleBrakeFactor(obj: usize) -> f32;
    fn VehicleSimWrapper_SetIdleBrakeFactor(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_OppositeBrakeFactor(obj: usize) -> f32;
    fn VehicleSimWrapper_SetOppositeBrakeFactor(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_bUseAckermannSteering(obj: usize) -> bool;
    fn VehicleSimWrapper_SetbUseAckermannSteering(obj: usize, new_val: bool);
    fn VehicleSim_TA_Get_bWasAttached(obj: usize) -> bool;
    fn VehicleSimWrapper_SetbWasAttached(obj: usize, new_val: bool);
    fn VehicleSim_TA_Get_OutputThrottle(obj: usize) -> f32;
    fn VehicleSimWrapper_SetOutputThrottle(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_OutputSteer(obj: usize) -> f32;
    fn VehicleSimWrapper_SetOutputSteer(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_OutputBrake(obj: usize) -> f32;
    fn VehicleSimWrapper_SetOutputBrake(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_OutputHandbrake(obj: usize) -> f32;
    fn VehicleSimWrapper_SetOutputHandbrake(obj: usize, new_val: f32);
    fn VehicleSim_TA_Get_Vehicle(obj: usize) -> usize;
    fn VehicleSimWrapper_SetVehicle(obj: usize, new_val: usize);
    fn VehicleSim_TA_Get_Car(obj: usize) -> usize;
    fn VehicleSimWrapper_SetCar(obj: usize, new_val: usize);
    fn VehicleSim_TA_Get_SteeringSensitivity(obj: usize) -> f32;
    fn VehicleSimWrapper_SetSteeringSensitivity(obj: usize, new_val: f32);

}