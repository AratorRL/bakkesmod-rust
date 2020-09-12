use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct StatGraphSystemWrapper(pub usize);
impl_object!(StatGraphSystemWrapper);

impl StatGraphSystem for StatGraphSystemWrapper {}

pub trait StatGraphSystem : Object {
    fn get_graph_sample_time(&self) -> f32 {
        unsafe {
            StatGraphSystem_TA_Get_GraphSampleTime(self.addr())
        }
    }
    fn get_graph_level(&self) -> u8 {
        unsafe {
            StatGraphSystem_TA_Get_GraphLevel(self.addr())
        }
    }
    fn get_perf_stat_graph(&self) -> Option<PerfStatGraphWrapper> {
        unsafe {
            PerfStatGraphWrapper::try_new(StatGraphSystem_TA_Get_PerfStatGraph(self.addr()))
        }
    }
    fn get_net_stat_graph(&self) -> Option<NetStatGraphWrapper> {
        unsafe {
            NetStatGraphWrapper::try_new(StatGraphSystem_TA_Get_NetStatGraph(self.addr()))
        }
    }
    fn get_input_buffer_graph(&self) -> Option<InputBufferGraphWrapper> {
        unsafe {
            InputBufferGraphWrapper::try_new(StatGraphSystem_TA_Get_InputBufferGraph(self.addr()))
        }
    }
    fn get_stat_graphs(&self) -> RLArray<StatGraphWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            StatGraphSystem_TA_Get_StatGraphs(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_visible_stat_graphs(&self) -> RLArray<StatGraphWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            StatGraphSystem_TA_Get_VisibleStatGraphs(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn graphtime(&self, seconds: f32) {
        unsafe {
            StatGraphSystem_TA_Graphtime(self.addr(), seconds);
        }
    }
    fn stat_graph_next(&self) {
        unsafe {
            StatGraphSystem_TA_StatGraphNext(self.addr());
        }
    }
    fn get_graph_sample_time2(&self, level: u8) -> f32 {
        unsafe {
            StatGraphSystem_TA_GetGraphSampleTime2(self.addr(), level)
        }
    }
    fn set_graph_level2(&self, level: u8) {
        unsafe {
            StatGraphSystem_TA_SetGraphLevel2(self.addr(), level);
        }
    }

}

extern "C" {
    fn StatGraphSystem_TA_Get_GraphSampleTime(obj: usize) -> f32;
    fn StatGraphSystemWrapper_SetGraphSampleTime(obj: usize, new_val: f32);
    fn StatGraphSystem_TA_Get_GraphLevel(obj: usize) -> u8;
    fn StatGraphSystemWrapper_SetGraphLevel(obj: usize, new_val: u8);
    fn StatGraphSystem_TA_Get_PerfStatGraph(obj: usize) -> usize;
    fn StatGraphSystemWrapper_SetPerfStatGraph(obj: usize, new_val: usize);
    fn StatGraphSystem_TA_Get_NetStatGraph(obj: usize) -> usize;
    fn StatGraphSystemWrapper_SetNetStatGraph(obj: usize, new_val: usize);
    fn StatGraphSystem_TA_Get_InputBufferGraph(obj: usize) -> usize;
    fn StatGraphSystemWrapper_SetInputBufferGraph(obj: usize, new_val: usize);
    fn StatGraphSystem_TA_Get_StatGraphs(obj: usize, result: *mut RLArrayRaw);
    fn StatGraphSystem_TA_Get_VisibleStatGraphs(obj: usize, result: *mut RLArrayRaw);
    fn StatGraphSystem_TA_Graphtime(obj: usize, Seconds: f32);
    fn StatGraphSystem_TA_StatGraphNext(obj: usize);
    fn StatGraphSystem_TA_GetGraphSampleTime2(obj: usize, Level: u8) -> f32;
    fn StatGraphSystem_TA_SetGraphLevel2(obj: usize, Level: u8);

}