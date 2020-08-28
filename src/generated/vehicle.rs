use crate::wrappers::*;
use crate::generated::*;

pub struct VehicleWrapper(pub usize);
impl_object!(VehicleWrapper);

impl Vehicle for VehicleWrapper {}
impl RBActor for VehicleWrapper {}
impl Actor for VehicleWrapper {}

pub trait Vehicle : RBActor {
    fn get_vehicle_sim(&self) -> VehicleSimWrapper {
        unsafe {
            VehicleSimWrapper::new(Vehicle_TA_Get_VehicleSim(self.addr()))
        }
    }
    fn get_sticky_force(&self) -> StickyForceData {
        unsafe {
            let mut result = StickyForceData::new();
            let result_ptr: *mut StickyForceData = &mut result as *mut StickyForceData;
            Vehicle_TA_Get_StickyForce(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_driving(&self) -> bool {
        unsafe {
            Vehicle_TA_Get_bDriving(self.addr())
        }
    }
    fn get_b_replicated_handbrake(&self) -> bool {
        unsafe {
            Vehicle_TA_Get_bReplicatedHandbrake(self.addr())
        }
    }
    fn get_b_jumped(&self) -> bool {
        unsafe {
            Vehicle_TA_Get_bJumped(self.addr())
        }
    }
    fn get_b_double_jumped(&self) -> bool {
        unsafe {
            Vehicle_TA_Get_bDoubleJumped(self.addr())
        }
    }
    fn get_b_on_ground(&self) -> bool {
        unsafe {
            Vehicle_TA_Get_bOnGround(self.addr())
        }
    }
    fn get_b_super_sonic(&self) -> bool {
        unsafe {
            Vehicle_TA_Get_bSuperSonic(self.addr())
        }
    }
    fn get_b_podium_mode(&self) -> bool {
        unsafe {
            Vehicle_TA_Get_bPodiumMode(self.addr())
        }
    }
    fn get_input(&self) -> VehicleInputs {
        unsafe {
            let mut result = VehicleInputs::new();
            let result_ptr: *mut VehicleInputs = &mut result as *mut VehicleInputs;
            Vehicle_TA_Get_Input(self.addr(), result_ptr);
            result
        }
    }
    fn get_replicated_throttle(&self) -> u8 {
        unsafe {
            Vehicle_TA_Get_ReplicatedThrottle(self.addr())
        }
    }
    fn get_replicated_steer(&self) -> u8 {
        unsafe {
            Vehicle_TA_Get_ReplicatedSteer(self.addr())
        }
    }
    fn get_player_controller(&self) -> PlayerControllerWrapper {
        unsafe {
            PlayerControllerWrapper::new(Vehicle_TA_Get_PlayerController(self.addr()))
        }
    }
    fn get_pri(&self) -> PriWrapper {
        unsafe {
            PriWrapper::new(Vehicle_TA_Get_PRI(self.addr()))
        }
    }
    fn get_vehicle_update_tag(&self) -> i32 {
        unsafe {
            Vehicle_TA_Get_VehicleUpdateTag(self.addr())
        }
    }
    fn get_local_collision_offset(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Vehicle_TA_Get_LocalCollisionOffset(self.addr(), result_ptr);
            result
        }
    }
    fn get_local_collision_extent(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Vehicle_TA_Get_LocalCollisionExtent(self.addr(), result_ptr);
            result
        }
    }
    fn get_last_ball_touch_frame(&self) -> i32 {
        unsafe {
            Vehicle_TA_Get_LastBallTouchFrame(self.addr())
        }
    }
    fn get_last_ball_impact_frame(&self) -> i32 {
        unsafe {
            Vehicle_TA_Get_LastBallImpactFrame(self.addr())
        }
    }
    fn get_boost_component(&self) -> BoostWrapper {
        unsafe {
            BoostWrapper::new(Vehicle_TA_Get_BoostComponent(self.addr()))
        }
    }
    fn get_dodge_component(&self) -> DodgeComponentWrapper {
        unsafe {
            DodgeComponentWrapper::new(Vehicle_TA_Get_DodgeComponent(self.addr()))
        }
    }
    fn get_air_control_component(&self) -> AirControlComponentWrapper {
        unsafe {
            AirControlComponentWrapper::new(Vehicle_TA_Get_AirControlComponent(self.addr()))
        }
    }
    fn get_jump_component(&self) -> JumpComponentWrapper {
        unsafe {
            JumpComponentWrapper::new(Vehicle_TA_Get_JumpComponent(self.addr()))
        }
    }
    fn get_double_jump_component(&self) -> DoubleJumpComponentWrapper {
        unsafe {
            DoubleJumpComponentWrapper::new(Vehicle_TA_Get_DoubleJumpComponent(self.addr()))
        }
    }
    fn get_time_below_supersonic_speed(&self) -> f32 {
        unsafe {
            Vehicle_TA_Get_TimeBelowSupersonicSpeed(self.addr())
        }
    }
    fn force_net_packet_if_near_ball(&self) {
        unsafe {
            Vehicle_TA_ForceNetPacketIfNearBall(self.addr());
        }
    }
    fn is_car_within_forward_elliptical_cone(&self, other_car: VehicleWrapper, yaw_angle_degrees: f32, pitch_angle_degrees: f32) -> bool {
        unsafe {
            Vehicle_TA_IsCarWithinForwardEllipticalCone(self.addr(), other_car.addr(), yaw_angle_degrees, pitch_angle_degrees)
        }
    }
    fn get_forward_speed(&self) -> f32 {
        unsafe {
            Vehicle_TA_GetForwardSpeed(self.addr())
        }
    }
    fn get_time_off_ground(&self) -> f32 {
        unsafe {
            Vehicle_TA_GetTimeOffGround(self.addr())
        }
    }
    fn get_time_on_ground(&self) -> f32 {
        unsafe {
            Vehicle_TA_GetTimeOnGround(self.addr())
        }
    }
    fn get_ground_normal(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Vehicle_TA_GetGroundNormal(self.addr(), result_ptr);
            result
        }
    }
    fn is_on_wall(&self) -> bool {
        unsafe {
            Vehicle_TA_IsOnWall(self.addr())
        }
    }
    fn is_on_ground(&self) -> bool {
        unsafe {
            Vehicle_TA_IsOnGround(self.addr())
        }
    }
    fn get_num_wheel_world_contacts(&self) -> i32 {
        unsafe {
            Vehicle_TA_GetNumWheelWorldContacts(self.addr())
        }
    }
    fn get_num_wheel_contacts(&self) -> i32 {
        unsafe {
            Vehicle_TA_GetNumWheelContacts(self.addr())
        }
    }
    fn zero_movement_variables(&self) {
        unsafe {
            Vehicle_TA_ZeroMovementVariables(self.addr());
        }
    }
    fn enable_podium_mode(&self) {
        unsafe {
            Vehicle_TA_EnablePodiumMode(self.addr());
        }
    }
    fn set_driving(&self, b_drive: bool) {
        unsafe {
            Vehicle_TA_SetDriving(self.addr(), b_drive);
        }
    }
    fn init_audio_params(&self) {
        unsafe {
            Vehicle_TA_InitAudioParams(self.addr());
        }
    }
    fn on_pri_changed(&self) {
        unsafe {
            Vehicle_TA_OnPRIChanged(self.addr());
        }
    }
    fn on_controller_changed(&self) {
        unsafe {
            Vehicle_TA_OnControllerChanged(self.addr());
        }
    }
    fn un_possessed(&self) {
        unsafe {
            Vehicle_TA_UnPossessed(self.addr());
        }
    }
    fn event_pri_changed(&self, vehicle: VehicleWrapper) {
        unsafe {
            Vehicle_TA_EventPRIChanged(self.addr(), vehicle.addr());
        }
    }

}

