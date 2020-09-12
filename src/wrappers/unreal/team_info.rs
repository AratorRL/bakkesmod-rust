use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct TeamInfoWrapper(pub usize);
impl_object!(TeamInfoWrapper);

impl TeamInfo for TeamInfoWrapper {}
impl Actor for TeamInfoWrapper {}

pub trait TeamInfo : Actor {
    fn get_team_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            TeamInfo_Get_TeamName(self.addr(), result_ptr);
            result
        }
    }
    fn get_size(&self) -> i32 {
        unsafe {
            TeamInfo_Get_Size(self.addr())
        }
    }
    fn get_score(&self) -> i32 {
        unsafe {
            TeamInfo_Get_Score(self.addr())
        }
    }
    fn get_team_index(&self) -> i32 {
        unsafe {
            TeamInfo_Get_TeamIndex(self.addr())
        }
    }
    fn get_team_color(&self) -> Color {
        unsafe {
            let mut result = Color::new();
            let result_ptr: *mut Color = &mut result as *mut Color;
            TeamInfo_Get_TeamColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_team_num(&self) -> u8 {
        unsafe {
            TeamInfo_GetTeamNum(self.addr())
        }
    }

}

extern "C" {
    fn TeamInfo_Get_TeamName(obj: usize, result: *mut RLString);
    fn TeamInfo_Get_Size(obj: usize) -> i32;
    fn TeamInfoWrapper_SetSize(obj: usize, new_val: i32);
    fn TeamInfo_Get_Score(obj: usize) -> i32;
    fn TeamInfoWrapper_SetScore(obj: usize, new_val: i32);
    fn TeamInfo_Get_TeamIndex(obj: usize) -> i32;
    fn TeamInfoWrapper_SetTeamIndex(obj: usize, new_val: i32);
    fn TeamInfo_Get_TeamColor(obj: usize, result: *mut Color);
    fn TeamInfoWrapper_SetTeamColor(obj: usize, new_val: *mut Color);
    fn TeamInfo_GetTeamNum(obj: usize) -> u8;

}