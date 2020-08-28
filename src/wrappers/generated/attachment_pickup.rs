use crate::wrappers::*;
use super::*;

pub struct AttachmentPickupWrapper(pub usize);
impl_object!(AttachmentPickupWrapper);

impl AttachmentPickup for AttachmentPickupWrapper {}
impl RumblePickupComponent for AttachmentPickupWrapper {}
impl CarComponent for AttachmentPickupWrapper {}
impl Actor for AttachmentPickupWrapper {}

pub trait AttachmentPickup : RumblePickupComponent {
    fn pickup_end(&self) {
        unsafe {
            SpecialPickup_Attachment_TA_PickupEnd(self.addr());
        }
    }
    fn pickup_start(&self) {
        unsafe {
            SpecialPickup_Attachment_TA_PickupStart(self.addr());
        }
    }

}

extern "C" {
    fn SpecialPickup_Attachment_TA_PickupEnd(obj: usize);
    fn SpecialPickup_Attachment_TA_PickupStart(obj: usize);

}