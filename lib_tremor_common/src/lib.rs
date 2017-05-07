extern crate lib_revert;
extern crate cgmath;

use cgmath::Matrix;
use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::Vector2;
use cgmath::Vector4;
use cgmath::SquareMatrix;
use cgmath::InnerSpace;
use cgmath::Transform;

use lib_revert::pipeline::Pipeline;
use lib_revert::util::Tri;

use lib_revert::util::PositionVertex;
use lib_revert::vertex_shader::TransformVertexShader;
use lib_revert::pixel_shader::SolidColorPixelShader;
use lib_revert::vertex_shader::ColorTransformVertexShader;
use lib_revert::pixel_shader::ColorPixelShader;
use lib_revert::util::Color;
use lib_revert::buffer::ColorBuffer;
use lib_revert::util::ColoredVertex;
//use lib_revert::util::None;
use lib_revert::buffer::DepthBufferF32;
use lib_revert::pixel_masker::DepthPixelMasker;
use lib_revert::buffer::WritableBuffer;

use lib_revert::util::TexturedVertex;
use lib_revert::pixel_shader::TexturePixelShader;
use lib_revert::buffer::Buffer;

use file_bsp::Texture;
use file_lmp::ColorLump;

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use std::cmp;
use std::f32;

pub mod input;
pub mod pak_loader;
pub mod file_utils;
pub mod file_bsp;
pub mod file_lmp;

pub trait Component {
    fn as_any(&self) -> &Any;
}

pub struct Renderable {
    
}

impl Component for Renderable {
    fn as_any(&self) -> &Any {
        self
    }
}

pub struct Object {
    name: String,
    components:Vec<Rc<RefCell<Component>>>
}



pub struct Space {
    name: String,
    objects: Vec<Object>
}

impl Space {
    pub fn new(name: String) -> Space {
        Space {
            name: name,
            objects: Vec::new()
        }
    }
}

pub struct State {
    level: file_bsp::Bsp,
    color_lump: ColorLump,
    pipeline: Pipeline<ColoredVertex,TransformVertexShader<ColoredVertex>,ColoredVertex, ColorPixelShader,Color, DepthPixelMasker<ColoredVertex>, f32>,
    pipeline2: Pipeline<TexturedVertex,TransformVertexShader<TexturedVertex>,TexturedVertex, TexturePixelShader<SimpleTextureSampler>,Color, DepthPixelMasker<TexturedVertex>, f32>,
    depthbuffer: Box<DepthBufferF32>,
    backbuffer: Box<ColorBuffer>,
    location: cgmath::Vector3<f32>,
//    rotation: cgmath::Quaternion<f32>
    pitch:f32,
    yaw:f32
}


