use crate::wrappers::*;
use crate::generated::*;

pub struct SampleRecordSettingsWrapper(pub usize);
impl_object!(SampleRecordSettingsWrapper);

impl SampleRecordSettings for SampleRecordSettingsWrapper {}

pub trait SampleRecordSettings : Object {
    fn get_max_sample_age(&self) -> f32 {
        unsafe {
            SampleRecordSettings_TA_Get_MaxSampleAge(self.addr())
        }
    }
    fn get_record_rate(&self) -> f32 {
        unsafe {
            SampleRecordSettings_TA_Get_RecordRate(self.addr())
        }
    }

}

extern "C" {
    fn SampleRecordSettings_TA_Get_MaxSampleAge(obj: usize) -> f32;
    fn SampleRecordSettingsWrapper_SetMaxSampleAge(obj: usize, new_val: f32);
    fn SampleRecordSettings_TA_Get_RecordRate(obj: usize) -> f32;
    fn SampleRecordSettingsWrapper_SetRecordRate(obj: usize, new_val: f32);

}