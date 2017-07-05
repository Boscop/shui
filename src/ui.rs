use prelude::*;

use glium;
use asprim::AsPrim;

#[derive(Debug, Clone)]
pub enum MyEvent {
	GEvent(Event),
	MousePos(V),
	Focus(Option<V>),
}

pub struct Ui<'a> {
	pub display: /*&'a mut*/ GlutinFacade,
	window_size: V,
	mouse_pos: V,
	//pub events_from_children: Vec<MyEvent>,
	pub renderer: UiRenderer<'a>,
}
impl<'a> Ui<'a> {
	pub fn new(/*parent: *mut c_void*/ display: /*&'a mut*/ GlutinFacade) -> Ui<'a> {
		//let display = WindowBuilder::new().with_title("title".to_string()).with_dimensions(800 as u32, 600 as u32).with_multisampling(4).with_depth_buffer(24)/*.with_parent(Some(WindowID::new(parent)))*/.build_glium().unwrap();
		let (w, h) = display.get_framebuffer_dimensions();
		Ui {
			window_size: V::new(w.as_(), h.as_()),
			renderer: UiRenderer::new(&display),
			display: display,
			mouse_pos: zero(),
			//events_from_children: vec![],
		}
	}
	//pub fn display(&mut self) -> &GlutinFacade { &mut self.display }
	pub fn mouse_tr(&self, r: &Rect) -> V { // could be outside of [0., 1.)^2
		V::new(percentage(self.mouse_pos.x, r.pos.x, r.max_x()),
		       percentage(self.mouse_pos.y, r.pos.y, r.max_y()))
	}
	pub fn set_cursor_state(&mut self, s: glium::glutin::CursorState) {
		self.display.get_window().unwrap().set_cursor_state(s).ok().expect("could not set cursor state");
	}
	pub fn set_cursor_pos(&mut self, p: V) {
		self.display.get_window().unwrap().set_cursor_position((p.x * self.window_size.x) as i32, (self.window_size.y - 1. - (p.y * self.window_size.y) + 0.5) as i32).expect("could not set cursor position");
	}
	pub fn frame(&mut self, rect: Rect) -> Option<Vec<MyEvent>> {
		let mut events = vec![];
		for event in self.display.poll_events() {
			// logit!("event: {:?}", event);
			match event {
				Closed => {
					// logit!("window closed");
					return None;
				}
				Resized(_w, _h) => {
					let (w, h) = self.display.get_framebuffer_dimensions();
					self.window_size = V::new(w.as_(), h.as_());
				}
				MouseMoved(px, py) => {
					//println!("MouseMoved {:?}", event);
					let mouse_pos = V::new(px.as_(), self.window_size.y - 1. - py as f32) / self.window_size;
					let mouse_pos = rect.inv_rel(mouse_pos);
					if mouse_pos != self.mouse_pos {
						self.mouse_pos = mouse_pos;
						events.push(MousePos(self.mouse_pos));
					}
				}
				/*Focused(true) => events.push(Focus(Some(self.mouse_pos))),
				Focused(false) => events.push(Focus(None)),
				MouseInput(_, _) | MouseWheel(_, _) | ReceivedCharacter(_) |
				KeyboardInput(_, _, _)*/
				_ => events.push(GEvent(event)),
				//_ => logit!("unhandled event: {:?}", event)
			}
		}
		Some(events)
	}
}
