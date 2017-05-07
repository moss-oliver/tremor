use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use file_utils::*;
use pak_loader::FileEntry;
use std::sync::Arc;
use lib_revert::util::Color;

#[derive(Clone)]
pub struct ColorLump {
    pub data:Arc<Vec<Color>>
}
impl ColorLump {
    pub fn load_lmp<T>(file_entry: FileEntry, mut file: &mut T) -> ColorLump where T: Read+Seek {
        let len = file_entry.size / 3;
        let mut data = Vec::with_capacity(len as usize);

        for i in 0..len {
            data.push(Color::new( read_u8(file),read_u8(file),read_u8(file) ));
        }
        ColorLump {data:Arc::new(data)}
    }
}