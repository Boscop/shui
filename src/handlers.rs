//use super::*;
use prelude::*;

use glium;
use glium::glutin::{ElementState, MouseButton/*, VirtualKeyCode*/};

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
	widget_knob(&mut ui.renderer, &mut ui.display, rect, knob, &format!("{:.2}", knob.val));
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

pub fn handle_multi_choice_knob(ui: &mut Ui, rect: Rect, events: Vec<MyEvent>, mcknob: &mut MultiChoiceKnobUiState) -> (Vec<MyEvent>, Option<usize>) {
	let val_idx = mcknob.value_idx();
	widget_knob(&mut ui.renderer, &mut ui.display, rect, &mcknob.knob, &mcknob.values[val_idx]);
	let mut val_changed = None;
	let leftover_events = events.into_iter().filter(|ev| {
		!match *ev {
			GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
				let knob_pos = V::new(0.5, 0.5);
				let mouse_pos = ui.mouse_tr(&rect);
				const KNOB_RADIUS: f32 = 0.5;
				let d = (mouse_pos - knob_pos).norm();
				if d < KNOB_RADIUS {
					mcknob.knob.twist = Some((mouse_pos, mcknob.knob.val));
					ui.set_cursor_state(glium::glutin::CursorState::Hide);
				}
				true
			}
			MousePos(p) => {
				mcknob.knob.twist = mcknob.knob.twist.and_then(|(press_pos, orig_val)| {
					let d = p - press_pos;
					let val = clamp(orig_val + d.y * 2., 0., 1.);
					mcknob.knob.val = val;
					ui.set_cursor_pos(rect.pos + rect.size * press_pos);
					let new_val_idx = mcknob.value_idx();
					if new_val_idx != val_idx {
						val_changed = Some(new_val_idx);
					}
					Some((press_pos, val))
				});
				true
			}
			GEvent(MouseInput(ElementState::Released, MouseButton::Left)) => {
				if mcknob.knob.twist.is_some() {
					mcknob.knob.twist = None;
					ui.set_cursor_state(glium::glutin::CursorState::Normal);
				}
				true
			}
			Focus(maybe) => {
				mcknob.knob.focused = maybe.is_some();
				true
			}
			_ => false
		}
	}).collect::<Vec<_>>();
	(leftover_events, val_changed)
}
