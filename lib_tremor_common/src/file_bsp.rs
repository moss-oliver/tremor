use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use file_utils::*;
use std::mem;
use cgmath::Vector3;
use std::sync::Arc;

pub struct BspHeader {
    pub version:        u32,
    pub entities:       FileLocation,
    pub planes:         FileLocation,
    
    pub miptex:         FileLocation,
    pub vertices:       FileLocation,
    
    pub visilist:       FileLocation,
    pub nodes:          FileLocation,
    
    pub texinfo:        FileLocation,
    
    pub faces:          FileLocation,
    
    pub lightmaps:      FileLocation,
    pub clipnodes:      FileLocation,
    
    pub leaves:         FileLocation,
    
    pub lface:          FileLocation,
    pub edges:          FileLocation,

    pub ledges:         FileLocation,
    pub models:         FileLocation,
}

pub struct BspEdge {
    pub vert1: u16,
    pub vert2: u16
}
pub struct BspFace {
    pub plane_id:       u16,
    pub side:           u16,
    pub edge_index:     u32,
    pub edge_count:     u16,
    pub tex_info_id:    u16,
    pub light_type:     u8,
    pub light_base:     u8,
    pub light1:         u8,
    pub light2:         u8,
    pub lightmap_index: i32
}
pub struct TexInfo {
    pub vec_s:          Vector3<f32>,
    pub dist_s:         f32,
    pub vec_t:          Vector3<f32>,
    pub dist_t:         f32,
    pub texture_id:     u32,
    pub animated:       u32,
}

pub struct TextureListHeader {
    pub numtex:     i32,
    pub offset:     Vec<i32>
}

pub struct Texture {
    pub name: String, //16 bytes
    pub width: u32,
    pub height: u32,
    //pub offset1: u32,
    //pub offset2: u32,
    //pub offset4: u32,
    //pub offset8: u32
    pub mip1: Vec<u8>,
    pub mip2: Vec<u8>,
    pub mip4: Vec<u8>,
    pub mip8: Vec<u8>
}

pub struct Bsp {
    pub header: BspHeader,

    pub vertices:               Vec<Vector3<f32>>,
    pub faces:                  Vec<BspFace>,
    pub edges:                  Vec<BspEdge>,
    pub edge_table:             Vec<i32>,
    pub texture_info:           Vec<TexInfo>,
    pub texture_list_header:    TextureListHeader,
    pub lightmap_buffer:        Arc<Vec<u8>>,

