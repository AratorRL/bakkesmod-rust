use crate::wrappers::*;
use crate::generated::*;

pub struct PrimitiveComponentWrapper(pub usize);
impl_object!(PrimitiveComponentWrapper);

impl PrimitiveComponent for PrimitiveComponentWrapper {}

pub trait PrimitiveComponent : Object {
    fn get_rb_channel(&self) -> u8 {
        unsafe {
            PrimitiveComponent_Get_RBChannel(self.addr())
        }
    }
    fn get_rb_dominance_group(&self) -> u8 {
        unsafe {
            PrimitiveComponent_Get_RBDominanceGroup(self.addr())
        }
    }
    fn get_b_only_block_actor_movement(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bOnlyBlockActorMovement(self.addr())
        }
    }
    fn get_hidden_game(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_HiddenGame(self.addr())
        }
    }
    fn get_b_owner_no_see(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bOwnerNoSee(self.addr())
        }
    }
    fn get_b_only_owner_see(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bOnlyOwnerSee(self.addr())
        }
    }
    fn get_b_ignore_owner_hidden(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bIgnoreOwnerHidden(self.addr())
        }
    }
    fn get_b_use_as_occluder(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bUseAsOccluder(self.addr())
        }
    }
    fn get_b_allow_approximate_occlusion(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bAllowApproximateOcclusion(self.addr())
        }
    }
    fn get_b_first_frame_occlusion(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bFirstFrameOcclusion(self.addr())
        }
    }
    fn get_b_ignore_near_plane_intersection(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bIgnoreNearPlaneIntersection(self.addr())
        }
    }
    fn get_b_accepts_static_decals(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bAcceptsStaticDecals(self.addr())
        }
    }
    fn get_b_accepts_dynamic_decals(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bAcceptsDynamicDecals(self.addr())
        }
    }
    fn get_b_is_refreshing_decals(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bIsRefreshingDecals(self.addr())
        }
    }
    fn get_cast_shadow(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_CastShadow(self.addr())
        }
    }
    fn get_b_force_direct_light_map(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bForceDirectLightMap(self.addr())
        }
    }
    fn get_b_cast_dynamic_shadow(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bCastDynamicShadow(self.addr())
        }
    }
    fn get_b_cast_static_shadow(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bCastStaticShadow(self.addr())
        }
    }
    fn get_b_self_shadow_only(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bSelfShadowOnly(self.addr())
        }
    }
    fn get_b_no_mod_self_shadow(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bNoModSelfShadow(self.addr())
        }
    }
    fn get_b_accepts_dynamic_dominant_light_shadows(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bAcceptsDynamicDominantLightShadows(self.addr())
        }
    }
    fn get_b_cast_hidden_shadow(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bCastHiddenShadow(self.addr())
        }
    }
    fn get_b_cast_shadow_as_two_sided(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bCastShadowAsTwoSided(self.addr())
        }
    }
    fn get_b_accepts_lights(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bAcceptsLights(self.addr())
        }
    }
    fn get_b_accepts_dynamic_lights(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bAcceptsDynamicLights(self.addr())
        }
    }
    fn get_b_use_one_pass_lighting_on_translucency(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bUseOnePassLightingOnTranslucency(self.addr())
        }
    }
    fn get_b_use_precomputed_shadows(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bUsePrecomputedShadows(self.addr())
        }
    }
    fn get_b_has_explicit_shadow_parent(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bHasExplicitShadowParent(self.addr())
        }
    }
    fn get_collide_actors(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_CollideActors(self.addr())
        }
    }
    fn get_always_check_collision(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_AlwaysCheckCollision(self.addr())
        }
    }
    fn get_block_actors(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_BlockActors(self.addr())
        }
    }
    fn get_block_zero_extent(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_BlockZeroExtent(self.addr())
        }
    }
    fn get_block_non_zero_extent(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_BlockNonZeroExtent(self.addr())
        }
    }
    fn get_can_block_camera(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_CanBlockCamera(self.addr())
        }
    }
    fn get_block_rigid_body(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_BlockRigidBody(self.addr())
        }
    }
    fn get_b_block_foot_placement(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bBlockFootPlacement(self.addr())
        }
    }
    fn get_b_disable_all_rigid_body(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bDisableAllRigidBody(self.addr())
        }
    }
    fn get_b_skip_rb_geom_creation(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bSkipRBGeomCreation(self.addr())
        }
    }
    fn get_b_notify_rigid_body_collision(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bNotifyRigidBodyCollision(self.addr())
        }
    }
    fn get_b_fluid_drain(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bFluidDrain(self.addr())
        }
    }
    fn get_b_fluid_two_way(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bFluidTwoWay(self.addr())
        }
    }
    fn get_b_ignore_radial_impulse(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bIgnoreRadialImpulse(self.addr())
        }
    }
    fn get_b_ignore_radial_force(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bIgnoreRadialForce(self.addr())
        }
    }
    fn get_b_ignore_force_field(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bIgnoreForceField(self.addr())
        }
    }
    fn get_b_use_compartment(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bUseCompartment(self.addr())
        }
    }
    fn get_always_load_on_client(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_AlwaysLoadOnClient(self.addr())
        }
    }
    fn get_always_load_on_server(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_AlwaysLoadOnServer(self.addr())
        }
    }
    fn get_b_ignore_hidden_actors_membership(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_bIgnoreHiddenActorsMembership(self.addr())
        }
    }
    fn get_absolute_translation(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_AbsoluteTranslation(self.addr())
        }
    }
    fn get_absolute_rotation(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_AbsoluteRotation(self.addr())
        }
    }
    fn get_absolute_scale(&self) -> bool {
        unsafe {
            PrimitiveComponent_Get_AbsoluteScale(self.addr())
        }
    }
    fn get_visibility_id(&self) -> i32 {
        unsafe {
            PrimitiveComponent_Get_VisibilityId(self.addr())
        }
    }
    fn get_translation(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            PrimitiveComponent_Get_Translation(self.addr(), result_ptr);
            result
        }
    }
    fn get_rotation(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            PrimitiveComponent_Get_Rotation(self.addr(), result_ptr);
            result
        }
    }
    fn get_scale(&self) -> f32 {
        unsafe {
            PrimitiveComponent_Get_Scale(self.addr())
        }
    }
    fn get_scale3_d(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            PrimitiveComponent_Get_Scale3D(self.addr(), result_ptr);
            result
        }
    }
    fn get_bounds_scale(&self) -> f32 {
        unsafe {
            PrimitiveComponent_Get_BoundsScale(self.addr())
        }
    }
    fn get_last_render_time(&self) -> f32 {
        unsafe {
            PrimitiveComponent_Get_LastRenderTime(self.addr())
        }
    }
    fn get_script_rigid_body_collision_threshold(&self) -> f32 {
        unsafe {
            PrimitiveComponent_Get_ScriptRigidBodyCollisionThreshold(self.addr())
        }
    }
    fn term_rb_phys(&self) {
        unsafe {
            PrimitiveComponent_TermRBPhys(self.addr());
        }
    }
    fn init_rb_phys(&self) {
        unsafe {
            PrimitiveComponent_InitRBPhys(self.addr());
        }
    }
    fn set_notify_rigid_body_collision(&self, b_new_notify_rigid_body_collision: bool) {
        unsafe {
            PrimitiveComponent_SetNotifyRigidBodyCollision(self.addr(), b_new_notify_rigid_body_collision);
        }
    }
    fn set_rb_channel2(&self, channel: u8) {
        unsafe {
            PrimitiveComponent_SetRBChannel2(self.addr(), channel);
        }
    }
    fn set_rb_collides_with_channel(&self, channel: u8, b_new_collides: bool) {
        unsafe {
            PrimitiveComponent_SetRBCollidesWithChannel(self.addr(), channel, b_new_collides);
        }
    }
    fn set_block_rigid_body2(&self, b_new_block_rigid_body: bool) {
        unsafe {
            PrimitiveComponent_SetBlockRigidBody2(self.addr(), b_new_block_rigid_body);
        }
    }
    fn retard_rb_linear_velocity(&self, retard_dir: Vector, vel_scale: f32) {
        unsafe {
            let mut retard_dir = retard_dir;
            let retard_dir: *mut Vector = &mut retard_dir as *mut Vector;
            PrimitiveComponent_RetardRBLinearVelocity(self.addr(), retard_dir, vel_scale);
        }
    }
    fn set_rb_angular_velocity(&self, new_ang_vel: Vector, b_add_to_current: bool) {
        unsafe {
            let mut new_ang_vel = new_ang_vel;
            let new_ang_vel: *mut Vector = &mut new_ang_vel as *mut Vector;
            PrimitiveComponent_SetRBAngularVelocity(self.addr(), new_ang_vel, b_add_to_current);
        }
    }
    fn set_rb_linear_velocity(&self, new_vel: Vector, b_add_to_current: bool) {
        unsafe {
            let mut new_vel = new_vel;
            let new_vel: *mut Vector = &mut new_vel as *mut Vector;
            PrimitiveComponent_SetRBLinearVelocity(self.addr(), new_vel, b_add_to_current);
        }
    }
    fn add_radial_force(&self, origin: Vector, radius: f32, strength: f32, falloff: u8) {
        unsafe {
            let mut origin = origin;
            let origin: *mut Vector = &mut origin as *mut Vector;
            PrimitiveComponent_AddRadialForce(self.addr(), origin, radius, strength, falloff);
        }
    }
    fn add_radial_impulse(&self, origin: Vector, radius: f32, strength: f32, falloff: u8, b_vel_change: bool) {
        unsafe {
            let mut origin = origin;
            let origin: *mut Vector = &mut origin as *mut Vector;
            PrimitiveComponent_AddRadialImpulse(self.addr(), origin, radius, strength, falloff, b_vel_change);
        }
    }

}

