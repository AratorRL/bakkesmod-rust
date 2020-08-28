use crate::wrappers::*;
use crate::generated::*;

pub struct ActorWrapper(pub usize);
impl_object!(ActorWrapper);

impl Actor for ActorWrapper {}

pub trait Actor : Object {
    fn get_location(&self) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Actor_GetLocation(self.addr(), result_ptr); }
        result
    }

    fn set_location(&self, new_loc: Vector) {
        let mut new_loc = new_loc;
        let new_loc = &mut new_loc as *mut Vector;
        unsafe { Actor_SetLocation(self.addr(), new_loc); }
    }

    fn get_velocity(&self) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Actor_GetVelocity(self.addr(), result_ptr); }
        result
    }

    fn set_velocity(&self, new_vel: Vector) {
        let mut new_vel = new_vel;
        let new_vel = &mut new_vel as *mut Vector;
        unsafe { Actor_SetVelocity(self.addr(), new_vel); }
    }

    fn add_velocity(&self, vel: Vector) {
        let mut vel = vel;
        let vel = &mut vel as *mut Vector;
        unsafe { Actor_AddVelocity(self.addr(), vel); }
    }
    
    fn get_rotation(&self) -> Rotator {
        let mut result = Rotator::new();
        let result_ptr = &mut result as *mut Rotator;
        unsafe { Actor_GetRotation(self.addr(), result_ptr); }
        result
    }

    fn set_rotation(&self, new_rot: Rotator) {
        let mut new_rot = new_rot;
        let new_rot = &mut new_rot as *mut Rotator;
        unsafe { Actor_SetRotation(self.addr(), new_rot); }
    }

    fn set_torque(&self, new_torque: Vector) {
        let mut new_torque = new_torque;
        let new_torque = &mut new_torque as *mut Vector;
        unsafe { Actor_SetTorque(self.addr(), new_torque); }
    }

    fn get_angular_velocity(&self) -> Vector {
        let mut result = Vector::new();
        let result_ptr = &mut result as *mut Vector;
        unsafe { Actor_GetAngularVelocity(self.addr(), result_ptr); }
        result
    }

    fn set_angular_velocity(&self, new_vel: Vector, add_to_current: bool) {
        let mut new_vel = new_vel;
        let new_vel = &mut new_vel as *mut Vector;
        unsafe { Actor_SetAngularVelocity(self.addr(), new_vel, add_to_current); }
    }

    fn stop(&self) {
        unsafe { Actor_Stop(self.addr()) }
    }

    fn is_null(&self) {
        unsafe { Actor_IsNull(self.addr()) }
    }

    fn get_draw_scale(&self) -> f32 {
        unsafe {
            Actor_Get_DrawScale(self.addr())
        }
    }
    fn get_draw_scale3_d(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Actor_Get_DrawScale3D(self.addr(), result_ptr);
            result
        }
    }
    fn get_pre_pivot(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Actor_Get_PrePivot(self.addr(), result_ptr);
            result
        }
    }
    fn get_editor_icon_color(&self) -> Color {
        unsafe {
            let mut result = Color::new();
            let result_ptr: *mut Color = &mut result as *mut Color;
            Actor_Get_EditorIconColor(self.addr(), result_ptr);
            result
        }
    }
    fn get_custom_time_dilation(&self) -> f32 {
        unsafe {
            Actor_Get_CustomTimeDilation(self.addr())
        }
    }
    fn get_physics(&self) -> u8 {
        unsafe {
            Actor_Get_Physics(self.addr())
        }
    }
    fn get_remote_role(&self) -> u8 {
        unsafe {
            Actor_Get_RemoteRole(self.addr())
        }
    }
    fn get_role(&self) -> u8 {
        unsafe {
            Actor_Get_Role(self.addr())
        }
    }
    fn get_collision_type(&self) -> u8 {
        unsafe {
            Actor_Get_CollisionType(self.addr())
        }
    }
    fn get_replicated_collision_type(&self) -> u8 {
        unsafe {
            Actor_Get_ReplicatedCollisionType(self.addr())
        }
    }
    fn get_owner(&self) -> Option<ActorWrapper> {
        unsafe {
            ActorWrapper::try_new(Actor_Get_Owner(self.addr()))
        }
    }
    fn get_base(&self) -> Option<ActorWrapper> {
        unsafe {
            ActorWrapper::try_new(Actor_Get_Base(self.addr()))
        }
    }
    fn get_b_static(&self) -> bool {
        unsafe {
            Actor_Get_bStatic(self.addr())
        }
    }
    fn get_b_hidden(&self) -> bool {
        unsafe {
            Actor_Get_bHidden(self.addr())
        }
    }
    fn get_b_hidden_self(&self) -> bool {
        unsafe {
            Actor_Get_bHiddenSelf(self.addr())
        }
    }
    fn get_b_no_delete(&self) -> bool {
        unsafe {
            Actor_Get_bNoDelete(self.addr())
        }
    }
    fn get_b_delete_me(&self) -> bool {
        unsafe {
            Actor_Get_bDeleteMe(self.addr())
        }
    }
    fn get_b_ticked(&self) -> bool {
        unsafe {
            Actor_Get_bTicked(self.addr())
        }
    }
    fn get_b_only_owner_see(&self) -> bool {
        unsafe {
            Actor_Get_bOnlyOwnerSee(self.addr())
        }
    }
    fn get_b_tick_is_disabled(&self) -> bool {
        unsafe {
            Actor_Get_bTickIsDisabled(self.addr())
        }
    }
    fn get_b_world_geometry(&self) -> bool {
        unsafe {
            Actor_Get_bWorldGeometry(self.addr())
        }
    }
    fn get_b_ignore_rigid_body_pawns(&self) -> bool {
        unsafe {
            Actor_Get_bIgnoreRigidBodyPawns(self.addr())
        }
    }
    fn get_b_orient_on_slope(&self) -> bool {
        unsafe {
            Actor_Get_bOrientOnSlope(self.addr())
        }
    }
    fn get_b_is_moving(&self) -> bool {
        unsafe {
            Actor_Get_bIsMoving(self.addr())
        }
    }
    fn get_b_always_encroach_check(&self) -> bool {
        unsafe {
            Actor_Get_bAlwaysEncroachCheck(self.addr())
        }
    }
    fn get_b_has_alternate_target_location(&self) -> bool {
        unsafe {
            Actor_Get_bHasAlternateTargetLocation(self.addr())
        }
    }
    fn get_b_always_relevant(&self) -> bool {
        unsafe {
            Actor_Get_bAlwaysRelevant(self.addr())
        }
    }
    fn get_b_replicate_instigator(&self) -> bool {
        unsafe {
            Actor_Get_bReplicateInstigator(self.addr())
        }
    }
    fn get_b_replicate_movement(&self) -> bool {
        unsafe {
            Actor_Get_bReplicateMovement(self.addr())
        }
    }
    fn get_b_update_simulated_position(&self) -> bool {
        unsafe {
            Actor_Get_bUpdateSimulatedPosition(self.addr())
        }
    }
    fn get_b_demo_recording(&self) -> bool {
        unsafe {
            Actor_Get_bDemoRecording(self.addr())
        }
    }
    fn get_b_demo_owner(&self) -> bool {
        unsafe {
            Actor_Get_bDemoOwner(self.addr())
        }
    }
    fn get_b_force_demo_relevant(&self) -> bool {
        unsafe {
            Actor_Get_bForceDemoRelevant(self.addr())
        }
    }
    fn get_b_net_initial_rotation(&self) -> bool {
        unsafe {
            Actor_Get_bNetInitialRotation(self.addr())
        }
    }
    fn get_b_replicate_rigid_body_location(&self) -> bool {
        unsafe {
            Actor_Get_bReplicateRigidBodyLocation(self.addr())
        }
    }
    fn get_b_kill_during_level_transition(&self) -> bool {
        unsafe {
            Actor_Get_bKillDuringLevelTransition(self.addr())
        }
    }
    fn get_b_post_render_if_not_visible(&self) -> bool {
        unsafe {
            Actor_Get_bPostRenderIfNotVisible(self.addr())
        }
    }
    fn get_b_force_net_update(&self) -> bool {
        unsafe {
            Actor_Get_bForceNetUpdate(self.addr())
        }
    }
    fn get_b_force_packet_update(&self) -> bool {
        unsafe {
            Actor_Get_bForcePacketUpdate(self.addr())
        }
    }
    fn get_b_pending_net_update(&self) -> bool {
        unsafe {
            Actor_Get_bPendingNetUpdate(self.addr())
        }
    }
    fn get_b_game_relevant(&self) -> bool {
        unsafe {
            Actor_Get_bGameRelevant(self.addr())
        }
    }
    fn get_b_movable(&self) -> bool {
        unsafe {
            Actor_Get_bMovable(self.addr())
        }
    }
    fn get_b_can_teleport(&self) -> bool {
        unsafe {
            Actor_Get_bCanTeleport(self.addr())
        }
    }
    fn get_b_always_tick(&self) -> bool {
        unsafe {
            Actor_Get_bAlwaysTick(self.addr())
        }
    }
    fn get_b_blocks_navigation(&self) -> bool {
        unsafe {
            Actor_Get_bBlocksNavigation(self.addr())
        }
    }
    fn get_block_rigid_body(&self) -> bool {
        unsafe {
            Actor_Get_BlockRigidBody(self.addr())
        }
    }
    fn get_b_collide_when_placing(&self) -> bool {
        unsafe {
            Actor_Get_bCollideWhenPlacing(self.addr())
        }
    }
    fn get_b_collide_actors(&self) -> bool {
        unsafe {
            Actor_Get_bCollideActors(self.addr())
        }
    }
    fn get_b_collide_world(&self) -> bool {
        unsafe {
            Actor_Get_bCollideWorld(self.addr())
        }
    }
    fn get_b_collide_complex(&self) -> bool {
        unsafe {
            Actor_Get_bCollideComplex(self.addr())
        }
    }
    fn get_b_block_actors(&self) -> bool {
        unsafe {
            Actor_Get_bBlockActors(self.addr())
        }
    }
    fn get_b_blocks_teleport(&self) -> bool {
        unsafe {
            Actor_Get_bBlocksTeleport(self.addr())
        }
    }
    fn get_b_phys_rigid_body_out_of_world_check(&self) -> bool {
        unsafe {
            Actor_Get_bPhysRigidBodyOutOfWorldCheck(self.addr())
        }
    }
    fn get_b_component_outside_world(&self) -> bool {
        unsafe {
            Actor_Get_bComponentOutsideWorld(self.addr())
        }
    }
    fn get_b_rigid_body_was_awake(&self) -> bool {
        unsafe {
            Actor_Get_bRigidBodyWasAwake(self.addr())
        }
    }
    fn get_b_call_rigid_body_wake_events(&self) -> bool {
        unsafe {
            Actor_Get_bCallRigidBodyWakeEvents(self.addr())
        }
    }
    fn get_b_bounce(&self) -> bool {
        unsafe {
            Actor_Get_bBounce(self.addr())
        }
    }
    fn get_b_editable(&self) -> bool {
        unsafe {
            Actor_Get_bEditable(self.addr())
        }
    }
    fn get_b_lock_location(&self) -> bool {
        unsafe {
            Actor_Get_bLockLocation(self.addr())
        }
    }
    fn get_net_update_time(&self) -> f32 {
        unsafe {
            Actor_Get_NetUpdateTime(self.addr())
        }
    }
    fn get_net_update_frequency(&self) -> f32 {
        unsafe {
            Actor_Get_NetUpdateFrequency(self.addr())
        }
    }
    fn get_net_priority(&self) -> f32 {
        unsafe {
            Actor_Get_NetPriority(self.addr())
        }
    }
    fn get_last_net_update_time(&self) -> f32 {
        unsafe {
            Actor_Get_LastNetUpdateTime(self.addr())
        }
    }
    fn get_last_force_packet_update_time(&self) -> f32 {
        unsafe {
            Actor_Get_LastForcePacketUpdateTime(self.addr())
        }
    }
    fn get_time_since_last_tick(&self) -> f32 {
        unsafe {
            Actor_Get_TimeSinceLastTick(self.addr())
        }
    }
    fn get_life_span(&self) -> f32 {
        unsafe {
            Actor_Get_LifeSpan(self.addr())
        }
    }
    fn get_creation_time(&self) -> f32 {
        unsafe {
            Actor_Get_CreationTime(self.addr())
        }
    }
    fn get_last_render_time(&self) -> f32 {
        unsafe {
            Actor_Get_LastRenderTime(self.addr())
        }
    }
    fn get_hidden_editor_views(&self) -> i64 {
        unsafe {
            Actor_Get_HiddenEditorViews(self.addr())
        }
    }
    fn get_attached(&self) -> RLArray<ActorWrapper> {
        unsafe {
            let mut result = RLArrayRaw::new();
            let result_ptr: *mut RLArrayRaw = &mut result as *mut RLArrayRaw;
            Actor_Get_Attached(self.addr(), result_ptr);
            RLArray::from_raw(result)
        }
    }
    fn get_relative_location(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Actor_Get_RelativeLocation(self.addr(), result_ptr);
            result
        }
    }
    fn get_relative_rotation(&self) -> Rotator {
        unsafe {
            let mut result = Rotator::new();
            let result_ptr: *mut Rotator = &mut result as *mut Rotator;
            Actor_Get_RelativeRotation(self.addr(), result_ptr);
            result
        }
    }
    fn get_collision_component(&self) -> Option<PrimitiveComponentWrapper> {
        unsafe {
            PrimitiveComponentWrapper::try_new(Actor_Get_CollisionComponent(self.addr()))
        }
    }
    fn force_net_update_packet(&self) {
        unsafe {
            Actor_ForceNetUpdatePacket(self.addr());
        }
    }
    fn force_net_update(&self) {
        unsafe {
            Actor_ForceNetUpdate(self.addr());
        }
    }
    fn will_overlap(&self, pos_a: Vector, vel_a: Vector, pos_b: Vector, vel_b: Vector, step_size: f32, radius: f32, time: f32) -> bool {
        unsafe {
            let mut pos_a = pos_a;
            let pos_a: *mut Vector = &mut pos_a as *mut Vector;
            let mut vel_a = vel_a;
            let vel_a: *mut Vector = &mut vel_a as *mut Vector;
            let mut pos_b = pos_b;
            let pos_b: *mut Vector = &mut pos_b as *mut Vector;
            let mut vel_b = vel_b;
            let vel_b: *mut Vector = &mut vel_b as *mut Vector;
            Actor_WillOverlap(self.addr(), pos_a, vel_a, pos_b, vel_b, step_size, radius, time)
        }
    }
    fn is_in_persistent_level(&self, b_include_level_streaming_persistent: bool) -> bool {
        unsafe {
            Actor_IsInPersistentLevel(self.addr(), b_include_level_streaming_persistent)
        }
    }
    fn set_hud_location(&self, new_hud_location: Vector) {
        unsafe {
            let mut new_hud_location = new_hud_location;
            let new_hud_location: *mut Vector = &mut new_hud_location as *mut Vector;
            Actor_SetHUDLocation(self.addr(), new_hud_location);
        }
    }
    fn get_target_location(&self, requested_by: ActorWrapper, b_request_alternate_loc: bool) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Actor_GetTargetLocation(self.addr(), requested_by.addr(), b_request_alternate_loc, result_ptr);
            result
        }
    }
    fn get_team_num(&self) -> u8 {
        unsafe {
            Actor_GetTeamNum(self.addr())
        }
    }
    fn is_player_owned(&self) -> bool {
        unsafe {
            Actor_IsPlayerOwned(self.addr())
        }
    }
    fn is_stationary(&self) -> bool {
        unsafe {
            Actor_IsStationary(self.addr())
        }
    }
    fn get_gravity_acceleration(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Actor_GetGravityAcceleration(self.addr(), result_ptr);
            result
        }
    }
    fn get_gravity_direction(&self) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Actor_GetGravityDirection(self.addr(), result_ptr);
            result
        }
    }
    fn get_gravity_z(&self) -> f32 {
        unsafe {
            Actor_GetGravityZ(self.addr())
        }
    }
    fn is_overlapping(&self, a: ActorWrapper) -> bool {
        unsafe {
            Actor_IsOverlapping(self.addr(), a.addr())
        }
    }
    fn contains_point(&self, spot: Vector) -> bool {
        unsafe {
            let mut spot = spot;
            let spot: *mut Vector = &mut spot as *mut Vector;
            Actor_ContainsPoint(self.addr(), spot)
        }
    }
    fn set_tick_is_disabled(&self, b_in_disabled: bool) {
        unsafe {
            Actor_SetTickIsDisabled(self.addr(), b_in_disabled);
        }
    }
    fn set_physics2(&self, new_physics: u8) {
        unsafe {
            Actor_SetPhysics2(self.addr(), new_physics);
        }
    }
    fn set_hidden(&self, b_new_hidden: bool) {
        unsafe {
            Actor_SetHidden(self.addr(), b_new_hidden);
        }
    }
    fn chart_data(&self, data_name: RLString, data_value: f32) {
        unsafe {
            let mut data_name = data_name;
            let data_name: *mut RLString = &mut data_name as *mut RLString;
            Actor_ChartData(self.addr(), data_name, data_value);
        }
    }
    fn draw_debug_string(&self, text_location: Vector, text: RLString, test_base_actor: ActorWrapper, text_color: Color, duration: f32) {
        unsafe {
            let mut text_location = text_location;
            let text_location: *mut Vector = &mut text_location as *mut Vector;
            let mut text = text;
            let text: *mut RLString = &mut text as *mut RLString;
            let mut text_color = text_color;
            let text_color: *mut Color = &mut text_color as *mut Color;
            Actor_DrawDebugString(self.addr(), text_location, text, test_base_actor.addr(), text_color, duration);
        }
    }
    fn draw_debug_cone(&self, origin: Vector, direction: Vector, length: f32, angle_width: f32, angle_height: f32, num_sides: i32, draw_color: Color, b_persistent_lines: bool) {
        unsafe {
            let mut origin = origin;
            let origin: *mut Vector = &mut origin as *mut Vector;
            let mut direction = direction;
            let direction: *mut Vector = &mut direction as *mut Vector;
            let mut draw_color = draw_color;
            let draw_color: *mut Color = &mut draw_color as *mut Color;
            Actor_DrawDebugCone(self.addr(), origin, direction, length, angle_width, angle_height, num_sides, draw_color, b_persistent_lines);
        }
    }
    fn get_aggregate_base_velocity(&self, test_base: ActorWrapper) -> Vector {
        unsafe {
            let mut result = Vector::new();
            let result_ptr: *mut Vector = &mut result as *mut Vector;
            Actor_GetAggregateBaseVelocity(self.addr(), test_base.addr(), result_ptr);
            result
        }
    }
    fn is_owned_by(&self, test_actor: ActorWrapper) -> bool {
        unsafe {
            Actor_IsOwnedBy(self.addr(), test_actor.addr())
        }
    }
    fn is_based_on(&self, test_actor: ActorWrapper) -> bool {
        unsafe {
            Actor_IsBasedOn(self.addr(), test_actor.addr())
        }
    }
    fn get_terminal_velocity(&self) -> f32 {
        unsafe {
            Actor_GetTerminalVelocity(self.addr())
        }
    }

}

