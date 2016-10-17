use super::*;

use glium::backend::glutin_backend::GlutinFacade;

const COLOR_WHITE: MyColor = [1., 1., 1., 1.];
const COLOR_BLACK: MyColor = [0., 0., 0., 1.];

pub fn widget_knob(gui_renderer: &mut GuiRenderer, display: &mut GlutinFacade, rect: Rect, knob: &KnobGuiState) {
	//gui_renderer.queue_rect(RenderRect {rect: rect, color: COLOR_WHITE, filled: true});
	gui_renderer.queue_knob(rect, knob);
	gui_renderer.queue_string(display, &format!("{:.2}", knob.val), rect.rel(V::new(0.5, 0.5)), 120. * rect.size.y, COLOR_WHITE, Centered::both());
	gui_renderer.queue_string(display, &knob.label, rect.rel(V::new(0.5, 0.02)), 80. * rect.size.y, COLOR_WHITE, Centered::horz());
}

pub fn widget_button(gui_renderer: &mut GuiRenderer, display: &mut GlutinFacade, rect: Rect, button: &ButtonGuiState) {
	const GRAY: f32 = 0.1;
	gui_renderer.queue_rect(RenderRect {rect: rect.pad(0.02), color: [GRAY, GRAY, GRAY, 1.], filled: button.pressed, line_width: 0.5});
	gui_renderer.queue_string(display, &button.label, rect.rel(V::new(0.5, 0.5)), 80. * rect.size.y, COLOR_WHITE, Centered::both());
}

pub fn widget_toggle_button(gui_renderer: &mut GuiRenderer, display: &mut GlutinFacade, rect: Rect, button: &ToggleButtonGuiState) {
	gui_renderer.queue_rect(RenderRect {rect: rect.pad(0.02), color: COLOR_WHITE, filled: button.on, line_width: 0.2});
	gui_renderer.queue_string(display, &button.label, rect.rel(V::new(0.5, 0.5)), 80. * rect.size.y, if button.on { COLOR_BLACK } else { COLOR_WHITE }, Centered::both());
}
