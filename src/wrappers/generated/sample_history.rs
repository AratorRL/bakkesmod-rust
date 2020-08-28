use crate::wrappers::*;
use super::*;

pub struct SampleHistoryWrapper(pub usize);
impl_object!(SampleHistoryWrapper);

impl SampleHistory for SampleHistoryWrapper {}

pub trait SampleHistory : Object {
    fn get_record_settings(&self) -> Option<SampleRecordSettingsWrapper> {
        unsafe {
            SampleRecordSettingsWrapper::try_new(SampleHistory_TA_Get_RecordSettings(self.addr()))
        }
    }
    fn get_title(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            SampleHistory_TA_Get_Title(self.addr(), result_ptr);
            result
        }
    }
    fn get_y_min(&self) -> f32 {
        unsafe {
            SampleHistory_TA_Get_YMin(self.addr())
        }
    }
    fn get_y_max(&self) -> f32 {
        unsafe {
            SampleHistory_TA_Get_YMax(self.addr())
        }
    }
    fn get_good_value(&self) -> f32 {
        unsafe {
            SampleHistory_TA_Get_GoodValue(self.addr())
        }
    }
    fn get_bad_value(&self) -> f32 {
        unsafe {
            SampleHistory_TA_Get_BadValue(self.addr())
        }
    }
    fn get_base_value(&self) -> f32 {
        unsafe {
            SampleHistory_TA_Get_BaseValue(self.addr())
        }
    }
    fn get_samples(&self) -> RLArray<Sample> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            SampleHistory_TA_Get_Samples(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_sample_index(&self) -> i32 {
        unsafe {
            SampleHistory_TA_Get_SampleIndex(self.addr())
        }
    }
    fn get_accum_time(&self) -> f32 {
        unsafe {
            SampleHistory_TA_Get_AccumTime(self.addr())
        }
    }
    fn get_pending_sample(&self) -> Sample {
        unsafe {
            let mut result = Sample::new();
            let result_ptr: *mut Sample = &mut result as *mut Sample;
            SampleHistory_TA_Get_PendingSample(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_has_pending_sample(&self) -> bool {
        unsafe {
            SampleHistory_TA_Get_bHasPendingSample(self.addr())
        }
    }
    fn tick(&self, delta_time: f32) {
        unsafe {
            SampleHistory_TA_Tick(self.addr(), delta_time);
        }
    }
    fn add_sample(&self, new_value: f32) {
        unsafe {
            SampleHistory_TA_AddSample(self.addr(), new_value);
        }
    }
    fn get_summary_value(&self, type_: u8, max_sample_age: f32, b_absolute_value: bool) -> f32 {
        unsafe {
            SampleHistory_TA_GetSummaryValue(self.addr(), type_, max_sample_age, b_absolute_value)
        }
    }
    fn set_base_value2(&self, in_base_value: f32) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(SampleHistory_TA_SetBaseValue2(self.addr(), in_base_value))
        }
    }
    fn set_good_bad_values(&self, in_good_value: f32, in_bad_value: f32) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(SampleHistory_TA_SetGoodBadValues(self.addr(), in_good_value, in_bad_value))
        }
    }
    fn set_graph_max_min(&self, max_value: f32, min_value: f32) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(SampleHistory_TA_SetGraphMaxMin(self.addr(), max_value, min_value))
        }
    }
    fn set_title(&self, in_title: RLString) -> Option<SampleHistoryWrapper> {
        unsafe {
            let mut in_title = in_title;
            let in_title: *mut RLString = &mut in_title as *mut RLString;
            SampleHistoryWrapper::try_new(SampleHistory_TA_SetTitle(self.addr(), in_title))
        }
    }

}

extern "C" {
    fn SampleHistory_TA_Get_RecordSettings(obj: usize) -> usize;
    fn SampleHistoryWrapper_SetRecordSettings(obj: usize, new_val: usize);
    fn SampleHistory_TA_Get_Title(obj: usize, result: *mut RLString);
    fn SampleHistory_TA_Get_YMin(obj: usize) -> f32;
    fn SampleHistoryWrapper_SetYMin(obj: usize, new_val: f32);
    fn SampleHistory_TA_Get_YMax(obj: usize) -> f32;
    fn SampleHistoryWrapper_SetYMax(obj: usize, new_val: f32);
    fn SampleHistory_TA_Get_GoodValue(obj: usize) -> f32;
    fn SampleHistoryWrapper_SetGoodValue(obj: usize, new_val: f32);
    fn SampleHistory_TA_Get_BadValue(obj: usize) -> f32;
    fn SampleHistoryWrapper_SetBadValue(obj: usize, new_val: f32);
    fn SampleHistory_TA_Get_BaseValue(obj: usize) -> f32;
    fn SampleHistoryWrapper_SetBaseValue(obj: usize, new_val: f32);
    fn SampleHistory_TA_Get_Samples(obj: usize, result: *mut RLArrayRaw);
    fn SampleHistory_TA_Get_SampleIndex(obj: usize) -> i32;
    fn SampleHistoryWrapper_SetSampleIndex(obj: usize, new_val: i32);
    fn SampleHistory_TA_Get_AccumTime(obj: usize) -> f32;
    fn SampleHistoryWrapper_SetAccumTime(obj: usize, new_val: f32);
    fn SampleHistory_TA_Get_PendingSample(obj: usize, result: *mut Sample);
    fn SampleHistoryWrapper_SetPendingSample(obj: usize, new_val: *mut Sample);
    fn SampleHistory_TA_Get_bHasPendingSample(obj: usize) -> bool;
    fn SampleHistoryWrapper_SetbHasPendingSample(obj: usize, new_val: bool);
    fn SampleHistory_TA_Tick(obj: usize, DeltaTime: f32);
    fn SampleHistory_TA_AddSample(obj: usize, NewValue: f32);
    fn SampleHistory_TA_GetSummaryValue(obj: usize, Type: u8, MaxSampleAge: f32, bAbsoluteValue: bool) -> f32;
    fn SampleHistory_TA_SetBaseValue2(obj: usize, InBaseValue: f32) -> usize;
    fn SampleHistory_TA_SetGoodBadValues(obj: usize, InGoodValue: f32, InBadValue: f32) -> usize;
    fn SampleHistory_TA_SetGraphMaxMin(obj: usize, MaxValue: f32, MinValue: f32) -> usize;
    fn SampleHistory_TA_SetTitle(obj: usize, InTitle: *mut RLString) -> usize;

}