extern "C" {
    fn Vehicle_TA_Get_VehicleSim(obj: usize) -> usize;
    fn VehicleWrapper_SetVehicleSim(obj: usize, new_val: usize);
    fn Vehicle_TA_Get_StickyForce(obj: usize, result: *mut StickyForceData);
    fn VehicleWrapper_SetStickyForce(obj: usize, new_val: *mut StickyForceData);
    fn Vehicle_TA_Get_bDriving(obj: usize) -> bool;
    fn VehicleWrapper_SetbDriving(obj: usize, new_val: bool);
    fn Vehicle_TA_Get_bReplicatedHandbrake(obj: usize) -> bool;
    fn VehicleWrapper_SetbReplicatedHandbrake(obj: usize, new_val: bool);
    fn Vehicle_TA_Get_bJumped(obj: usize) -> bool;
    fn VehicleWrapper_SetbJumped(obj: usize, new_val: bool);
    fn Vehicle_TA_Get_bDoubleJumped(obj: usize) -> bool;
    fn VehicleWrapper_SetbDoubleJumped(obj: usize, new_val: bool);
    fn Vehicle_TA_Get_bOnGround(obj: usize) -> bool;
    fn VehicleWrapper_SetbOnGround(obj: usize, new_val: bool);
    fn Vehicle_TA_Get_bSuperSonic(obj: usize) -> bool;
    fn VehicleWrapper_SetbSuperSonic(obj: usize, new_val: bool);
    fn Vehicle_TA_Get_bPodiumMode(obj: usize) -> bool;
    fn VehicleWrapper_SetbPodiumMode(obj: usize, new_val: bool);
    fn Vehicle_TA_Get_Input(obj: usize, result: *mut VehicleInputs);
    fn VehicleWrapper_SetInput(obj: usize, new_val: *mut VehicleInputs);
    fn Vehicle_TA_Get_ReplicatedThrottle(obj: usize) -> u8;
    fn VehicleWrapper_SetReplicatedThrottle(obj: usize, new_val: u8);
    fn Vehicle_TA_Get_ReplicatedSteer(obj: usize) -> u8;
    fn VehicleWrapper_SetReplicatedSteer(obj: usize, new_val: u8);
    fn Vehicle_TA_Get_PlayerController(obj: usize) -> usize;
    fn VehicleWrapper_SetPlayerController(obj: usize, new_val: usize);
    fn Vehicle_TA_Get_PRI(obj: usize) -> usize;
    fn VehicleWrapper_SetPRI(obj: usize, new_val: usize);
    fn Vehicle_TA_Get_VehicleUpdateTag(obj: usize) -> i32;
    fn VehicleWrapper_SetVehicleUpdateTag(obj: usize, new_val: i32);
    fn Vehicle_TA_Get_LocalCollisionOffset(obj: usize, result: *mut Vector);
    fn VehicleWrapper_SetLocalCollisionOffset(obj: usize, new_val: *mut Vector);
    fn Vehicle_TA_Get_LocalCollisionExtent(obj: usize, result: *mut Vector);
    fn VehicleWrapper_SetLocalCollisionExtent(obj: usize, new_val: *mut Vector);
    fn Vehicle_TA_Get_LastBallTouchFrame(obj: usize) -> i32;
    fn VehicleWrapper_SetLastBallTouchFrame(obj: usize, new_val: i32);
    fn Vehicle_TA_Get_LastBallImpactFrame(obj: usize) -> i32;
    fn VehicleWrapper_SetLastBallImpactFrame(obj: usize, new_val: i32);
    fn Vehicle_TA_Get_BoostComponent(obj: usize) -> usize;
    fn Vehicle_TA_Get_DodgeComponent(obj: usize) -> usize;
    fn Vehicle_TA_Get_AirControlComponent(obj: usize) -> usize;
    fn Vehicle_TA_Get_JumpComponent(obj: usize) -> usize;
    fn Vehicle_TA_Get_DoubleJumpComponent(obj: usize) -> usize;
    fn VehicleWrapper_SetDoubleJumpComponent(obj: usize, new_val: usize);
    fn Vehicle_TA_Get_TimeBelowSupersonicSpeed(obj: usize) -> f32;
    fn VehicleWrapper_SetTimeBelowSupersonicSpeed(obj: usize, new_val: f32);
    fn Vehicle_TA_ForceNetPacketIfNearBall(obj: usize);
    fn Vehicle_TA_IsCarWithinForwardEllipticalCone(obj: usize, OtherCar: usize, YawAngleDegrees: f32, PitchAngleDegrees: f32) -> bool;
    fn Vehicle_TA_GetForwardSpeed(obj: usize) -> f32;
    fn Vehicle_TA_GetTimeOffGround(obj: usize) -> f32;
    fn Vehicle_TA_GetTimeOnGround(obj: usize) -> f32;
    fn Vehicle_TA_GetGroundNormal(obj: usize, result: *mut Vector);
    fn Vehicle_TA_IsOnWall(obj: usize) -> bool;
    fn Vehicle_TA_IsOnGround(obj: usize) -> bool;
    fn Vehicle_TA_GetNumWheelWorldContacts(obj: usize) -> i32;
    fn Vehicle_TA_GetNumWheelContacts(obj: usize) -> i32;
    fn Vehicle_TA_ZeroMovementVariables(obj: usize);
    fn Vehicle_TA_EnablePodiumMode(obj: usize);
    fn Vehicle_TA_SetDriving(obj: usize, bDrive: bool);
    fn Vehicle_TA_InitAudioParams(obj: usize);
    fn Vehicle_TA_OnPRIChanged(obj: usize);
    fn Vehicle_TA_OnControllerChanged(obj: usize);
    fn Vehicle_TA_UnPossessed(obj: usize);
    fn Vehicle_TA_EventPRIChanged(obj: usize, Vehicle: usize);

}