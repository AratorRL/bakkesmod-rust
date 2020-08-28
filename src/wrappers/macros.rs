macro_rules! impl_object {
    ($name: ident) => {
        impl Object for $name {
            fn new(addr: usize) -> Self { 
                Self(addr)
            }

            fn try_new(addr: usize) -> Option<Self> { 
                match addr {
                    0 => None,
                    _ => Some(Self(addr))
                }
            }

            fn addr(&self) -> usize { 
                self.0
            }
        }

        impl UnrealPointer for $name {
            fn from_ptr(addr: usize) -> Self { Self(addr) }
        }
    }
}

macro_rules! impl_unreal_pointer_struct {
    ($name: ident) => {
        impl UnrealPointer for $name {
            fn from_ptr(addr: usize) -> Self {
                unsafe { *(addr as *const Self) }
            }
        }
    }
}

macro_rules! struct_default_new {
    ($name: ident) => {
        impl $name {
            pub fn new() -> Self { Self(0) }
        }
    }
}