pub fn init() -> State {
    //init renderer
    //let vs = TransformVertexShader{world_proj: Matrix4::identity()};
    //let ps = SolidColorPixelShader{color: Color::new(255,0,0)};
    //let vs = TransformVertexShader::<ColoredVertex>::new(Matrix4::identity());
    //let ps = ColorPixelShader{};
    let vs = TransformVertexShader::<ColoredVertex>::new(Matrix4::identity());
    let ps = ColorPixelShader{};

    let pipe = Pipeline::new(vs,ps, DepthPixelMasker::new());

    let vs2 = TransformVertexShader::<TexturedVertex>::new(Matrix4::identity());
    
    /*let mut tex = ColorBuffer::new(16,16);
    tex.set_pixel(0,0, Color::new(0,0,0));
    tex.set_pixel(1,0, Color::new(255,0,0));
    tex.set_pixel(0,1, Color::new(0,255,0));
    tex.set_pixel(1,1, Color::new(255,255,0));
    tex.set_pixel(14,14, Color::new(255,255,255));*/

    //let ps2 = TexturePixelShader{texture: tex};

    let pipe2 = Pipeline { vs: Option::Some(vs2), ps: Option::None, masker: Option::Some(DepthPixelMasker::new())};
    //Pipeline::new(vs2,Option::None, DepthPixelMasker::new());

    //load pak
    let file = pak_loader::Pak::from_file("C:\\Personal\\Quake\\Id1\\PAK0.PAK");
    
    println!("header id: {:?}", String::from_utf8( file.header.id.to_vec() ).unwrap());
    for f in &file.files {
        println!("file id: {:?}", f.name);
    }

    let lump_file_entry = file.find_file("gfx/palette.lmp".to_string()).expect("Failed to load file");
    let mut lump_file = file.open_file(lump_file_entry.clone());
    let mut map_file = file.open_file(file.find_file("maps/e1m1.bsp".to_string()).expect("Failed to load file"));
    let mut file = file_bsp::Bsp::load_bsp(&mut map_file);
    let lump = file_lmp::ColorLump::load_lmp(lump_file_entry, &mut lump_file);

    println!("e1m1 offset: {:?}", map_file.baseOffset);

    println!("file.header.entities.offset: {:?}", file.header.entities.offset);
    println!("file.header.entities.length: {:?}", file.header.entities.length);
    println!("file.header.planes.offset: {:?}", file.header.planes.offset);
    println!("file.header.miptex.offset: {:?}", file.header.miptex.offset);
    println!("file.header.vertices.offset: {:?}", file.header.vertices.offset);
    println!("file.header.visilist.offset: {:?}", file.header.visilist.offset);
    println!("file.header.nodes.offset: {:?}", file.header.nodes.offset);
    println!("file.header.texinfo.offset: {:?}", file.header.texinfo.offset);
    println!("file.header.faces.offset: {:?}", file.header.faces.offset);
    println!("file.header.faces.length: {:?}", file.header.faces.length);
    println!("file.header.lightmaps.offset: {:?}", file.header.lightmaps.offset);
    println!("file.header.clipnodes.offset: {:?}", file.header.clipnodes.offset);
    println!("file.header.leaves.offset: {:?}", file.header.leaves.offset);
    println!("file.header.lface.offset: {:?}", file.header.lface.offset);
    println!("file.header.edges.offset: {:?}", file.header.edges.offset);
    println!("file.header.ledges.offset: {:?}", file.header.ledges.offset);
    println!("file.header.models.offset: {:?}", file.header.models.offset);
    println!("file.header.models.length: {:?}", file.header.models.length);
    

    println!("state.level.faces.len(): {:?}", file.faces.len());
    for face_count in 0..file.faces.len() {
        let face = &file.faces[face_count];
        
        let face_offset = face.edge_index / 4;
        //let vert1 = face.
        //Tri::new(
        //    ColoredVertex::new(,0.0,255.0,255.0),
        //    ColoredVertex::new(vert2.x,vert2.y,vert2.z,0.0,255.0,255.0),
        //    ColoredVertex::new(vert3.x,vert3.y,vert3.z,0.0,255.0,255.0)
        //)
        
        //println!("face.edge_count: {:?}", face.edge_count);
        let face_texinfo = &file.texture_info[face.tex_info_id as usize];
        //println!("face.texture_id: {:?}", face_texinfo.texture_id);

        //let e1 = &file.edges[0]
        //let v1 = &file.vertices[ file.edges[edge as usize].vert1 as usize ];
        //for edge in 1..face.edge_count-2 {
            //let v2 = &file.vertices[ file.edges[(face.edge_index /4 ) + edge as usize].vert1 as usize ];
            //let v3 = &file.vertices[ file.edges[(face.edge_index /4 ) + edge as usize].vert2 as usize ];
            //let e2 = &file.edges[(edge-1) as usize];
            //let e3 = &file.edges[edge as usize];
            /*Tri::new(
                ColoredVertex::new(v1.x,v1.y,v1.z ,0.0,255.0,255.0),
                ColoredVertex::new(v2.x,v2.y,v2.z,255.0,0.0,255.0),
                ColoredVertex::new(v3.x,v3.y,v3.z,255.0,255.0,0.0)
            );*/
            //unimplemented!();
        //}

        /*println!("({:?},{:?}),({:?},{:?}),({:?},{:?}), ({:?},{:?}),({:?},{:?}),({:?},{:?})"
            , file.edges[(face.edge_index+0) as usize].vert1,file.edges[(face.edge_index+0) as usize].vert2
            , file.edges[(face.edge_index+1) as usize].vert1,file.edges[(face.edge_index+1) as usize].vert2
            , file.edges[(face.edge_index+2) as usize].vert1,file.edges[(face.edge_index+2) as usize].vert2
            // tri 2
            , file.edges[(face.edge_index+3) as usize].vert1,file.edges[(face.edge_index+3) as usize].vert2
            , file.edges[(face.edge_index+4) as usize].vert1,file.edges[(face.edge_index+4) as usize].vert2
            , file.edges[(face.edge_index+5) as usize].vert1,file.edges[(face.edge_index+5) as usize].vert2);
        */
    }

    //return state.
    State {
        level: file,
        color_lump: lump,
        pipeline: pipe,
        pipeline2: pipe2,
        depthbuffer: Box::new(DepthBufferF32::new(512,512)),
        backbuffer: Box::new(ColorBuffer::new(512,512)),
        location: cgmath::Vector3::new(0.0,0.0,2.0),
        pitch: 0.0,
        yaw: 180.0
    }
}

