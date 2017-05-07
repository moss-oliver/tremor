use std::ops::Mul;
use std::ops::Index;
use std::ops::IndexMut;
use base_num::BaseFloat;
use base_num::BaseNum;

extern crate num_traits;

use vector::Vector3;
use vector::Vector4;

type DefaultFloatType = f32;

#[derive(Debug)]
#[derive(Clone)]
pub struct Matrix4<T> where T: BaseNum, T: Copy {
    //Column Major - each vector is a column.
    data: [Vector4<T>;4]
}

pub type Matrix4f = Matrix4<DefaultFloatType>;

impl<T> Matrix4<T> where T: BaseNum {
    pub fn new(
            m11: T, m12: T, m13: T, m14: T,
            m21: T, m22: T, m23: T, m24: T,
            m31: T, m32: T, m33: T, m34: T,
            m41: T, m42: T, m43: T, m44: T) -> Matrix4<T> {
        Matrix4::<T> {
            data: [
                Vector4::<T>::new( m11, m21, m31, m41 ),
                Vector4::<T>::new( m12, m22, m32, m42 ),
                Vector4::<T>::new( m13, m23, m33, m43 ),
                Vector4::<T>::new( m14, m24, m34, m44 )
            ]
        }
    }

    pub fn identity() -> Matrix4<T> {
        Matrix4::<T>::new(
            T::one(),T::zero(),T::zero(),T::zero(),
            T::zero(),T::one(),T::zero(),T::zero(),
            T::zero(),T::zero(),T::one(),T::zero(),
            T::zero(),T::zero(),T::zero(),T::one())
    }

    pub fn translation<V: Into<Vector3<T>>>(vec: V) -> Matrix4<T> {
        let v = vec.into();
        Matrix4::<T>::new(
            T::one(),T::zero(),T::zero(),v.x(),
            T::zero(),T::one(),T::zero(),v.y(),
            T::zero(),T::zero(),T::one(),v.z(),
            T::zero(),T::zero(),T::zero(),T::one())
    }

    pub fn perspective_fov(fov_rad:T, aspect:T, near:T, far:T) -> Matrix4<T> where T: BaseNum, T: BaseFloat {
        let one = T::one();
        let two = one+one;
        let y_scale = one / ((fov_rad/two).tan());
        let x_scale = y_scale / aspect;
        let nearmfar = near - far;

        Matrix4::<T>::new(
            x_scale,T::zero(),T::zero(),T::zero(),
            T::zero(),y_scale,T::zero(),T::zero(),
            T::zero(),T::zero(), far / nearmfar,T::zero()-one,
            T::zero(),T::zero(),(near*far) / nearmfar,T::zero())
        
        //Matrix4::<T>::new(
        //    x_scale,T::zero(),T::zero(),T::zero(),
        //    T::zero(),y_scale,T::zero(),T::zero(),
        //    T::zero(),T::zero(),(-(far + near)) / nearmfar,T::zero()-one,
        //    T::zero(),T::zero(),(-(two*far*near)) / nearmfar,T::zero())
    }

    pub fn x(&self) -> Vector4<T> where T: Copy {
        self.data[0]
    }
    pub fn y(&self) -> Vector4<T> where T: Copy {
        self.data[1]
    }
    pub fn z(&self) -> Vector4<T> where T: Copy {
        self.data[2]
    }
    pub fn w(&self) -> Vector4<T> where T: Copy {
        self.data[3]
    }

    fn mul(a: Matrix4<T>, b: Matrix4<T>) -> Matrix4<T> {
        let mut product = Matrix4::<T>::identity();
        for col in 0..4 {
            for row in 0..4 {
                let mut val:T = T::zero();
                for inner in 0..4 {
                    val = val + (a[col][inner] * b[inner][row]);
                }
                product[col][row] = val;
            }
        }
        product
    }

}

impl<T> Copy for Matrix4<T> where T: BaseNum { }

impl<T> Index<usize> for Matrix4<T> where T: BaseNum {    
    type Output = Vector4<T>;
    fn index<'a>(&'a self, index: usize) -> &'a Vector4<T> {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Matrix4<T> where T: BaseNum {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Vector4<T> {
        &mut self.data[index]
    }
}

impl<T> Mul for Matrix4<T> where T: Mul<Output = T>, T: BaseNum {
    type Output = Matrix4<T>;

    fn mul(self, other: Matrix4<T>) -> Self::Output {
        Matrix4::<T>::mul(self,other)
    }
}

impl<T> PartialEq for Matrix4<T> where T: BaseNum, T: PartialEq {
    fn eq(&self, other: &Matrix4<T>) -> bool {
        return self.data == other.data;
    }
}


