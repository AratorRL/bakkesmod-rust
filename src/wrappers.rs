#![allow(unused)]
pub struct CarWrapper(pub usize);

pub trait Car {
    fn demolish(&self);
}

impl Car for CarWrapper {
    fn demolish(&self) {
        unsafe { Car_Demolish(self.0); }
    }
}


extern "C" {
    fn Car_Demolish(p_car: usize);
}