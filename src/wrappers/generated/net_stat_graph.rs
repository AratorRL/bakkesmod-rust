use crate::wrappers::*;
use super::*;

pub struct NetStatGraphWrapper(pub usize);
impl_object!(NetStatGraphWrapper);

impl NetStatGraph for NetStatGraphWrapper {}
impl StatGraph for NetStatGraphWrapper {}

pub trait NetStatGraph : StatGraph {
    fn get_packets_out(&self) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(NetStatGraph_TA_Get_PacketsOut(self.addr()))
        }
    }
    fn get_packets_in(&self) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(NetStatGraph_TA_Get_PacketsIn(self.addr()))
        }
    }
    fn get_lost_packets_out(&self) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(NetStatGraph_TA_Get_LostPacketsOut(self.addr()))
        }
    }
    fn get_lost_packets_in(&self) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(NetStatGraph_TA_Get_LostPacketsIn(self.addr()))
        }
    }
    fn get_bytes_out(&self) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(NetStatGraph_TA_Get_BytesOut(self.addr()))
        }
    }
    fn get_bytes_in(&self) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(NetStatGraph_TA_Get_BytesIn(self.addr()))
        }
    }
    fn get_latency(&self) -> Option<SampleHistoryWrapper> {
        unsafe {
            SampleHistoryWrapper::try_new(NetStatGraph_TA_Get_Latency(self.addr()))
        }
    }
    fn get_expected_out_packet_rate(&self) -> f32 {
        unsafe {
            NetStatGraph_TA_Get_ExpectedOutPacketRate(self.addr())
        }
    }
    fn get_expected_in_packet_rate(&self) -> f32 {
        unsafe {
            NetStatGraph_TA_Get_ExpectedInPacketRate(self.addr())
        }
    }
    fn get_max_bytes_rate(&self) -> f32 {
        unsafe {
            NetStatGraph_TA_Get_MaxBytesRate(self.addr())
        }
    }
    fn create_bytes_summary(&self, title: RLString) -> Option<SampleHistoryWrapper> {
        unsafe {
            let mut title = title;
            let title: *mut RLString = &mut title as *mut RLString;
            SampleHistoryWrapper::try_new(NetStatGraph_TA_CreateBytesSummary(self.addr(), title))
        }
    }
    fn create_loss_summary(&self, title: RLString) -> Option<SampleHistoryWrapper> {
        unsafe {
            let mut title = title;
            let title: *mut RLString = &mut title as *mut RLString;
            SampleHistoryWrapper::try_new(NetStatGraph_TA_CreateLossSummary(self.addr(), title))
        }
    }
    fn create_pkt_summary(&self, title: RLString) -> Option<SampleHistoryWrapper> {
        unsafe {
            let mut title = title;
            let title: *mut RLString = &mut title as *mut RLString;
            SampleHistoryWrapper::try_new(NetStatGraph_TA_CreatePktSummary(self.addr(), title))
        }
    }

}

extern "C" {
    fn NetStatGraph_TA_Get_PacketsOut(obj: usize) -> usize;
    fn NetStatGraphWrapper_SetPacketsOut(obj: usize, new_val: usize);
    fn NetStatGraph_TA_Get_PacketsIn(obj: usize) -> usize;
    fn NetStatGraphWrapper_SetPacketsIn(obj: usize, new_val: usize);
    fn NetStatGraph_TA_Get_LostPacketsOut(obj: usize) -> usize;
    fn NetStatGraphWrapper_SetLostPacketsOut(obj: usize, new_val: usize);
    fn NetStatGraph_TA_Get_LostPacketsIn(obj: usize) -> usize;
    fn NetStatGraphWrapper_SetLostPacketsIn(obj: usize, new_val: usize);
    fn NetStatGraph_TA_Get_BytesOut(obj: usize) -> usize;
    fn NetStatGraphWrapper_SetBytesOut(obj: usize, new_val: usize);
    fn NetStatGraph_TA_Get_BytesIn(obj: usize) -> usize;
    fn NetStatGraphWrapper_SetBytesIn(obj: usize, new_val: usize);
    fn NetStatGraph_TA_Get_Latency(obj: usize) -> usize;
    fn NetStatGraphWrapper_SetLatency(obj: usize, new_val: usize);
    fn NetStatGraph_TA_Get_ExpectedOutPacketRate(obj: usize) -> f32;
    fn NetStatGraphWrapper_SetExpectedOutPacketRate(obj: usize, new_val: f32);
    fn NetStatGraph_TA_Get_ExpectedInPacketRate(obj: usize) -> f32;
    fn NetStatGraphWrapper_SetExpectedInPacketRate(obj: usize, new_val: f32);
    fn NetStatGraph_TA_Get_MaxBytesRate(obj: usize) -> f32;
    fn NetStatGraphWrapper_SetMaxBytesRate(obj: usize, new_val: f32);
    fn NetStatGraph_TA_CreateBytesSummary(obj: usize, Title: *mut RLString) -> usize;
    fn NetStatGraph_TA_CreateLossSummary(obj: usize, Title: *mut RLString) -> usize;
    fn NetStatGraph_TA_CreatePktSummary(obj: usize, Title: *mut RLString) -> usize;

}