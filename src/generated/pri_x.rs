use crate::wrappers::*;
use crate::generated::*;

pub struct PriXWrapper(pub usize);
impl_object!(PriXWrapper);

impl PriX for PriXWrapper {}
impl PlayerReplicationInfo for PriXWrapper {}
impl Actor for PriXWrapper {}

pub trait PriX : PlayerReplicationInfo {
	fn on_unique_id_changed(&self) {
		unsafe {
			PRI_X_OnUniqueIdChanged(self.addr());
		}
	}
	fn set_unique_id(&self, player_unique_id: UniqueNetId) {
		unsafe {
			let mut player_unique_id = player_unique_id;
			let player_unique_id: *mut UniqueNetId = &mut player_unique_id as *mut UniqueNetId;
			PRI_X_SetUniqueId(self.addr(), player_unique_id);
		}
	}
	fn unregister_player_from_session(&self) {
		unsafe {
			PRI_X_UnregisterPlayerFromSession(self.addr());
		}
	}
	fn register_player_with_session(&self) {
		unsafe {
			PRI_X_RegisterPlayerWithSession(self.addr());
		}
	}
	fn event_destroyed(&self, pri: PriXWrapper) {
		unsafe {
			PRI_X_EventDestroyed(self.addr(), pri.addr());
		}
	}
	fn event_unique_id_changed(&self, pri: PriXWrapper) {
		unsafe {
			PRI_X_EventUniqueIdChanged(self.addr(), pri.addr());
		}
	}
	fn event_player_name_changed(&self, pri: PriXWrapper) {
		unsafe {
			PRI_X_EventPlayerNameChanged(self.addr(), pri.addr());
		}
	}

}

extern "C" {
	fn PRI_X_OnUniqueIdChanged(obj: usize);
	fn PRI_X_SetUniqueId(obj: usize, PlayerUniqueId: *mut UniqueNetId);
	fn PRI_X_UnregisterPlayerFromSession(obj: usize);
	fn PRI_X_RegisterPlayerWithSession(obj: usize);
	fn PRI_X_EventDestroyed(obj: usize, PRI: usize);
	fn PRI_X_EventUniqueIdChanged(obj: usize, PRI: usize);
	fn PRI_X_EventPlayerNameChanged(obj: usize, PRI: usize);

}