// test only, unstable
const SP: [[u8; 2048]; 8] = [[0; 2048]; 8];

use sbi::sbi_hart_get_status;
use crate::context;
use crate::sbi;
use crate::sbi::HSMHartStates;

pub fn spawn<'a, F, T>(f: F)
where
    F: FnOnce() -> T,
    F: 'static + Send,
    T: 'static + Send
{
    use alloc::boxed::Box;
    use context::Context;

    extern "C" { fn _restore_hart(); }    

    let main = move || {
        __rust_begin_short_backtrace(f);
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

    (0..8).find(|&hart| {
        HSMHartStates::STOPPED == sbi_hart_get_status(
            hart
        ).into()
    }).map_or(0, |hart| {
        let cx = Context::init_context(
            riscv::register::sstatus::SPP::Supervisor, 
            thread_start as usize,
            crate::mm::kernel_satp(),
            SP[hart].as_ptr() as usize + SP[hart].len(),
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
            hart, 
            _restore_hart as usize, 
            cx_ptr
        );
        hart
    });
}

pub fn thread_start(hart_id: usize, pointer: usize, vtable: usize, cx_ptr: usize) {
    use alloc::boxed::Box;
    use context::Context;

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
