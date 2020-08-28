use crate::wrappers::*;
use crate::generated::*;

pub struct BoostWrapper(pub usize);
impl_object!(BoostWrapper);

impl Boost for BoostWrapper {}
impl CarComponent for BoostWrapper {}
impl Actor for BoostWrapper {}

pub trait Boost : CarComponent {
    fn get_boost_consumption_rate(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_BoostConsumptionRate(self.addr())
        }
    }
    fn get_max_boost_amount(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_MaxBoostAmount(self.addr())
        }
    }
    fn get_start_boost_amount(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_StartBoostAmount(self.addr())
        }
    }
    fn get_current_boost_amount(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_CurrentBoostAmount(self.addr())
        }
    }
    fn get_boost_modifier(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_BoostModifier(self.addr())
        }
    }
    fn get_last_boost_amount_request_time(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_LastBoostAmountRequestTime(self.addr())
        }
    }
    fn get_last_boost_amount(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_LastBoostAmount(self.addr())
        }
    }
    fn get_b_pending_confirm_boost_amount(&self) -> bool {
        unsafe {
            CarComponent_Boost_TA_Get_bPendingConfirmBoostAmount(self.addr())
        }
    }
    fn get_b_no_boost(&self) -> bool {
        unsafe {
            CarComponent_Boost_TA_Get_bNoBoost(self.addr())
        }
    }
    fn get_boost_force(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_BoostForce(self.addr())
        }
    }
    fn get_min_boost_time(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_MinBoostTime(self.addr())
        }
    }
    fn get_recharge_rate(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_RechargeRate(self.addr())
        }
    }
    fn get_recharge_delay(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_Get_RechargeDelay(self.addr())
        }
    }
    fn get_unlimited_boost_ref_count(&self) -> i32 {
        unsafe {
            CarComponent_Boost_TA_Get_UnlimitedBoostRefCount(self.addr())
        }
    }
    fn get_replicated_boost_amount(&self) -> u8 {
        unsafe {
            CarComponent_Boost_TA_Get_ReplicatedBoostAmount(self.addr())
        }
    }
    fn should_predict_boost_consumption(&self) -> bool {
        unsafe {
            CarComponent_Boost_TA_ShouldPredictBoostConsumption(self.addr())
        }
    }
    fn read_replicated_boost_amount(&self) {
        unsafe {
            CarComponent_Boost_TA_ReadReplicatedBoostAmount(self.addr());
        }
    }
    fn apply_forces(&self, active_time: f32) {
        unsafe {
            CarComponent_Boost_TA_ApplyForces(self.addr(), active_time);
        }
    }
    fn client_give_boost(&self, amount: f32) {
        unsafe {
            CarComponent_Boost_TA_ClientGiveBoost(self.addr(), amount);
        }
    }
    fn confirm_boost_amount(&self) {
        unsafe {
            CarComponent_Boost_TA_ConfirmBoostAmount(self.addr());
        }
    }
    fn send_confirm_boost_amount(&self) {
        unsafe {
            CarComponent_Boost_TA_SendConfirmBoostAmount(self.addr());
        }
    }
    fn client_fix_boost_amount(&self, time_stamp: f32, amount: f32) {
        unsafe {
            CarComponent_Boost_TA_ClientFixBoostAmount(self.addr(), time_stamp, amount);
        }
    }
    fn server_confirm_boost_amount(&self, time_stamp: f32, amount: f32) {
        unsafe {
            CarComponent_Boost_TA_ServerConfirmBoostAmount(self.addr(), time_stamp, amount);
        }
    }
    fn set_recharge_delay2(&self, in_recharge_delay: f32) {
        unsafe {
            CarComponent_Boost_TA_SetRechargeDelay2(self.addr(), in_recharge_delay);
        }
    }
    fn set_recharge_rate2(&self, in_recharge_rate: f32) {
        unsafe {
            CarComponent_Boost_TA_SetRechargeRate2(self.addr(), in_recharge_rate);
        }
    }
    fn set_no_boost(&self, enabled: bool) {
        unsafe {
            CarComponent_Boost_TA_SetNoBoost(self.addr(), enabled);
        }
    }
    fn set_unlimited_boost(&self, enabled: bool) {
        unsafe {
            CarComponent_Boost_TA_SetUnlimitedBoost(self.addr(), enabled);
        }
    }
    fn set_unlimited_boost_delayed(&self, enabled: bool) {
        unsafe {
            CarComponent_Boost_TA_SetUnlimitedBoostDelayed(self.addr(), enabled);
        }
    }
    fn set_boost_modifier2(&self, modifier: f32) {
        unsafe {
            CarComponent_Boost_TA_SetBoostModifier2(self.addr(), modifier);
        }
    }
    fn set_boost_amount(&self, amount: f32) {
        unsafe {
            CarComponent_Boost_TA_SetBoostAmount(self.addr(), amount);
        }
    }
    fn give_boost(&self, amount: f32) {
        unsafe {
            CarComponent_Boost_TA_GiveBoost(self.addr(), amount);
        }
    }
    fn give_starting_boost(&self) {
        unsafe {
            CarComponent_Boost_TA_GiveStartingBoost(self.addr());
        }
    }
    fn give_full_boost(&self) {
        unsafe {
            CarComponent_Boost_TA_GiveFullBoost(self.addr());
        }
    }
    fn get_percent_boost_full(&self) -> f32 {
        unsafe {
            CarComponent_Boost_TA_GetPercentBoostFull(self.addr())
        }
    }
    fn is_full(&self) -> bool {
        unsafe {
            CarComponent_Boost_TA_IsFull(self.addr())
        }
    }
    fn remove_from_car(&self) {
        unsafe {
            CarComponent_Boost_TA_RemoveFromCar(self.addr());
        }
    }
    fn can_deactivate(&self) -> bool {
        unsafe {
            CarComponent_Boost_TA_CanDeactivate(self.addr())
        }
    }
    fn can_activate(&self) -> bool {
        unsafe {
            CarComponent_Boost_TA_CanActivate(self.addr())
        }
    }

}

