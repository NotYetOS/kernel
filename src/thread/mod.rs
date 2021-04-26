// test only, unstable
#![allow(unused)]

const SP: [[u8; 2048]; 8] = [[0; 2048]; 8];

#[cfg(feature = "1t")]
pub const NUM_HART: usize = 1;

// #[cfg(not(feature = "1t"))]， make inactive for ide
#[cfg(not(feature = "1t"))]
#[cfg(feature = "2t")]
pub const NUM_HART: usize = 2;

#[cfg(not(feature = "1t"))]
#[cfg(feature = "3t")]
pub const NUM_HART: usize = 3;

#[cfg(not(feature = "1t"))]
#[cfg(feature = "4t")]
pub const NUM_HART: usize = 4;

#[cfg(not(feature = "1t"))]
#[cfg(feature = "5t")]
pub const NUM_HART: usize = 5;

#[cfg(not(feature = "1t"))]
#[cfg(feature = "6t")]
pub const NUM_HART: usize = 6;

#[cfg(not(feature = "1t"))]
#[cfg(feature = "7t")]
pub const NUM_HART: usize = 7;

#[cfg(not(feature = "1t"))]
#[cfg(feature = "8t")]
pub const NUM_HART: usize = 8;

mod pool;

use pool::{
    THREAD_POOL,
    Thread
};
use alloc::sync::Arc;
use alloc::boxed::Box;
use core::any::Any;
use core::cell::UnsafeCell;
use context::Context;
use riscv::register::sstatus::SPP;
use crate::context;
use crate::sbi;
use crate::sbi::HSMHartStates;

global_asm!(include_str!("thread.s"));

type Result<T> = core::result::Result<T, Box<dyn Any + Send + 'static>>;

pub fn spawn<'a, F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: 'static + Send,
    T: 'static + Send
{
    extern "C" { fn _load_hart(); }
    
    let my_packet: Arc<UnsafeCell<Option<Result<T>>>> = Arc::new(UnsafeCell::new(None));
    let their_packet = my_packet.clone();

    let main = move || {
        let t = __rust_begin_short_backtrace(f);
        unsafe { *their_packet.get() = Some(Ok(t)) };
    };
    
    let main_box = unsafe {
        core::mem::transmute::<
            Box<dyn FnOnce() + 'a>,
            Box<dyn FnOnce() + 'static>
        >(
            Box::new(main),
        )
    };

    let raw = Box::into_raw(main_box);

    let (pointer, vtable) = unsafe {
        core::mem::transmute::<*mut dyn FnOnce(), (usize, usize)>(raw)
    };

    let thread = (0..NUM_HART).find(|&hart_id| {
        HSMHartStates::STOPPED == sbi::sbi_hart_get_status(hart_id).into()
    }).map_or(None, |hart_id| {
        let cx = Context::init_context(
            SPP::Supervisor, 
            thread_start as usize,
            crate::mm::kernel_satp(),
            SP[hart_id].as_ptr() as usize + SP[hart_id].len(),
            crate::mm::kernel_satp(),
            0,
            crate::trap::trap_handler as usize,
        );
    
        let leak = Box::leak(Box::new(cx));
        let cx_ptr = leak as *const _ as usize;

        leak.a1 = pointer;
        leak.a2 = vtable;
        leak.a3 = cx_ptr;
        
        sbi::sbi_hart_start(
            hart_id, 
            _load_hart as usize, 
            cx_ptr
        );
        
        Some(Thread {
            hart_id,
            name: "test".into(),
            state: HSMHartStates::STOPPED,
            context: leak,
        })
    }).unwrap();

    JoinHandle(
        JoinInner {
            thread,
            packet: Packet(my_packet),
        }
    )
}

pub fn thread_start(hart_id: usize, pointer: usize, vtable: usize, cx_ptr: usize) {
    // drop leak context
    unsafe {
        use alloc::alloc::dealloc;
        use alloc::alloc::Layout;
        dealloc(cx_ptr as *mut u8, Layout::new::<Context>());
    }
    
    let main = unsafe {
        let raw = core::mem::transmute::<
            (usize, usize), 
            *mut dyn FnOnce()
        >((pointer, vtable));
        Box::from_raw(raw)
    };

    main();
    sbi::sbi_hart_stop();
}

fn __rust_begin_short_backtrace<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let result = f();

    // prevent this frame from being tail-call optimised away
    core::hint::black_box(());

    result
}

struct Packet<T>(Arc<UnsafeCell<Option<Result<T>>>>);

unsafe impl<T: Send> Send for Packet<T> {}
unsafe impl<T: Sync> Sync for Packet<T> {}

pub struct JoinInner<T> {
    thread: Thread,
    packet: Packet<T>,
}

impl<T> JoinInner<T> {
    fn join(&mut self) -> Result<T> {
        let hart_id = self.thread.hart_id;
        while HSMHartStates::STOPPED != sbi::sbi_hart_get_status(hart_id).into() {}
        unsafe { (*self.packet.0.get()).take().unwrap() }
    }
}

pub struct JoinHandle<T>(JoinInner<T>);

unsafe impl<T> Send for JoinHandle<T> {}
unsafe impl<T> Sync for JoinHandle<T> {}

impl<T> JoinHandle<T> {
    pub fn thread(&self) -> &Thread {
        &self.0.thread
    }

    pub fn join(mut self) -> Result<T> {
        self.0.join()
    }
}
