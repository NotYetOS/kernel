use fefs::dir::DirEntry;
use fefs::system::FileSystem;
use lazy_static::lazy_static;
use alloc::sync::Arc;
use spin::Mutex;
use crate::drivers::BLOCK_DEVICE;

lazy_static! {
    pub static ref ROOT: Arc<Mutex<DirEntry>> = {
        let fs = FileSystem::open(BLOCK_DEVICE.clone());
        let fs = fs.lock();
        let root = fs.root();
        Arc::new(Mutex::new(root))
    };
}

pub fn fefs_test() {
    use fefs::file::WriteType;
    use fefs::BLOCK_SIZE;
    use fefs::dir::DirError;
    
    println!("");
    println!("this is fefs tests");
    let mut root = ROOT.lock();
    root.mkdir("fefs").unwrap();
    assert_eq!(root.mkdir("fefs").err().unwrap(), DirError::DirExist);
    let mut dir = root.cd("fefs").unwrap();
    let mut file = dir.create_file("tlnb").unwrap();

    file.write("hello fefs".as_bytes(), WriteType::OverWritten).unwrap();
    let mut buf = [0; BLOCK_SIZE];
    let len = file.read(&mut buf).unwrap();
    let ret = core::str::from_utf8(&buf[0..len]).unwrap();
    assert_eq!(ret, "hello fefs");
    println!("{}", ret);
    println!("{:#?}", dir.ls());

    root.delete("fefs").unwrap();
    assert_eq!(root.delete("fefs").err().unwrap(), DirError::NotFound);

    root.mkdir("fefs").unwrap();
    root.delete("fefs").unwrap();
    println!("{:#?}", root.ls());
}