//use super::*;
use prelude::*;

// pub type FractLayout = Vec<f32>;
// pub type WeightLayout = Vec<f32>;

/*struct ChildData {
	fract_weight: f32,
	events: Vec<Event>,
}*/

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
	X,
	Y,
	//Z,
}

pub type ChildId = usize;

#[derive(Debug)]
pub struct Layout {
	axis: Axis,
	layout_weight: Vec<f32>, //WeightLayout,
	layout_fract: Vec<f32>, //FractLayout,
	//size: V,
	//child_data: Vec<ChildData>
	//mouse_x: f32,
	//pub events_for_children: Vec<Vec<MyEvent>>,
	//pub events_from_children: Vec<MyEvent>,
	focused: Option<ChildId>,
}
impl Layout {
	/*pub fn new(axis: Axis) -> Layout {
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
		/*match self.axis {
			Axis::X => self.layout_weight.push(weight),
			Axis::Y => self.layout_weight.insert(0, weight),
		}*/
		self.layout_weight.push(weight);
		let total: f32 = self.layout_weight.iter().sum();
		self.layout_fract = self.layout_weight.iter().map(|x| x / total).fold((vec![], 0.), |(mut v, a), x| {let a = a+x; v.push(a); (v, a)}).0;
		//self.events_for_children.push(vec![]);
		// println!("{:?}", self);
		id
	}*/
	pub fn new(axis: Axis, mut layout_weight: Vec<f32>) -> Layout {
		if axis == Axis::Y { layout_weight.reverse(); }
		let total: f32 = layout_weight.iter().sum();
		let layout_fract = layout_weight.iter().map(|x| x / total).fold((vec![], 0.), |(mut v, a), x| {let a = a+x; v.push(a); (v, a)}).0;
		Layout {
			axis: axis,
			layout_weight: layout_weight,
			layout_fract: layout_fract,
			focused: None,
		}
	}
	pub fn events_for_children(&mut self, rect: Rect, events: Vec<MyEvent>) -> Vec<(Rect, Vec<MyEvent>)> {
		let mut events_for_children = vec![vec![]; self.child_count()];
		for ev in events {
			match ev {
				MousePos(p) => {
					let focused = self.child_at(p);
					let self_focused = self.focused;
					self.focused = Some(focused);
					// logit!("set focused {:?} {:?} {:?}", self.axis, p, self.focused);
					let p = self.pos_in_focused_child_rect(p);
					if let Some(self_focused) = self_focused {
						if focused != self_focused {
							events_for_children[self_focused].push(Focus(None));
							events_for_children[focused].push(Focus(Some(p)));
						}
					}
					events_for_children[focused].push(MousePos(p));
				}
				Focus(None) => {
					// logit!("{:?} {:?}", self.axis, ev);
					if let Some(self_focused) = self.focused {
						events_for_children[self_focused].push(ev);
					}
					self.focused = None;
					// logit!("{:?} {:?}", self.axis, self.focused);
				}
				Focus(Some(p)) => {
					// logit!("{:?} {:?} {:?} {:?}", self.axis, ev, p, self.focused);
					// assert!(self.focused.is_none());
					// logit!("---");
					let focused = self.child_at(p);
					self.focused = Some(focused);
					let p = self.pos_in_focused_child_rect(p);
					events_for_children[focused].push(Focus(Some(p)));
				}
				/*GEvent(KeyboardInput(_, _, _)) |
				GEvent(MouseInput(_, _))*/
				_ => if let Some(self_focused) = self.focused {
					events_for_children[self_focused].push(ev);
				},
			}
		}
		events_for_children.into_iter().enumerate().map(|(i, e)| (self.child_rect(&rect, i), e)).collect()
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
	#[inline(always)]
	pub fn child_count(&self) -> usize {
		self.layout_fract.len()
	}
	fn child_boundaries(&self, id: ChildId) -> (f32, f32) {
		(if id == 0 {0.} else {self.layout_fract[id-1]}, self.layout_fract[id])
	}
	pub fn child_rect(&self, r: &Rect, id: ChildId) -> Rect {
		let (a, b) = self.child_boundaries(id);
		let d = b - a;
		match self.axis {
			//Axis::X => Rect::new(V::new(a, r.pos.y), V::new(d, r.size.y)),
			Axis::X => Rect::new(V::new(r.pos.x + r.size.x * a, r.pos.y), V::new(r.size.x * d, r.size.y)),
			//Axis::Y => Rect::new(V::new(r.pos.x, a), V::new(r.size.x, d)),
			Axis::Y => Rect::new(V::new(r.pos.x, r.pos.y + r.size.y * a), V::new(r.size.x, r.size.y * d)),
		}
	}
	pub fn process<F: FnMut((ChildId, (Rect, Vec<MyEvent>))) -> Vec<MyEvent>>(&mut self, rect: Rect, events: Vec<MyEvent>, f: F) -> Vec<MyEvent> {
		match self.axis {
			Axis::X => self.events_for_children(rect, events).into_iter().enumerate().flat_map(f).collect::<Vec<_>>(),
			Axis::Y => self.events_for_children(rect, events).into_iter().rev().enumerate().flat_map(f).collect::<Vec<_>>()
		}
	}
}

