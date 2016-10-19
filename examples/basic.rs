#![allow(unused_imports)]
#![feature(box_syntax)]

use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

extern crate shui;
use shui::*;

#[macro_use] extern crate glium;
use glium::glutin::{WindowBuilder, ElementState, MouseButton, VirtualKeyCode};
use glium::{DisplayBuild, Surface};

extern crate nalgebra as na;
//use na::{Vector2, RotationTo, Norm, Dot, Rotation2, Vector1, Vector3, zero, one};

fn main() {
	let mut display = WindowBuilder::new().with_title("title".to_string()).with_dimensions(800 as u32, 600 as u32).with_multisampling(4).with_depth_buffer(24)/*.with_parent(Some(WindowID::new(parent)))*/.build_glium().unwrap();
	let mut ui = Ui::new(&mut display);
	//let mut lay_y = Layout::new(Axis::Y, vec![1., 1.]); //, 0);
	// let mut _lay_x0_id = lay_y.add(1.);
	// let mut _lay_x1_id = lay_y.add(1.);
	// let mut lay_x0 = Layout::new(Axis::X, vec![1., 1., 2.]); //, lay_y.add(1.));
	// let mut lay_x1 = Layout::new(Axis::X, vec![1., 1.]); //, lay_y.add(1.));
	// let _button00 = lay_x0.add(1.);
	// let _button01 = lay_x0.add(1.);
	// let _button02 = lay_x0.add(2.);
	// let _button10 = lay_x1.add(1.);
	// let _button11 = lay_x1.add(1.);
	//let mut total_button_presses = 0;
	let buttons = Rc::new(RefCell::new(vec![
		ButtonUiState { label: "button0".into(), pressed: false, focused: false, },
		ButtonUiState { label: "button1".into(), pressed: false, focused: false, },
	])).clone();
	let mut toggle_button = ToggleButtonUiState { label: "toggle".into(), on: false, };
	let knobs = Rc::new(RefCell::new(vec![
		KnobUiState { val: 0.0, label: "knob0".into(), twist: None, focused: false, },
		KnobUiState { val: 0.5, label: "knob1".into(), twist: None, focused: false, },
		KnobUiState { val: 0.5, label: "knob2".into(), twist: None, focused: false, },
	])).clone();
	//let knob_renderer = KnobRenderer::new(gui.display());
	//let mut gui_renderer = UiRenderer::new(gui.display());
	/*let mut font = MyFont::new(&gui.display(),
		//r"D:\projects\MadBoys9000\FlowStudio_data\fonts\Bank Gothic Light.ttf"
		r"D:\projects\MadBoys9000\FlowStudio_data\fonts\BankGothic Bold.ttf"
		//r"D:\projects\MadBoys9000\FlowStudio_data\fonts\Arial Unicode.ttf"
	).unwrap();*/
	let total_button_presses = Rc::new(RefCell::new(0));
	//let knobs = RefCell::new(&mut knobs);

	let total_button_presses_knob_handler = total_button_presses.clone();
	let total_button_presses_button_handler = total_button_presses.clone();
	let total_button_presses_toggle_button_handler = total_button_presses.clone();

	let mut knob_handler = move |ui: &mut Ui, rect, events, i| {
		//let knob = &mut knobs[i];
		let (leftover_events, val_changed) = handle_knob(ui, rect, events, &mut knobs.borrow_mut()[i]);
		if let Some(val) = val_changed {
			println!("knob changed: {} {:.2}", i, val);
			*total_button_presses_knob_handler.borrow_mut() += 1;
			println!("total_button_presses {}", *total_button_presses_knob_handler.borrow());
		}
		leftover_events
	};

	let mut button_handler = move |ui: &mut Ui, rect, events| {
		let (leftover_events, val_changed) = handle_button(ui, rect, events, &mut buttons.borrow_mut()[0]);
		if val_changed {
			println!("button clicked");
			*total_button_presses_button_handler.borrow_mut() += 1;
			println!("total_button_presses {}", *total_button_presses_button_handler.borrow());
		}
		leftover_events
	};

	let mut toggle_button_handler = move |ui: &mut Ui, rect, events| {
		let (leftover_events, val_changed) = handle_toggle_button(ui, rect, events, &mut toggle_button);
		if let Some(on) = val_changed {
			println!("button toggled: {}", on);
			*total_button_presses_toggle_button_handler.borrow_mut() += 1;
			println!("total_button_presses {}", *total_button_presses_toggle_button_handler.borrow());
		}
		leftover_events
		//events
	};

	const ROWS: usize = 2;
	const COLS: usize = 5;
	let mut toggle_buttons: Vec<Vec<ToggleButtonUiState>> = (0..ROWS).map(|row| (0..COLS).map(|col| ToggleButtonUiState { label: format!("toggle[{}][{}]", row, col), on: false, }).collect::<Vec<_>>()).collect::<Vec<_>>();
	let mut multi_handler = move |ui: &mut Ui, rect, events, row: ChildId, col: ChildId| {
		let (leftover_events, val_changed) = handle_toggle_button(ui, rect, events, &mut toggle_buttons[row][col]);
		if let Some(on) = val_changed {
			println!("toggle[{}][{}].on = {}", row, col, on);
		}
		leftover_events
	};

	//fn id<T>(x: T) -> T { x }

	let mut layout = Lay::Y(vec![
		/*(1., Lay::X(vec![
			(1., Lay::XMulti(vec![1., 1., 2.], id(&mut move |ui: &mut Ui, rect, events, i| {
				let (leftover_events, val_changed) = handle_knob(ui, rect, events, &mut knobs.borrow_mut()[i]);
				if let Some(val) = val_changed {
					println!("knob changed: {} {:.2}", i, val);
					// **total_button_presses.borrow_mut() += 1;
					// println!("total_button_presses {}", *total_button_presses.borrow());
				}
				leftover_events
			}))),
		])),*/
		(1., Lay::X(vec![
			(1., Lay::XMulti(vec![1., 1.], &mut knob_handler)),
		])),
		(1., Lay::X(vec![
			(1., Lay::Single(&mut button_handler)),
			(1., Lay::Single(&mut toggle_button_handler)),
		])),
		(0.5, Lay::XYMultiEq(ROWS, COLS, &mut multi_handler)),
	]);
	let mut event_handler = layout.build();

	'out: while let Some(leftover_events) = {
		let mut target = ui.display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);
		//knob_renderer.draw(gui.display(), &mut target, &knob_queue);
		//gui_renderer.queue_string(gui.display(), r"The vertical metrics of a font at a particular scale. This is useful for calculating the amount of vertical space to give a line of text, and for computing the vertical offset between successive lines.", /*parent_rect.pos*/V::new(0.5, 0.5), 16. * parent_rect.size.y, [1., 1., 1., 1.], Centered::both());
		let leftover_events = event_handler(&mut ui);
		ui.renderer.draw(ui.display, &mut target);
		target.finish().unwrap();
		thread::sleep(Duration::from_millis(20));
		leftover_events
	} {
		for ev in leftover_events {
			match ev {
				GEvent(KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape))) => break 'out,
				_ => ()
			}
		}
	}
	/*'out: while let Some(events) = ui.frame() {
		let mut target = ui.display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);
		//let parent_rect = Rect::new(one(), one());
		thread::sleep(Duration::from_millis(20));
		/*let leftover_events = {
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
		};*/
		let leftover_events = event_handler();
		//knob_renderer.draw(gui.display(), &mut target, &knob_queue);
		//gui_renderer.queue_string(gui.display(), r"The vertical metrics of a font at a particular scale. This is useful for calculating the amount of vertical space to give a line of text, and for computing the vertical offset between successive lines.", /*parent_rect.pos*/V::new(0.5, 0.5), 16. * parent_rect.size.y, [1., 1., 1., 1.], Centered::both());
		ui.renderer.draw(ui.display, &mut target);
		target.finish().unwrap();
		for ev in leftover_events {
			match ev {
				GEvent(KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape))) => break 'out,
				_ => ()
			}
			//println!("leftover_event: {:?}", ev);
		}
	}*/
}

