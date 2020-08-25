use crate::wrappers::*;
use crate::generated::*;

pub struct FXActorWrapper(pub usize);
impl_object!(FXActorWrapper);

impl FXActor for FXActorWrapper {}
impl Actor for FXActorWrapper {}

pub trait FXActor : Actor {
	fn get_b_deactivate_when_owner_destroyed(&self) -> bool {
		unsafe {
			FXActor_X_Get_bDeactivateWhenOwnerDestroyed(self.addr())
		}
	}
	fn get_b_allow_shadow_casting(&self) -> bool {
		unsafe {
			FXActor_X_Get_bAllowShadowCasting(self.addr())
		}
	}
	fn get_b_auto_activate(&self) -> bool {
		unsafe {
			FXActor_X_Get_bAutoActivate(self.addr())
		}
	}
	fn get_b_render_inactive(&self) -> bool {
		unsafe {
			FXActor_X_Get_bRenderInactive(self.addr())
		}
	}
	fn get_b_active(&self) -> bool {
		unsafe {
			FXActor_X_Get_bActive(self.addr())
		}
	}
	fn get_b_had_owner(&self) -> bool {
		unsafe {
			FXActor_X_Get_bHadOwner(self.addr())
		}
	}
	fn get_parent(&self) -> FXActorWrapper {
		unsafe {
			FXActorWrapper::new(FXActor_X_Get_Parent(self.addr()))
		}
	}
	fn get_attachment_actor(&self) -> ActorWrapper {
		unsafe {
			ActorWrapper::new(FXActor_X_Get_AttachmentActor(self.addr()))
		}
	}
	fn get_destroy_wait_time(&self) -> f32 {
		unsafe {
			FXActor_X_Get_DestroyWaitTime(self.addr())
		}
	}
	fn get_destroy_time(&self) -> f32 {
		unsafe {
			FXActor_X_Get_DestroyTime(self.addr())
		}
	}
	fn get_edit_id(&self) -> i32 {
		unsafe {
			FXActor_X_Get_EditID(self.addr())
		}
	}
	fn inherit(&self, other: FXActorWrapper) {
		unsafe {
			FXActor_X_Inherit(self.addr(), other.addr());
		}
	}
	fn reset_particles(&self) {
		unsafe {
			FXActor_X_ResetParticles(self.addr());
		}
	}
	fn stop_all_effects(&self) {
		unsafe {
			FXActor_X_StopAllEffects(self.addr());
		}
	}
	fn update_fx_states(&self) {
		unsafe {
			FXActor_X_UpdateFXStates(self.addr());
		}
	}
	fn is_locally_controlled(&self) -> bool {
		unsafe {
			FXActor_X_IsLocallyControlled(self.addr())
		}
	}
	fn deactivate(&self) {
		unsafe {
			FXActor_X_Deactivate(self.addr());
		}
	}
	fn activate(&self) {
		unsafe {
			FXActor_X_Activate(self.addr());
		}
	}
	fn bind_to(&self, parent_fx_actor: FXActorWrapper) {
		unsafe {
			FXActor_X_BindTo(self.addr(), parent_fx_actor.addr());
		}
	}
	fn set_attachment_actor2(&self, attach_to_actor: ActorWrapper) {
		unsafe {
			FXActor_X_SetAttachmentActor2(self.addr(), attach_to_actor.addr());
		}
	}
	fn post_begin_play(&self) {
		unsafe {
			FXActor_X_PostBeginPlay(self.addr());
		}
	}

}

extern "C" {
	fn FXActor_X_Get_bDeactivateWhenOwnerDestroyed(obj: usize) -> bool;
	fn FXActorWrapper_SetbDeactivateWhenOwnerDestroyed(obj: usize, new_val: bool);
	fn FXActor_X_Get_bAllowShadowCasting(obj: usize) -> bool;
	fn FXActorWrapper_SetbAllowShadowCasting(obj: usize, new_val: bool);
	fn FXActor_X_Get_bAutoActivate(obj: usize) -> bool;
	fn FXActorWrapper_SetbAutoActivate(obj: usize, new_val: bool);
	fn FXActor_X_Get_bRenderInactive(obj: usize) -> bool;
	fn FXActorWrapper_SetbRenderInactive(obj: usize, new_val: bool);
	fn FXActor_X_Get_bActive(obj: usize) -> bool;
	fn FXActorWrapper_SetbActive(obj: usize, new_val: bool);
	fn FXActor_X_Get_bHadOwner(obj: usize) -> bool;
	fn FXActorWrapper_SetbHadOwner(obj: usize, new_val: bool);
	fn FXActor_X_Get_Parent(obj: usize) -> usize;
	fn FXActorWrapper_SetParent(obj: usize, new_val: usize);
	fn FXActor_X_Get_AttachmentActor(obj: usize) -> usize;
	fn FXActorWrapper_SetAttachmentActor(obj: usize, new_val: usize);
	fn FXActor_X_Get_DestroyWaitTime(obj: usize) -> f32;
	fn FXActorWrapper_SetDestroyWaitTime(obj: usize, new_val: f32);
	fn FXActor_X_Get_DestroyTime(obj: usize) -> f32;
	fn FXActorWrapper_SetDestroyTime(obj: usize, new_val: f32);
	fn FXActor_X_Get_EditID(obj: usize) -> i32;
	fn FXActorWrapper_SetEditID(obj: usize, new_val: i32);
	fn FXActor_X_Inherit(obj: usize, Other: usize);
	fn FXActor_X_ResetParticles(obj: usize);
	fn FXActor_X_StopAllEffects(obj: usize);
	fn FXActor_X_UpdateFXStates(obj: usize);
	fn FXActor_X_IsLocallyControlled(obj: usize) -> bool;
	fn FXActor_X_Deactivate(obj: usize);
	fn FXActor_X_Activate(obj: usize);
	fn FXActor_X_BindTo(obj: usize, ParentFXActor: usize);
	fn FXActor_X_SetAttachmentActor2(obj: usize, AttachToActor: usize);
	fn FXActor_X_PostBeginPlay(obj: usize);

}