pub fn update(state: &mut State, input: &input::InputManager) {
    if input.is_key_down(input::KeyboardKey::Left) {
        state.yaw-=1.0;
    }
    if input.is_key_down(input::KeyboardKey::Right) {
        state.yaw+=1.0;
    }
    if input.is_key_down(input::KeyboardKey::Up) {
        state.pitch+=1.0;
    }
    if input.is_key_down(input::KeyboardKey::Down) {
        state.pitch-=1.0;
    }

    while state.yaw < 0.0 {
        state.yaw -= 360.0;
    }
    while state.yaw > 360.0 {
        state.yaw += 360.0;
    }

    while state.pitch < 0.0 {
        state.pitch += 360.0;
    }
    while state.pitch > 360.0 {
        state.pitch -= 360.0;
    }

    let rot = (
        cgmath::Matrix4::from_angle_x(cgmath::Deg(state.pitch)) *
        cgmath::Matrix4::from_angle_y(cgmath::Deg(state.yaw)) *
        cgmath::Matrix4::from_angle_z(cgmath::Deg(0.0))
        ).invert().expect("Error inverting view mat")
    ;

    if input.is_key_down(input::KeyboardKey::W) {
        state.location += rot.transform_vector(cgmath::Vector3::new(0.0,0.0,1.5))
    }
    if input.is_key_down(input::KeyboardKey::S) {
        state.location += rot.transform_vector(cgmath::Vector3::new(0.0,0.0,-1.5))
    }
    if input.is_key_down(input::KeyboardKey::A) {
        state.location += rot.transform_vector(cgmath::Vector3::new(1.5,0.0,0.0))
    }
    if input.is_key_down(input::KeyboardKey::D) {
        state.location += rot.transform_vector(cgmath::Vector3::new(-1.5,0.0,0.0))
    }
}

fn min<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b}}

fn max<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b}}

