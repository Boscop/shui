use super::*;

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
					let p = self.pos_in_focused_child_rect(p);
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
					assert!(self.focused.is_none());
					let focused = self.child_at(p);
					self.focused = Some(focused);
					let p = self.pos_in_focused_child_rect(p);
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
	pub fn pos_in_focused_child_rect(&self, p: V) -> V {
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
	pub fn child_rect(&self, rect: Rect, id: ChildId) -> Rect {
		let (a, b) = self.child_boundaries(id);
		let d = b - a;
		match self.axis {
			Axis::X => Rect::new(V::new(a, rect.pos.y), V::new(d, rect.size.y)),
			Axis::Y => Rect::new(V::new(rect.pos.x, a), V::new(rect.size.x, d)),
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
	pos: V,
	size: V,
}
impl Rect {
	pub fn new(pos: V, size: V) -> Rect {
		Rect {pos: pos, size: size}
	}
}