extern "C" {
    fn PrimitiveComponent_Get_RBChannel(obj: usize) -> u8;
    fn PrimitiveComponentWrapper_SetRBChannel(obj: usize, new_val: u8);
    fn PrimitiveComponent_Get_RBDominanceGroup(obj: usize) -> u8;
    fn PrimitiveComponentWrapper_SetRBDominanceGroup(obj: usize, new_val: u8);
    fn PrimitiveComponent_Get_bOnlyBlockActorMovement(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbOnlyBlockActorMovement(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_HiddenGame(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetHiddenGame(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bOwnerNoSee(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbOwnerNoSee(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bOnlyOwnerSee(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbOnlyOwnerSee(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bIgnoreOwnerHidden(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbIgnoreOwnerHidden(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bUseAsOccluder(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbUseAsOccluder(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bAllowApproximateOcclusion(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbAllowApproximateOcclusion(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bFirstFrameOcclusion(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbFirstFrameOcclusion(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bIgnoreNearPlaneIntersection(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbIgnoreNearPlaneIntersection(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bAcceptsStaticDecals(obj: usize) -> bool;
    fn PrimitiveComponent_Get_bAcceptsDynamicDecals(obj: usize) -> bool;
    fn PrimitiveComponent_Get_bIsRefreshingDecals(obj: usize) -> bool;
    fn PrimitiveComponent_Get_CastShadow(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetCastShadow(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bForceDirectLightMap(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbForceDirectLightMap(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bCastDynamicShadow(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbCastDynamicShadow(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bCastStaticShadow(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbCastStaticShadow(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bSelfShadowOnly(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbSelfShadowOnly(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bNoModSelfShadow(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbNoModSelfShadow(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bAcceptsDynamicDominantLightShadows(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbAcceptsDynamicDominantLightShadows(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bCastHiddenShadow(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbCastHiddenShadow(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bCastShadowAsTwoSided(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbCastShadowAsTwoSided(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bAcceptsLights(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbAcceptsLights(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bAcceptsDynamicLights(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbAcceptsDynamicLights(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bUseOnePassLightingOnTranslucency(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbUseOnePassLightingOnTranslucency(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bUsePrecomputedShadows(obj: usize) -> bool;
    fn PrimitiveComponent_Get_bHasExplicitShadowParent(obj: usize) -> bool;
    fn PrimitiveComponent_Get_CollideActors(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetCollideActors(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_AlwaysCheckCollision(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetAlwaysCheckCollision(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_BlockActors(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetBlockActors(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_BlockZeroExtent(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetBlockZeroExtent(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_BlockNonZeroExtent(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetBlockNonZeroExtent(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_CanBlockCamera(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetCanBlockCamera(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_BlockRigidBody(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetBlockRigidBody(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bBlockFootPlacement(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbBlockFootPlacement(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bDisableAllRigidBody(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbDisableAllRigidBody(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bSkipRBGeomCreation(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbSkipRBGeomCreation(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bNotifyRigidBodyCollision(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbNotifyRigidBodyCollision(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bFluidDrain(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbFluidDrain(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bFluidTwoWay(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbFluidTwoWay(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bIgnoreRadialImpulse(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbIgnoreRadialImpulse(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bIgnoreRadialForce(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbIgnoreRadialForce(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bIgnoreForceField(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbIgnoreForceField(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bUseCompartment(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbUseCompartment(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_AlwaysLoadOnClient(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetAlwaysLoadOnClient(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_AlwaysLoadOnServer(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetAlwaysLoadOnServer(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_bIgnoreHiddenActorsMembership(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetbIgnoreHiddenActorsMembership(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_AbsoluteTranslation(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetAbsoluteTranslation(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_AbsoluteRotation(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetAbsoluteRotation(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_AbsoluteScale(obj: usize) -> bool;
    fn PrimitiveComponentWrapper_SetAbsoluteScale(obj: usize, new_val: bool);
    fn PrimitiveComponent_Get_VisibilityId(obj: usize) -> i32;
    fn PrimitiveComponentWrapper_SetVisibilityId(obj: usize, new_val: i32);
    fn PrimitiveComponent_Get_Translation(obj: usize, result: *mut Vector);
    fn PrimitiveComponentWrapper_SetTranslation(obj: usize, new_val: *mut Vector);
    fn PrimitiveComponent_Get_Rotation(obj: usize, result: *mut Rotator);
    fn PrimitiveComponentWrapper_SetRotation(obj: usize, new_val: *mut Rotator);
    fn PrimitiveComponent_Get_Scale(obj: usize) -> f32;
    fn PrimitiveComponentWrapper_SetScale(obj: usize, new_val: f32);
    fn PrimitiveComponent_Get_Scale3D(obj: usize, result: *mut Vector);
    fn PrimitiveComponentWrapper_SetScale3D(obj: usize, new_val: *mut Vector);
    fn PrimitiveComponent_Get_BoundsScale(obj: usize) -> f32;
    fn PrimitiveComponentWrapper_SetBoundsScale(obj: usize, new_val: f32);
    fn PrimitiveComponent_Get_LastRenderTime(obj: usize) -> f32;
    fn PrimitiveComponent_Get_ScriptRigidBodyCollisionThreshold(obj: usize) -> f32;
    fn PrimitiveComponentWrapper_SetScriptRigidBodyCollisionThreshold(obj: usize, new_val: f32);
    fn PrimitiveComponent_TermRBPhys(obj: usize);
    fn PrimitiveComponent_InitRBPhys(obj: usize);
    fn PrimitiveComponent_SetNotifyRigidBodyCollision(obj: usize, bNewNotifyRigidBodyCollision: bool);
    fn PrimitiveComponent_SetRBChannel2(obj: usize, Channel: u8);
    fn PrimitiveComponent_SetRBCollidesWithChannel(obj: usize, Channel: u8, bNewCollides: bool);
    fn PrimitiveComponent_SetBlockRigidBody2(obj: usize, bNewBlockRigidBody: bool);
    fn PrimitiveComponent_RetardRBLinearVelocity(obj: usize, RetardDir: *mut Vector, VelScale: f32);
    fn PrimitiveComponent_SetRBAngularVelocity(obj: usize, NewAngVel: *mut Vector, bAddToCurrent: bool);
    fn PrimitiveComponent_SetRBLinearVelocity(obj: usize, NewVel: *mut Vector, bAddToCurrent: bool);
    fn PrimitiveComponent_AddRadialForce(obj: usize, Origin: *mut Vector, Radius: f32, Strength: f32, Falloff: u8);
    fn PrimitiveComponent_AddRadialImpulse(obj: usize, Origin: *mut Vector, Radius: f32, Strength: f32, Falloff: u8, bVelChange: bool);

}