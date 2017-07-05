#![allow(unused_imports)]
#![feature(box_syntax)]

use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

extern crate shui;
use shui::*;
use shui::ui::*;
use shui::layout::*;
use shui::render::*;
use shui::handlers::*;

#[macro_use] extern crate glium;
use glium::glutin::{WindowBuilder, ElementState, MouseButton, VirtualKeyCode};
use glium::{DisplayBuild, Surface};
use glium::glutin::{Event};

extern crate nalgebra as na;
use na::{zero, one};

fn main() {
	let display = WindowBuilder::new().with_title("title".to_string()).with_dimensions(800 as u32, 600 as u32).with_multisampling(4).with_depth_buffer(24).build_glium().unwrap();
	let mut ui = Ui::new(display);
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
	let mut multi_choice_knob = MultiChoiceKnobUiState { knob: KnobUiState { val: 0., label: "OSC".into(), twist: None, focused: false }, values: vec!["sin", "tri", "saw", "square", "pulse"].into_iter().map(|s| s.into()).collect() };
	let total_button_presses = Rc::new(RefCell::new(0));

	let total_button_presses_knob_handler = total_button_presses.clone();
	let total_button_presses_button_handler = total_button_presses.clone();
	let total_button_presses_toggle_button_handler = total_button_presses.clone();

	let mut knob_handler = move |ui: &mut Ui, rect, events, i| {
		let (leftover_events, val_changed) = handle_knob(ui, rect, events, &mut knobs.borrow_mut()[i]);
		if let Some(val) = val_changed {
			println!("knob changed: {} {:.2}", i, val);
			*total_button_presses_knob_handler.borrow_mut() += 1;
			println!("total_button_presses {}", *total_button_presses_knob_handler.borrow());
		}
		leftover_events
	};

	let mut multi_choice_knob_handler = move |ui: &mut Ui, rect, events| {
		let (leftover_events, val_changed) = handle_multi_choice_knob(ui, rect, events, &mut multi_choice_knob);
		if let Some(val) = val_changed {
			println!("multi_choice_knob changed: {}", val);
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

	let mut layout = Lay::Y(vec![
		(1., Lay::X(vec![
			(1., Lay::XMulti(vec![1., 1.], &mut knob_handler)),
			(1., Lay::Single(&mut multi_choice_knob_handler)),
		])),
		(1., Lay::X(vec![
			(1., Lay::Single(&mut button_handler)),
			(1., Lay::Single(&mut toggle_button_handler)),
		])),
		(0.5, Lay::XYMultiEq(ROWS, COLS, &mut multi_handler)),
	]);
	let mut event_handler = layout.build_full_window();

	'out: while let Some(leftover_events) = {
		let mut target = ui.display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);

		let leftover_events = event_handler(&mut ui);
		ui.renderer.draw(&mut ui.display, &mut target);
		target.finish().unwrap();
		thread::sleep(Duration::from_millis(20));
		leftover_events
	} {
		for ev in leftover_events {
			match ev {
				MyEvent::GEvent(Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape))) => break 'out,
				_ => println!("unused event: {:?}", ev)
			}
		}
	}
}

