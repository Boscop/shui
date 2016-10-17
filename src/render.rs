use super::*;

use glium;
use glium::backend::Facade;
use glium::vertex::VertexBufferAny;
use glium::Surface;
use glium::backend::glutin_backend::GlutinFacade;

/*pub trait Widget {
	fn calc_vertex_buffer() -> VertexBufferAny;
}*/

/*impl Widget for Knob {
	fn calc_vertex_buffer() -> VertexBufferAny {

	}
}*/

pub struct KnobRenderer {
	program: glium::Program,
}
impl KnobRenderer {
	pub fn new<T: Facade>(display: &T) -> Self {
		KnobRenderer {
			program: glium::Program::from_source(display, r#"
				#version 140

				in vec2 pos;
				in vec2 uv;
				in float val;
				in int highlight;

				out vec2 v_uv;
				out float v_val;
				flat out int v_highlight;

				void main() {
					gl_Position = vec4(pos, 0., 1.);
					v_uv = uv;
					v_val = val;
					v_highlight = highlight;
				}
			"#, r#"
				#version 430

				in vec2 v_uv;
				in float v_val;
				flat in int v_highlight;

				//uniform vec2 resolution;
				//uniform float time;
				//uniform vec2 mouse;
				//uniform float val;

				out vec4 color;

				const float PI = 3.141592653589793238462643383;
				/*const float invlog210 = 0.301029995663981195213738894724493026768189881462108541310;

				float log10(float x) {
					return log2(x) * invlog210;
				}*/

				float t0 = 7.0/4.0 * PI;
				float t1 = 13.0/4.0 * PI;
				vec3 black = vec3(0.0,0.0,0.0);
				vec3 dot_color = vec3(1.0,1.0,1.0);
				vec3 gray50 = vec3(.5,.5,.5);

				// Returns distance to the outline of the knob.
				float knob_distance(vec2 p, float r) {
					
					float costheta = dot(normalize(p), vec2(0,1));
					
					float t = 5.0/4.0 * PI;
					
					if(costheta > cos(t))
						return abs(length(p)-r);
					
					// Return minimum distance to endpoints.
					vec2 p0 = vec2(cos(t0), sin(t0)) * r;
					vec2 p1 = vec2(cos(t1), sin(t1)) * r;
					
					float d0 = length(p - p0);
					float d1 = length(p - p1);
					
					return min(d0, d1);
					
				}

				// SRC_ALPHA, ONE_MINUS_SRC_ALPHA
				vec4 blend(vec4 fb, vec4 c) {
					return vec4(mix(fb.rgb, c.rgb, c.a), max(c.a,fb.a));
				}
					
				vec4 knob(vec2 p, float r, float value) {
					value = clamp(value, 0.0, 1.0);
					
					// Color of the background layer.
					vec3 bgColor = gray50;
					
					// theta for the knob's value.
					float tValue = mix(t1, t0, value);
					
					vec2 valueP = vec2(cos(tValue), sin(tValue) /* * 1.2*/) * r;
					float vd = length(p-valueP);
				   
					// Make some sort of blobby thing out of the value dot
					// distance and the outline.
					float d = sqrt(knob_distance(p, r) * vd);
					
					// Start with black.
					//vec3 c = black;
					vec4 c = vec4(0.0);

					float s = 0.003;
					
					// Calculate the color for the background layer.
					vec4 bg = vec4(bgColor, 1.0-smoothstep(30.0 * s, 35.0 * s, d));
					c = blend(c, bg);
					
					// Calculate the color for the value dot layer.
					vec4 vdc = vec4(dot_color, 1.0-smoothstep(19.0 * s, 20.0 * s, vd));
					c = blend(c, vdc);
					
					return c;
				}

				void main() {
					//vec2 uv = fragCoord.xy - .5 * iResolution.xy;
					//vec2 p = v_uv - 0.5;
					float val = v_val;
					color = v_highlight > 0 ? vec4(.01, .01, .01, 1.) : vec4(0., 0., 0., 0.);
					color = blend(color, knob(v_uv - vec2(0.5, 0.5 /*- 0.07*/), 0.4, val));
				}
			"#, None).unwrap(),
		}
	}
	/*fn calc_buffers<T: Facade>(&self, display: &T) -> (glium::VertexBuffer<RVertexTerrain>, glium::index::IndexBuffer<u32>) {
	fn calc_vertex_buffer(queue: &[Knob]) -> VertexBufferAny {}
	pub fn draw<T: Surface>(&self, target: &mut T, vertex_buffer: &VertexBufferAny) {
	}*/
	fn calc_vertex_buffer<T: Facade>(display: &T, queue: &[RenderKnob]) -> VertexBufferAny {
		let vertices = queue.iter().flat_map(|e| {
			let p = (e.rect.pos - V::new(0.5, 0.5)) * 2.;
			let r = e.rect.size * 2.;
			let val = e.val;
			let (max_x, max_y) = (p.x + r.x, p.y + r.y);
			let highlight = if e.highlight { 1 } else { 0 };
			let (bottom_left, bottom_right, top_right, top_left) = (
				KnobVertex { // bottom left
					pos: [p.x, p.y],
					uv:  [0., 0.],
					val: val,
					highlight: highlight,
				},
				KnobVertex { // bottom right
					pos: [max_x, p.y],
					uv:  [1., 0.],
					val: val,
					highlight: highlight,
				},
				KnobVertex { // top right
					pos: [max_x, max_y],
					uv:  [1., 1.],
					val: val,
					highlight: highlight,
				},
				KnobVertex { // top left
					pos: [p.x, max_y],
					uv:  [0., 1.],
					val: val,
					highlight: highlight,
				},
			);
			vec![bottom_left, bottom_right, top_right, bottom_left, top_right, top_left] // CCW
		}).collect::<Vec<_>>();
		glium::VertexBuffer::new(display, &vertices).unwrap().into_vertex_buffer_any()
	}
	pub fn draw<T: Surface>(&self, display: &mut GlutinFacade, target: &mut T, queue: &[RenderKnob]) {
		let params = glium::DrawParameters {
			blend: glium::Blend::alpha_blending(),
			/*depth: glium::Depth {
				test: glium::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},*/
			..Default::default()
		};
		let vertex_buffer = KnobRenderer::calc_vertex_buffer(display, queue);
		target.draw(&vertex_buffer, glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &self.program, &glium::uniforms::EmptyUniforms, &params).unwrap();
	}
}

pub struct RectRenderer {
	program: glium::Program,
}
impl RectRenderer {
	pub fn new<T: Facade>(display: &T) -> Self {
		RectRenderer {
			program: glium::Program::from_source(display, r#"
				#version 140

				in vec2 pos;
				in vec2 uv;
				in vec4 color;
				in int filled;
				in float line_width;

				out vec2 v_uv;
				out vec4 v_color;
				flat out int v_filled;
				out float v_line_width;

				void main() {
					gl_Position = vec4(pos, 0., 1.);
					v_uv = uv;
					v_color = color;
					v_filled = filled;
					v_line_width = line_width;
				}
			"#,r#"
				#version 140

				in vec2 v_uv;
				in vec4 v_color;
				flat in int v_filled;
				in float v_line_width;

				out vec4 f_color;

				vec2 line_width = /*sqrt*/(vec2(/*0.5*/v_line_width)); //1. / resolution.xy;

				void main() {
					float fill = v_filled > 0 ? 1. : clamp(
					  (1. - smoothstep(0., line_width.x, sqrt(abs(mod(v_uv.x + 0.5, 1.) - 0.5)))) +
					  (1. - smoothstep(0., line_width.y, sqrt(abs(mod(v_uv.y + 0.5, 1.) - 0.5)))),
					  0., 1.);
					f_color = v_color * fill;
				}
			"#, None).unwrap(),
		}
	}
	fn calc_vertex_buffer<T: Facade>(display: &T, queue: &[RenderRect]) -> VertexBufferAny {
		let vertices = queue.iter().flat_map(|e| {
			let p = (e.rect.pos - V::new(0.5, 0.5)) * 2.;
			let r = e.rect.size * 2.;
			let (max_x, max_y) = (p.x + r.x, p.y + r.y);
			let filled = if e.filled { 1 } else { 0 };
			let line_width = e.line_width;
			let (bottom_left, bottom_right, top_right, top_left) = (
				RectVertex { // bottom left
					pos: [p.x, p.y],
					uv:  [0., 0.],
					color: e.color,
					filled: filled,
					line_width: line_width,
				},
				RectVertex { // bottom right
					pos: [max_x, p.y],
					uv:  [1., 0.],
					color: e.color,
					filled: filled,
					line_width: line_width,
				},
				RectVertex { // top right
					pos: [max_x, max_y],
					uv:  [1., 1.],
					color: e.color,
					filled: filled,
					line_width: line_width,
				},
				RectVertex { // top left
					pos: [p.x, max_y],
					uv:  [0., 1.],
					color: e.color,
					filled: filled,
					line_width: line_width,
				},
			);
			vec![bottom_left, bottom_right, top_right, bottom_left, top_right, top_left] // CCW
		}).collect::<Vec<_>>();
		glium::VertexBuffer::new(display, &vertices).unwrap().into_vertex_buffer_any()
	}
	pub fn draw<T: Surface>(&self, display: &mut GlutinFacade, target: &mut T, queue: &[RenderRect]) {
		let params = glium::DrawParameters {
			blend: glium::Blend::alpha_blending(),
			/*depth: glium::Depth {
				test: glium::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},*/
			..Default::default()
		};
		let vertex_buffer = RectRenderer::calc_vertex_buffer(display, queue);
		target.draw(&vertex_buffer, glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &self.program, &glium::uniforms::EmptyUniforms, &params).unwrap();
	}
}

pub struct RenderKnob {
	pub rect: Rect,
	pub val: f32,
	pub highlight: bool,
}

#[derive(Copy, Clone)]
struct KnobVertex {
	pos: [f32; 2],
	uv: [f32; 2],
	val: f32,
	highlight: i32,
	//color: [f32; 4]
}
implement_vertex!(KnobVertex, pos, uv, val, highlight);

pub struct RenderRect {
	pub rect: Rect,
	pub color: MyColor,
	pub filled: bool,
	pub line_width: f32,
}

#[derive(Copy, Clone)]
struct RectVertex {
	pos: [f32; 2],
	uv: [f32; 2],
	color: [f32; 4],
	filled: i32,
	line_width: f32,
}
implement_vertex!(RectVertex, pos, uv, color, filled, line_width);

pub struct KnobGuiState {
	pub val: f32,
	pub label: String,
	pub twist: Option<(V, f32)>, // mouse pos when pressed, knob value when pressed
	pub focused: bool,
}

/*pub struct MultiChoiceKnobGuiState {
	pub val: f32,
	pub labels: Vec<String>,
	pub twist: Option<(V, f32)>, // mouse pos when pressed, knob value when pressed
	pub focused: bool,
}*/

pub struct ButtonGuiState {
	pub label: String,
	pub pressed: bool,
	pub focused: bool,
}

pub struct ToggleButtonGuiState {
	pub label: String,
	//pub pressed: bool,
	//pub focused: bool,
	pub on: bool,
}

pub struct GuiRenderer<'a> {
	knob_renderer: KnobRenderer,
	knob_queue: Vec<RenderKnob>,
	rect_renderer: RectRenderer,
	rect_queue: Vec<RenderRect>,
	font: MyFont<'a>,
	//render_queue: RenderableQueue,
	glyph_queue: Vec<GlyphVertex>,
}
impl<'a> GuiRenderer<'a> {
	pub fn new(display: &GlutinFacade) -> Self {
		GuiRenderer {
			knob_renderer: KnobRenderer::new(display),
			rect_renderer: RectRenderer::new(display),
			knob_queue: vec![],
			rect_queue: vec![],
			font: MyFont::new(display,
				//r"D:\projects\MadBoys9000\FlowStudio_data\fonts\Bank Gothic Light.ttf"
				r"D:\projects\MadBoys9000\FlowStudio_data\fonts\BankGothic Bold.ttf"
				//r"D:\projects\MadBoys9000\FlowStudio_data\fonts\Arial Unicode.ttf"
			).unwrap(),
			glyph_queue: vec![],
		}
	}
	pub fn queue_knob(&mut self, rect: Rect, knob: &KnobGuiState) {
		self.knob_queue.push(RenderKnob {rect: rect, val: knob.val, highlight: /*knob.focused*/false});
	}
	pub fn queue_rect(&mut self, rect: RenderRect) {
		self.rect_queue.push(rect);
	}
	pub fn queue_string(&mut self, display: &GlutinFacade, text: &str, pos: V, size: f32, color: MyColor, centered: Centered) {
		self.glyph_queue.extend(self.font.string_vertices(display, text, pos, size, color, centered));
	}
	pub fn draw<T: Surface>(&mut self, display: &mut GlutinFacade, target: &mut T) {
		self.rect_renderer.draw(display, target, &self.rect_queue.move_contents());
		self.knob_renderer.draw(display, target, &self.knob_queue.move_contents());
		self.font.draw(display, target, &self.glyph_queue.move_contents());
	}
}
