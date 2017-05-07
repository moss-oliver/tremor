use util::Rasterizable;
use util::PositionVertex;
use std::marker::PhantomData;
use util::None;

pub trait PixelMasker : Clone {
    type V : Rasterizable;
    type P : Sized;

    fn test(&self, vert: &Self::V, current: Self::P) -> Option<Self::P>;
}

#[derive(Clone)]
pub struct NonePixelMasker<T> where T:Rasterizable+Clone {
    phantom: PhantomData<T>,
}

impl<T> NonePixelMasker<T> where T:Rasterizable+Clone {
    pub fn new() -> NonePixelMasker<T> {
        return NonePixelMasker {phantom: PhantomData}
    }
}

impl<T> PixelMasker for NonePixelMasker<T> where T:Rasterizable+Clone {
    type V = T;
    type P = None;

    fn test(&self, vert: &Self::V, current: Self::P) -> Option<Self::P> {
        return Option::Some(current);
    }
}

#[derive(Clone)]
pub struct DepthPixelMasker<T> where T:Rasterizable+Clone {
    phantom: PhantomData<T>,
}

impl<T> DepthPixelMasker<T> where T:Rasterizable+Clone {
    pub fn new() -> DepthPixelMasker<T> {
        return DepthPixelMasker {phantom: PhantomData}
    }
}

impl<T> PixelMasker for DepthPixelMasker<T> where T:Rasterizable+Clone {
    type V = T;
    type P = f32;

    fn test(&self, vert: &Self::V, current: Self::P) -> Option<Self::P> {
        //return Option::Some(current);
        let vert_pos = vert.get_position().truncate() * (1.0 /vert.get_position().w.abs());
        if vert_pos.z < current {
            return Option::Some(vert_pos.z);
        }
        return Option::None;
    }
}