extern "C" {
    fn CarComponent_Boost_TA_Get_BoostConsumptionRate(obj: usize) -> f32;
    fn BoostWrapper_SetBoostConsumptionRate(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_MaxBoostAmount(obj: usize) -> f32;
    fn BoostWrapper_SetMaxBoostAmount(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_StartBoostAmount(obj: usize) -> f32;
    fn BoostWrapper_SetStartBoostAmount(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_CurrentBoostAmount(obj: usize) -> f32;
    fn BoostWrapper_SetCurrentBoostAmount(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_BoostModifier(obj: usize) -> f32;
    fn BoostWrapper_SetBoostModifier(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_LastBoostAmountRequestTime(obj: usize) -> f32;
    fn BoostWrapper_SetLastBoostAmountRequestTime(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_LastBoostAmount(obj: usize) -> f32;
    fn BoostWrapper_SetLastBoostAmount(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_bPendingConfirmBoostAmount(obj: usize) -> bool;
    fn BoostWrapper_SetbPendingConfirmBoostAmount(obj: usize, new_val: bool);
    fn CarComponent_Boost_TA_Get_bNoBoost(obj: usize) -> bool;
    fn BoostWrapper_SetbNoBoost(obj: usize, new_val: bool);
    fn CarComponent_Boost_TA_Get_BoostForce(obj: usize) -> f32;
    fn BoostWrapper_SetBoostForce(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_MinBoostTime(obj: usize) -> f32;
    fn BoostWrapper_SetMinBoostTime(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_RechargeRate(obj: usize) -> f32;
    fn BoostWrapper_SetRechargeRate(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_RechargeDelay(obj: usize) -> f32;
    fn BoostWrapper_SetRechargeDelay(obj: usize, new_val: f32);
    fn CarComponent_Boost_TA_Get_UnlimitedBoostRefCount(obj: usize) -> i32;
    fn BoostWrapper_SetUnlimitedBoostRefCount(obj: usize, new_val: i32);
    fn CarComponent_Boost_TA_Get_ReplicatedBoostAmount(obj: usize) -> u8;
    fn BoostWrapper_SetReplicatedBoostAmount(obj: usize, new_val: u8);
    fn CarComponent_Boost_TA_ShouldPredictBoostConsumption(obj: usize) -> bool;
    fn CarComponent_Boost_TA_ReadReplicatedBoostAmount(obj: usize);
    fn CarComponent_Boost_TA_ApplyForces(obj: usize, ActiveTime: f32);
    fn CarComponent_Boost_TA_ClientGiveBoost(obj: usize, Amount: f32);
    fn CarComponent_Boost_TA_ConfirmBoostAmount(obj: usize);
    fn CarComponent_Boost_TA_SendConfirmBoostAmount(obj: usize);
    fn CarComponent_Boost_TA_ClientFixBoostAmount(obj: usize, TimeStamp: f32, Amount: f32);
    fn CarComponent_Boost_TA_ServerConfirmBoostAmount(obj: usize, TimeStamp: f32, Amount: f32);
    fn CarComponent_Boost_TA_SetRechargeDelay2(obj: usize, InRechargeDelay: f32);
    fn CarComponent_Boost_TA_SetRechargeRate2(obj: usize, InRechargeRate: f32);
    fn CarComponent_Boost_TA_SetNoBoost(obj: usize, Enabled: bool);
    fn CarComponent_Boost_TA_SetUnlimitedBoost(obj: usize, Enabled: bool);
    fn CarComponent_Boost_TA_SetUnlimitedBoostDelayed(obj: usize, Enabled: bool);
    fn CarComponent_Boost_TA_SetBoostModifier2(obj: usize, Modifier: f32);
    fn CarComponent_Boost_TA_SetBoostAmount(obj: usize, Amount: f32);
    fn CarComponent_Boost_TA_GiveBoost(obj: usize, Amount: f32);
    fn CarComponent_Boost_TA_GiveStartingBoost(obj: usize);
    fn CarComponent_Boost_TA_GiveFullBoost(obj: usize);
    fn CarComponent_Boost_TA_GetPercentBoostFull(obj: usize) -> f32;
    fn CarComponent_Boost_TA_IsFull(obj: usize) -> bool;
    fn CarComponent_Boost_TA_RemoveFromCar(obj: usize);
    fn CarComponent_Boost_TA_CanDeactivate(obj: usize) -> bool;
    fn CarComponent_Boost_TA_CanActivate(obj: usize) -> bool;

}