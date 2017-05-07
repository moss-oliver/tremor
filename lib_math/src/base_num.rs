extern crate num_traits;

use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::ops::RemAssign;

use std::fmt::Debug;
use self::num_traits::Num;
use self::num_traits::NumCast;
pub trait BaseNum where
    Self: Copy + Clone + Debug,
    Self: Num + NumCast,
    Self: AddAssign + SubAssign,
    Self: MulAssign + DivAssign + RemAssign,
{}

impl BaseNum for f32 {}


pub trait BaseFloat where
    Self: BaseNum,
    Self: self::num_traits::Float,
{}

impl BaseFloat for f32 {}
