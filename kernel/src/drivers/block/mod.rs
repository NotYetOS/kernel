pub mod virtio;

use alloc::sync::Arc;
use fefs::device::BlockDevice;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = Arc::new(virtio::VirtIOBlock::new());
}