pub fn render_frame(state: &mut State) 
{
    let flip: Matrix4<f32> = Matrix4 { x: Vector4 { x:-1.0, y: 0.0, z: 0.0, w: 0.0 },
                                       y: Vector4 { x: 0.0, y:-1.0, z: 0.0, w: 0.0 },
                                       z: Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
                                       w: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } };

    let rot = 
        cgmath::Matrix4::from_angle_x(cgmath::Deg(state.pitch)) *
        cgmath::Matrix4::from_angle_y(cgmath::Deg(state.yaw)) *
        cgmath::Matrix4::from_angle_z(cgmath::Deg(0.0))
    ;
    let pos = cgmath::Matrix4::from_translation(state.location) ;//* cgmath::Matrix4::from_diagonal(cgmath::Vector4::new(0.01,-0.01,0.01,0.01));
    let view = rot * pos * flip;
    let proj = cgmath::perspective(cgmath::Deg(75.0),1.0,0.01,2000.0);

    //state.pipeline.vs.world_proj = (proj * view);
    //state.pipeline.vs
    if let Some(ref mut x) = state.pipeline.vs {
        x.world_proj = (proj * view);
    }
    if let Some(ref mut x) = state.pipeline2.vs {
        x.world_proj = (proj * view);
    }
    //state.pipeline2.vs.world_proj = (proj * view);
    /*match state.pipeline.vs {
        Some(expr) => {
            expr.world_proj = (proj * view);
        },
        None => {},
    }*/

    state.backbuffer.as_mut().clear(Color::new(0,0,0));
    state.depthbuffer.as_mut().clear(1.0);
    {
        let mut session = state.pipeline.create_session(state.backbuffer.as_mut(), state.depthbuffer.as_mut()).unwrap();

        // front
        let tri = Tri::new(
            ColoredVertex::new(0.0,0.0,0.5,0.0,0.0,255.0),
            ColoredVertex::new(1.0,0.0,0.5,255.0,0.0,0.0),
            ColoredVertex::new(0.0,1.0,0.5,0.0,255.0,0.0)
            );
        session.draw_tri(tri);

        let tri = Tri::new(
            ColoredVertex::new(1.0,1.0,0.5,0.0,0.0,255.0),
            ColoredVertex::new(1.0,0.0,0.5,255.0,0.0,0.0),
            ColoredVertex::new(0.0,1.0,0.5,0.0,255.0,0.0)
            );
        session.draw_tri(tri);

        //back
        let tri = Tri::new(
            ColoredVertex::new(0.0,0.0,-0.5,0.0,0.0,255.0),
            ColoredVertex::new(1.0,0.0,-0.5,255.0,0.0,0.0),
            ColoredVertex::new(0.0,1.0,-0.5,0.0,255.0,0.0)
            );
        session.draw_tri(tri);

        let tri = Tri::new(
            ColoredVertex::new(1.0,1.0,-0.5,0.0,0.0,255.0),
            ColoredVertex::new(1.0,0.0,-0.5,255.0,0.0,0.0),
            ColoredVertex::new(0.0,1.0,-0.5,0.0,255.0,0.0)
            );
        session.draw_tri(tri);
    }
    {
        for vert_count in 0..(state.level.vertices.len()-2)/3 {
        //for vert_count in 0..100 {
            let vert1 = state.level.vertices[vert_count];
            let vert2 = state.level.vertices[vert_count+1];
            let vert3 = state.level.vertices[vert_count+2];
        }

        for face_count in 0..state.level.faces.len() {
            let face = &state.level.faces[face_count];
            
            //Get lightmap size
            let mut max_s_f32: f32 = f32::MIN;
            let mut max_t_f32: f32 = f32::MIN;
            let mut min_s_f32: f32 = f32::MAX;
            let mut min_t_f32: f32 = f32::MAX;
            for edge in 0..face.edge_count-1 {

                let edge_lookup = state.level.edge_table[(face.edge_index + edge as u32) as usize];

                let v1;
                let v2;
                
                v1 = &state.level.vertices[ state.level.edges[edge_lookup.abs() as usize].vert1 as usize / 1 ];
                v2 = &state.level.vertices[ state.level.edges[edge_lookup.abs() as usize].vert2 as usize / 1 ];

                let face_texinfo = &state.level.texture_info[face.tex_info_id as usize];
                let s1 = v1.dot(face_texinfo.vec_s) + face_texinfo.dist_s;
                let t1 = v1.dot(face_texinfo.vec_t) + face_texinfo.dist_t;
                let s2 = v2.dot(face_texinfo.vec_s) + face_texinfo.dist_s;
                let t2 = v2.dot(face_texinfo.vec_t) + face_texinfo.dist_t;



                max_s_f32 = max(max_s_f32, max(s1,s2));
                max_t_f32 = max(max_t_f32, max(t1,t2));
                min_s_f32 = min(min_s_f32, min(s1,s2));
                min_t_f32 = min(min_t_f32, min(t1,t2));
            }
            
            let max_s = <f32>::ceil(max_s_f32 / 16.0);
            let max_t = <f32>::ceil(max_t_f32 / 16.0);
            let min_s = <f32>::floor(min_s_f32 / 16.0);
            let min_t = <f32>::floor(min_t_f32 / 16.0);
            let lightmap_size_s = (max_s - min_s) as u32 + 1;
            let lightmap_size_t = (max_t - min_t) as u32 + 1;

            let v1;
            if state.level.edge_table[(face.edge_index ) as usize] < 0 {
                v1 = &state.level.vertices[ state.level.edges[state.level.edge_table[(face.edge_index ) as usize].abs() as usize].vert2 as usize / 1 ];
            } else {
                v1 = &state.level.vertices[ state.level.edges[state.level.edge_table[(face.edge_index ) as usize].abs() as usize].vert1 as usize / 1 ];
            }

            for edge in 1..face.edge_count-1 {
                let edge_lookup = state.level.edge_table[(face.edge_index + edge as u32) as usize];

                let v2;
                let v3;
                
                if edge_lookup < 0
                {
                    v3 = &state.level.vertices[ state.level.edges[edge_lookup.abs() as usize].vert1 as usize / 1 ];
                    v2 = &state.level.vertices[ state.level.edges[edge_lookup.abs() as usize].vert2 as usize / 1 ];
                }
                else {
                    v2 = &state.level.vertices[ state.level.edges[edge_lookup.abs() as usize].vert1 as usize / 1 ];
                    v3 = &state.level.vertices[ state.level.edges[edge_lookup.abs() as usize].vert2 as usize / 1 ];
                }
                let face_texinfo = &state.level.texture_info[face.tex_info_id as usize];
                let s1 = v1.dot(face_texinfo.vec_s) + face_texinfo.dist_s;
                let t1 = v1.dot(face_texinfo.vec_t) + face_texinfo.dist_t;
                let s2 = v2.dot(face_texinfo.vec_s) + face_texinfo.dist_s;
                let t2 = v2.dot(face_texinfo.vec_t) + face_texinfo.dist_t;
                let s3 = v3.dot(face_texinfo.vec_s) + face_texinfo.dist_s;
                let t3 = v3.dot(face_texinfo.vec_t) + face_texinfo.dist_t;
                if (face.lightmap_index == -1)
                {
                    //state.pipeline2.ps = Option::Some(TexturePixelShader{texture: LightmapSampler::new(state.level.lightmap_buffer.clone(), face.lightmap_index,lightmap_size_s, lightmap_size_t)});
                    //if let Some(ref mut ps) = state.pipeline2.ps {
                    //    ps.texture = LightmapSampler::new(state.level.lightmap_buffer.clone(), face.lightmap_index,lightmap_size_s, lightmap_size_t);
                    //}
                    {
                        let mut session = state.pipeline.create_session(state.backbuffer.as_mut(), state.depthbuffer.as_mut()).unwrap();
                        session.draw_tri(
                            Tri::new(
                                ColoredVertex::new(v1.x,v1.y,v1.z,255.0,0.0,0.0),
                                ColoredVertex::new(v2.x,v2.y,v2.z,0.0,255.0,0.0),
                                ColoredVertex::new(v3.x,v3.y,v3.z,0.0,0.0,255.0)
                            )
                        );
                    }
                } else {
                    //println!("Lightmap size x: {:?}, y: {:?}", lightmap_size_s, lightmap_size_t);
                    //SimpleTextureSampler
                    //state.pipeline2.ps = Option::Some(TexturePixelShader{texture: LightmapSampler::new(state.level.lightmap_buffer.clone(), face.lightmap_index as u32,lightmap_size_s, lightmap_size_t)});
                    let tex = state.level.texture_list[face_texinfo.texture_id as usize].clone();
                    //let w = tex.width;
                    //let h = tex.height;
                    let w = 1.0;
                    let h = 1.0;
                    state.pipeline2.ps = Option::Some(TexturePixelShader{texture: SimpleTextureSampler::new(tex, state.color_lump.clone(), 0)});
                    {
                        let mut session = state.pipeline2.create_session(state.backbuffer.as_mut(), state.depthbuffer.as_mut()).unwrap();
                        session.draw_tri(
                            Tri::new(
                                TexturedVertex::new(v1.x,v1.y,v1.z, s1 / w,t1 / h),
                                TexturedVertex::new(v2.x,v2.y,v2.z, s2 / w,t2 / h),
                                TexturedVertex::new(v3.x,v3.y,v3.z, s3 / w,t3 / h)
                            )
                        );
                    }
                }
            }
        }
    }
    {
        let mut session = state.pipeline.create_session(state.backbuffer.as_mut(), state.depthbuffer.as_mut()).unwrap();

        //X
        session.draw_tri(
            Tri::new(
                ColoredVertex::new(0.0,0.0,0.0,255.0,0.0,0.0),
                ColoredVertex::new(1.0,0.03,0.0,255.0,0.0,0.0),
                ColoredVertex::new(1.0,-0.03,0.0,255.0,0.0,0.0)
            )
        );
        session.draw_tri(
            Tri::new(
                ColoredVertex::new(0.0,0.0,0.0,255.0,0.0,0.0),
                ColoredVertex::new(1.0,0.0,0.03,255.0,0.0,0.0),
                ColoredVertex::new(1.0,0.0,-0.03,255.0,0.0,0.0)
            )
        );
        //Y
        session.draw_tri(
            Tri::new(
                ColoredVertex::new(0.0,0.0,0.0,0.0,255.0,0.0),
                ColoredVertex::new(0.03,1.0,0.0,0.0,255.0,0.0),
                ColoredVertex::new(-0.03,1.0,0.0,0.0,255.0,0.0)
            )
        );
        session.draw_tri(
            Tri::new(
                ColoredVertex::new(0.0,0.0,0.0,0.0,255.0,0.0),
                ColoredVertex::new(0.0,1.0,0.03,0.0,255.0,0.0),
                ColoredVertex::new(0.0,1.0,-0.03,0.0,255.0,0.0)
            )
        );
        //Z
        session.draw_tri(
            Tri::new(
                ColoredVertex::new(0.0,0.0,0.0,0.0,0.0,255.0),
                ColoredVertex::new(0.03,0.0,1.0,0.0,0.0,255.0),
                ColoredVertex::new(-0.03,0.0,1.0,0.0,0.0,255.0)
            )
        );
        session.draw_tri(
            Tri::new(
                ColoredVertex::new(0.0,0.0,0.0,0.0,0.0,255.0),
                ColoredVertex::new(0.0,0.03,1.0,0.0,0.0,255.0),
                ColoredVertex::new(0.0,-0.03,1.0,0.0,0.0,255.0)
            )
        );

    }
}
pub struct BackbufferDetails {
    pub size_x: u32,
    pub size_y: u32,
    pub backbuffer: *const u8
}
pub fn get_backbuffer_details(renderer: &State) -> BackbufferDetails {
    let bb = renderer.backbuffer.as_ref();
    //let ps = renderer.pipeline2.ps.clone().unwrap();
    //ps.texture.

    //SimpleTextureSampler ts = 
        //SimpleTextureSampler::new(state.level.texture_list[0 as usize].clone(), state.color_lump.clone(), 0,lightmap_size_s, lightmap_size_t);

    //
    //let bb = ps.texture;
    //let bb = renderer.level.texture_list[0].clone();
    /*let mut cb = ColorBuffer::new(bb.get_width(), bb.get_height());
    for y in 0..cb.get_height() {
        for x in 0..cb.get_width() {
            cb.set_pixel(x,y,bb.get_pixel(x,y));
        }
    }*/
    BackbufferDetails {
        size_x: bb.get_width(),
        size_y: bb.get_height(),
        backbuffer: bb.get_ptr()
        /*size_x: cb.get_width(),
        size_y: cb.get_height(),
        backbuffer: cb.get_ptr()*/
    }
}

