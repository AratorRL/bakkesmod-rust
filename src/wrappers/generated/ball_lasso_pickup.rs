use crate::wrappers::*;
use super::*;

pub struct BallLassoPickupWrapper(pub usize);
impl_object!(BallLassoPickupWrapper);

impl BallLassoPickup for BallLassoPickupWrapper {}
impl SpringPickup for BallLassoPickupWrapper {}
impl TargetedPickup for BallLassoPickupWrapper {}
impl RumblePickupComponent for BallLassoPickupWrapper {}
impl CarComponent for BallLassoPickupWrapper {}
impl Actor for BallLassoPickupWrapper {}

pub trait BallLassoPickup : SpringPickup {
    fn scale_spring_mesh_to_location(&self, new_location: Vector, target_location: Vector) {
        unsafe {
            let mut new_location = new_location;
            let new_location: *mut Vector = &mut new_location as *mut Vector;
            let mut target_location = target_location;
            let target_location: *mut Vector = &mut target_location as *mut Vector;
            SpecialPickup_BallLasso_TA_ScaleSpringMeshToLocation(self.addr(), new_location, target_location);
        }
    }
    fn do_spring(&self, b_first_hit: bool) {
        unsafe {
            SpecialPickup_BallLasso_TA_DoSpring(self.addr(), b_first_hit);
        }
    }

}

extern "C" {
    fn SpecialPickup_BallLasso_TA_ScaleSpringMeshToLocation(obj: usize, NewLocation: *mut Vector, TargetLocation: *mut Vector);
    fn SpecialPickup_BallLasso_TA_DoSpring(obj: usize, bFirstHit: bool);

}