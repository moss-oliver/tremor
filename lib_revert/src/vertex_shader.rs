use cgmath::Transform;
use cgmath::Point3;
use cgmath::EuclideanSpace;
use util::PositionVertex;
use util::ColoredVertex;
use util::Rasterizable;
use util;
use Mat4f32;
use std::marker::PhantomData;

pub trait VertexShader : Clone {
    type V : Sized;
    type R : Rasterizable;

    fn transform(&self, vertex: Self::V) -> Self::R;
}

#[derive(Clone, Copy)]
pub struct SimpleVertexShader {
}

impl VertexShader for SimpleVertexShader {
    type V = PositionVertex;
    type R = PositionVertex;

    fn transform(&self, vertex: Self::V) -> Self::R {
        return vertex;
    }
}

#[derive(Clone, Copy)]
pub struct TransformVertexShader<T> {
    pub world_proj: Mat4f32,
    _marker: PhantomData<T>
}

/*impl <T> Clone for TransformVertexShader<T> {
    fn clone(&self) -> TransformVertexShader<T> {
        TransformVertexShader {world_proj: self.world_proj, _marker:  PhantomData}
    }
}

impl <T> Copy for TransformVertexShader<T> {}*/

impl<T> TransformVertexShader<T> where T: Sized, T: Rasterizable, T: Clone {
    pub fn new(world_proj: Mat4f32) -> TransformVertexShader<T> {
        TransformVertexShader { world_proj: world_proj, _marker:  PhantomData}
    }
}

impl<T> VertexShader for TransformVertexShader<T> where T: Sized, T: Rasterizable, T: Clone {
    type V = T;
    type R = T;

    fn transform(&self, vertex: Self::V) -> Self::R {
        let pos = vertex.get_position();
        let mut v = vertex.clone();
        v.set_position(self.world_proj * pos);
        v
        //return PositionVertex {
        //    position: self.world_proj * vertex.get_position()
        //}
    }
}

#[derive(Clone, Copy)]
pub struct ColorTransformVertexShader {
    pub world_proj: Mat4f32
}

impl VertexShader for ColorTransformVertexShader {
    type V = ColoredVertex;
    type R = ColoredVertex;

    fn transform(&self, vertex: Self::V) -> Self::R {
        return ColoredVertex {
            position: self.world_proj * vertex.get_position(),
            color: vertex.color
        }
    }
}
