use crate::wrappers::*;
use crate::generated::*;

pub struct CarComponentWrapper(pub usize);
impl_object!(CarComponentWrapper);

impl CarComponent for CarComponentWrapper {}
impl Actor for CarComponentWrapper {}

pub trait CarComponent : Actor {
	fn get_fx_actor_archetype(&self) -> FXActorWrapper {
		unsafe {
			FXActorWrapper::new(CarComponent_TA_Get_FXActorArchetype(self.addr()))
		}
	}
	fn get_b_disabled(&self) -> bool {
		unsafe {
			CarComponent_TA_Get_bDisabled(self.addr())
		}
	}
	fn get_b_auto_activate(&self) -> bool {
		unsafe {
			CarComponent_TA_Get_bAutoActivate(self.addr())
		}
	}
	fn get_b_simulate_component(&self) -> bool {
		unsafe {
			CarComponent_TA_Get_bSimulateComponent(self.addr())
		}
	}
	fn get_b_created(&self) -> bool {
		unsafe {
			CarComponent_TA_Get_bCreated(self.addr())
		}
	}
	fn get_b_active(&self) -> bool {
		unsafe {
			CarComponent_TA_Get_bActive(self.addr())
		}
	}
	fn get_b_removed_from_car(&self) -> bool {
		unsafe {
			CarComponent_TA_Get_bRemovedFromCar(self.addr())
		}
	}
	fn get_component_data(&self) -> u8 {
		unsafe {
			CarComponent_TA_Get_ComponentData(self.addr())
		}
	}
	fn get_replicated_active(&self) -> u8 {
		unsafe {
			CarComponent_TA_Get_ReplicatedActive(self.addr())
		}
	}
	fn get_activator(&self) -> PriWrapper {
		unsafe {
			PriWrapper::new(CarComponent_TA_Get_Activator(self.addr()))
		}
	}
	fn get_vehicle(&self) -> VehicleWrapper {
		unsafe {
			VehicleWrapper::new(CarComponent_TA_Get_Vehicle(self.addr()))
		}
	}
	fn get_car(&self) -> CarWrapper {
		unsafe {
			CarWrapper::new(CarComponent_TA_Get_Car(self.addr()))
		}
	}
	fn get_activity_time(&self) -> f32 {
		unsafe {
			CarComponent_TA_Get_ActivityTime(self.addr())
		}
	}
	fn get_replicated_activity_time(&self) -> f32 {
		unsafe {
			CarComponent_TA_Get_ReplicatedActivityTime(self.addr())
		}
	}
	fn get_fx_actor(&self) -> FXActorWrapper {
		unsafe {
			FXActorWrapper::new(CarComponent_TA_Get_FXActor(self.addr()))
		}
	}
	fn get_inactive_time(&self) -> f32 {
		unsafe {
			CarComponent_TA_GetInactiveTime(self.addr())
		}
	}
	fn get_active_time(&self) -> f32 {
		unsafe {
			CarComponent_TA_GetActiveTime(self.addr())
		}
	}
	fn apply_forces(&self, active_time: f32) {
		unsafe {
			CarComponent_TA_ApplyForces(self.addr(), active_time);
		}
	}
	fn pre_physics_step(&self, delta_time: f32) {
		unsafe {
			CarComponent_TA_PrePhysicsStep(self.addr(), delta_time);
		}
	}
	fn remove_from_car(&self) {
		unsafe {
			CarComponent_TA_RemoveFromCar(self.addr());
		}
	}
	fn can_deactivate(&self) -> bool {
		unsafe {
			CarComponent_TA_CanDeactivate(self.addr())
		}
	}
	fn conditional_deactivate(&self) -> bool {
		unsafe {
			CarComponent_TA_ConditionalDeactivate(self.addr())
		}
	}
	fn can_activate(&self) -> bool {
		unsafe {
			CarComponent_TA_CanActivate(self.addr())
		}
	}
	fn conditional_activate(&self) -> bool {
		unsafe {
			CarComponent_TA_ConditionalActivate(self.addr())
		}
	}
	fn set_active(&self, b_new_active: bool) {
		unsafe {
			CarComponent_TA_SetActive(self.addr(), b_new_active);
		}
	}
	fn deactivate(&self) {
		unsafe {
			CarComponent_TA_Deactivate(self.addr());
		}
	}
	fn activate(&self) {
		unsafe {
			CarComponent_TA_Activate(self.addr());
		}
	}
	fn unregister_car_events(&self) {
		unsafe {
			CarComponent_TA_UnregisterCarEvents(self.addr());
		}
	}
	fn register_car_events(&self) {
		unsafe {
			CarComponent_TA_RegisterCarEvents(self.addr());
		}
	}
	fn handle_vehicle_setup(&self, in_car: CarWrapper) {
		unsafe {
			CarComponent_TA_HandleVehicleSetup(self.addr(), in_car.addr());
		}
	}
	fn on_vehicle_setup_complete(&self) {
		unsafe {
			CarComponent_TA_OnVehicleSetupComplete(self.addr());
		}
	}
	fn create(&self, owner_car: CarWrapper, in_activator: PriWrapper) {
		unsafe {
			CarComponent_TA_Create(self.addr(), owner_car.addr(), in_activator.addr());
		}
	}
	fn client_update_active(&self) {
		unsafe {
			CarComponent_TA_ClientUpdateActive(self.addr());
		}
	}
	fn event_activation_changed(&self, car_component: CarComponentWrapper) {
		unsafe {
			CarComponent_TA_EventActivationChanged(self.addr(), car_component.addr());
		}
	}

}

