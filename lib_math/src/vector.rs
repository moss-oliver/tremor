use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Index;
use std::ops::IndexMut;
use std::cmp::PartialEq;
use std::ops::Deref;
use base_num::BaseNum;
use matrix::Matrix4;

type DefaultFloatType = f32;

//trait Transformable<T> where T: BaseNum, T: Copy {
//    fn transform(&self, mat: Matrix4<T>) -> N<T>;
//}


#[derive(Debug)]
#[derive(Clone)]
pub struct Vector3<T> where T: BaseNum, T: Copy {
    data: [T;3]
}

//pub type Vector3f = Vector3<DefaultFloatType>;
//pub type Position3f = Vector3<DefaultFloatType>;

#[derive(Debug)]
#[derive(Clone)]
pub struct Vector3f (Vector3<DefaultFloatType>);

#[derive(Debug)]
#[derive(Clone)]
pub struct Position3f (Vector3<DefaultFloatType>);


#[derive(Debug)]
#[derive(Clone)]
pub struct Vector4<T> where T: BaseNum {
    data: [T;4]
}

pub type Vector4f = Vector4<DefaultFloatType>;

//Vector3
impl<T> Vector3<T> where T: BaseNum {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3::<T> {
            data: [x,y,z]
        }
    }
    
    pub fn x(&self) -> T {
        self.data[0]
    }
    pub fn y(&self) -> T {
        self.data[1]
    }
    pub fn z(&self) -> T {
        self.data[2]
    }

    pub fn add(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> {
        Vector3::<T>::new(a.x() + b.x(), a.y() + b.y(), a.z() + b.z())
    }
    pub fn sub(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> where T:Sub<Output=T> {
        Vector3::new(a.x() - b.x(), a.y() - b.y(), a.z() - b.z())
    }
    pub fn mul(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> where T:Mul<Output=T> {
        Vector3::new(a.x() * b.x(), a.y() * b.y(), a.z() * b.z())
    }
    pub fn div(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> where T:Div<Output=T> {
        Vector3::new(a.x() / b.x(), a.y() / b.y(), a.z() / b.z())
    }
}

impl<T> Copy for Vector3<T> where T: BaseNum { }

impl<T> Add for Vector3<T> where T: BaseNum {
    type Output = Vector3<T>;

    fn add(self, other: Self::Output) ->  Self::Output {
        Vector3::add(self, other)
    }
}

impl<T> Sub for Vector3<T> where T: BaseNum {
    type Output = Vector3<T>;

    fn sub(self, other: Self::Output) -> Self::Output {
        Vector3::sub(self, other)
    }
}

impl<T> Mul for Vector3<T> where T: BaseNum {
    type Output = Vector3<T>;

    fn mul(self, other: Self::Output) -> Self::Output {
        Vector3::mul(self, other)
    }
}

impl<T> Div for Vector3<T> where T: BaseNum {
    type Output = Vector3<T>;

    fn div(self, other: Self::Output) -> Self::Output {
        Vector3::div(self, other)
    }
}

impl<T> From<Vector4<T>> for Vector3<T> where T: BaseNum {
    fn from(in_value: Vector4<T>) -> Vector3<T> {
        Vector3::new(in_value.x(), in_value.y(), in_value.z())
    }
}

impl<T> Index<usize> for Vector3<T> where T: BaseNum {    
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector3<T> where T: BaseNum {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        &mut self.data[index]
    }
}

impl<T> PartialEq for Vector3<T> where T: BaseNum, T: PartialEq {
    fn eq(&self, other: &Vector3<T>) -> bool {
        return self.data == other.data;
    }
}

impl Position3f {
    pub fn new(x:DefaultFloatType, y:DefaultFloatType, z:DefaultFloatType) -> Position3f {
        Position3f(Vector3::<DefaultFloatType>::new(x,y,z))
    }
    pub fn transform(&self, mat: Matrix4<DefaultFloatType>) -> Position3f {
        Position3f(Vector3::<DefaultFloatType>::new(
            (mat[0][0] * self[0]) + (mat[1][0] * self[1]) + (mat[2][0] * self[2]) + (mat[3][0] * (1.0)),
            (mat[0][1] * self[0]) + (mat[1][1] * self[1]) + (mat[2][1] * self[2]) + (mat[3][1] * (1.0)),
            (mat[0][2] * self[0]) + (mat[1][2] * self[1]) + (mat[2][2] * self[2]) + (mat[3][2] * (1.0))
        ))
    }
    
    pub fn x(&self) -> DefaultFloatType {
        self.0.data[0]
    }
    pub fn y(&self) -> DefaultFloatType {
        self.0.data[1]
    }
    pub fn z(&self) -> DefaultFloatType {
        self.0.data[2]
    }
}

impl Deref for Position3f {
    type Target = Vector3<DefaultFloatType>;
    fn deref(&self) -> &Vector3<DefaultFloatType> { &self.0 }
}

impl Index<usize> for Position3f {    
    type Output = DefaultFloatType;
    fn index<'a>(&'a self, index: usize) -> &'a DefaultFloatType {
        &self.0.data[index]
    }
}

impl IndexMut<usize> for Position3f {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut DefaultFloatType {
        &mut self.0.data[index]
    }
}
impl From<Position3f> for Vector3<DefaultFloatType> {
    fn from(from_val: Position3f) -> Self {
        from_val.0
    }
}

impl Vector3f {
    pub fn new(x:DefaultFloatType, y:DefaultFloatType, z:DefaultFloatType) -> Vector3f {
        Vector3f(Vector3::<DefaultFloatType>::new(x,y,z))
    }

    pub fn transform(&self, mat: Matrix4<DefaultFloatType>) -> Vector3f {
        Vector3f(Vector3::<DefaultFloatType>::new(
            (mat[0][0] * self[0]) + (mat[1][0] * self[1]) + (mat[2][0] * self[2]) + (mat[3][0] * (0.0)),
            (mat[0][1] * self[0]) + (mat[1][1] * self[1]) + (mat[2][1] * self[2]) + (mat[3][1] * (0.0)),
            (mat[0][2] * self[0]) + (mat[1][2] * self[1]) + (mat[2][2] * self[2]) + (mat[3][2] * (0.0))
        ))
    }
    
    pub fn x(&self) -> DefaultFloatType {
        self.0.data[0]
    }
    pub fn y(&self) -> DefaultFloatType {
        self.0.data[1]
    }
    pub fn z(&self) -> DefaultFloatType {
        self.0.data[2]
    }
}
impl Index<usize> for Vector3f {    
    type Output = DefaultFloatType;
    fn index<'a>(&'a self, index: usize) -> &'a DefaultFloatType {
        &self.0.data[index]
    }
}

