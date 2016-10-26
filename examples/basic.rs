#![allow(unused_imports)]
#![feature(box_syntax)]

use std::thread;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

extern crate shui;
//use shui::prelude::*;
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
use na::{/*Vector2, RotationTo, Norm, Dot, Rotation2, Vector1, Vector3,*/ zero, one};

fn main() {
	let display = WindowBuilder::new().with_title("title".to_string()).with_dimensions(800 as u32, 600 as u32).with_multisampling(4).with_depth_buffer(24)/*.with_parent(Some(WindowID::new(parent)))*/.build_glium().unwrap();
	let mut ui = Ui::new(display);
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
	let mut multi_choice_knob = MultiChoiceKnobUiState { knob: KnobUiState { val: 0., label: "OSC".into(), twist: None, focused: false }, values: vec!["sin", "tri", "saw", "square", "pulse"].into_iter().map(|s| s.into()).collect() };
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
			(1., Lay::Single(&mut multi_choice_knob_handler)),
		])),
		(1., Lay::X(vec![
			(1., Lay::Single(&mut button_handler)),
			(1., Lay::Single(&mut toggle_button_handler)),
		])),
		(0.5, Lay::XYMultiEq(ROWS, COLS, &mut multi_handler)),
	]);
	let mut event_handler = layout.build();

	let mut button_request_to_be_master = ButtonUiState::new("this is the\nmaster\ninstance".into());

	let mut lay_y = Layout::new(Axis::Y, vec![1.]);

	'out: while let Some(leftover_events) = {
		let mut target = ui.display.draw();
		target.clear_color(0.0, 0.0, 1.0, 0.0);
		//knob_renderer.draw(gui.display(), &mut target, &knob_queue);
		//gui_renderer.queue_string(gui.display(), r"The vertical metrics of a font at a particular scale. This is useful for calculating the amount of vertical space to give a line of text, and for computing the vertical offset between successive lines.", /*parent_rect.pos*/V::new(0.5, 0.5), 16. * parent_rect.size.y, [1., 1., 1., 1.], Centered::both());

		let leftover_events = event_handler(&mut ui);
		/*let leftover_events = ui.frame().map(|events| {
			lay_y.events_for_children(events).into_iter().enumerate().flat_map(|(child_id, events)| {
				let rect = Rect::new(zero(), one());
				let (leftover_events, clicked) = button_request_to_be_master.draw(&mut ui, rect, events);
				if clicked {
					println!("button clicked: request to be master");
				}
				leftover_events
			}).collect::<Vec<_>>()
		});*/

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

