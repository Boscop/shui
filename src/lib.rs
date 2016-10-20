#![feature(box_syntax)]
#![allow(dead_code)] //unused_imports

#[macro_use] extern crate glium;
//pub use glium::{/*DisplayBuild,*/ Surface};
//pub use glium::backend::glutin_backend::GlutinFacade;
//use glium::glutin::*;
//use glium::backend::Facade;
//pub use glium::glutin::{/*WindowBuilder,*/ /*Window,*/ Event/*, ElementState, MouseButton*/};

extern crate nalgebra as na;
//pub use na::{Vector2, RotationTo, Norm, Dot, Rotation2, Vector1, Vector3, zero, one};

extern crate asprim;
//use asprim::AsPrim;

extern crate num;
//use num::{Float, NumCast, PrimInt, ToPrimitive};

extern crate rusttype;
extern crate unicode_normalization;
//extern crate euclid;

#[macro_export] macro_rules! logit {
	($($args:tt)*) => {
		/*if /*cfg!(debug_assertions)*/false {
			trace!($($args)*)
		} else*/ {
			println!($($args)*)
		}
	}
}

pub mod ui;

pub mod layout;
// pub use layout::*;

pub mod render;
// pub use render::*;

mod font;
// pub use font::*;

mod widget;
// pub use widget::*;

pub mod handlers;
//pub use handlers::*;

pub mod util;
//pub use util::*;

mod prelude;
