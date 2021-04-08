use alloc::sync::{
    Arc,
    Weak,
};
use spin::Mutex;
use crate::process;
use super::{
    File, 
    UserBuffer
};

const RING_BUFFER_SIZE: usize = 32;

#[derive(Copy, Clone)]
enum RingBufferStatus {
    Full,
    Empty,
    Normal,
}

pub struct PipeRingBuffer {
    array: [u8; RING_BUFFER_SIZE],
    head: usize,
    tail: usize,
    status: RingBufferStatus,
    write_end_status: Option<Weak<Pipe>>,
}

impl PipeRingBuffer {
    pub fn new() -> Self {
        Self {
            array: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: RingBufferStatus::Empty,
            write_end_status: None,
        }
    }

    pub fn set_write_end_status(
        &mut self, 
        write_end_status: &Arc<Pipe>
    ) {
        self.write_end_status = Some(
            Arc::downgrade(write_end_status)
        );
    }

    pub fn read_byte(&mut self) -> u8 {
        let ch = self.array[self.head];

        self.status = RingBufferStatus::Normal;
        self.head = (self.head + 1) % RING_BUFFER_SIZE;

        if self.head == self.tail {
            self.status = RingBufferStatus::Empty;
        }

        ch
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.array[self.tail] = byte;
        self.status = RingBufferStatus::Normal;
        self.tail = (self.tail + 1) % RING_BUFFER_SIZE;
        if self.tail == self.head {
            self.status = RingBufferStatus::Full;
        }
    }

    pub fn available_read(&self) -> usize {
        match self.status {
            RingBufferStatus::Empty => 0,
            _ => {
                if self.tail > self.head {
                    self.tail - self.head
                } else {
                    self.tail + RING_BUFFER_SIZE - self.head
                }
            }
        }
    }

    pub fn available_write(&self) -> usize {
        match self.status {
            RingBufferStatus::Full => 0,
            _ => RING_BUFFER_SIZE - self.available_read()
        }
    }

    pub fn check_write_end_closed(&self) -> bool {
        let weak = self.write_end_status.as_ref().unwrap();
        weak.upgrade().is_none()
    }
}

pub struct Pipe {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
}

impl Pipe {
    pub fn read_end(
        buffer: Arc<Mutex<PipeRingBuffer>>
    ) -> Self {
        Self {
            readable: true,
            writable: false,
            buffer,
        }
    }

    pub fn write_end(
        buffer: Arc<Mutex<PipeRingBuffer>>
    ) -> Self {
        Self {
            readable: false,
            writable: true,
            buffer,
        }
    }
}

impl File for Pipe {
    fn read(&self, buf: UserBuffer) -> usize {
        assert_eq!(self.readable, true);
        let mut buf_iter = buf.into_iter();
        let mut read_size = 0;
        

        let mut read_func = || {
            let mut ring_buffer = self.buffer.lock();
            let num_read = ring_buffer.available_read();

            (0..num_read).for_each(|_| {
                match buf_iter.next() {
                    Some(byte_ref) => {
                        unsafe { *byte_ref = ring_buffer.read_byte(); }
                        read_size += 1;
                    }
                    None => return
                }
            });

            ring_buffer
        };

        loop {
            let ring_buffer= read_func();
            if ring_buffer.check_write_end_closed() { 
                break read_size; 
            } else {
                drop(ring_buffer);
                process::save_call_context();
                process::ret();
            }
            continue;
        }
    }

    fn write(&self, buf: UserBuffer) -> usize {
        assert_eq!(self.writable, true);
        let mut buf_iter = buf.into_iter();
        let mut write_size = 0;
        let mut ring_buffer = self.buffer.lock();
        let num_write = ring_buffer.available_write();

        (0..num_write).for_each(|_| {
            match buf_iter.next() {
                Some(byte_ref) => {
                    ring_buffer.write_byte(
                        unsafe { *byte_ref }
                    );
                    write_size += 1;
                }
                None => return 
            }
        });

        write_size
    }
}

pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    let buffer = Arc::new(
        Mutex::new(
            PipeRingBuffer::new()
        )
    );

    let read_end = Arc::new(
        Pipe::read_end(buffer.clone())
    );
    let write_end = Arc::new(
        Pipe::write_end(buffer.clone())
    );

    buffer.lock().set_write_end_status(&write_end);
    (read_end, write_end)
}
