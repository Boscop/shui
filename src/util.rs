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
/*trait Vector2Ext {
	fn lperp(&self) -> V;
	fn rperp(&self) -> V;
	fn is_nan(&self) -> bool;
	fn to_tuple(&self) -> (f32, f32);
}
impl Vector2Ext for V {
	fn lperp(&self) -> V {
		V::new(-self.y, self.x)
	}
	fn rperp(&self) -> V {
		V::new(self.y, -self.x)
	}
	fn is_nan(&self) -> bool {
		self.x.is_nan() || self.y.is_nan()
	}
	fn to_tuple(&self) -> (f32, f32) {
		(self.x, self.y)
	}
}*/

pub fn percentage<T: Float + NumCast>(value: T, min: T, max: T) -> f32 {
	let v: f32 = NumCast::from(value).unwrap();
	let mn: f32 = NumCast::from(min).unwrap();
	let mx: f32 = NumCast::from(max).unwrap();
	(v - mn) / (mx - mn)
}

pub fn clamp<T: PartialOrd>(x: T, lb: T, ub: T) -> T { if x < lb { lb } else if x > ub { ub } else { x } }
