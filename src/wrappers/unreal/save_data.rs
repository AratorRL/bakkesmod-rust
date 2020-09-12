use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct SaveDataWrapper(pub usize);
impl_object!(SaveDataWrapper);

impl SaveData for SaveDataWrapper {}

pub trait SaveData : Object {
    fn get_directory_path(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Save_TA_Get_DirectoryPath(self.addr(), result_ptr);
            result
        }
    }
    fn get_save_type(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Save_TA_Get_SaveType(self.addr(), result_ptr);
            result
        }
    }
    fn get_save_ext(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            Save_TA_Get_SaveExt(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_exact_file_match(&self) -> bool {
        unsafe {
            Save_TA_Get_bExactFileMatch(self.addr())
        }
    }
    fn init(&self) {
        unsafe {
            Save_TA_Init(self.addr());
        }
    }

}

extern "C" {
    fn Save_TA_Get_DirectoryPath(obj: usize, result: *mut RLString);
    fn Save_TA_Get_SaveType(obj: usize, result: *mut RLString);
    fn Save_TA_Get_SaveExt(obj: usize, result: *mut RLString);
    fn Save_TA_Get_bExactFileMatch(obj: usize) -> bool;
    fn SaveDataWrapper_SetbExactFileMatch(obj: usize, new_val: bool);
    fn Save_TA_Init(obj: usize);

}