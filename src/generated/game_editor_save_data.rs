use crate::wrappers::*;
use crate::generated::*;

pub struct GameEditorSaveDataWrapper(pub usize);
impl_object!(GameEditorSaveDataWrapper);

impl GameEditorSaveData for GameEditorSaveDataWrapper {}
impl SaveData for GameEditorSaveDataWrapper {}

pub trait GameEditorSaveData : SaveData {
    fn get_loaded_save_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            SaveData_GameEditor_Training_TA_Get_LoadedSaveName(self.addr(), result_ptr);
            result
        }
    }
    fn get_training_data(&self) -> Option<TrainingEditorSaveDataWrapper> {
        unsafe {
            TrainingEditorSaveDataWrapper::try_new(SaveData_GameEditor_Training_TA_Get_TrainingData(self.addr()))
        }
    }
    fn get_player_team_number(&self) -> i32 {
        unsafe {
            SaveData_GameEditor_Training_TA_Get_PlayerTeamNumber(self.addr())
        }
    }
    fn get_b_unowned(&self) -> bool {
        unsafe {
            SaveData_GameEditor_Training_TA_Get_bUnowned(self.addr())
        }
    }
    fn get_shots_completed(&self) -> i32 {
        unsafe {
            SaveData_GameEditor_Training_TA_Get_ShotsCompleted(self.addr())
        }
    }
    fn get_favorites_folder_path(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            SaveData_GameEditor_Training_TA_Get_FavoritesFolderPath(self.addr(), result_ptr);
            result
        }
    }
    fn get_my_training_folder_path(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            SaveData_GameEditor_Training_TA_Get_MyTrainingFolderPath(self.addr(), result_ptr);
            result
        }
    }
    fn get_downloaded_folder_path(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            SaveData_GameEditor_Training_TA_Get_DownloadedFolderPath(self.addr(), result_ptr);
            result
        }
    }
    fn get_training_save_type(&self, b_owned: bool, b_favorited: bool) -> u8 {
        unsafe {
            SaveData_GameEditor_Training_TA_GetTrainingSaveType(self.addr(), b_owned, b_favorited)
        }
    }
    fn init(&self) {
        unsafe {
            SaveData_GameEditor_Training_TA_Init(self.addr());
        }
    }

}

extern "C" {
    fn SaveData_GameEditor_Training_TA_Get_LoadedSaveName(obj: usize, result: *mut RLString);
    fn SaveData_GameEditor_Training_TA_Get_TrainingData(obj: usize) -> usize;
    fn GameEditorSaveDataWrapper_SetTrainingData(obj: usize, new_val: usize);
    fn SaveData_GameEditor_Training_TA_Get_PlayerTeamNumber(obj: usize) -> i32;
    fn GameEditorSaveDataWrapper_SetPlayerTeamNumber(obj: usize, new_val: i32);
    fn SaveData_GameEditor_Training_TA_Get_bUnowned(obj: usize) -> bool;
    fn GameEditorSaveDataWrapper_SetbUnowned(obj: usize, new_val: bool);
    fn SaveData_GameEditor_Training_TA_Get_ShotsCompleted(obj: usize) -> i32;
    fn GameEditorSaveDataWrapper_SetShotsCompleted(obj: usize, new_val: i32);
    fn SaveData_GameEditor_Training_TA_Get_FavoritesFolderPath(obj: usize, result: *mut RLString);
    fn SaveData_GameEditor_Training_TA_Get_MyTrainingFolderPath(obj: usize, result: *mut RLString);
    fn SaveData_GameEditor_Training_TA_Get_DownloadedFolderPath(obj: usize, result: *mut RLString);
    fn SaveData_GameEditor_Training_TA_GetTrainingSaveType(obj: usize, bOwned: bool, bFavorited: bool) -> u8;
    fn SaveData_GameEditor_Training_TA_Init(obj: usize);

}