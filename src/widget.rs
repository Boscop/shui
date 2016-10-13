use super::*;

use glium;
use glium::backend::Facade;
use glium::vertex::VertexBufferAny;

/*pub trait Widget {
	fn calc_vertex_buffer() -> VertexBufferAny;
}*/

/*impl Widget for Knob {
	fn calc_vertex_buffer() -> VertexBufferAny {

	}
}*/

struct RenderKnob {
	program: glium::Program,
}
impl RenderKnob {
	pub fn new<T: Facade>(display: &T) -> Self {
		RenderKnob {
			program: glium::Program::from_source(display, r#"
				#version 140

				in vec2 position;
				in vec2 uv;

				out vec2 v_uv;

				void main() {
					gl_Position = vec4(position, 0., 1.);
					v_uv = uv;
				}
			"#, r#"
				#version 430

				in vec2 uv;

				//uniform vec2 resolution;
				//uniform float time;
				//uniform vec2 mouse;
				uniform float val;

				out vec4 color;

				const float PI = 3.141592653589793238462643383;
				/*const float invlog210 = 0.301029995663981195213738894724493026768189881462108541310;

				float log10(float x) {
					return log2(x) * invlog210;
				}*/

				float t0 = 7.0/4.0 * PI;
				float t1 = 13.0/4.0 * PI;
				vec3 black = vec3(0.0,0.0,0.0);
				vec3 yellow = vec3(1.0,1.0,0.0);
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
					
					vec2 valueP = vec2(cos(tValue), sin(tValue)) * r;
					float vd = length(p-valueP);
				   
					// Make some sort of blobby thing out of the value dot
					// distance and the outline.
					float d = sqrt(knob_distance(p, r) * vd);
					
					// Start with black.
					//vec3 c = black;
					vec4 c = vec4(0.0);
					
					// Calculate the color for the background layer.
					vec4 bg = vec4(bgColor, 1.0-smoothstep(30.0 * s, 35.0 * s, d));
					c = blend(c, bg);
					
					// Calculate the color for the value dot layer.
					vec4 vdc = vec4(yellow, 1.0-smoothstep(19.0 * s, 20.0 * s, vd));
					c = blend(c, vdc);
					
					return c;
				}

				void main() {
					// overlay knob
					//vec2 uv = fragCoord.xy - .5 * iResolution.xy;
					vec2 p = uv - 0.5;
					color = blend(color, knob(p - vec2(0.5, 0.5), 0.25, val));
				}
			"#, None).unwrap(),
		}
	}
	/*fn calc_vertex_buffer(queue: &[Knob]) -> VertexBufferAny {

	}
	pub fn render<T: Surface>(&self, target: &mut T, vertex_buffers: &[VertexBufferAny]) {
		let uniforms = uniform! {};
		params: glium::DrawParameters {
			blend: glium::Blend::alpha_blending(),
			depth: glium::Depth {
				test: glium::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},
			..Default::default()
		},
			target.draw(vertex_buffer, glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &self.grid_program, &uniforms, params).unwrap();
	}*/
}
