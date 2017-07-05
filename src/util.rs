use num::traits::{/*Num,*/ Float};
use num::cast::NumCast;
use na::Vector2;

pub trait VecExt {
	fn move_contents(&mut self) -> Self;
}
impl<T> VecExt for Vec<T> {
	fn move_contents(&mut self) -> Self {
		::std::mem::replace(self, Vec::new())
	}
}

pub type V = Vector2<f32>;
pub fn v(x: f32, y: f32) -> V { V::new(x, y) }

pub fn percentage<T: Float + NumCast>(value: T, min: T, max: T) -> f32 {
	let v: f32 = NumCast::from(value).unwrap();
	let mn: f32 = NumCast::from(min).unwrap();
	let mx: f32 = NumCast::from(max).unwrap();
	(v - mn) / (mx - mn)
}

pub fn clamp<T: PartialOrd>(x: T, lb: T, ub: T) -> T { if x < lb { lb } else if x > ub { ub } else { x } }
