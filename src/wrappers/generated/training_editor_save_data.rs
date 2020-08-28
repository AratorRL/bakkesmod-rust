use crate::wrappers::*;
use super::*;

pub struct TrainingEditorSaveDataWrapper(pub usize);
impl_object!(TrainingEditorSaveDataWrapper);

impl TrainingEditorSaveData for TrainingEditorSaveDataWrapper {}

pub trait TrainingEditorSaveData : Object {
    fn get_code(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            TrainingEditorData_TA_Get_Code(self.addr(), result_ptr);
            result
        }
    }
    fn get_tm_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            TrainingEditorData_TA_Get_TM_Name(self.addr(), result_ptr);
            result
        }
    }
    fn get_type(&self) -> u8 {
        unsafe {
            TrainingEditorData_TA_Get_Type(self.addr())
        }
    }
    fn get_difficulty(&self) -> u8 {
        unsafe {
            TrainingEditorData_TA_Get_Difficulty(self.addr())
        }
    }
    fn get_creator_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            TrainingEditorData_TA_Get_CreatorName(self.addr(), result_ptr);
            result
        }
    }
    fn get_description(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            TrainingEditorData_TA_Get_Description(self.addr(), result_ptr);
            result
        }
    }
    fn get_num_rounds(&self) -> i32 {
        unsafe {
            TrainingEditorData_TA_Get_NumRounds(self.addr())
        }
    }
    fn get_created_at(&self) -> i64 {
        unsafe {
            TrainingEditorData_TA_Get_CreatedAt(self.addr())
        }
    }
    fn get_updated_at(&self) -> i64 {
        unsafe {
            TrainingEditorData_TA_Get_UpdatedAt(self.addr())
        }
    }
    fn get_creator_player_id(&self) -> UniqueNetId {
        unsafe {
            let mut result = UniqueNetId::new();
            let result_ptr: *mut UniqueNetId = &mut result as *mut UniqueNetId;
            TrainingEditorData_TA_Get_CreatorPlayerID(self.addr(), result_ptr);
            result
        }
    }
    fn init(&self) {
        unsafe {
            TrainingEditorData_TA_Init(self.addr());
        }
    }

}

extern "C" {
    fn TrainingEditorData_TA_Get_Code(obj: usize, result: *mut RLString);
    fn TrainingEditorData_TA_Get_TM_Name(obj: usize, result: *mut RLString);
    fn TrainingEditorData_TA_Get_Type(obj: usize) -> u8;
    fn TrainingEditorSaveDataWrapper_SetType(obj: usize, new_val: u8);
    fn TrainingEditorData_TA_Get_Difficulty(obj: usize) -> u8;
    fn TrainingEditorSaveDataWrapper_SetDifficulty(obj: usize, new_val: u8);
    fn TrainingEditorData_TA_Get_CreatorName(obj: usize, result: *mut RLString);
    fn TrainingEditorData_TA_Get_Description(obj: usize, result: *mut RLString);
    fn TrainingEditorData_TA_Get_NumRounds(obj: usize) -> i32;
    fn TrainingEditorSaveDataWrapper_SetNumRounds(obj: usize, new_val: i32);
    fn TrainingEditorData_TA_Get_CreatedAt(obj: usize) -> i64;
    fn TrainingEditorSaveDataWrapper_SetCreatedAt(obj: usize, new_val: i64);
    fn TrainingEditorData_TA_Get_UpdatedAt(obj: usize) -> i64;
    fn TrainingEditorSaveDataWrapper_SetUpdatedAt(obj: usize, new_val: i64);
    fn TrainingEditorData_TA_Get_CreatorPlayerID(obj: usize, result: *mut UniqueNetId);
    fn TrainingEditorSaveDataWrapper_SetCreatorPlayerID(obj: usize, new_val: *mut UniqueNetId);
    fn TrainingEditorData_TA_Init(obj: usize);

}