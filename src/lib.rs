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

macro_rules! logit {
	($($args:tt)*) => {
		/*if /*cfg!(debug_assertions)*/false {
			trace!($($args)*)
		} else*/ {
			println!($($args)*)
		}
	}
}

type V = Vector2<f32>;

#[derive(Debug, Clone)]
pub enum MyEvent {
	GEvent(Event),
	MousePos(V),
}
pub use self::MyEvent::*;
pub use glium::glutin::Event::*;

pub struct Gui {
	display: Display,
	window_size: V,
	mouse_pos: V,
	pub events_from_children: Vec<MyEvent>,
}
impl Gui {
	pub fn new(/*parent: *mut c_void*/) -> Gui {
		let display = WindowBuilder::new().with_title("title".to_string()).with_dimensions(800 as u32, 600 as u32)/*.with_parent(Some(WindowID::new(parent)))*/.build_glium().unwrap();
		let (w, h) = display.get_framebuffer_dimensions();
		Gui {
			window_size: V::new(w.as_(), h.as_()),
			display: display,
			mouse_pos: zero(),
			events_from_children: vec![],
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
				Resized(_, _) => {
					let (w, h) = self.display.get_framebuffer_dimensions();
					self.window_size = V::new(w.as_(), h.as_());
				}
				MouseMoved(px, py) => {
					self.mouse_pos = V::new(px.as_(), self.window_size.y - 1. - py as f32) / self.window_size;
					events.push(MousePos(self.mouse_pos));
				}
				MouseInput(_, _) => events.push(GEvent(event)),
				/*MouseWheel(MouseScrollDelta::LineDelta(px, py), _touchphase) => {
				}
				KeyboardInput(state, scan_code, Some(key)) =>*/
				Focused(_) => events.push(GEvent(event)),
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

pub struct LayX {
	layout_weight: WeightLayout,
	layout_fract: FractLayout,
	size: V,
	//child_data: Vec<ChildData>
	mouse_x: f32,
	pub events_for_children: Vec<Vec<MyEvent>>,
	pub events_from_children: Vec<MyEvent>,
	focused: Option<usize>,
}
impl LayX {
	pub fn new() -> LayX {
		LayX {
			layout_weight: vec![],
			layout_fract: vec![],
			size: one(),
			mouse_x: zero(),
			events_for_children: vec![],
			events_from_children: vec![],
			focused: None,
		}
	}
	pub fn add(&mut self, weight: f32) -> usize {
		let id = self.layout_weight.len();
		self.layout_weight.push(weight);
		let total: f32 = self.layout_weight.iter().sum();
		self.layout_fract = self.layout_weight.iter().map(|x| x / total).fold((vec![], 0.), |(mut v, a), x| {let a = a+x; v.push(a); (v, a)}).0;
		self.events_for_children.push(vec![]);
		id
	}
	pub fn take_events_for_children(&mut self, events: Vec<MyEvent>) {
		for v in &mut self.events_for_children {
			v.clear();
		}
		for ev in events {
			match ev {
				MousePos(p) => {
					self.mouse_x = p.x;
					let focused = self.layout_fract.iter().position(|&e| p.x < e).unwrap();
					if let Some(self_focused) = self.focused {
						if focused != self_focused {
							self.events_for_children[self_focused].push(GEvent(Focused(false)));
							self.events_for_children[focused].push(GEvent(Focused(true)));
						}
					}
					self.focused = Some(focused);
					let (l, r) = (if focused == 0 {0.} else {self.layout_fract[focused-1]}, self.layout_fract[focused]);
					let mouse_x_rel = (p.x - l) / (r - l);
					let pos = V::new(mouse_x_rel, p.y);
					self.events_for_children[focused].push(MousePos(pos));
				}
				GEvent(MouseInput(_, _)) => if let Some(self_focused) = self.focused {
					self.events_for_children[self_focused].push(ev);
				},
				GEvent(Focused(false)) => if let Some(self_focused) = self.focused {
					self.events_for_children[self_focused].push(ev);
				},
				_ => (),
			}
		}

	}
}