use glium::backend::glutin_backend::GlutinFacade;

pub fn handle_toggle_button(ui: &mut Ui, rect: Rect, events: Vec<MyEvent>, toggle_button: &mut ToggleButtonUiState) -> (Vec<MyEvent>, Option<bool>) {
	widget_toggle_button(&mut ui.renderer, &mut ui.display, rect, toggle_button);
	let mut val_changed = None;
	let leftover_events = events.into_iter().filter(|ev| {
		!match *ev {
			GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
				toggle_button.on ^= true;
				val_changed = Some(toggle_button.on);
				true
			}
			_ => false
		}
	}).collect::<Vec<_>>();
	(leftover_events, val_changed)
}

pub fn handle_button(ui: &mut Ui, rect: Rect, events: Vec<MyEvent>, button: &mut ButtonUiState) -> (Vec<MyEvent>, bool) {
	widget_button(&mut ui.renderer, &mut ui.display, rect, button);
	let mut val_changed = false;
	let leftover_events = events.into_iter().filter(|ev| {
		!match *ev {
			GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
				val_changed = true;
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
	}).collect::<Vec<_>>();
	(leftover_events, val_changed)
}

pub fn handle_knob(ui: &mut Ui, rect: Rect, events: Vec<MyEvent>, knob: &mut KnobUiState) -> (Vec<MyEvent>, Option<f32>) {
	widget_knob(&mut ui.renderer, &mut ui.display, rect, knob);
	let mut val_changed = None;
	let leftover_events = events.into_iter().filter(|ev| {
		//println!("knob ev {:?}", ev);
		!match *ev {
			GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
				let knob_pos = V::new(0.5, 0.5);
				let mouse_pos = ui.mouse_tr(&rect);
				const KNOB_RADIUS: f32 = 0.5;
				let d = (mouse_pos - knob_pos).norm();
				if d < KNOB_RADIUS {
					knob.twist = Some((mouse_pos, knob.val));
					ui.set_cursor_state(glium::glutin::CursorState::Hide);
				}
				true
			}
			MousePos(p) => {
				knob.twist = knob.twist.and_then(|(press_pos, orig_val)| {
					let d = p - press_pos;
					let val = clamp(orig_val + d.y * 2., 0., 1.);
					knob.val = val;
					ui.set_cursor_pos(rect.pos + rect.size * press_pos);
					val_changed = Some(val);
					Some((press_pos, val))
				});
				true
			}
			GEvent(MouseInput(ElementState::Released, MouseButton::Left)) => {
				if knob.twist.is_some() {
					knob.twist = None;
					ui.set_cursor_state(glium::glutin::CursorState::Normal);
				}
				true
			}
			Focus(maybe) => {
				knob.focused = maybe.is_some();
				true
			}
			_ => false
		}
	}).collect::<Vec<_>>();
	(leftover_events, val_changed)
}