#[derive(Clone)]
struct LightmapSampler {
    buffer: Arc<Vec<u8>>,
    offset: usize,
    w:u32,
    h:u32
}

impl LightmapSampler {
    fn new(buffer: Arc<Vec<u8>>, offset:u32, width:u32, height:u32) -> LightmapSampler {
        LightmapSampler {buffer:buffer, offset:offset as usize, w:width, h:height}
    }
}

impl Buffer for LightmapSampler {
    type T = Color;
    fn get_width(&self) -> u32 {
        return self.w;
    }
    fn get_height(&self) -> u32 {
        return self.h;
    }
    fn get_pixel(&self, x:u32,y:u32) -> Color {
        if x > self.get_width() {
            return Color {r:255,g:0,b:0}
        }
        if y > self.get_height() {
            return Color {r:255,g:0,b:0}
        }

        return Color {
            r: self.buffer[(self.offset + ((x as usize)+(y*self.w) as usize))],
            g: self.buffer[(self.offset + ((x as usize)+(y*self.w) as usize))],
            b: self.buffer[(self.offset + ((x as usize)+(y*self.w) as usize))]
        }
    }
}



#[derive(Clone)]
struct SimpleTextureSampler {
    texture: Arc<Texture>, //buffer: Arc<Vec<u8>>,
    color_lump: ColorLump,
    offset: usize,
    w:u32,
    h:u32
}

impl SimpleTextureSampler {
    fn new(texture: Arc<Texture>, color_lump: ColorLump, offset:u32) -> SimpleTextureSampler {
        SimpleTextureSampler {texture:texture.clone(), color_lump:color_lump.clone(), offset:offset as usize, w:texture.width, h:texture.height}
    }
}

impl Buffer for SimpleTextureSampler {
    type T = Color;
    fn get_width(&self) -> u32 {
        return self.w;
    }
    fn get_height(&self) -> u32 {
        return self.h;
    }
    fn get_pixel(&self, x:u32,y:u32) -> Color {
        if x > self.get_width() {
            return Color {r:255,g:0,b:0}
        }
        if y > self.get_height() {
            return Color {r:255,g:0,b:0}
        }
        let index = self.texture.mip1[(((x as usize)+(y*self.w) as usize))];

        return self.color_lump.data[index as usize];
        /*return Color {
            r: self.color_lump.[((index*3)+0],
            g: self.color_lump[((index*3)+1],
            b: self.color_lump[((index*3)+2]
        }*/
    }
}