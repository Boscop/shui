#![allow(unused_imports)]

#[macro_use] extern crate glium;
use glium::{DisplayBuild, Surface, Display};
//use glium::glutin::*;
use glium::backend::Facade;
use glium::glutin::{WindowBuilder, Event, ElementState, MouseButton};

extern crate nalgebra as na;
use na::{Vector2, RotationTo, Norm, Dot, Rotation2, Vector1, Vector3, zero, one};

extern crate asprim;
use asprim::AsPrim;

extern crate num;
use num::{Float, NumCast, PrimInt, ToPrimitive};

mod layout;
pub use layout::*;

mod widget;
pub use widget::*;

macro_rules! logit {
	($($args:tt)*) => {
		/*if /*cfg!(debug_assertions)*/false {
			trace!($($args)*)
		} else*/ {
			println!($($args)*)
		}
	}
}

pub type V = Vector2<f32>;

#[derive(Debug, Clone)]
pub enum MyEvent {
	GEvent(Event),
	MousePos(V),
	Focus(Option<V>),
}
pub use self::MyEvent::*;
pub use glium::glutin::Event::*;

pub struct Gui {
	display: Display,
	window_size: V,
	mouse_pos: V,
	//pub events_from_children: Vec<MyEvent>,
}
impl Gui {
	pub fn new(/*parent: *mut c_void*/) -> Gui {
		let display = WindowBuilder::new().with_title("title".to_string()).with_dimensions(800 as u32, 600 as u32).with_depth_buffer(24)/*.with_parent(Some(WindowID::new(parent)))*/.build_glium().unwrap();
		let (w, h) = display.get_framebuffer_dimensions();
		Gui {
			window_size: V::new(w.as_(), h.as_()),
			display: display,
			mouse_pos: zero(),
			//events_from_children: vec![],
		}
	}
	pub fn frame(&mut self) -> Option<Vec<MyEvent>> {
		let mut events = vec![];
		for event in self.display.poll_events() {
			logit!("event: {:?}", event);
			match event {
				Closed => {
					logit!("window closed");
					return None;
				}
				Resized(_w, _h) => {
					let (w, h) = self.display.get_framebuffer_dimensions();
					self.window_size = V::new(w.as_(), h.as_());
				}
				MouseMoved(px, py) => {
					self.mouse_pos = V::new(px.as_(), self.window_size.y - 1. - py as f32) / self.window_size;
					events.push(MousePos(self.mouse_pos));
				}
				MouseInput(_x, _y) => events.push(GEvent(event)),
				/*MouseWheel(MouseScrollDelta::LineDelta(px, py), _touchphase) => {
				}*/
				KeyboardInput(_state, _scan_code, _maybe_key) => events.push(GEvent(event)),
				Focused(true) => events.push(Focus(Some(self.mouse_pos))),
				Focused(false) => events.push(Focus(None)),
				_ => ()
			}
		}
		Some(events)
	}
}

// utils

pub fn percentage<T: Float + NumCast>(value: T, min: T, max: T) -> f32 {
	let v: f32 = NumCast::from(value).unwrap();
	let mn: f32 = NumCast::from(min).unwrap();
	let mx: f32 = NumCast::from(max).unwrap();
	(v - mn) / (mx - mn)
}

/*
let v = vec![1,2,3];
assert_eq!(v.clone(), [
	Box::new(|e: Vec<usize>| Box::new(e.into_iter().flat_map(|x|vec![x])) as Box<Iterator<Item=usize>>)
	as Box<FnMut(Vec<usize>) -> Box<Iterator<Item=usize>>>,
	Box::new(|e: Vec<usize>| Box::new(e.into_iter().flat_map(|x|vec![x])) as Box<Iterator<Item=usize>>)
	as Box<FnMut(Vec<usize>) -> Box<Iterator<Item=usize>>>,
][1](v).collect::<Vec<_>>());

({
	let v = vec![1,2,3];
	assert_eq!(v, [
		&mut (
			|e: Vec<usize>| e.into_iter().flat_map(|x| vec![x]).collect::<Vec<_>>()
		) as &mut FnMut(Vec<usize>) -> Vec<usize>,
		&mut (
			|e: Vec<usize>| e.into_iter().flat_map(|x| vec![x]).collect::<Vec<_>>()
		) as &mut FnMut(Vec<usize>) -> Vec<usize>,
	][1](vec![1,2,3]));
});
*/