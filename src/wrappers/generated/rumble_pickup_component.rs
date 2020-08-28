use crate::wrappers::*;
use super::*;

pub struct RumblePickupComponentWrapper(pub usize);
impl_object!(RumblePickupComponentWrapper);

impl RumblePickupComponent for RumblePickupComponentWrapper {}
impl CarComponent for RumblePickupComponentWrapper {}
impl Actor for RumblePickupComponentWrapper {}

pub trait RumblePickupComponent : CarComponent {
    fn get_pickup_name(&self) -> RLString {
        unsafe {
            let mut result = RLString::new();
            let result_ptr: *mut RLString = &mut result as *mut RLString;
            SpecialPickup_TA_Get_PickupName(self.addr(), result_ptr);
            result
        }
    }
    fn get_b_hud_ignore_use_time(&self) -> bool {
        unsafe {
            SpecialPickup_TA_Get_bHudIgnoreUseTime(self.addr())
        }
    }
    fn get_b_has_activated(&self) -> bool {
        unsafe {
            SpecialPickup_TA_Get_bHasActivated(self.addr())
        }
    }
    fn get_b_is_active(&self) -> bool {
        unsafe {
            SpecialPickup_TA_Get_bIsActive(self.addr())
        }
    }
    fn get_activation_duration(&self) -> f32 {
        unsafe {
            SpecialPickup_TA_Get_ActivationDuration(self.addr())
        }
    }
    fn get_pickup_fx_archetype(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(SpecialPickup_TA_Get_PickupFXArchetype(self.addr()))
        }
    }
    fn get_pickup_fx(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(SpecialPickup_TA_Get_PickupFX(self.addr()))
        }
    }
    fn has_activated(&self) -> bool {
        unsafe {
            SpecialPickup_TA_HasActivated(self.addr())
        }
    }
    fn get_client_target(&self) -> Option<RBActorWrapper> {
        unsafe {
            RBActorWrapper::try_new(SpecialPickup_TA_GetClientTarget(self.addr()))
        }
    }
    fn on_vehicle_setup_complete(&self) {
        unsafe {
            SpecialPickup_TA_OnVehicleSetupComplete(self.addr());
        }
    }
    fn get_active_time_percent(&self) -> f32 {
        unsafe {
            SpecialPickup_TA_GetActiveTimePercent(self.addr())
        }
    }
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_TA_PickupEnd(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_TA_PickupStart(self.addr());
        }
    }
    fn get_boost_component(&self) -> Option<BoostWrapper> {
        unsafe {
            BoostWrapper::try_new(SpecialPickup_TA_GetBoostComponent(self.addr()))
        }
    }
    fn deactivate_pickup(&self) {
        unsafe {
            SpecialPickup_TA_DeactivatePickup(self.addr());
        }
    }
    fn try_activate(&self, target_override: RBActorWrapper) -> bool {
        unsafe {
            SpecialPickup_TA_TryActivate(self.addr(), target_override.addr())
        }
    }
    fn on_created(&self) {
        unsafe {
            SpecialPickup_TA_OnCreated(self.addr());
        }
    }
    fn can_pickup(&self, in_car: CarWrapper) -> bool {
        unsafe {
            SpecialPickup_TA_CanPickup(self.addr(), in_car.addr())
        }
    }
    fn apply_pickup(&self, in_car: CarWrapper) {
        unsafe {
            SpecialPickup_TA_ApplyPickup(self.addr(), in_car.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_TA_Get_PickupName(obj: usize, result: *mut RLString);
    fn SpecialPickup_TA_Get_bHudIgnoreUseTime(obj: usize) -> bool;
    fn RumblePickupComponentWrapper_SetbHudIgnoreUseTime(obj: usize, new_val: bool);
    fn SpecialPickup_TA_Get_bHasActivated(obj: usize) -> bool;
    fn RumblePickupComponentWrapper_SetbHasActivated(obj: usize, new_val: bool);
    fn SpecialPickup_TA_Get_bIsActive(obj: usize) -> bool;
    fn RumblePickupComponentWrapper_SetbIsActive(obj: usize, new_val: bool);
    fn SpecialPickup_TA_Get_ActivationDuration(obj: usize) -> f32;
    fn RumblePickupComponentWrapper_SetActivationDuration(obj: usize, new_val: f32);
    fn SpecialPickup_TA_Get_PickupFXArchetype(obj: usize) -> usize;
    fn RumblePickupComponentWrapper_SetPickupFXArchetype(obj: usize, new_val: usize);
    fn SpecialPickup_TA_Get_PickupFX(obj: usize) -> usize;
    fn RumblePickupComponentWrapper_SetPickupFX(obj: usize, new_val: usize);
    fn SpecialPickup_TA_HasActivated(obj: usize) -> bool;
    fn SpecialPickup_TA_GetClientTarget(obj: usize) -> usize;
    fn SpecialPickup_TA_OnVehicleSetupComplete(obj: usize);
    fn SpecialPickup_TA_GetActiveTimePercent(obj: usize) -> f32;
    fn SpecialPickup_TA_PickupEnd(obj: usize);
    fn SpecialPickup_TA_PickupStart(obj: usize);
    fn SpecialPickup_TA_GetBoostComponent(obj: usize) -> usize;
    fn SpecialPickup_TA_DeactivatePickup(obj: usize);
    fn SpecialPickup_TA_TryActivate(obj: usize, TargetOverride: usize) -> bool;
    fn SpecialPickup_TA_OnCreated(obj: usize);
    fn SpecialPickup_TA_CanPickup(obj: usize, InCar: usize) -> bool;
    fn SpecialPickup_TA_ApplyPickup(obj: usize, InCar: usize);

}