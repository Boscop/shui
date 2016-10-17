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
	let mut buttons = vec![
		ButtonGuiState { label: "button00".to_string(), pressed: false, focused: false, },
		ButtonGuiState { label: "button01".to_string(), pressed: false, focused: false, },
	];
	let mut toggle_button = ToggleButtonGuiState { label: "toggle".to_string(), on: false, };
	let mut knobs = vec![
		KnobGuiState { val: 0.0, label: "knob10".to_string(), twist: None, focused: false, },
		KnobGuiState { val: 0.5, label: "knob11".to_string(), twist: None, focused: false, },
		KnobGuiState { val: 0.5, label: "knob12".to_string(), twist: None, focused: false, },
	];
	//let knob_renderer = KnobRenderer::new(gui.display());
	let mut gui_renderer = GuiRenderer::new(gui.display());
	/*let mut font = MyFont::new(&gui.display(),
		//r"D:\projects\MadBoys9000\FlowStudio_data\fonts\Bank Gothic Light.ttf"
		r"D:\projects\MadBoys9000\FlowStudio_data\fonts\BankGothic Bold.ttf"
		//r"D:\projects\MadBoys9000\FlowStudio_data\fonts\Arial Unicode.ttf"
	).unwrap();*/
	let total_button_presses = RefCell::new(&mut total_button_presses);
	'out: while let Some(events) = gui.frame() {
        let mut target = gui.display().draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
		let parent_rect = Rect::new(one(), one());
		thread::sleep(Duration::from_millis(20));
		let leftover_events = {
			let gui = RefCell::new(&mut gui);
			let gui_renderer = RefCell::new(&mut gui_renderer);
			lay_y.events_for_children(events).into_iter().enumerate().flat_map(|(lay_x_id, events)| {
				//let lay_x0 = &mut lay_x0;
				let parent_rect = lay_y.child_rect(&parent_rect, lay_x_id);
				let r = [
					//(&mut |events| {
					(box |events| {
						let mut gui = gui.borrow_mut();
						let mut gui_renderer = gui_renderer.borrow_mut();
						//let lay_x0 = &mut *lay_x0;
						//lay_x0
						//let lay_xx = &mut [&mut lay_x0, &mut lay_x1][lay_x_id];
						let lay_xx = &mut lay_x0;
						lay_xx.events_for_children(events).into_iter().enumerate().flat_map(|(knob_id, events)| {
							let parent_rect = lay_xx.child_rect(&parent_rect, knob_id);
							let knob = &mut knobs/*[lay_x_id]*/[knob_id];
							//knob_queue.push(RenderKnob {rect: parent_rect, val: knob.val});
							widget_knob(&mut gui_renderer, gui.display(), parent_rect, &knob);
							events.into_iter().filter(|ev| {
								!match *ev {
									GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
										**total_button_presses.borrow_mut() += 1;
										println!("total_button_presses {}", *total_button_presses.borrow());
										let knob_pos = V::new(0.5, 0.5);
										let mouse_pos = gui.mouse_tr(&parent_rect);
										const KNOB_RADIUS: f32 = 0.5;
										let d = (mouse_pos - knob_pos).norm();
										if d < KNOB_RADIUS {
											knob.twist = Some((mouse_pos, knob.val));
											gui.set_cursor_state(glium::glutin::CursorState::Hide);
										}
										true
									}
									MousePos(p) => {
										knob.twist = knob.twist.and_then(|(press_pos, orig_val)| {
											let d = p - press_pos;
											let val = clamp(orig_val + d.y * 2., 0., 1.);
											knob.val = val;
											gui.set_cursor_pos(parent_rect.pos + parent_rect.size * press_pos);
											Some((press_pos, val))
										});
										true
									}
									GEvent(MouseInput(ElementState::Released, MouseButton::Left)) => {
										if knob.twist.is_some() {
											knob.twist = None;
											gui.set_cursor_state(glium::glutin::CursorState::Normal);
										}
										true
									}
									Focus(maybe) => {
										knob.focused = maybe.is_some();
										true
									}
									_ => false
								}
							}).collect::<Vec<_>>()
						}).collect::<Vec<_>>()
					//}) as &mut FnMut(Vec<MyEvent>) -> Vec<MyEvent>,
					}) as Box<FnMut(Vec<MyEvent>) -> Vec<MyEvent>>,
					//&mut |events| {
					box |events| {
						let mut gui = gui.borrow_mut();
						let mut gui_renderer = gui_renderer.borrow_mut();
						lay_x1.events_for_children(events).into_iter().enumerate().flat_map(|(button_id, events)| {
							let parent_rect = lay_x1.child_rect(&parent_rect, button_id);
							if button_id == 0 {
								let button = &mut buttons[button_id];
								widget_button(&mut gui_renderer, gui.display(), parent_rect, &button);
								events.into_iter().filter(|ev| {
									!match *ev {
										GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
											**total_button_presses.borrow_mut() += 1;
											println!("total_button_presses {}", **total_button_presses.borrow_mut());
											button.pressed = true;
											true
										}
										GEvent(MouseInput(ElementState::Released, MouseButton::Left)) => {
											button.pressed = false;
											true
										}
										Focus(maybe) => {
											button.focused = maybe.is_some();
											true
										}
										_ => false
									}
								}).collect::<Vec<_>>()
							} else {
								widget_toggle_button(&mut gui_renderer, gui.display(), parent_rect, &toggle_button);
								events.into_iter().filter(|ev| {
									!match *ev {
										GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
											toggle_button.on ^= true;
											true
										}
										_ => false
									}
								}).collect::<Vec<_>>()
							}
						}).collect()
					}
				][lay_x_id](events);
				r
			}).collect::<Vec<_>>()
		};
		//knob_renderer.draw(gui.display(), &mut target, &knob_queue);
		//gui_renderer.queue_string(gui.display(), r"The vertical metrics of a font at a particular scale. This is useful for calculating the amount of vertical space to give a line of text, and for computing the vertical offset between successive lines.", /*parent_rect.pos*/V::new(0.5, 0.5), 16. * parent_rect.size.y, [1., 1., 1., 1.], Centered::both());
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
