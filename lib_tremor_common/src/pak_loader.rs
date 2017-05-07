use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::Seek;
use std::io::SeekFrom;
use std::mem;
use std::ffi::CString;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use file_utils::FileLocation;

#[derive(Clone)]
pub struct Pak<PATH> where PATH : AsRef<Path> + Sized {
    pub filepath: PATH,
    pub header: Header,
    pub files: Vec<FileEntry>
}

impl<PATH> Pak<PATH> where PATH : AsRef<Path> + Sized {
    pub fn from_file(path: PATH) -> Pak<PATH> where PATH : AsRef<Path> + Sized {
        let mut file = File::open(&path).expect("Failed to open pak file.");
        
        //let mut header_bytes = [0u8;12];
        //file.read_exact(&mut header_bytes).expect("Opened file, but failed to read header.");

        let mut header_id = [0u8;4];
        let mut header_offset_vec = [0u8;4];
        let mut header_size_vec = [0u8;4];
        file.read_exact(&mut header_id).expect("Opened file, but failed to read header id.");
        //file.read_exact(&mut header_offset_vec).expect("Opened file, but failed to read header offset.");
        //file.read_exact(&mut header_size_vec).expect("Opened file, but failed to read header size.");

        //let mut header_offset = 0;
        //let mut header_size = 0;
        //unsafe {
        //    header_offset = mem::transmute::<[u8; 4], u32>(header_offset_vec) as u64;
        //    header_size = mem::transmute::<[u8; 4], u32>(header_size_vec) as u64;
        //}
        let header = Header {
            id: header_id,
            file_entry_table: FileLocation::read_from_file(&mut file)
            //file_entry_table: FileLocation::new(header_offset, header_size)
            //offset: header_offset,
            //size: header_size
        };

        println!("offset: {:?}", header.file_entry_table.offset);
        let file_count = header.file_entry_table.length/64;
        println!("file count: {:?}", file_count);

        //read file table
        //let &mut files = &[FileEntry; file_count] ;
        let mut files = Vec::new();
        file.seek(SeekFrom::Start(header.file_entry_table.offset as u64));
        for i in 0..(file_count) {
            let mut file_name = [0u8;56];
            let mut file_offset_vec = [0u8;4];
            let mut file_size_vec = [0u8;4];
            file.read_exact(&mut file_name).expect("Opened file, but failed to read file.");
            file.read_exact(&mut file_offset_vec).expect("Opened file, but failed to read file.");
            file.read_exact(&mut file_size_vec).expect("Opened file, but failed to read file.");
            
            let mut file_offset = 0;
            let mut file_size = 0;

            unsafe {
                file_offset = mem::transmute::<[u8; 4], u32>(file_offset_vec);
                file_size = mem::transmute::<[u8; 4], u32>(file_size_vec);
            }

            let chars_to_trim: &[char] = &[0u8 as char];
            let converted_name = String::from_utf8_lossy( &file_name );
            let formatted_name : String = converted_name[0..converted_name.find(chars_to_trim).unwrap()].to_string();
            //println!("file name: {:?}", formatted_name);
            //println!("file size: {:?}", file_size);
            files.push(FileEntry {
                name: formatted_name,
                offset: file_offset,
                size: file_offset
            });
        }

        Pak {
            filepath: path,
            header:header,
            files:files
        }
    }

    pub fn find_file(&self, file: String) -> Option<FileEntry> {
        for i in 0..self.files.len() {
            if(file == self.files[i].name) {
                return Option::Some(self.files[i].clone());
            }
        }
        return Option::None;
    }

    pub fn open_file(&self, fileEntry: FileEntry) -> PakFile {
        let mut file = File::open(&self.filepath).expect("Failed to open pak file.");
        file.seek(SeekFrom::Start(fileEntry.offset as u64));
        PakFile {
            file: file,
            baseOffset: fileEntry.offset as u64,
            fileLength: fileEntry.size as u64
        }
    }
}

pub struct PakFile {
    file: File,
    pub baseOffset: u64,
    pub fileLength: u64
}

impl Read for PakFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for PakFile {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {

        let adjpos = match pos {
            SeekFrom::Start(offset) => {
                if offset > self.fileLength {
                    return Err(Error::new(ErrorKind::UnexpectedEof, "Seek out of file."));
                }
                println!("seeking to: {:?}", offset+self.baseOffset);
                SeekFrom::Start(offset+self.baseOffset)
            },
            SeekFrom::End(offset) => {
                return Err(Error::new(ErrorKind::Other, "Unsupported seekfrom."));
                //SeekFrom::End(offset+self.baseOffset as i64)
            },
            SeekFrom::Current(offset) => {
                return Err(Error::new(ErrorKind::Other, "Unsupported seekfrom."));
                //SeekFrom::Current(offset+self.baseOffset as i64)
            }
        };
        self.file.seek(adjpos)

    }
}

#[derive(Clone, Copy)]
pub struct Header {
    pub id: [u8; 4],
    pub file_entry_table: FileLocation
    //pub offset: u32,
    //pub size: u32
}

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub offset: u32,
    pub size: u32
}