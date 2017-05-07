use util::Rasterizable;
use util::Color;
use util::PositionVertex;
use util::ColoredVertex;
use util::TexturedVertex;
use buffer::Buffer;

pub trait PixelShader : Clone {
    type R: Rasterizable;
    type P: Sized;

    fn shade(&self, vertex: Self::R) -> Self::P;
}

#[derive(Clone, Copy)]
pub struct SolidColorPixelShader {
    pub color: Color
}

impl PixelShader for SolidColorPixelShader {
    type R = PositionVertex;
    type P = Color;

    fn shade(&self, vertex: Self::R) -> Self::P {
        return self.color.clone();
    }
}

#[derive(Clone, Copy)]
pub struct ColorPixelShader {
}

impl PixelShader for ColorPixelShader {
    type R = ColoredVertex;
    type P = Color;

    fn shade(&self, vertex: Self::R) -> Self::P {
        return vertex.color;
        //let dist = 1.0 - vertex.get_position().z / vertex.get_position().w;
        //return Color::new(  (vertex.color.r as f32 * (dist * 0.01) ) as u8,
        //                    (vertex.color.g as f32 * (dist * 0.01) ) as u8,
        //                    (vertex.color.b as f32 * (dist * 0.01) ) as u8
        //);
        
        //return Color::new( vertex.get_position().z as u8,vertex.get_position().z as u8,vertex.get_position().z as u8);
    }
}

#[derive(Clone, Copy)]
pub struct TexturePixelShader<T> where T: Buffer+Clone {
    pub texture:T
}

impl<T> PixelShader for TexturePixelShader<T> where T: Buffer+Clone {
    type R = TexturedVertex;
    type P = Color;//T::T; //Pixel type

    fn shade(&self, vertex: Self::R) -> Self::P {
        return Color::new((vertex.uv.x) as u8, (vertex.uv.y) as u8, 0);
        //return self.texture.get_pixel_by_coords(vertex.uv.x,vertex.uv.y);
        //return self.texture.get_pixel(vertex.uv.x as u32,vertex.uv.y as u32);
    }
}