    pub texture_list:           Vec<Arc<Texture>>
}
impl Bsp {
    pub fn load_bsp<T>(mut file: &mut T) -> Bsp where T: Read+Seek {
        let mut version_vec = [0u8;4];
        file.read_exact(&mut version_vec);
        let mut version: u32;
        unsafe {
            version = mem::transmute::<[u8; 4], u32>(version_vec) as u32;
        }

        let header = BspHeader {
            version: version,

            entities:FileLocation::read_from_file(&mut file),
            planes:FileLocation::read_from_file(&mut file),
            
            miptex:FileLocation::read_from_file(&mut file),
            vertices:FileLocation::read_from_file(&mut file),
            
            visilist:FileLocation::read_from_file(&mut file),
            nodes:FileLocation::read_from_file(&mut file),
            
            texinfo:FileLocation::read_from_file(&mut file),
            
            faces:FileLocation::read_from_file(&mut file),
            
            lightmaps:FileLocation::read_from_file(&mut file),
            clipnodes:FileLocation::read_from_file(&mut file),
            
            leaves:FileLocation::read_from_file(&mut file),
            
            lface:FileLocation::read_from_file(&mut file),
            edges:FileLocation::read_from_file(&mut file),

            ledges:FileLocation::read_from_file(&mut file),
            models:FileLocation::read_from_file(&mut file),
        };

        //vertices
        let mut vertices : Vec<Vector3<f32>> = Vec::new();
        {
            let seek_val = file.seek(SeekFrom::Start(header.vertices.offset as u64));
            let vert_count = header.vertices.length / (mem::size_of::<f32>() as u64 *3);
            for v in 0..vert_count {
                let x = read_f32(file);
                let z = read_f32(file);
                let y = read_f32(file);
                //println!("vert {:?} x: {:?}", v, x);
                //println!("vert {:?} y: {:?}", v, y);
                //println!("vert {:?} z: {:?}", v, z);
                vertices.push(Vector3::new(x,y,z));
            }
        }
        //faces
        let mut faces : Vec<BspFace> = Vec::new();
        {
            let seek_val = file.seek(SeekFrom::Start(header.faces.offset as u64));
            let count = header.faces.length / ( 20 );
            for v in 0..count {
                let plane_id = read_u16(file) ;
                let side = read_u16(file) ;
                let edge_id = read_u32(file) ;
                let edge_count = read_u16(file) ;
                let tex_info_id = read_u16(file) ;
                let light_type = read_u8(file) ;
                let light_base = read_u8(file) ;
                let light1 = read_u8(file) ;
                let light2 = read_u8(file) ;
                let lightmap = read_i32(file) ;

                faces.push(BspFace {
                    plane_id: plane_id,
                    side: side,
                    edge_index: edge_id,
                    edge_count: edge_count,
                    tex_info_id: tex_info_id,
                    light_type: light_type,
                    light_base: light_base,
                    light1: light1,
                    light2: light2,
                    lightmap_index: lightmap
                });
            }
        }
        //edges
        let mut edges : Vec<BspEdge> = Vec::new();
        {
            println!("loading edges");
            let seek_val = file.seek(SeekFrom::Start(header.edges.offset as u64));
            let vert_count = header.edges.length / (mem::size_of::<u16>() as u64 *2);
            for v in 0..vert_count {
                let vert1 = read_u16(file);
                //println!("vert1: {:?}", vert1);
                let vert2 = read_u16(file);
                //println!("vert1: {:?}", vert2);
                edges.push(BspEdge{vert1:vert1, vert2:vert2});
            }
        }
        //edge table
        let mut edge_table : Vec<i32> = Vec::new();
        {
            println!("loading edges");
            let seek_val = file.seek(SeekFrom::Start(header.ledges.offset as u64));
            let vert_count = header.ledges.length / (mem::size_of::<i32>() as u64);
            for v in 0..vert_count {
                let edge = read_i32(file);
                edge_table.push(edge);
            }
        }
        //texinfo table
        let mut texinfo_table : Vec<TexInfo> = Vec::new();
        {
            println!("loading texinfos");
            let seek_val = file.seek(SeekFrom::Start(header.texinfo.offset as u64));
            let vert_count = header.texinfo.length / (40 as u64);
            for v in 0..vert_count {
                let sx = read_f32(file);
                let sz = read_f32(file);
                let sy = read_f32(file);
                let sdist = read_f32(file);
                let tx = read_f32(file);
                let tz = read_f32(file);
                let ty = read_f32(file);
                let tdist = read_f32(file);
                let texture_id = read_u32(file);
                let animated = read_u32(file);

                texinfo_table.push(TexInfo{
                    vec_s : Vector3::new(sx,sy,sz),
                    dist_s : sdist,
                    vec_t : Vector3::new(tx,ty,tz),
                    dist_t : tdist,
                    texture_id : texture_id,
                    animated : animated
                });
            }
        }
        let mut texture_list_header : TextureListHeader;
        {
            let seek_val = file.seek(SeekFrom::Start(header.miptex.offset as u64));
            let numtex = read_i32(file);
            let mut offset: Vec<i32> = Vec::new();
            for pat in 0..numtex {
                offset.push(read_i32(file));
            }
            println!("Number of textures: {:?}", numtex);
            texture_list_header = TextureListHeader {
                numtex : numtex,
                offset : offset
            };
        }

        let mut texture_list : Vec<Arc<Texture>> = Vec::with_capacity(texture_list_header.numtex as usize);
        {
            let list = &(texture_list_header.offset);
            for off in list {
                if *off >= 0 {
                    let seek_val = file.seek(SeekFrom::Start(header.miptex.offset + *off as u64));
                    
                    let mut tex_name = [0u8;16];
                    file.read_exact(&mut tex_name).expect("Opened texture, but failed to read name.");
                    
                    let chars_to_trim: &[char] = &[0u8 as char];
                    let converted_name = String::from_utf8_lossy( &tex_name );
                    let formatted_name : String = converted_name[0..converted_name.find(chars_to_trim).unwrap()].to_string();
                    println!("Loading texture: {:?}", formatted_name);

                    let w = read_u32(file);
                    let h = read_u32(file);
                    
                    let off1 = read_u32(file);
                    let off2 = read_u32(file);
                    let off4 = read_u32(file);
                    let off8 = read_u32(file);
                    let len1 = w*h;
                    let len2 = w*h/2;
                    let len4 = w*h/4;
                    let len8 = w*h/8;

                    let mut tex1 : Vec<u8> = Vec::with_capacity(len1 as usize);
                    {
                        let tex1_seek_val = file.seek(SeekFrom::Start(header.miptex.offset + off1 as u64));
                        for pat in 0..len1 {
                            tex1.push(read_u8(file));
                        }
                    }
                    let mut tex2 : Vec<u8> = Vec::with_capacity(len2 as usize);
                    {
                        let tex2_seek_val = file.seek(SeekFrom::Start(header.miptex.offset + off2 as u64));
                        for pat in 0..len2 {
                            tex2.push(read_u8(file));
                        }
                    }
                    let mut tex4 : Vec<u8> = Vec::with_capacity(len4 as usize);
                    {
                        let tex4_seek_val = file.seek(SeekFrom::Start(header.miptex.offset + off4 as u64));
                        for pat in 0..len4 {
                            tex4.push(read_u8(file));
                        }
                    }
                    let mut tex8 = Vec::with_capacity(len8 as usize);
                    {
                        let tex8_seek_val = file.seek(SeekFrom::Start(header.miptex.offset + off8 as u64));
                        for pat in 0..len8 {
                            tex8.push(read_u8(file));
                        }
                    }

                    texture_list.push(Arc::new(Texture {name: formatted_name, width: w, height: h, mip1: tex1, mip2: tex2, mip4: tex4, mip8: tex8}));
                } else {
                    let mut tex1 : Vec<u8> = Vec::with_capacity(8*8);
                    {
                        for pat in 0..(8*8) {
                            tex1.push(0);
                        }
                    }
                    let mut tex2 : Vec<u8> = Vec::with_capacity(4*4);
                    {
                        for pat in 0..(4*4) {
                            tex2.push(0);
                        }
                    }
                    let mut tex4 : Vec<u8> = Vec::with_capacity(2*2);
                    {
                        for pat in 0..(2*2) {
                            tex4.push(0);
                        }
                    }
                    let mut tex8 = Vec::with_capacity(1*1);
                    {
                        for pat in 0..(1*1) {
                            tex8.push(0);
                        }
                    }

                    texture_list.push(Arc::new(Texture {name: "NONE".to_string(), width: 8, height: 8, mip1: tex1, mip2: tex2, mip4: tex4, mip8: tex8}));
                }
            }
        }

        //lightmap
        let mut lightmap = Vec::with_capacity(header.lightmaps.length as usize);
        {
            let seek_val = file.seek(SeekFrom::Start(header.lightmaps.offset as u64));
            for pat in 0..header.lightmaps.length {
                lightmap.push(read_u8(file));
            }
        }

        Bsp {
            header: header,
            vertices: vertices,
            faces: faces,
            edges: edges,
            edge_table: edge_table,
            texture_info: texinfo_table,
            texture_list_header: texture_list_header,
            lightmap_buffer: Arc::new(lightmap),
            texture_list: texture_list
        }
    }
}