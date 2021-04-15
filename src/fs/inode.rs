#![allow(unused)]

use core::cmp::min;
use alloc::sync::Arc;
use alloc::vec::Vec;
use fefs::dir::DirEntry;
use fefs::file::FileEntry;

use alloc::string::String;

use spin::{
    Mutex, 
    MutexGuard
};

use super::{
    ROOT,
    File,
    UserBuffer,
};

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

pub struct OSINode {
    readable: bool,
    writable: bool,
    inner: OSINodeInner,
}

pub struct OSINodeInner {
    path: String,
    file: Arc<Mutex<FileEntry>>,
}

impl File for OSINode {
    fn readable(&self) -> bool { self.readable }
    fn writable(&self) -> bool { self.writable }

    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut file_lock = self.inner.file.lock();
        let min_len = min(file_lock.size(), buf.len());
        let mut read_len = 0;
        for idx in 0..buf.inner.len() {
            let read_buf = buf.inner.get_mut(idx).unwrap();
            read_len += file_lock.read(read_buf).unwrap();
            if min_len == read_len { break };
        }
        min_len
    }

    fn write(&self, buf: UserBuffer) -> usize {
        0
    }
}

pub fn open_file(path: &str, flags: OpenFlags) -> Option<Arc<OSINode>> {
    let (readable, writable) = flags.read_write();

    let path: String = path.into();
    let mut path_vec: Vec<String> = path.split('/')
                                    .filter(|&s| !s.is_empty())
                                    .map(|s| s.into())
                                    .collect();
    let file_name = path_vec.remove(path_vec.len() - 1);

    let root_lock = ROOT.lock();
    let mut dir: Option<DirEntry> = None;

    for name in path_vec.iter() {
        dir = match dir {
            Some(dir) => dir.cd(&name).map_or(
                None, 
                |entry| Some(entry)
            ),
            None => root_lock.cd(&name).map_or(
                None, 
                |entry| Some(entry)
            ),
        };
        if dir.is_none() { return None; }
    }
    
    let option_file = if path_vec.is_empty() {
        root_lock.open_file(&file_name).map_or(
            None,
            |file| Some(file)
        )
    } else {
        let dir = dir.take().unwrap();
        dir.open_file(&file_name).map_or(
            None,
            |file| Some(file)
        )
    };

    if option_file.is_none() { return None; }
    let file = option_file.unwrap();

    let inner = OSINodeInner {
        path,
        file: Arc::new(
            Mutex::new(
                file
            )
        ),
    };

    Some(
        Arc::new(
            OSINode {
                readable,
                writable,
                inner,
            }
        )
    )
}
