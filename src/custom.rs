use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use crate::*;

pub trait CustomWrappers : Object {
    fn demolish(&self) {
        unsafe { Car_Demolish(self.addr()); }
    }

    fn get_location(&self) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Actor_GetLocation(self.addr(), result_ptr); }
        result
    }

    fn set_location(&self, new_loc: Vector) {
        let mut new_loc = new_loc;
        let new_loc = &mut new_loc as *mut Vector;
        unsafe { Actor_SetLocation(self.addr(), new_loc); }
    }

    fn get_velocity(&self) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Actor_GetVelocity(self.addr(), result_ptr); }
        result
    }

    fn set_velocity(&self, new_vel: Vector) {
        let mut new_vel = new_vel;
        let new_vel = &mut new_vel as *mut Vector;
        unsafe { Actor_SetVelocity(self.addr(), new_vel); }
    }

    fn add_velocity(&self, vel: Vector) {
        let mut vel = vel;
        let vel = &mut vel as *mut Vector;
        unsafe { Actor_AddVelocity(self.addr(), vel); }
    }
    
    fn get_rotation(&self) -> Rotator {
        let mut result = Rotator::new();
        let result_ptr = &mut result as *mut Rotator;
        unsafe { Actor_GetRotation(self.addr(), result_ptr); }
        result
    }

    fn set_rotation(&self, new_rot: Rotator) {
        let mut new_rot = new_rot;
        let new_rot = &mut new_rot as *mut Rotator;
        unsafe { Actor_SetRotation(self.addr(), new_rot); }
    }

    fn set_torque(&self, new_torque: Vector) {
        let mut new_torque = new_torque;
        let new_torque = &mut new_torque as *mut Vector;
        unsafe { Actor_SetTorque(self.addr(), new_torque); }
    }

    fn get_angular_velocity(&self) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Actor_GetAngularVelocity(self.addr(), result_ptr); }
        result
    }

    fn set_angular_velocity(&self, new_vel: Vector, add_to_current: bool) {
        let mut new_vel = new_vel;
        let new_vel = &mut new_vel as *mut Vector;
        unsafe { Actor_SetAngularVelocity(self.addr(), new_vel, add_to_current); }
    }

    fn stop(&self) {
        unsafe { Actor_Stop(self.addr()) }
    }

    fn is_null(&self) {
        unsafe { Actor_IsNull(self.addr()) }
    }



    fn get_ball(&self) -> Option<BallWrapper> {
        unsafe { BallWrapper::try_new(Server_GetBall(self.addr())) }
    }

    fn spawn_car(&self, car_body: i32, name: &str) {
        let c_name = CString::new(name).unwrap();
        let c_name: *const c_char = c_name.as_ptr();
        unsafe { Server_SpawnCar(self.addr(), car_body, c_name) }
    }

    fn spawn_bot(&self, car_body: i32, name: &str) {
        let c_name = CString::new(name).unwrap();
        let c_name: *const c_char = c_name.as_ptr();
        unsafe { Server_SpawnBot(self.addr(), car_body, c_name) }
    }

    fn spawn_ball(&self, position: Vector, wake: bool, spawn_cannon: bool) -> Option<BallWrapper> {
        let mut position = position;
        let position = &mut position as *mut Vector;
        unsafe { BallWrapper::try_new(Server_SpawnBall(self.addr(), position, wake, spawn_cannon)) }
    }

    fn has_authority(&self) -> bool {
        unsafe { Server_HasAuthority(self.addr()) }
    }

    fn set_game_speed(&self, game_speed: f32) {
        unsafe { Server_SetGameSpeed(self.addr(), game_speed); }
    }

    fn get_game_speed(&self) -> f32 {
        unsafe { Server_GetGameSpeed(self.addr()) }
    }

    fn set_seconds_elapsed(&self, seconds_elapsed: f32) {
        unsafe { Server_SetSecondsElapsed(self.addr(), seconds_elapsed); }
    }

    fn get_seconds_elapsed(&self) -> f32 {
        unsafe { Server_GetSecondsElapsed(self.addr()) }
    }

    fn get_game_car(&self) -> Option<CarWrapper> {
        unsafe { CarWrapper::try_new(Server_GetGameCar(self.addr())) }
    }

    fn is_ball_moving_towards_goal(&self, goal_no: i32, ball: BallWrapper) -> bool {
        unsafe { Server_IsBallMovingTowardsGoal(self.addr(), goal_no, ball.addr()) }
    }

    fn is_in_goal(&self, vec: Vector) -> bool {
        let mut vec = vec;
        let vec = &mut vec as *mut Vector;
        unsafe { Server_IsInGoal(self.addr(), vec) }
    }

    fn disable_goal_reset(&self) {
        unsafe { Server_DisableGoalReset(self.addr()) }
    }

    fn enable_goal_reset(&self) {
        unsafe { Server_EnableGoalReset(self.addr()) }
    }

    fn generate_shot(&self, start_pos: Vector, destination: Vector, speed: f32) -> Vector {
        let mut start_pos = start_pos;
        let start_pos = &mut start_pos as *mut Vector;
        let mut destination = destination;
        let destination = &mut destination as *mut Vector;
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Server_GenerateShot(self.addr(), start_pos, destination, speed, result_ptr) }
        result
    }

    fn generate_goal_aim_location(&self, goal_number: i32, current_ball_location: Vector) -> Vector {
        let mut current_ball_location = current_ball_location;
        let current_ball_location = &mut current_ball_location as *mut Vector;
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Server_GenerateGoalAimLocation(self.addr(), goal_number, current_ball_location, result_ptr)}
        result
    }

    fn get_goal_extent(&self, goal_number: i32) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Server_GetGoalExtent(self.addr(), goal_number, result_ptr)}
        result
    }

    fn get_goal_location(&self, goal_number: i32) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Server_GetGoalLocation(self.addr(), goal_number, result_ptr)}
        result
    }

}