pub type EventHandler0D = FnMut(&mut Ui, Rect, Vec<MyEvent>) -> Vec<MyEvent>;
pub type EventHandler1D = FnMut(&mut Ui, Rect, Vec<MyEvent>, /* row or col */ ChildId) -> Vec<MyEvent>;
pub type EventHandler2D = FnMut(&mut Ui, Rect, Vec<MyEvent>, /* row */ ChildId, /* col */ ChildId) -> Vec<MyEvent>;

pub enum Lay<'a> {
	X(Vec<(f32, Lay<'a>)>),
	Y(Vec<(f32, Lay<'a>)>),
	Single(&'a mut EventHandler0D),
	XMulti(Vec<f32>, &'a mut EventHandler1D),
	XMultiEq(usize,  &'a mut EventHandler1D), // equally spaced
	YMulti(Vec<f32>, &'a mut EventHandler1D),
	YMultiEq(usize,  &'a mut EventHandler1D), // equally spaced
	XYMultiEq(/* rows: */ usize, /* cols: */ usize, &'a mut EventHandler2D), // equally spaced
	Empty,
}
impl<'a> Lay<'a> {
	//fn finish(&'a mut self) -> Box<FnMut(&mut Ui, Rect, Vec<MyEvent>, ChildId) -> Vec<MyEvent> + 'a> {
	fn finish<'b>(&'b mut self) -> Box<FnMut(&mut Ui, Rect, Vec<MyEvent>, ChildId) -> Vec<MyEvent> + 'b> {
		use self::Lay::*;
		match self {
			&mut X(ref mut v) => {
				let (weights, children): (Vec<f32>, Vec<&'b mut Lay<'a>>) = v.iter_mut().map(|&mut (w, ref mut c)| (w, c)).unzip();
				//: Vec<Box<FnMut(&mut Ui, Rect, Vec<MyEvent>, ChildId) -> Vec<MyEvent> + 'b>>
				let mut child_handlers = children.into_iter().map(|c| c.finish()).collect::<Vec<_>>();
				let mut parent = Layout::new(Axis::X, weights);
				box move |ui, rect, events, _| {
					parent.events_for_children(rect, events).into_iter().enumerate().zip(child_handlers.iter_mut()).flat_map(|((child_id, (rect, events)), handler)| {
						handler(ui, /*parent.child_rect(&rect, child_id)*/ rect, events, child_id)
					}).collect()
				}
			}
			&mut Y(ref mut v) => {
				let (weights, children): (Vec<f32>, Vec<&'b mut Lay<'a>>) = v.iter_mut()/*.rev()*/.map(|&mut (w, ref mut c)| (w, c)).unzip();
				let mut child_handlers = children.into_iter().map(|c| c.finish()).collect::<Vec<_>>();
				let mut parent = Layout::new(Axis::Y, weights);
				box move |ui, rect, events, _| {
					parent.events_for_children(rect, events).into_iter().rev().enumerate().zip(child_handlers.iter_mut()).flat_map(|((child_id, (rect, events)), handler)| {
						handler(ui, /*parent.child_rect(&rect, child_id)*/ rect, events, child_id)
					}).collect()
				}
			}
			&mut Single(ref mut handler) => box move |ui, rect, events, _| {
				handler(ui, rect.pad(PADDING), events)
			},
			&mut XMulti(ref weights, ref mut handler) => {
				let mut parent = Layout::new(Axis::X, weights.clone());
				box move |ui, rect, events, _| {
					parent.events_for_children(rect, events).into_iter().enumerate().flat_map(|(child_id, (rect, events))| {
						handler(ui, /*parent.child_rect(&rect, child_id).pad(PADDING)*/ rect, events, child_id)
					}).collect()
				}
			}
			&mut XMultiEq(n, ref mut handler) => {
				let mut parent = Layout::new(Axis::X, vec![1.; n]);
				box move |ui, rect, events, _| {
					parent.events_for_children(rect, events).into_iter().enumerate().flat_map(|(child_id, (rect, events))| {
						handler(ui, /*parent.child_rect(&rect, child_id).pad(PADDING)*/ rect, events, child_id)
					}).collect()
				}
			}
			&mut YMulti(ref weights, ref mut handler) => {
				let mut parent = Layout::new(Axis::Y, weights.clone());
				box move |ui, rect, events, _| {
					parent.events_for_children(rect, events).into_iter().rev().enumerate().flat_map(|(child_id, (rect, events))| {
						handler(ui, /*parent.child_rect(&rect, child_id).pad(PADDING)*/ rect, events, child_id)
					}).collect()
				}
			}
			&mut YMultiEq(n, ref mut handler) => {
				let mut parent = Layout::new(Axis::Y, vec![1.; n]);
				box move |ui, rect, events, _| {
					parent.events_for_children(rect, events).into_iter().rev().enumerate().flat_map(|(child_id, (rect, events))| {
						handler(ui, /*parent.child_rect(&rect, child_id).pad(PADDING)*/ rect, events, child_id)
					}).collect()
				}
			}
			&mut XYMultiEq(rows, cols, ref mut handler) => {
				let mut parent_y = Layout::new(Axis::Y, vec![1.; rows]);
				let mut parents_x = (0..rows).map(|_| Layout::new(Axis::X, vec![1.; cols])).collect::<Vec<_>>();
				box move |ui, rect, events, _| {
					parent_y.events_for_children(rect, events).into_iter().rev().enumerate().flat_map(|(row, (rect, events))| {
						// let row_rect = parent_y.child_rect(&rect, row);
						// let parent = &mut parents_x[row];
						/*parent*/parents_x[row].events_for_children(rect, events).into_iter().enumerate().flat_map(|(col, (rect, events))| {
							handler(ui, /*parent.child_rect(&row_rect, col).pad(PADDING)*/ rect, events, row, col)
						}).collect::<Vec<_>>()
					}).collect()
				}
			}
			&mut Empty => box move |_, _, events, _| events,
		}
	}
	/*pub fn event_handler(&'a mut self) -> Box<FnMut(&mut Ui, Vec<MyEvent>) -> Vec<MyEvent> + 'a> {
		let mut handler = self.finish();
		box move |ui, events| {
 			handler(ui, Rect::new(one(), one()), events, 0)
 		}
 	}*/
 	pub fn build_full_window(&'a mut self) -> Box<FnMut(&mut Ui) -> Option<Vec<MyEvent>> + 'a> {
 		self.build(Rect::new(zero(), one()))
 	}
 	pub fn build(&'a mut self, rect: Rect) -> Box<FnMut(&mut Ui) -> Option<Vec<MyEvent>> + 'a> {
		let mut handler = self.finish();
		box move |ui| {
 			ui.frame(rect).map(|events| handler(ui, Rect::new(zero(), one()), events, 0))
 		}
 	}
	/*pub fn build<'b: 'a>(&'a mut self, ui: &'b mut Ui<'b>) -> Box<FnMut() -> Option<Vec<MyEvent>> + 'a> {
		let mut handler = self.finish();
		box move || {
			ui.frame().map(|events| handler(ui, Rect::new(one(), one()), events, 0))
 		}
 	}*/
}
const PADDING: f32 = 0.; //0.02; // TODO: translate MousePos events into child rect when PADDING != 0.

//let (weights, children): (Vec<f32>, Vec<&'a GuiLay<'a>>) = v.iter().map(|&(w, ref c)| (w, c)).unzip();
/*pub enum Lay<'a, T> { // TODO: optional handler for leftover events
	X(Vec<(f32, T)>),
	Y(Vec<(f32, T)>),
	Single(&'a mut FnMut(Rect, Vec<MyEvent>) -> Vec<MyEvent>),
	XMulti(Vec<f32>, &'a mut FnMut(Rect, Vec<MyEvent>, ChildId) -> Vec<MyEvent>),
	YMulti(Vec<f32>, &'a mut FnMut(Rect, Vec<MyEvent>, ChildId) -> Vec<MyEvent>),
	XYMulti(Vec<f32>, &'a mut FnMut(Rect, Vec<MyEvent>, ChildId, ChildId) -> Vec<MyEvent>),
	Empty,
}

struct LayDef<'a>(Lay<'a, LayDef<'a>>);
impl<'a> LayDef<'a> {
	pub fn finish(self) -> GuiLay<'a> {
		use self::Lay::*;
		match self.0 {
			X(v) => GuiLay { focused: None, lay: X(v.into_iter().map(|(weight, lay)| (weight, lay.finish())).collect()) },
			Y(v) => GuiLay { focused: None, lay: Y(v.into_iter().map(|(weight, lay)| (weight, lay.finish())).collect()) },
			Single(f) => GuiLay { focused: None, lay: Single(f) },
			XMulti(v, f) => GuiLay { focused: None, lay: XMulti(v, f) },
			YMulti(v, f) => GuiLay { focused: None, lay: YMulti(v, f) },
			XYMulti(v, f) => GuiLay { focused: None, lay: XYMulti(v, f) },
			Empty => GuiLay { focused: None, lay: Empty },
		}
	}
}

struct GuiLay<'a> {
	focused: Option<ChildId>,
	lay: Lay<'a, GuiLay<'a>>,
}
impl<'a> GuiLay<'a> {
	pub fn process_events(&'a mut self, rect: Rect, events: Vec<MyEvent>) -> Vec<MyEvent> {
		use self::Lay::*;
		fn child_handler<'a>(/*focused: Option<ChildId>,*/ /*parent: Layout,*/ lay: &GuiLay<'a>) -> Box<FnMut(Rect, Vec<MyEvent>, ChildId) -> Vec<MyEvent>> {
			match lay.lay {
				//X(ref v) => v.iter().map(|&(_, ref c)| c).map(|c| child_handler(&c)).collect(),
				X(ref v) => {
					let (weights, children): (Vec<f32>, Vec<&'a GuiLay<'a>>) = v.iter().map(|&(w, ref c)| (w, c)).unzip();
					let mut parent = Layout::new(Axis::X, weights);
					parent.focused = self.focused;
					let mut handler = |rect: Rect, events: Vec<MyEvent>, child_id: ChildId| -> Vec<MyEvent> {
						parent.events_for_children(events).into_iter().enumerate().flat_map(|(child_id, events)| {
							let rect = parent.child_rect(&rect, child_id);
							child_handler()
						}.collect()
					}
				}
				_ => unimplemented!(),
			}
		}
		match self.lay {
			X(ref v) => {
				let (weights, children): (Vec<f32>, Vec<&'a GuiLay<'a>>) = v.iter().map(|&(w, ref c)| (w, c)).unzip();
				let mut parent = Layout::new(Axis::X, weights);
				parent.focused = self.focused;
				parent.events_for_children(events).into_iter().enumerate().zip(children.iter().map(|c| child_handler(parent, *c))).flat_map(|((child_id, events), handler)| handler(rect, events, child_id)).collect()
			}
			_ => unimplemented!(),
		}
	}
}*/

#[derive(Debug, Copy, Clone)]
pub struct Rect {
	pub pos: V,
	pub size: V,
}
impl Rect {
	pub fn new(pos: V, size: V) -> Rect {
		Rect {pos: pos, size: size}
	}
	pub fn max_x(&self) -> f32 { self.pos.x + self.size.x }
	pub fn max_y(&self) -> f32 { self.pos.y + self.size.y }
	pub fn center(&self) -> V {
		self.pos + self.size / 2.
	}
	pub fn rel(&self, p: V) -> V {
		self.pos + self.size * p
	}
	pub fn rel_rect(&self, r: Rect) -> Rect {
		Rect {
			pos: self.rel(r.pos),
			size: self.size * r.size,
		}
	}
	pub fn inv_rel(&self, p: V) -> V {
		(p - self.pos) / self.size
	}
	pub fn pad(&self, s: f32) -> Rect {
		Rect {pos: self.pos + self.size * s, size: self.size * (1. - 2. * s)}
	}
}
