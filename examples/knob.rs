#![feature(box_syntax)]

use std::thread;
use std::time::Duration;

extern crate shui;
use shui::ui::*;
use shui::layout::*;
use shui::render::*;
use shui::handlers::*;

extern crate glium;
use glium::glutin::{WindowBuilder, ElementState, VirtualKeyCode};
use glium::{DisplayBuild, Surface};
use glium::glutin::{Event};

extern crate nalgebra as na;

fn main() {

	// Create our window 
	let display = WindowBuilder::new()
		.with_title("title".to_string())
		.with_dimensions(800 as u32, 600 as u32)
		.with_multisampling(4)
		.with_depth_buffer(24)
		.build_glium()
		.unwrap();

	// Create a new UI canvas to which we will draw our elements
	let mut ui = Ui::new(display);

	// Declare our knob state
	let mut knob = KnobUiState { val: 0.0, label: "My Knob".into(), twist: None, focused: false };

	// Define what to do when our knob moves
	let mut knob_handler = move |ui: &mut Ui, rect, events| {
		let (leftover_events, val_changed) = handle_knob(ui, rect, events, &mut knob);
		if let Some(val) = val_changed {
			println!("knob changed: {:.2}", val);
		}
		leftover_events
	};

	// Define our layout
	const ROWS: usize = 1;
	const COLS: usize = 1;
	let mut layout = Lay::Y(vec![
		(1., Lay::X(vec![
			(1., Lay::Single(&mut knob_handler)),
		])
	)]);

	// Build and draw our window
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

