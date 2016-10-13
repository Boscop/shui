#![allow(unused_imports)]
#![feature(box_syntax)]

use std::thread;
use std::time::Duration;
use std::cell::RefCell;

extern crate shui;
use shui::*;

#[macro_use] extern crate glium;
use glium::glutin::{ElementState, MouseButton, VirtualKeyCode};

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
	'out: while let Some(events) = gui.frame() {
		let parent_rect = Rect::new(one(), one());
		thread::sleep(Duration::from_millis(20));
		for leftover_event in lay_y.events_for_children(events).into_iter().enumerate().flat_map(|(lay_x_id, events)| {
			//let lay_x0 = &mut lay_x0;
			let parent_rect = lay_y.child_rect(parent_rect, lay_x_id);
			let r = [
				//(&mut |events| {
				(box |events| {
					//let lay_x0 = &mut *lay_x0;
					let r = lay_x0.events_for_children(events).into_iter().enumerate().flat_map(|(button_id, events)| {
						let parent_rect = lay_x0.child_rect(parent_rect, button_id);
						events.into_iter().filter(|ev| {
							!match *ev {
								GEvent(MouseInput(ElementState::Pressed, MouseButton::Left)) => {
									println!("button0{} clicked!", button_id);
									**cell.borrow_mut() += 1;
									println!("total_button_presses {}", **cell.borrow_mut());
									println!("rect {:?}", parent_rect);
									true
								}
								Focus(p) => {
									println!("button0{} focus: {:?}", button_id, p);
									true
								}
								_ => false
							}
						}).collect::<Vec<_>>()
					}).collect();
					r
				//}) as &mut FnMut(Vec<MyEvent>) -> Vec<MyEvent>,
				}) as Box<FnMut(Vec<MyEvent>) -> Vec<MyEvent>>,
				//&mut |events| {
				box |events| {
					let r = lay_x1.events_for_children(events).into_iter().enumerate().flat_map(|(button_id, events)| {
						let parent_rect = lay_x1.child_rect(parent_rect, button_id);
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
					}).collect();
					r
				}
			][lay_x_id](events);
			r
		}).collect::<Vec<_>>() {
			match leftover_event {
				GEvent(KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape))) => break 'out,
				_ => ()
			}
			println!("leftover_event: {:?}", leftover_event);
		}
	}
}
