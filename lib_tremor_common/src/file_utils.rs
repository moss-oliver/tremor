use std::fs::File;
use std::io::Read;
use std::mem;

#[derive(Clone, Copy)]
pub struct FileLocation {
    pub offset:u64,
    pub length:u64
}

impl FileLocation {
    pub fn new(offset:u64, length:u64) -> FileLocation {
        FileLocation {
            offset:offset,
            length:length
        }
    }
    pub fn read_from_file(from_file: &mut Read) -> FileLocation {
        let mut offset_vec = [0u8;4];
        let mut size_vec = [0u8;4];
        
        from_file.read_exact(&mut offset_vec).expect("Opened file, but failed to read header offset.");
        from_file.read_exact(&mut size_vec).expect("Opened file, but failed to read header size.");

        let mut offset = 0;
        let mut size = 0;
        unsafe {
            offset = mem::transmute::<[u8; 4], u32>(offset_vec) as u64;
            size = mem::transmute::<[u8; 4], u32>(size_vec) as u64;
        }
        return FileLocation::new(offset, size);
    }
}

pub fn read_i32(from_file: &mut Read) -> i32 {
    let mut int = 0;
    let mut vec = [0u8;4];
    from_file.read_exact(&mut vec).expect("Opened file, but failed to read i32.");
    unsafe {
        int = mem::transmute::<[u8; 4], i32>(vec);
    }
    int
}

pub fn read_u32(from_file: &mut Read) -> u32 {
    let mut int = 0;
    let mut vec = [0u8;4];
    from_file.read_exact(&mut vec).expect("Opened file, but failed to read u32.");
    unsafe {
        int = mem::transmute::<[u8; 4], u32>(vec);
    }
    int
}

pub fn read_u16(from_file: &mut Read) -> u16 {
    let mut int = 0;
    let mut vec = [0u8;2];
    from_file.read_exact(&mut vec).expect("Opened file, but failed to read u32.");
    unsafe {
        int = mem::transmute::<[u8; 2], u16>(vec);
    }
    //println!("u16: {:?}", int);
    int
}

pub fn read_u8(from_file: &mut Read) -> u8 {
    let mut int = 0;
    let mut vec = [0u8;1];
    from_file.read_exact(&mut vec).expect("Opened file, but failed to read u32.");
    unsafe {
        int = mem::transmute::<[u8; 1], u8>(vec);
    }
    int
}

pub fn read_f32(from_file: &mut Read) -> f32 {
    let mut f = 0.0;
    let mut int = 0;
    let mut vec = [0u8;4];
    from_file.read_exact(&mut vec).expect("Opened file, but failed to read f32.");
    unsafe {
        int = mem::transmute::<[u8; 4], u32>(vec);
        f = mem::transmute::<[u8; 4], f32>(vec);
    }

    f
}