use crate::wrappers::*;
use crate::generated::*;

pub struct BallCarSpringPickupWrapper(pub usize);
impl_object!(BallCarSpringPickupWrapper);

impl BallCarSpringPickup for BallCarSpringPickupWrapper {}
impl SpringPickup for BallCarSpringPickupWrapper {}
impl TargetedPickup for BallCarSpringPickupWrapper {}
impl RumblePickupComponent for BallCarSpringPickupWrapper {}
impl CarComponent for BallCarSpringPickupWrapper {}
impl Actor for BallCarSpringPickupWrapper {}

pub trait BallCarSpringPickup : SpringPickup {
    fn scale_spring_mesh_to_location(&self, new_location: Vector, target_location: Vector) {
        unsafe {
            let mut new_location = new_location;
            let new_location: *mut Vector = &mut new_location as *mut Vector;
            let mut target_location = target_location;
            let target_location: *mut Vector = &mut target_location as *mut Vector;
            SpecialPickup_BallCarSpring_TA_ScaleSpringMeshToLocation(self.addr(), new_location, target_location);
        }
    }

}

extern "C" {
    fn SpecialPickup_BallCarSpring_TA_ScaleSpringMeshToLocation(obj: usize, NewLocation: *mut Vector, TargetLocation: *mut Vector);

}