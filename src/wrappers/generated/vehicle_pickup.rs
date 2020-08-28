use crate::wrappers::*;
use super::*;

pub struct VehiclePickupWrapper(pub usize);
impl_object!(VehiclePickupWrapper);

impl VehiclePickup for VehiclePickupWrapper {}
impl Actor for VehiclePickupWrapper {}

pub trait VehiclePickup : Actor {
    fn get_respawn_delay(&self) -> f32 {
        unsafe {
            VehiclePickup_TA_Get_RespawnDelay(self.addr())
        }
    }
    fn get_fx_actor_archetype(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(VehiclePickup_TA_Get_FXActorArchetype(self.addr()))
        }
    }
    fn get_fx_actor(&self) -> Option<FXActorWrapper> {
        unsafe {
            FXActorWrapper::try_new(VehiclePickup_TA_Get_FXActor(self.addr()))
        }
    }
    fn get_b_net_relevant(&self) -> bool {
        unsafe {
            VehiclePickup_TA_Get_bNetRelevant(self.addr())
        }
    }
    fn get_b_no_pickup(&self) -> bool {
        unsafe {
            VehiclePickup_TA_Get_bNoPickup(self.addr())
        }
    }
    fn play_picked_up_fx(&self) {
        unsafe {
            VehiclePickup_TA_PlayPickedUpFX(self.addr());
        }
    }
    fn is_touching_a_vehicle(&self) -> bool {
        unsafe {
            VehiclePickup_TA_IsTouchingAVehicle(self.addr())
        }
    }
    fn update_tick_disabled(&self) {
        unsafe {
            VehiclePickup_TA_UpdateTickDisabled(self.addr());
        }
    }
    fn set_net_relevant(&self, b_relevant: bool) {
        unsafe {
            VehiclePickup_TA_SetNetRelevant(self.addr(), b_relevant);
        }
    }
    fn respawn(&self) {
        unsafe {
            VehiclePickup_TA_Respawn(self.addr());
        }
    }
    fn pickup(&self, car: CarWrapper) {
        unsafe {
            VehiclePickup_TA_Pickup(self.addr(), car.addr());
        }
    }
    fn can_pickup(&self, car: CarWrapper) -> bool {
        unsafe {
            VehiclePickup_TA_CanPickup(self.addr(), car.addr())
        }
    }
    fn on_touch(&self, car: CarWrapper) {
        unsafe {
            VehiclePickup_TA_OnTouch(self.addr(), car.addr());
        }
    }
    fn on_pick_up(&self) {
        unsafe {
            VehiclePickup_TA_OnPickUp(self.addr());
        }
    }
    fn on_spawn(&self) {
        unsafe {
            VehiclePickup_TA_OnSpawn(self.addr());
        }
    }
    fn set_no_pickup(&self) {
        unsafe {
            VehiclePickup_TA_SetNoPickup(self.addr());
        }
    }
    fn setup_replicate_no_pickup(&self) {
        unsafe {
            VehiclePickup_TA_SetupReplicateNoPickup(self.addr());
        }
    }
    fn init_fx(&self) {
        unsafe {
            VehiclePickup_TA_InitFX(self.addr());
        }
    }
    fn event_picked_up(&self, pickup: VehiclePickupWrapper) {
        unsafe {
            VehiclePickup_TA_EventPickedUp(self.addr(), pickup.addr());
        }
    }
    fn event_spawned(&self, pickup: VehiclePickupWrapper) {
        unsafe {
            VehiclePickup_TA_EventSpawned(self.addr(), pickup.addr());
        }
    }

}

extern "C" {
    fn VehiclePickup_TA_Get_RespawnDelay(obj: usize) -> f32;
    fn VehiclePickupWrapper_SetRespawnDelay(obj: usize, new_val: f32);
    fn VehiclePickup_TA_Get_FXActorArchetype(obj: usize) -> usize;
    fn VehiclePickupWrapper_SetFXActorArchetype(obj: usize, new_val: usize);
    fn VehiclePickup_TA_Get_FXActor(obj: usize) -> usize;
    fn VehiclePickupWrapper_SetFXActor(obj: usize, new_val: usize);
    fn VehiclePickup_TA_Get_bNetRelevant(obj: usize) -> bool;
    fn VehiclePickupWrapper_SetbNetRelevant(obj: usize, new_val: bool);
    fn VehiclePickup_TA_Get_bNoPickup(obj: usize) -> bool;
    fn VehiclePickupWrapper_SetbNoPickup(obj: usize, new_val: bool);
    fn VehiclePickup_TA_PlayPickedUpFX(obj: usize);
    fn VehiclePickup_TA_IsTouchingAVehicle(obj: usize) -> bool;
    fn VehiclePickup_TA_UpdateTickDisabled(obj: usize);
    fn VehiclePickup_TA_SetNetRelevant(obj: usize, bRelevant: bool);
    fn VehiclePickup_TA_Respawn(obj: usize);
    fn VehiclePickup_TA_Pickup(obj: usize, Car: usize);
    fn VehiclePickup_TA_CanPickup(obj: usize, Car: usize) -> bool;
    fn VehiclePickup_TA_OnTouch(obj: usize, Car: usize);
    fn VehiclePickup_TA_OnPickUp(obj: usize);
    fn VehiclePickup_TA_OnSpawn(obj: usize);
    fn VehiclePickup_TA_SetNoPickup(obj: usize);
    fn VehiclePickup_TA_SetupReplicateNoPickup(obj: usize);
    fn VehiclePickup_TA_InitFX(obj: usize);
    fn VehiclePickup_TA_EventPickedUp(obj: usize, Pickup: usize);
    fn VehiclePickup_TA_EventSpawned(obj: usize, Pickup: usize);

}