extern "C" {
	fn CarComponent_TA_Get_FXActorArchetype(obj: usize) -> usize;
	fn CarComponentWrapper_SetFXActorArchetype(obj: usize, new_val: usize);
	fn CarComponent_TA_Get_bDisabled(obj: usize) -> bool;
	fn CarComponentWrapper_SetbDisabled(obj: usize, new_val: bool);
	fn CarComponent_TA_Get_bAutoActivate(obj: usize) -> bool;
	fn CarComponentWrapper_SetbAutoActivate(obj: usize, new_val: bool);
	fn CarComponent_TA_Get_bSimulateComponent(obj: usize) -> bool;
	fn CarComponentWrapper_SetbSimulateComponent(obj: usize, new_val: bool);
	fn CarComponent_TA_Get_bCreated(obj: usize) -> bool;
	fn CarComponentWrapper_SetbCreated(obj: usize, new_val: bool);
	fn CarComponent_TA_Get_bActive(obj: usize) -> bool;
	fn CarComponentWrapper_SetbActive(obj: usize, new_val: bool);
	fn CarComponent_TA_Get_bRemovedFromCar(obj: usize) -> bool;
	fn CarComponentWrapper_SetbRemovedFromCar(obj: usize, new_val: bool);
	fn CarComponent_TA_Get_ComponentData(obj: usize) -> u8;
	fn CarComponentWrapper_SetComponentData(obj: usize, new_val: u8);
	fn CarComponent_TA_Get_ReplicatedActive(obj: usize) -> u8;
	fn CarComponentWrapper_SetReplicatedActive(obj: usize, new_val: u8);
	fn CarComponent_TA_Get_Activator(obj: usize) -> usize;
	fn CarComponentWrapper_SetActivator(obj: usize, new_val: usize);
	fn CarComponent_TA_Get_Vehicle(obj: usize) -> usize;
	fn CarComponentWrapper_SetVehicle(obj: usize, new_val: usize);
	fn CarComponent_TA_Get_Car(obj: usize) -> usize;
	fn CarComponentWrapper_SetCar(obj: usize, new_val: usize);
	fn CarComponent_TA_Get_ActivityTime(obj: usize) -> f32;
	fn CarComponentWrapper_SetActivityTime(obj: usize, new_val: f32);
	fn CarComponent_TA_Get_ReplicatedActivityTime(obj: usize) -> f32;
	fn CarComponentWrapper_SetReplicatedActivityTime(obj: usize, new_val: f32);
	fn CarComponent_TA_Get_FXActor(obj: usize) -> usize;
	fn CarComponentWrapper_SetFXActor(obj: usize, new_val: usize);
	fn CarComponent_TA_GetInactiveTime(obj: usize) -> f32;
	fn CarComponent_TA_GetActiveTime(obj: usize) -> f32;
	fn CarComponent_TA_ApplyForces(obj: usize, ActiveTime: f32);
	fn CarComponent_TA_PrePhysicsStep(obj: usize, DeltaTime: f32);
	fn CarComponent_TA_RemoveFromCar(obj: usize);
	fn CarComponent_TA_CanDeactivate(obj: usize) -> bool;
	fn CarComponent_TA_ConditionalDeactivate(obj: usize) -> bool;
	fn CarComponent_TA_CanActivate(obj: usize) -> bool;
	fn CarComponent_TA_ConditionalActivate(obj: usize) -> bool;
	fn CarComponent_TA_SetActive(obj: usize, bNewActive: bool);
	fn CarComponent_TA_Deactivate(obj: usize);
	fn CarComponent_TA_Activate(obj: usize);
	fn CarComponent_TA_UnregisterCarEvents(obj: usize);
	fn CarComponent_TA_RegisterCarEvents(obj: usize);
	fn CarComponent_TA_HandleVehicleSetup(obj: usize, InCar: usize);
	fn CarComponent_TA_OnVehicleSetupComplete(obj: usize);
	fn CarComponent_TA_Create(obj: usize, OwnerCar: usize, InActivator: usize);
	fn CarComponent_TA_ClientUpdateActive(obj: usize);
	fn CarComponent_TA_EventActivationChanged(obj: usize, CarComponent: usize);

}