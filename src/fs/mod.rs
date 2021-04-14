use fefs::dir::DirEntry;
use fefs::system::FileSystem;
use lazy_static::lazy_static;
use alloc::sync::Arc;
use spin::Mutex;
use alloc::vec::Vec;
use crate::drivers::BLOCK_DEVICE;

mod stdio;
mod pipe;
mod inode;

lazy_static! {
    pub static ref ROOT: Arc<Mutex<DirEntry>> = {
        let fs = FileSystem::open(BLOCK_DEVICE.clone());
        let fs = fs.lock();
        let root = fs.root();
        Arc::new(Mutex::new(root)) 
    };
}

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
}

pub struct UserBuffer {
    inner: Vec<&'static mut [u8]>,
}

impl UserBuffer {
    pub fn new(
        buffers: Vec<&'static mut [u8]>
    ) -> Self {
        Self { inner: buffers }
    }

    pub fn len(&self) -> usize {
        let mut total: usize = 0;
        for b in self.inner.iter() {
            total += b.len();
        }
        total
    }
}

pub struct UserBufferIterator {
    buffers: Vec<&'static mut [u8]>,
    current_buffer: usize,
    buffer_idx: usize,
}

impl Iterator for UserBufferIterator {
    type Item = *mut u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_buffer >= self.buffers.len() {
            return None;
        }
        let ret = &mut self.buffers
        [self.current_buffer]
        [self.buffer_idx] as *mut u8;

        if self.buffer_idx + 1 == self.buffers
        [self.current_buffer].len() {
            self.current_buffer += 1;
            self.buffer_idx = 0;
        } else {
            self.buffer_idx += 1;
        }

        Some(ret)
    }
}

impl IntoIterator for UserBuffer {
    type Item = *mut u8;
    type IntoIter = UserBufferIterator;

    fn into_iter(self) -> Self::IntoIter {
        UserBufferIterator {
            buffers: self.inner,
            current_buffer: 0,
            buffer_idx: 0,
        }
    }
}

pub fn test() {
    use fefs::file::WriteType;
    use fefs::dir::DirError;
    use fefs::file::FileError;
    
    println!("");
    println!("[test] fefs");
    println!("----------------------->");

    let mut root = ROOT.lock();

    root.mkdir("fefs").unwrap();
    assert_eq!(root.mkdir("fefs").err().unwrap(), DirError::DirExist);
    let mut dir = root.cd("fefs").unwrap();
    let mut file = dir.create_file("tlnb").unwrap();
    assert!(dir.exist("tlnb"));

    let mut buf = [0; 10];
    let mut vec_buf = Vec::new();

    let str_len = "hello fefs abc".len();
    file.write("hello fefs abc".as_bytes(), WriteType::OverWritten).unwrap();
    let len = file.read(&mut buf).unwrap();
    let ret = core::str::from_utf8(&buf[0..len]).unwrap();
    assert_eq!(ret, "hello fefs");
    println!("{}", ret);

    file.seek(6).unwrap();
    let len = file.read_to_vec(&mut vec_buf).unwrap();
    let ret = core::str::from_utf8(&vec_buf[0..len]).unwrap();
    assert_eq!(ret, "fefs abc");

    file.seek(str_len).unwrap();
    let len = file.read_to_vec(&mut vec_buf).unwrap();
    let ret = core::str::from_utf8(&vec_buf[0..len]).unwrap();
    assert_eq!(ret, "");
    assert_eq!(file.seek(str_len + 1).err().unwrap(), FileError::SeekValueOverFlow);

    root.delete("fefs").unwrap();
    assert!(!root.exist("fefs"));
    assert_eq!(root.delete("fefs").err().unwrap(), DirError::NotFound);

    let bin = root.cd("bin").unwrap();
    println!("{:#?}", bin.ls());
    println!("<-----------------------");
    println!("[passed] fefs test");
}

pub use stdio::{
    Stdin,
    Stdout
};
pub use pipe::make_pipe;
pub use inode::{
    open_file,
    OpenFlags
};
