use crate::wrappers::{*, structs::*, unreal::*};
use super::*;

pub struct PhysicalMaterialPropertyWrapper(pub usize);
impl_object!(PhysicalMaterialPropertyWrapper);

impl PhysicalMaterialProperty for PhysicalMaterialPropertyWrapper {}

pub trait PhysicalMaterialProperty : Object {
    fn get_b_sticky_wheels(&self) -> bool {
        unsafe {
            PhysicalMaterialProperty_TA_Get_bStickyWheels(self.addr())
        }
    }
    fn get_b_consider_for_ground(&self) -> bool {
        unsafe {
            PhysicalMaterialProperty_TA_Get_bConsiderForGround(self.addr())
        }
    }

}

extern "C" {
    fn PhysicalMaterialProperty_TA_Get_bStickyWheels(obj: usize) -> bool;
    fn PhysicalMaterialPropertyWrapper_SetbStickyWheels(obj: usize, new_val: bool);
    fn PhysicalMaterialProperty_TA_Get_bConsiderForGround(obj: usize) -> bool;
    fn PhysicalMaterialPropertyWrapper_SetbConsiderForGround(obj: usize, new_val: bool);

}