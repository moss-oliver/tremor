extern crate cgmath;
use cgmath::Point2;
use cgmath::Vector2;
use cgmath::Point3;
use cgmath::Vector3;
use cgmath::Vector4;
use cgmath::Matrix4;

type Vec2u32 = Point2<u32>;
type Vec2f32 = Vector2<f32>;
type Vec3f32 = Vector3<f32>;
type Point3f32 = Point3<f32>;
type Vec4f32 = Vector4<f32>;
type Mat4f32 = Matrix4<f32>;

pub mod buffer;
pub mod pipeline;
pub mod util;
pub mod vertex_shader;
pub mod pixel_shader;
pub mod pixel_masker;