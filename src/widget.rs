//use super::*;
use prelude::*;

const COLOR_WHITE: MyColor = [1., 1., 1., 1.];
const COLOR_BLACK: MyColor = [0., 0., 0., 1.];
//const COLOR_DARKGRAY: MyColor = [0.1, 0.1, 0.1, 1.];
const COLOR_LIGHTGRAY: MyColor = [0.5, 0.5, 0.5, 1.];
const COLOR_TRANSPARENT: MyColor = [0., 0., 0., 0.];

pub fn widget_knob(gui_renderer: &mut UiRenderer, display: &mut GlutinFacade, rect: Rect, knob: &KnobUiState, value: &str) {
	//gui_renderer.queue_rect(RenderRect {rect: rect, color: COLOR_WHITE, filled: true});
	gui_renderer.queue_knob(rect, knob);
	gui_renderer.queue_string(display, value, rect.rel(V::new(0.5, 0.5)), 100. * rect.size.y, COLOR_WHITE, Centered::both());
	gui_renderer.queue_string(display, &knob.label, rect.rel(V::new(0.5, 0.02)), 80. * rect.size.y, COLOR_WHITE, Centered::horz());
}

pub fn widget_button(gui_renderer: &mut UiRenderer, display: &mut GlutinFacade, rect: Rect, button: &ButtonUiState) {
	gui_renderer.queue_rect(RenderRect {rect: rect, border_width: /*0.5*/0.3, border_color: COLOR_TRANSPARENT, fill_color: if button.pressed { COLOR_WHITE } else { COLOR_LIGHTGRAY }});
	gui_renderer.queue_string(display, &button.label, rect.rel(V::new(0.5, 0.5)), 80. * rect.size.y, COLOR_BLACK, Centered::both());
}

pub fn widget_toggle_button(gui_renderer: &mut UiRenderer, display: &mut GlutinFacade, rect: Rect, button: &ToggleButtonUiState) {
	gui_renderer.queue_rect(RenderRect {rect: rect, border_width: /*0.2*/0.3, border_color: COLOR_LIGHTGRAY, fill_color: if button.on { COLOR_WHITE } else { COLOR_TRANSPARENT }});
	gui_renderer.queue_string(display, &button.label, rect.rel(V::new(0.5, 0.5)), 80. * rect.size.y, if button.on { COLOR_BLACK } else { COLOR_WHITE }, Centered::both());
}

pub fn widget_label(gui_renderer: &mut UiRenderer, display: &mut GlutinFacade, rect: Rect, label: &LabelUiState) {
	gui_renderer.queue_rect(RenderRect {rect: rect, border_width: 0., border_color: COLOR_TRANSPARENT, fill_color: COLOR_WHITE});
	gui_renderer.queue_string(display, &label.label, rect.rel(V::new(0.5, 0.5)), 80. * rect.size.y, COLOR_BLACK, Centered::both());
}
