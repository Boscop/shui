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
		let display = WindowBuilder::new().with_title("title".to_string()).with_dimensions(800 as u32, 600 as u32)/*.with_parent(Some(WindowID::new(parent)))*/.build_glium().unwrap();
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

/*pub struct WeightLayout {
	partition: Vec<f32>,
}
impl WeightLayout {
	pub fn new(partition: Vec<f32>) -> WeightLayout {
		WeightLayout {
			partition: partition,
		}
	}
	pub fn fract(&self) -> FractLayout {
		let total: f32 = self.partition.iter().sum();
		self.partition.iter().map(|x| x / total).fold((vec![], 0.), |(mut v, a), x| {let a = a+x; v.push(a); (v, a)}).0
	}
}*/

pub type FractLayout = Vec<f32>;
pub type WeightLayout = Vec<f32>;

/*struct ChildData {
	fract_weight: f32,
	events: Vec<Event>,
}*/

#[derive(Debug, Copy, Clone)]
pub enum Axis {
	X,
	Y,
	//Z,
}

pub type ChildId = usize;

#[derive(Debug)]
pub struct Layout {
	axis: Axis,
	layout_weight: WeightLayout,
	layout_fract: FractLayout,
	//size: V,
	//child_data: Vec<ChildData>
	//mouse_x: f32,
	//pub events_for_children: Vec<Vec<MyEvent>>,
	//pub events_from_children: Vec<MyEvent>,
	focused: Option<ChildId>,
}
impl Layout {
	pub fn new(axis: Axis) -> Layout {
		Layout {
			axis: axis,
			layout_weight: vec![],
			layout_fract: vec![],
			//size: one(),
			//mouse_x: zero(),
			//events_for_children: vec![],
			//events_from_children: vec![],
			focused: None,
		}
	}
	pub fn add(&mut self, weight: f32) -> ChildId {
		assert!(weight > 0.);
		let id = self.layout_weight.len();
		self.layout_weight.push(weight);
		let total: f32 = self.layout_weight.iter().sum();
		self.layout_fract = self.layout_weight.iter().map(|x| x / total).fold((vec![], 0.), |(mut v, a), x| {let a = a+x; v.push(a); (v, a)}).0;
		//self.events_for_children.push(vec![]);
		println!("{:?}", self);
		id
	}
	pub fn events_for_children(&mut self, events: Vec<MyEvent>) -> Vec<Vec<MyEvent>> {
		let mut events_for_children = vec![vec![]; self.child_count()];
		for ev in events {
			match ev {
				MousePos(p) => {
					let focused = self.child_at(p);
					let self_focused = self.focused;
					self.focused = Some(focused);
					let p = self.pos_in_focused_child_area(p);
					if let Some(self_focused) = self_focused {
						if focused != self_focused {
							events_for_children[self_focused].push(Focus(None));
							events_for_children[focused].push(Focus(Some(p)));
						}
					}
					events_for_children[focused].push(MousePos(p));
				}
				GEvent(KeyboardInput(_, _, _)) |
				GEvent(MouseInput(_, _)) => if let Some(self_focused) = self.focused {
					events_for_children[self_focused].push(ev);
				},
				Focus(None) => {
					if let Some(self_focused) = self.focused {
						events_for_children[self_focused].push(ev);
					}
					self.focused = None;
				}
				Focus(Some(p)) => {
					//println!("{:?}", self.axis);
					assert!(self.focused.is_none());
					let focused = self.child_at(p);
					self.focused = Some(focused);
					let p = self.pos_in_focused_child_area(p);
					events_for_children[focused].push(Focus(Some(p)));
				}
				_ => ()
			}
		}
		events_for_children
	}
	fn child_at(&self, p: V) -> ChildId {
		let pos = match self.axis {
			Axis::X => p.x,
			Axis::Y => p.y,
		};
		self.layout_fract.iter().position(|&e| pos < e).unwrap()
	}
	pub fn pos_in_focused_child_area(&self, p: V) -> V {
		let (a, b) = self.child_boundaries(self.focused.unwrap());
		match self.axis {
			Axis::X => V::new(percentage(p.x, a, b), p.y),
			Axis::Y => V::new(p.x, percentage(p.y, a, b)),
		}
	}
	fn child_count(&self) -> usize {
		self.layout_fract.len()
	}
	fn child_boundaries(&self, id: ChildId) -> (f32, f32) {
		(if id == 0 {0.} else {self.layout_fract[id-1]}, self.layout_fract[id])
	}
	pub fn child_size(&self, size: V, id: ChildId) -> V {
		let (a, b) = self.child_boundaries(id);
		let d = b - a;
		match self.axis {
			Axis::X => V::new(d, size.y),
			Axis::Y => V::new(size.x, d),
		}
	}
}

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