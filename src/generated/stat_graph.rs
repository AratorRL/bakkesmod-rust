use crate::wrappers::*;
use crate::generated::*;

pub struct StatGraphWrapper(pub usize);
impl_object!(StatGraphWrapper);

impl StatGraph for StatGraphWrapper {}

pub trait StatGraph : Object {
    fn get_record_settings(&self) -> Option<SampleRecordSettingsWrapper> {
        unsafe {
            SampleRecordSettingsWrapper::try_new(StatGraph_TA_Get_RecordSettings(self.addr()))
        }
    }
    fn get_last_tick_time(&self) -> Double {
        unsafe {
            let mut result = Double::new();
            let result_ptr: *mut Double = &mut result as *mut Double;
            StatGraph_TA_Get_LastTickTime(self.addr(), result_ptr);
            result
        }
    }
    fn get_sample_histories(&self) -> RLArray<SampleHistoryWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            StatGraph_TA_Get_SampleHistories(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn stop_drawing(&self) {
        unsafe {
            StatGraph_TA_StopDrawing(self.addr());
        }
    }
    fn create_sample_history(&self, title: RLString) -> Option<SampleHistoryWrapper> {
        unsafe {
            let mut title = title;
            let title: *mut RLString = &mut title as *mut RLString;
            SampleHistoryWrapper::try_new(StatGraph_TA_CreateSampleHistory(self.addr(), title))
        }
    }
    fn add_sample_history(&self, history: SampleHistoryWrapper) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(StatGraph_TA_AddSampleHistory(self.addr(), history.addr()))
        }
    }

}

extern "C" {
    fn StatGraph_TA_Get_RecordSettings(obj: usize) -> usize;
    fn StatGraphWrapper_SetRecordSettings(obj: usize, new_val: usize);
    fn StatGraph_TA_Get_LastTickTime(obj: usize, result: *mut Double);
    fn StatGraphWrapper_SetLastTickTime(obj: usize, new_val: *mut Double);
    fn StatGraph_TA_Get_SampleHistories(obj: usize, result: *mut RLArrayRaw);
    fn StatGraph_TA_StopDrawing(obj: usize);
    fn StatGraph_TA_CreateSampleHistory(obj: usize, Title: *mut RLString) -> usize;
    fn StatGraph_TA_AddSampleHistory(obj: usize, History: usize) -> usize;

}