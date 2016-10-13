use std::thread;
use std::time::Duration;

extern crate shui;
use shui::*;

#[macro_use] extern crate glium;
use glium::glutin::{ElementState, MouseButton};


fn main() {
	let mut gui = Gui::new();
	//let layout = Layout::new(vec![1., 1., 2.]); // 1/4, 1/4, 1/2
	//let fract = layout.fract();
	let mut lay_x = LayX::new();
	let button0 = lay_x.add(1.);
	let button1 = lay_x.add(1.);
	let button2 = lay_x.add(2.);
	let mut total_button_presses = 0;
	while let Some(events) = gui.frame() {
		thread::sleep(Duration::from_millis(20));
		lay_x.take_events_for_children(events);
		for &button_id in &[button0, button1, button2] {
			lay_x.events_from_children.extend(lay_x.events_for_children[button_id].drain(..).filter(|ev| {
				!match *ev {
					GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
						println!("button{} clicked!", button_id);
						total_button_presses += 1;
						true
					}
					GEvent(Focused(focused)) => {
						println!("button{} focus: {}", button_id, focused);
						true
					}
					_ => false
				}
			}));
		}
		gui.events_from_children.extend(lay_x.events_from_children.drain(..).filter(|ev| {
			!match *ev {
				MousePos(p) => {
					println!("mouse pos {:?}", p);
					true
				}
				GEvent(Focused(focused)) => {
					println!("lay_x focus: {}", focused);
					true
				}
				_ => false
			}
		}));
		for ev in gui.events_from_children.drain(..) {
			println!("unused event: {:?}", ev);
		}
	}
}
