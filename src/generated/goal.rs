use crate::wrappers::*;
use crate::generated::*;

pub struct GoalWrapper(pub usize);
impl_object!(GoalWrapper);

impl Goal for GoalWrapper {}

pub trait Goal : Object {
	fn get_goal_orientation(&self) -> Option<ActorWrapper> {
		unsafe {
			ActorWrapper::try_new(Goal_TA_Get_GoalOrientation(self.addr()))
		}
	}
	fn get_override_goal_indicator_orientations(&self) -> RLArray<ActorWrapper> {
		unsafe {
			let mut result = RLArrayRaw::new();
			let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
			Goal_TA_Get_OverrideGoalIndicatorOrientations(self.addr(), result_ptr);
			RLArray::from_raw(result)
		}
	}
	fn get_team_num(&self) -> u8 {
		unsafe {
			Goal_TA_Get_TeamNum(self.addr())
		}
	}
	fn get_score_fx(&self) -> Option<FXActorWrapper> {
		unsafe {
			FXActorWrapper::try_new(Goal_TA_Get_ScoreFX(self.addr()))
		}
	}
	fn get_goal_indicator_archetype(&self) -> RLString {
		unsafe {
			let mut result = RLString::new();
			let result_ptr: *mut RLString = &mut result as *mut RLString;
			Goal_TA_Get_GoalIndicatorArchetype(self.addr(), result_ptr);
			result
		}
	}
	fn get_b_no_goal_indicator(&self) -> bool {
		unsafe {
			Goal_TA_Get_bNoGoalIndicator(self.addr())
		}
	}
	fn get_b_only_goals_from_direction(&self) -> bool {
		unsafe {
			Goal_TA_Get_bOnlyGoalsFromDirection(self.addr())
		}
	}
	fn get_b_show_focus_extent(&self) -> bool {
		unsafe {
			Goal_TA_Get_bShowFocusExtent(self.addr())
		}
	}
	fn get_goal_direction(&self) -> Option<ActorWrapper> {
		unsafe {
			ActorWrapper::try_new(Goal_TA_Get_GoalDirection(self.addr()))
		}
	}
	fn get_points_to_award(&self) -> i32 {
		unsafe {
			Goal_TA_Get_PointsToAward(self.addr())
		}
	}
	fn get_auto_cam_focus_extent(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_AutoCamFocusExtent(self.addr(), result_ptr);
			result
		}
	}
	fn get_goal_focus_location_offset(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_GoalFocusLocationOffset(self.addr(), result_ptr);
			result
		}
	}
	fn get_max_goal_scorer_attach_radius(&self) -> f32 {
		unsafe {
			Goal_TA_Get_MaxGoalScorerAttachRadius(self.addr())
		}
	}
	fn get_goal_scored_dot_direction(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_GoalScoredDotDirection(self.addr(), result_ptr);
			result
		}
	}
	fn get_min_attach_goal_to_scorer_dot(&self) -> f32 {
		unsafe {
			Goal_TA_Get_MinAttachGoalToScorerDot(self.addr())
		}
	}
	fn get_location(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_Location(self.addr(), result_ptr);
			result
		}
	}
	fn get_direction(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_Direction(self.addr(), result_ptr);
			result
		}
	}
	fn get_right(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_Right(self.addr(), result_ptr);
			result
		}
	}
	fn get_up(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_Up(self.addr(), result_ptr);
			result
		}
	}
	fn get_rotation(&self) -> Rotator {
		unsafe {
			let mut result = Rotator::new();
			let result_ptr: *mut Rotator = &mut result as *mut Rotator;
			Goal_TA_Get_Rotation(self.addr(), result_ptr);
			result
		}
	}
	fn get_local_extent(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_LocalExtent(self.addr(), result_ptr);
			result
		}
	}
	fn get_world_center(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_WorldCenter(self.addr(), result_ptr);
			result
		}
	}
	fn get_world_extent(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_WorldExtent(self.addr(), result_ptr);
			result
		}
	}
	fn get_world_front_center(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_Get_WorldFrontCenter(self.addr(), result_ptr);
			result
		}
	}
	fn get_goal_focus_extent_center(&self) -> Vector {
		unsafe {
			let mut result = Vector::new();
			let result_ptr: *mut Vector = &mut result as *mut Vector;
			Goal_TA_GetGoalFocusExtentCenter(self.addr(), result_ptr);
			result
		}
	}
	fn init(&self) {
		unsafe {
			Goal_TA_Init(self.addr());
		}
	}

}

extern "C" {
	fn Goal_TA_Get_GoalOrientation(obj: usize) -> usize;
	fn GoalWrapper_SetGoalOrientation(obj: usize, new_val: usize);
	fn Goal_TA_Get_OverrideGoalIndicatorOrientations(obj: usize, result: *mut RLArrayRaw);
	fn Goal_TA_Get_TeamNum(obj: usize) -> u8;
	fn GoalWrapper_SetTeamNum(obj: usize, new_val: u8);
	fn Goal_TA_Get_ScoreFX(obj: usize) -> usize;
	fn GoalWrapper_SetScoreFX(obj: usize, new_val: usize);
	fn Goal_TA_Get_GoalIndicatorArchetype(obj: usize, result: *mut RLString);
	fn Goal_TA_Get_bNoGoalIndicator(obj: usize) -> bool;
	fn GoalWrapper_SetbNoGoalIndicator(obj: usize, new_val: bool);
	fn Goal_TA_Get_bOnlyGoalsFromDirection(obj: usize) -> bool;
	fn GoalWrapper_SetbOnlyGoalsFromDirection(obj: usize, new_val: bool);
	fn Goal_TA_Get_bShowFocusExtent(obj: usize) -> bool;
	fn GoalWrapper_SetbShowFocusExtent(obj: usize, new_val: bool);
	fn Goal_TA_Get_GoalDirection(obj: usize) -> usize;
	fn GoalWrapper_SetGoalDirection(obj: usize, new_val: usize);
	fn Goal_TA_Get_PointsToAward(obj: usize) -> i32;
	fn GoalWrapper_SetPointsToAward(obj: usize, new_val: i32);
	fn Goal_TA_Get_AutoCamFocusExtent(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetAutoCamFocusExtent(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_GoalFocusLocationOffset(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetGoalFocusLocationOffset(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_MaxGoalScorerAttachRadius(obj: usize) -> f32;
	fn GoalWrapper_SetMaxGoalScorerAttachRadius(obj: usize, new_val: f32);
	fn Goal_TA_Get_GoalScoredDotDirection(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetGoalScoredDotDirection(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_MinAttachGoalToScorerDot(obj: usize) -> f32;
	fn GoalWrapper_SetMinAttachGoalToScorerDot(obj: usize, new_val: f32);
	fn Goal_TA_Get_Location(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetLocation(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_Direction(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetDirection(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_Right(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetRight(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_Up(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetUp(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_Rotation(obj: usize, result: *mut Rotator);
	fn GoalWrapper_SetRotation(obj: usize, new_val: *mut Rotator);
	fn Goal_TA_Get_LocalExtent(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetLocalExtent(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_WorldCenter(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetWorldCenter(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_WorldExtent(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetWorldExtent(obj: usize, new_val: *mut Vector);
	fn Goal_TA_Get_WorldFrontCenter(obj: usize, result: *mut Vector);
	fn GoalWrapper_SetWorldFrontCenter(obj: usize, new_val: *mut Vector);
	fn Goal_TA_GetGoalFocusExtentCenter(obj: usize, result: *mut Vector);
	fn Goal_TA_Init(obj: usize);

}