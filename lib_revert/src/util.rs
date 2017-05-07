use Vec3f32;
//use Point3f32;
use Vec4f32;
use Vec2f32;

#[derive(Clone, Copy)]
pub struct None {}

pub trait Interpolatable {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn scale(&self, scalar: f32) -> Self;
}

pub trait Rasterizable : Interpolatable {
    fn get_position(&self) -> &Vec4f32;
    fn set_position(&mut self, Vec4f32);
}

pub struct Tri<T> where T: Sized {
    pub p1:T,
    pub p2:T,
    pub p3:T
}

impl<T> Tri<T> where T: Sized {
    pub fn new(p1:T,p2:T,p3:T) -> Tri<T> {
        Tri{p1:p1, p2:p2, p3:p3}
    }
}

#[derive(Clone, Copy)]
pub struct PositionVertex {
    pub position: Vec4f32
}
impl PositionVertex {
    pub fn new(px:f32, py:f32, pz:f32) -> PositionVertex {
        PositionVertex {
            position:Vec4f32{x:px,y:py,z:pz,w:1.0}
        }
    }
}

impl Rasterizable for PositionVertex {
    fn get_position(&self) -> &Vec4f32 {
        & self.position
    }
    fn set_position(&mut self, pos: Vec4f32) {
        self.position = pos;
    }
}
impl Interpolatable for PositionVertex {
    fn add(&self, other: &Self) -> Self {
        return PositionVertex { position: (self.position + other.position) }
    }
    fn sub(&self, other: &Self) -> Self {
        return PositionVertex { position: (self.position - other.position) }
    }
    fn scale(&self, scalar: f32) -> Self {
        return PositionVertex { position: self.position * scalar }
    }
}

#[derive(Clone, Copy)]
pub struct ColoredVertex {
    pub position: Vec4f32,
    pub color: Color
}
impl ColoredVertex {
    pub fn new(px:f32, py:f32, pz:f32, r:f32, g:f32,b:f32) -> ColoredVertex {
        ColoredVertex {
            position:Vec4f32{x:px,y:py,z:pz,w:1.0},
            color:Color{r:r as u8,g:g as u8,b:b as u8}
        }
    }
}

impl Rasterizable for ColoredVertex {
    fn get_position(&self) -> &Vec4f32 {
        & self.position
    }
    fn set_position(&mut self, pos: Vec4f32) {
        self.position = pos;
    }
}
impl Interpolatable for ColoredVertex {
    fn add(&self, other: &Self) -> Self {
        return ColoredVertex { position: (self.position + other.position), color: self.color.add(&other.color) }
    }
    fn sub(&self, other: &Self) -> Self {
        return ColoredVertex { position: (self.position - other.position), color: self.color.sub(&other.color) }
    }
    fn scale(&self, scalar: f32) -> Self {
        return ColoredVertex { position: self.position * scalar, color: self.color.scale(scalar) }
    }
}


#[derive(Clone, Copy)]
pub struct Color {
    pub r:u8,
    pub g:u8,
    pub b:u8
}
impl Color {
    pub fn new(r:u8,g:u8,b:u8) -> Color {
        Color {
            r:r,g:g,b:b
        }
    }
}
impl Interpolatable for Color {
    fn add(&self, other: &Self) -> Self {
        Color{
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b)
        }
    }
    fn sub(&self, other: &Self) -> Self {
        Color{
            r: self.r.saturating_sub(other.r),
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b)
        }
    }
    fn scale(&self, scalar: f32) -> Self {
        Color{
            r: (self.r as f32 * scalar) as u8,
            g: (self.g as f32 * scalar) as u8,
            b: (self.b as f32 * scalar) as u8,
        }
    }
}
//impl Clone for Color {
//    fn clone(&self) -> Color {
//        Color{r:self.r,g:self.g,b:self.b}
//    }
//}

//pub fn from_vec(vec: Vec4f32) -> Vec3f32 {
//    return Vec3f32 {
//        x: vec.x,
//        y: vec.y,
//        z: vec.z
//    }
//}

//pub fn to_vec(vec: &Vec3f32) -> Vec4f32 {
//    return Vec4f32::new(
//        vec.x,
//        vec.y,
//        vec.z, 1.0
//    );
//}

#[derive(Clone, Copy)]
pub struct TexturedVertex {
    pub position: Vec4f32,
    pub uv: Vec2f32
}
impl TexturedVertex {
    pub fn new(px:f32, py:f32, pz:f32, u:f32, v:f32) -> TexturedVertex {
        TexturedVertex {
            position:Vec4f32{x:px,y:py,z:pz,w:1.0},
            uv:Vec2f32{x: u, y: v}
        }
    }
}

impl Rasterizable for TexturedVertex {
    fn get_position(&self) -> &Vec4f32 {
        & self.position
    }
    fn set_position(&mut self, pos: Vec4f32) {
        self.position = pos;
    }
}

impl Interpolatable for TexturedVertex {
    fn add(&self, other: &Self) -> Self {
        return TexturedVertex { position: (self.position + other.position), uv: self.uv + other.uv }
    }
    fn sub(&self, other: &Self) -> Self {
        return TexturedVertex { position: (self.position - other.position), uv: self.uv - other.uv }
    }
    fn scale(&self, scalar: f32) -> Self {
        return TexturedVertex { position: self.position * scalar, uv: self.uv * scalar }
    }
}
