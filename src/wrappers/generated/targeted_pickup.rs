use crate::wrappers::*;
use super::*;

pub struct TargetedPickupWrapper(pub usize);
impl_object!(TargetedPickupWrapper);

impl TargetedPickup for TargetedPickupWrapper {}
impl RumblePickupComponent for TargetedPickupWrapper {}
impl CarComponent for TargetedPickupWrapper {}
impl Actor for TargetedPickupWrapper {}

pub trait TargetedPickup : RumblePickupComponent {
    fn get_b_can_target_ball(&self) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_Get_bCanTargetBall(self.addr())
        }
    }
    fn get_b_can_target_cars(&self) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_Get_bCanTargetCars(self.addr())
        }
    }
    fn get_b_can_target_enemy_cars(&self) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_Get_bCanTargetEnemyCars(self.addr())
        }
    }
    fn get_b_can_target_team_cars(&self) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_Get_bCanTargetTeamCars(self.addr())
        }
    }
    fn get_b_use_directional_targeting(&self) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_Get_bUseDirectionalTargeting(self.addr())
        }
    }
    fn get_b_require_trace(&self) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_Get_bRequireTrace(self.addr())
        }
    }
    fn get_range(&self) -> f32 {
        unsafe {
            SpecialPickup_Targeted_TA_Get_Range(self.addr())
        }
    }
    fn get_directional_targeting_accuracy(&self) -> f32 {
        unsafe {
            SpecialPickup_Targeted_TA_Get_DirectionalTargetingAccuracy(self.addr())
        }
    }
    fn get_client_target(&self) -> Option<RBActorWrapper> {
        unsafe {
            RBActorWrapper::try_new(SpecialPickup_Targeted_TA_Get_ClientTarget(self.addr()))
        }
    }
    fn get_targeted(&self) -> Option<RBActorWrapper> {
        unsafe {
            RBActorWrapper::try_new(SpecialPickup_Targeted_TA_Get_Targeted(self.addr()))
        }
    }
    fn get_client_target2(&self) -> Option<RBActorWrapper> {
        unsafe {
            RBActorWrapper::try_new(SpecialPickup_Targeted_TA_GetClientTarget2(self.addr()))
        }
    }
    fn target_changed(&self) {
        unsafe {
            SpecialPickup_Targeted_TA_TargetChanged(self.addr());
        }
    }
    fn on_target_changed(&self) {
        unsafe {
            SpecialPickup_Targeted_TA_OnTargetChanged(self.addr());
        }
    }
    fn try_activate(&self, target_override: RBActorWrapper) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_TryActivate(self.addr(), target_override.addr())
        }
    }
    fn validate_target_trace(&self, in_target: RBActorWrapper) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_ValidateTargetTrace(self.addr(), in_target.addr())
        }
    }
    fn validate_target(&self, in_target: RBActorWrapper) -> bool {
        unsafe {
            SpecialPickup_Targeted_TA_ValidateTarget(self.addr(), in_target.addr())
        }
    }
    fn get_target(&self) -> Option<RBActorWrapper> {
        unsafe {
            RBActorWrapper::try_new(SpecialPickup_Targeted_TA_GetTarget(self.addr()))
        }
    }

}

extern "C" {
    fn SpecialPickup_Targeted_TA_Get_bCanTargetBall(obj: usize) -> bool;
    fn TargetedPickup_SetbCanTargetBall(obj: usize, new_val: bool);
    fn SpecialPickup_Targeted_TA_Get_bCanTargetCars(obj: usize) -> bool;
    fn TargetedPickup_SetbCanTargetCars(obj: usize, new_val: bool);
    fn SpecialPickup_Targeted_TA_Get_bCanTargetEnemyCars(obj: usize) -> bool;
    fn TargetedPickup_SetbCanTargetEnemyCars(obj: usize, new_val: bool);
    fn SpecialPickup_Targeted_TA_Get_bCanTargetTeamCars(obj: usize) -> bool;
    fn TargetedPickup_SetbCanTargetTeamCars(obj: usize, new_val: bool);
    fn SpecialPickup_Targeted_TA_Get_bUseDirectionalTargeting(obj: usize) -> bool;
    fn TargetedPickup_SetbUseDirectionalTargeting(obj: usize, new_val: bool);
    fn SpecialPickup_Targeted_TA_Get_bRequireTrace(obj: usize) -> bool;
    fn TargetedPickup_SetbRequireTrace(obj: usize, new_val: bool);
    fn SpecialPickup_Targeted_TA_Get_Range(obj: usize) -> f32;
    fn TargetedPickup_SetRange(obj: usize, new_val: f32);
    fn SpecialPickup_Targeted_TA_Get_DirectionalTargetingAccuracy(obj: usize) -> f32;
    fn TargetedPickup_SetDirectionalTargetingAccuracy(obj: usize, new_val: f32);
    fn SpecialPickup_Targeted_TA_Get_ClientTarget(obj: usize) -> usize;
    fn TargetedPickup_SetClientTarget(obj: usize, new_val: usize);
    fn SpecialPickup_Targeted_TA_Get_Targeted(obj: usize) -> usize;
    fn TargetedPickup_SetTargeted(obj: usize, new_val: usize);
    fn SpecialPickup_Targeted_TA_GetClientTarget2(obj: usize) -> usize;
    fn SpecialPickup_Targeted_TA_TargetChanged(obj: usize);
    fn SpecialPickup_Targeted_TA_OnTargetChanged(obj: usize);
    fn SpecialPickup_Targeted_TA_TryActivate(obj: usize, TargetOverride: usize) -> bool;
    fn SpecialPickup_Targeted_TA_ValidateTargetTrace(obj: usize, InTarget: usize) -> bool;
    fn SpecialPickup_Targeted_TA_ValidateTarget(obj: usize, InTarget: usize) -> bool;
    fn SpecialPickup_Targeted_TA_GetTarget(obj: usize) -> usize;

}