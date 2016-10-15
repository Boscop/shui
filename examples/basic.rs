#![allow(unused_imports)]
#![feature(box_syntax)]

use std::thread;
use std::time::Duration;
use std::cell::RefCell;

extern crate shui;
use shui::*;

#[macro_use] extern crate glium;
use glium::glutin::{ElementState, MouseButton, VirtualKeyCode};
use glium::Surface;

extern crate nalgebra as na;
use na::{Vector2, RotationTo, Norm, Dot, Rotation2, Vector1, Vector3, zero, one};

fn main() {
	let mut gui = Gui::new();
	let mut lay_y = Layout::new(Axis::Y); //, 0);
	let mut _lay_x0_id = lay_y.add(1.);
	let mut _lay_x1_id = lay_y.add(1.);
	let mut lay_x0 = Layout::new(Axis::X); //, lay_y.add(1.));
	let mut lay_x1 = Layout::new(Axis::X); //, lay_y.add(1.));
	let _button00 = lay_x0.add(1.);
	let _button01 = lay_x0.add(1.);
	let _button02 = lay_x0.add(2.);
	let _button10 = lay_x1.add(1.);
	let _button11 = lay_x1.add(1.);
	let mut total_button_presses = 0;
	let cell = RefCell::new(&mut total_button_presses);
	let mut knobs = [
		vec![
			KnobGuiState { val: 0.0, twist: None, },
			KnobGuiState { val: 0.5, twist: None, },
			KnobGuiState { val: 1.0, twist: None, },
		],
		vec![
			KnobGuiState { val: 0.0, twist: None, },
			KnobGuiState { val: 0.5, twist: None, },
		],
	];
	//let knob_renderer = KnobRenderer::new(gui.display());
	let mut gui_renderer = GuiRenderer::new(gui.display());
	/*let mut font = MyFont::new(&gui.display(),
		//r"D:\projects\MadBoys9000\FlowStudio_data\fonts\Bank Gothic Light.ttf"
		r"D:\projects\MadBoys9000\FlowStudio_data\fonts\BankGothic Bold.ttf"
		//r"D:\projects\MadBoys9000\FlowStudio_data\fonts\Arial Unicode.ttf"
	).unwrap();*/
	'out: while let Some(events) = gui.frame() {
        let mut target = gui.display().draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
		let parent_rect = Rect::new(one(), one());
		thread::sleep(Duration::from_millis(20));
		//let mut knob_queue = vec![];
		let leftover_events = lay_y.events_for_children(events).into_iter().enumerate().flat_map(|(lay_x_id, events)| {
			//let lay_x0 = &mut lay_x0;
			let parent_rect = lay_y.child_rect(&parent_rect, lay_x_id);
			/*let r = [
				//(&mut |events| {
				(box |events| {*/
					//let lay_x0 = &mut *lay_x0;
					//lay_x0
					let lay_xx = &mut [&mut lay_x0, &mut lay_x1][lay_x_id];
					lay_xx.events_for_children(events).into_iter().enumerate().flat_map(|(knob_id, events)| {
						let parent_rect = lay_xx.child_rect(&parent_rect, knob_id);
						let knob = &mut knobs[lay_x_id][knob_id];
						//knob_queue.push(RenderKnob {rect: parent_rect, val: knob.val});
						gui_renderer.queue_knob(parent_rect, knob);
						events.into_iter().filter(|ev| {
							!match *ev {
								GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
									// println!("button0{} clicked!", knob_id);
									**cell.borrow_mut() += 1;
									println!("total_button_presses {}", **cell.borrow_mut());
									println!("rect {:?} {}", parent_rect, parent_rect.center());

									//let knob_pos = parent_rect.center();
									let knob_pos = V::new(0.5, 0.5);
									let mouse_pos = gui.mouse_tr(&parent_rect);
									const KNOB_RADIUS: f32 = 0.5;
									let d = (mouse_pos - knob_pos).norm();
									// println!("! {:?}", d);
									if d < KNOB_RADIUS {
										knob.twist = Some((mouse_pos, knob.val));
										//gui.display.get_window().unwrap().set_cursor_state(glium::glutin::CursorState::Hide).ok().expect("could not set cursor state");
										gui.set_cursor_state(glium::glutin::CursorState::Hide);
									}
									// println!("!!! mouse1 {:?}", mouse_pos);

									true
								}
								MousePos(p) => {
									knob.twist = knob.twist.and_then(|(press_pos, orig_val)| {
										let d = p - press_pos;
										// println!("!!! d {:?}", d);
										let val = clamp(orig_val + d.y * 2., 0., 1.);
										knob.val = val;
										//self.writable.set_param_norm(Alpha, val);
										//self.writable.host.automate(Alpha as i32, val);
										//gui.display().get_window().unwrap().set_cursor_position(press_pos.x as i32, WINDOW_HEIGHT - press_pos.y as i32).expect("could not set cursor position");
										gui.set_cursor_pos(parent_rect.pos + parent_rect.size * press_pos);
										Some((press_pos, val))
									});
									// println!("!!! mouse2 {:?}", p);
									true
								}
								Focus(None) | GEvent(MouseInput(ElementState::Released, MouseButton::Left)) => {
									if knob.twist.is_some() {
										knob.twist = None;
										//gui.display.get_window().unwrap().set_cursor_state(glium::glutin::CursorState::Normal).ok().expect("could not set cursor state");
										gui.set_cursor_state(glium::glutin::CursorState::Normal);
									}
									true
								}
								/*Focus(p) => {
									println!("button0{} focus: {:?}", knob_id, p);
									true
								}*/
								_ => false
							}
						}).collect::<Vec<_>>()
					}).collect::<Vec<_>>()
				/*//}) as &mut FnMut(Vec<MyEvent>) -> Vec<MyEvent>,
				}) as Box<FnMut(Vec<MyEvent>) -> Vec<MyEvent>>,
				//&mut |events| {
				box |events| {
					lay_x1.events_for_children(events).into_iter().enumerate().flat_map(|(button_id, events)| {
						let parent_rect = lay_x1.child_rect(&parent_rect, button_id);
						events.into_iter().filter(|ev| {
							!match *ev {
								GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
									println!("button1{} clicked!", button_id);
									**cell.borrow_mut() += 1;
									println!("total_button_presses {}", **cell.borrow_mut());
									println!("rect {:?}", parent_rect);
									true
								}
								Focus(p) => {
									println!("button1{} focus: {:?}", button_id, p);
									true
								}
								_ => false
							}
						}).collect::<Vec<_>>()
					}).collect()
				}
			][lay_x_id](events);
			r*/
		}).collect::<Vec<_>>();
		//knob_renderer.draw(gui.display(), &mut target, &knob_queue);
		gui_renderer.queue_string(gui.display(), r"The vertical metrics of a font at a particular scale. This is useful for calculating the amount of vertical space to give a line of text, and for computing the vertical offset between successive lines.", /*parent_rect.pos*/V::new(0.5, 0.5), 16., [1., 1., 1., 1.]);
		gui_renderer.draw(gui.display(), &mut target);
		target.finish().unwrap();
		for ev in leftover_events {
			match ev {
				GEvent(KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape))) => break 'out,
				_ => ()
			}
			//println!("leftover_event: {:?}", ev);
		}
	}
}