extern "C" {
    fn Car_Demolish(p_car: usize);

    fn Actor_GetLocation(p_actor: usize, result: *mut Vector);
    fn Actor_SetLocation(p_actor: usize, new_loc: *mut Vector);
    fn Actor_GetVelocity(p_actor: usize, result: *mut Vector);
    fn Actor_SetVelocity(p_actor: usize, velocity: *mut Vector);
    fn Actor_AddVelocity(p_actor: usize, velocity: *mut Vector);
    fn Actor_GetRotation(p_actor: usize, result: *mut Rotator);
    fn Actor_SetRotation(p_actor: usize, rotation: *mut Rotator);
    fn Actor_SetTorque(p_actor: usize, torq: *mut Vector);
    fn Actor_Stop(p_actor: usize);
    fn Actor_GetAngularVelocity(p_actor: usize, result: *mut Vector);
    fn Actor_SetAngularVelocity(p_actor: usize, v: *mut Vector, add_to_current: bool);
    fn Actor_IsNull(p_actor: usize);

    fn Server_GetBall(obj: usize) -> usize;
    fn Server_SpawnCar(obj: usize, carBody: i32, name: *const c_char);
    fn Server_SpawnBot(obj: usize, carBody: i32, name: *const c_char);
    fn Server_SpawnBall(obj: usize, position: *mut Vector, wake: bool, spawnCannon: bool) -> usize;
    fn Server_HasAuthority(obj: usize) -> bool;
    fn Server_SetGameSpeed(obj: usize, GameSpeed: f32);
    fn Server_GetGameSpeed(obj: usize) -> f32;
    fn Server_SetSecondsElapsed(obj: usize, SecondsElapsed: f32);
    fn Server_GetSecondsElapsed(obj: usize) -> f32;
    fn Server_GetGameCar(obj: usize) -> usize;
    fn Server_IsBallMovingTowardsGoal(obj: usize, goalNo: i32, bw: usize) -> bool;
    fn Server_IsInGoal(obj: usize, vec: *mut Vector) -> bool;
    fn Server_DisableGoalReset(obj: usize);
    fn Server_EnableGoalReset(obj: usize);
    fn Server_GenerateShot(obj: usize, startPos: *mut Vector, destination: *mut Vector, speed: f32, result: *mut Vector);
    fn Server_GenerateGoalAimLocation(obj: usize, goalNumber: i32, currentBallLocation: *mut Vector, result: *mut Vector);
    fn Server_GetGoalExtent(obj: usize, goalNumber: i32, result: *mut Vector);
    fn Server_GetGoalLocation(obj: usize, goalNumber: i32, result: *mut Vector);
}