extern "C" {
    fn Actor_GetLocation(p_actor: usize, result: *mut Vector);
    fn Actor_SetLocation(p_actor: usize, new_loc: *mut Vector);
    fn Actor_GetVelocity(p_actor: usize, result: *mut Vector);
    fn Actor_SetVelocity(p_actor: usize, velocity: *mut Vector);
    fn Actor_AddVelocity(p_actor: usize, velocity: *mut Vector);
    fn Actor_GetRotation(p_actor: usize, result: *mut Rotator);
    fn Actor_SetRotation(p_actor: usize, rotation: *mut Rotator);
    fn Actor_SetTorque(p_actor: usize, torq: *mut Vector);
    fn Actor_Stop(p_actor: usize);
    fn Actor_GetAngularVelocity(p_actor: usize, result: *mut Vector);
    fn Actor_SetAngularVelocity(p_actor: usize, v: *mut Vector, add_to_current: bool);
    fn Actor_IsNull(p_actor: usize);

    fn Actor_Get_DrawScale(obj: usize) -> f32;
    fn ActorWrapper_SetDrawScale(obj: usize, new_val: f32);
    fn Actor_Get_DrawScale3D(obj: usize, result: *mut Vector);
    fn ActorWrapper_SetDrawScale3D(obj: usize, new_val: *mut Vector);
    fn Actor_Get_PrePivot(obj: usize, result: *mut Vector);
    fn ActorWrapper_SetPrePivot(obj: usize, new_val: *mut Vector);
    fn Actor_Get_EditorIconColor(obj: usize, result: *mut Color);
    fn ActorWrapper_SetEditorIconColor(obj: usize, new_val: *mut Color);
    fn Actor_Get_CustomTimeDilation(obj: usize) -> f32;
    fn ActorWrapper_SetCustomTimeDilation(obj: usize, new_val: f32);
    fn Actor_Get_Physics(obj: usize) -> u8;
    fn ActorWrapper_SetPhysics(obj: usize, new_val: u8);
    fn Actor_Get_RemoteRole(obj: usize) -> u8;
    fn ActorWrapper_SetRemoteRole(obj: usize, new_val: u8);
    fn Actor_Get_Role(obj: usize) -> u8;
    fn ActorWrapper_SetRole(obj: usize, new_val: u8);
    fn Actor_Get_CollisionType(obj: usize) -> u8;
    fn ActorWrapper_SetCollisionType(obj: usize, new_val: u8);
    fn Actor_Get_ReplicatedCollisionType(obj: usize) -> u8;
    fn ActorWrapper_SetReplicatedCollisionType(obj: usize, new_val: u8);
    fn Actor_Get_Owner(obj: usize) -> usize;
    fn Actor_Get_Base(obj: usize) -> usize;
    fn Actor_Get_bStatic(obj: usize) -> bool;
    fn Actor_Get_bHidden(obj: usize) -> bool;
    fn Actor_Get_bHiddenSelf(obj: usize) -> bool;
    fn ActorWrapper_SetbHiddenSelf(obj: usize, new_val: bool);
    fn Actor_Get_bNoDelete(obj: usize) -> bool;
    fn ActorWrapper_SetbNoDelete(obj: usize, new_val: bool);
    fn Actor_Get_bDeleteMe(obj: usize) -> bool;
    fn ActorWrapper_SetbDeleteMe(obj: usize, new_val: bool);
    fn Actor_Get_bTicked(obj: usize) -> bool;
    fn ActorWrapper_SetbTicked(obj: usize, new_val: bool);
    fn Actor_Get_bOnlyOwnerSee(obj: usize) -> bool;
    fn ActorWrapper_SetbOnlyOwnerSee(obj: usize, new_val: bool);
    fn Actor_Get_bTickIsDisabled(obj: usize) -> bool;
    fn ActorWrapper_SetbTickIsDisabled(obj: usize, new_val: bool);
    fn Actor_Get_bWorldGeometry(obj: usize) -> bool;
    fn ActorWrapper_SetbWorldGeometry(obj: usize, new_val: bool);
    fn Actor_Get_bIgnoreRigidBodyPawns(obj: usize) -> bool;
    fn ActorWrapper_SetbIgnoreRigidBodyPawns(obj: usize, new_val: bool);
    fn Actor_Get_bOrientOnSlope(obj: usize) -> bool;
    fn ActorWrapper_SetbOrientOnSlope(obj: usize, new_val: bool);
    fn Actor_Get_bIsMoving(obj: usize) -> bool;
    fn Actor_Get_bAlwaysEncroachCheck(obj: usize) -> bool;
    fn ActorWrapper_SetbAlwaysEncroachCheck(obj: usize, new_val: bool);
    fn Actor_Get_bHasAlternateTargetLocation(obj: usize) -> bool;
    fn Actor_Get_bAlwaysRelevant(obj: usize) -> bool;
    fn Actor_Get_bReplicateInstigator(obj: usize) -> bool;
    fn Actor_Get_bReplicateMovement(obj: usize) -> bool;
    fn Actor_Get_bUpdateSimulatedPosition(obj: usize) -> bool;
    fn ActorWrapper_SetbUpdateSimulatedPosition(obj: usize, new_val: bool);
    fn Actor_Get_bDemoRecording(obj: usize) -> bool;
    fn ActorWrapper_SetbDemoRecording(obj: usize, new_val: bool);
    fn Actor_Get_bDemoOwner(obj: usize) -> bool;
    fn ActorWrapper_SetbDemoOwner(obj: usize, new_val: bool);
    fn Actor_Get_bForceDemoRelevant(obj: usize) -> bool;
    fn ActorWrapper_SetbForceDemoRelevant(obj: usize, new_val: bool);
    fn Actor_Get_bNetInitialRotation(obj: usize) -> bool;
    fn ActorWrapper_SetbNetInitialRotation(obj: usize, new_val: bool);
    fn Actor_Get_bReplicateRigidBodyLocation(obj: usize) -> bool;
    fn ActorWrapper_SetbReplicateRigidBodyLocation(obj: usize, new_val: bool);
    fn Actor_Get_bKillDuringLevelTransition(obj: usize) -> bool;
    fn ActorWrapper_SetbKillDuringLevelTransition(obj: usize, new_val: bool);
    fn Actor_Get_bPostRenderIfNotVisible(obj: usize) -> bool;
    fn ActorWrapper_SetbPostRenderIfNotVisible(obj: usize, new_val: bool);
    fn Actor_Get_bForceNetUpdate(obj: usize) -> bool;
    fn ActorWrapper_SetbForceNetUpdate(obj: usize, new_val: bool);
    fn Actor_Get_bForcePacketUpdate(obj: usize) -> bool;
    fn ActorWrapper_SetbForcePacketUpdate(obj: usize, new_val: bool);
    fn Actor_Get_bPendingNetUpdate(obj: usize) -> bool;
    fn ActorWrapper_SetbPendingNetUpdate(obj: usize, new_val: bool);
    fn Actor_Get_bGameRelevant(obj: usize) -> bool;
    fn ActorWrapper_SetbGameRelevant(obj: usize, new_val: bool);
    fn Actor_Get_bMovable(obj: usize) -> bool;
    fn ActorWrapper_SetbMovable(obj: usize, new_val: bool);
    fn Actor_Get_bCanTeleport(obj: usize) -> bool;
    fn ActorWrapper_SetbCanTeleport(obj: usize, new_val: bool);
    fn Actor_Get_bAlwaysTick(obj: usize) -> bool;
    fn ActorWrapper_SetbAlwaysTick(obj: usize, new_val: bool);
    fn Actor_Get_bBlocksNavigation(obj: usize) -> bool;
    fn ActorWrapper_SetbBlocksNavigation(obj: usize, new_val: bool);
    fn Actor_Get_BlockRigidBody(obj: usize) -> bool;
    fn ActorWrapper_SetBlockRigidBody(obj: usize, new_val: bool);
    fn Actor_Get_bCollideWhenPlacing(obj: usize) -> bool;
    fn ActorWrapper_SetbCollideWhenPlacing(obj: usize, new_val: bool);
    fn Actor_Get_bCollideActors(obj: usize) -> bool;
    fn ActorWrapper_SetbCollideActors(obj: usize, new_val: bool);
    fn Actor_Get_bCollideWorld(obj: usize) -> bool;
    fn ActorWrapper_SetbCollideWorld(obj: usize, new_val: bool);
    fn Actor_Get_bCollideComplex(obj: usize) -> bool;
    fn ActorWrapper_SetbCollideComplex(obj: usize, new_val: bool);
    fn Actor_Get_bBlockActors(obj: usize) -> bool;
    fn ActorWrapper_SetbBlockActors(obj: usize, new_val: bool);
    fn Actor_Get_bBlocksTeleport(obj: usize) -> bool;
    fn ActorWrapper_SetbBlocksTeleport(obj: usize, new_val: bool);
    fn Actor_Get_bPhysRigidBodyOutOfWorldCheck(obj: usize) -> bool;
    fn ActorWrapper_SetbPhysRigidBodyOutOfWorldCheck(obj: usize, new_val: bool);
    fn Actor_Get_bComponentOutsideWorld(obj: usize) -> bool;
    fn Actor_Get_bRigidBodyWasAwake(obj: usize) -> bool;
    fn ActorWrapper_SetbRigidBodyWasAwake(obj: usize, new_val: bool);
    fn Actor_Get_bCallRigidBodyWakeEvents(obj: usize) -> bool;
    fn ActorWrapper_SetbCallRigidBodyWakeEvents(obj: usize, new_val: bool);
    fn Actor_Get_bBounce(obj: usize) -> bool;
    fn ActorWrapper_SetbBounce(obj: usize, new_val: bool);
    fn Actor_Get_bEditable(obj: usize) -> bool;
    fn ActorWrapper_SetbEditable(obj: usize, new_val: bool);
    fn Actor_Get_bLockLocation(obj: usize) -> bool;
    fn ActorWrapper_SetbLockLocation(obj: usize, new_val: bool);
    fn Actor_Get_NetUpdateTime(obj: usize) -> f32;
    fn ActorWrapper_SetNetUpdateTime(obj: usize, new_val: f32);
    fn Actor_Get_NetUpdateFrequency(obj: usize) -> f32;
    fn ActorWrapper_SetNetUpdateFrequency(obj: usize, new_val: f32);
    fn Actor_Get_NetPriority(obj: usize) -> f32;
    fn ActorWrapper_SetNetPriority(obj: usize, new_val: f32);
    fn Actor_Get_LastNetUpdateTime(obj: usize) -> f32;
    fn Actor_Get_LastForcePacketUpdateTime(obj: usize) -> f32;
    fn ActorWrapper_SetLastForcePacketUpdateTime(obj: usize, new_val: f32);
    fn Actor_Get_TimeSinceLastTick(obj: usize) -> f32;
    fn Actor_Get_LifeSpan(obj: usize) -> f32;
    fn Actor_Get_CreationTime(obj: usize) -> f32;
    fn Actor_Get_LastRenderTime(obj: usize) -> f32;
    fn Actor_Get_HiddenEditorViews(obj: usize) -> i64;
    fn ActorWrapper_SetHiddenEditorViews(obj: usize, new_val: i64);
    fn Actor_Get_Attached(obj: usize, result: *mut RLArrayRaw);
    fn Actor_Get_RelativeLocation(obj: usize, result: *mut Vector);
    fn ActorWrapper_SetRelativeLocation(obj: usize, new_val: *mut Vector);
    fn Actor_Get_RelativeRotation(obj: usize, result: *mut Rotator);
    fn ActorWrapper_SetRelativeRotation(obj: usize, new_val: *mut Rotator);
    fn Actor_Get_CollisionComponent(obj: usize) -> usize;
    fn Actor_ForceNetUpdatePacket(obj: usize);
    fn Actor_ForceNetUpdate(obj: usize);
    fn Actor_WillOverlap(obj: usize, PosA: *mut Vector, VelA: *mut Vector, PosB: *mut Vector, VelB: *mut Vector, StepSize: f32, Radius: f32, Time: f32) -> bool;
    fn Actor_IsInPersistentLevel(obj: usize, bIncludeLevelStreamingPersistent: bool) -> bool;
    fn Actor_SetHUDLocation(obj: usize, NewHUDLocation: *mut Vector);
    fn Actor_GetTargetLocation(obj: usize, RequestedBy: usize, bRequestAlternateLoc: bool, result: *mut Vector);
    fn Actor_GetTeamNum(obj: usize) -> u8;
    fn Actor_IsPlayerOwned(obj: usize) -> bool;
    fn Actor_IsStationary(obj: usize) -> bool;
    fn Actor_GetGravityAcceleration(obj: usize, result: *mut Vector);
    fn Actor_GetGravityDirection(obj: usize, result: *mut Vector);
    fn Actor_GetGravityZ(obj: usize) -> f32;
    fn Actor_IsOverlapping(obj: usize, A: usize) -> bool;
    fn Actor_ContainsPoint(obj: usize, Spot: *mut Vector) -> bool;
    fn Actor_SetTickIsDisabled(obj: usize, bInDisabled: bool);
    fn Actor_SetPhysics2(obj: usize, newPhysics: u8);
    fn Actor_SetHidden(obj: usize, bNewHidden: bool);
    fn Actor_ChartData(obj: usize, DataName: *mut RLString, DataValue: f32);
    fn Actor_DrawDebugString(obj: usize, TextLocation: *mut Vector, Text: *mut RLString, TestBaseActor: usize, TextColor: *mut Color, Duration: f32);
    fn Actor_DrawDebugCone(obj: usize, Origin: *mut Vector, Direction: *mut Vector, Length: f32, AngleWidth: f32, AngleHeight: f32, NumSides: i32, DrawColor: *mut Color, bPersistentLines: bool);
    fn Actor_GetAggregateBaseVelocity(obj: usize, TestBase: usize, result: *mut Vector);
    fn Actor_IsOwnedBy(obj: usize, TestActor: usize) -> bool;
    fn Actor_IsBasedOn(obj: usize, TestActor: usize) -> bool;
    fn Actor_GetTerminalVelocity(obj: usize) -> f32;

}