impl IndexMut<usize> for Vector3f {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut DefaultFloatType {
        &mut self.0.data[index]
    }
}
impl From<Vector3f> for Vector3<DefaultFloatType> {
    fn from(from_val: Vector3f) -> Self {
        from_val.0
    }
}


//Vector4
impl<T> Vector4<T> where T: BaseNum {
    pub fn new(x: T, y: T, z: T, w: T) -> Vector4<T> {
        return Vector4::<T> {
            data: [x,y,z,w]
        };
    }
    pub fn x(&self) -> T {
        self.data[0]
    }
    pub fn y(&self) -> T {
        self.data[1]
    }
    pub fn z(&self) -> T {
        self.data[2]
    }
    pub fn w(&self) -> T {
        self.data[3]
    }

    pub fn add(a: Vector4<T>, b: Vector4<T>) -> Vector4<T> where T: Add<Output = T> {
        Vector4::new(a.x() + b.x(), a.y() + b.y(), a.z() + b.z(), a.w() + b.w())
    }
    pub fn sub(a: Vector4<T>, b: Vector4<T>) -> Vector4<T> where T: Sub<Output = T> {
        Vector4::new(a.x() - b.x(), a.y() - b.y(), a.z() - b.z(), a.w() - b.w())
    }
    pub fn mul(a: Vector4<T>, b: Vector4<T>) -> Vector4<T> where T: Mul<Output = T> {
        Vector4::new(a.x() * b.x(), a.y() * b.y(), a.z() * b.z(), a.w() * b.w())
    }
    pub fn div(a: Vector4<T>, b: Vector4<T>) -> Vector4<T> where T: Div<Output = T> {
        Vector4::new(a.x() / b.x(), a.y() / b.y(), a.z() / b.z(), a.w() / b.w())
    }
}

impl<T> //Transformable<T> for 
Vector4<T> where T: BaseNum, T: Copy {
    pub fn transform(&self, mat: Matrix4<T>) -> Vector4<T> where T: BaseNum {
        Vector4::<T>::new(
            (mat[0][0] * self[0]) + (mat[1][0] * self[1]) + (mat[2][0] * self[2]) + (mat[3][0] * self[3]),
            (mat[0][1] * self[0]) + (mat[1][1] * self[1]) + (mat[2][1] * self[2]) + (mat[3][1] * self[3]),
            (mat[0][2] * self[0]) + (mat[1][2] * self[1]) + (mat[2][2] * self[2]) + (mat[3][2] * self[3]),
            (mat[0][3] * self[0]) + (mat[1][3] * self[1]) + (mat[2][3] * self[2]) + (mat[3][3] * self[3]),
        )
    }
}

impl<T> Copy for Vector4<T> where T: BaseNum { }

impl<T> Add for Vector4<T> where T: Add<Output = T>, T: BaseNum {
    type Output = Vector4<T>;

    fn add(self, other: Self::Output) -> Self::Output {
        Vector4::add(self, other)
    }
}

impl<T> Sub for Vector4<T> where T: Sub<Output = T>, T: BaseNum {
    type Output = Vector4<T>;

    fn sub(self, other: Self::Output) -> Self::Output {
        Vector4::sub(self, other)
    }
}

impl<T> Mul for Vector4<T> where T: Mul<Output = T>, T: BaseNum {
    type Output = Vector4<T>;

    fn mul(self, other: Self::Output) -> Self::Output {
        Vector4::mul(self, other)
    }
}

impl<T> Div for Vector4<T> where T: Div<Output = T>, T: BaseNum {
    type Output = Vector4<T>;

    fn div(self, other: Self::Output) -> Self::Output {
        Vector4::div(self, other)
    }
}

impl<T> Index<usize> for Vector4<T> where T: BaseNum {    
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector4<T> where T: BaseNum {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        &mut self.data[index]
    }
}

impl<T> PartialEq for Vector4<T> where T: BaseNum {
    fn eq(&self, other: &Vector4<T>) -> bool {
        return self.data == other.data;
    }
}
