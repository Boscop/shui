use util::V;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::borrow::Cow;
use std::io::Result;

use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Program;
use glium::texture::Texture2d;
use glium::Surface;

use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use rusttype::Rect;

use num::Float;

pub type MyColor = [f32; 4];

pub struct MyFont<'a> {
	font: Font<'a>,
	dpi_factor: f64,
	cache: Cache,
	program: Program,
	cache_tex: Texture2d,
	last_screen_height: u32
}
impl<'a> MyFont<'a> {
	pub fn new<P: AsRef<Path>>(display: &GlutinFacade, path: P) -> Result<MyFont<'a>> {
		let (w, h) = display.get_framebuffer_dimensions();
		//let (w, h) = (512, 512);
		let dpi_factor = display.get_window().unwrap().hidpi_factor();
		let (cache_width, cache_height) = (w * dpi_factor as u32, h * dpi_factor as u32);
		Ok(MyFont {
			font: FontCollection::from_bytes({
				let mut f = try!(File::open(path));
				let mut font_data = Vec::new();
				try!(f.read_to_end(&mut font_data));
				font_data
			}).into_font().unwrap(),
			dpi_factor: dpi_factor as f64,
			cache:  Cache::new(cache_width, cache_height, 0.1, 0.1),
			program: program!(display, 140 => {
				vertex: "
					#version 140

					in vec2 position;
					in vec2 tex_coords;
					in vec4 color;

					out vec2 v_tex_coords;
					out vec4 v_color;

					void main() {
						gl_Position = vec4(position, 0.0, 1.0);
						v_tex_coords = tex_coords;
						v_color = color;
					}
				",
				fragment: "
					#version 140

					uniform sampler2D tex;
					in vec2 v_tex_coords;
					in vec4 v_color;
					
					out vec4 f_color;

					void main() {
                        f_color = v_color * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
					}
				"
			}).unwrap(),
			cache_tex: glium::texture::Texture2d::with_format(
				display,
				glium::texture::RawImage2d {
					data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
					width: cache_width,
					height: cache_height,
					format: glium::texture::ClientFormat::U8
				},
				glium::texture::UncompressedFloatFormat::U8,
				glium::texture::MipmapsOption::NoMipmap).unwrap(),
			last_screen_height: 0,
		})
	}
	/*pub fn draw_string<T: Surface>(&mut self, display: &GlutinFacade, target: &mut T, text: &str, pos: V, size: f32, color: MyColor) {
		let (width, height) = display.get_framebuffer_dimensions();
		let (screen_width, screen_height) = (width as f32, height as f32);
		//let screen_width = target.get_dimensions().0;
		let text_width = (screen_width * (1. - pos.x)) as u32;
		let glyphs = layout_paragraph(&self.font, Scale::uniform(size * self.dpi_factor as f32), text_width, text);
		for glyph in &glyphs {
			self.cache.queue_glyph(0, glyph.clone());
		}

		let cache_tex = &self.cache_tex;
		self.cache.cache_queued(|rect, data| {
			cache_tex.main_level().write(glium::Rect {
				left: rect.min.x,
				bottom: rect.min.y,
				width: rect.width(),
				height: rect.height()
			}, glium::texture::RawImage2d {
				data: Cow::Borrowed(data),
				width: rect.width(),
				height: rect.height(),
				format: glium::texture::ClientFormat::U8
			});
		}).unwrap();

		let uniforms = uniform! {
			tex: self.cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
		};

		let pos = V::new(pos.x, pos.y - 1.) * 2.;

		let vertex_buffer = {
			//let origin = point(/*0.9*/0., 0.);
			let origin = point(pos.x, pos.y);
			let vertices: Vec<GlyphVertex> = glyphs.iter().flat_map(|g| {
				if let Ok(Some((uv_rect, screen_rect))) = self.cache.rect_for(0, g) {
					let gl_rect = Rect {
						min: origin + (vector(screen_rect.min.x as f32 / screen_width - 0.5,
						                1.0 - screen_rect.min.y as f32 / screen_height - 0.5)) * 2.0,
						max: origin + (vector(screen_rect.max.x as f32 / screen_width - 0.5,
						                1.0 - screen_rect.max.y as f32 / screen_height - 0.5)) * 2.0
					};
					//arrayvec::ArrayVec::<[GlyphVertex; 6]>::from([
					vec![
						GlyphVertex {
							position: [gl_rect.min.x, gl_rect.max.y],
							tex_coords: [uv_rect.min.x, uv_rect.max.y],
							color: color
						},
						GlyphVertex {
							position: [gl_rect.min.x,  gl_rect.min.y],
							tex_coords: [uv_rect.min.x, uv_rect.min.y],
							color: color
						},
						GlyphVertex {
							position: [gl_rect.max.x,  gl_rect.min.y],
							tex_coords: [uv_rect.max.x, uv_rect.min.y],
							color: color
						},
						GlyphVertex {
							position: [gl_rect.max.x,  gl_rect.min.y],
							tex_coords: [uv_rect.max.x, uv_rect.min.y],
							color: color
						},
						GlyphVertex {
							position: [gl_rect.max.x, gl_rect.max.y],
							tex_coords: [uv_rect.max.x, uv_rect.max.y],
							color: color
						},
						GlyphVertex {
							position: [gl_rect.min.x, gl_rect.max.y],
							tex_coords: [uv_rect.min.x, uv_rect.max.y],
							color: color
						}]
				} else {
					//arrayvec::ArrayVec::new()
					vec![]
				}
			}).collect();
			glium::VertexBuffer::new(display, &vertices).unwrap()
		};

		target.draw(&vertex_buffer,
			glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
			&self.program, &uniforms,
			&glium::DrawParameters {
				blend: glium::Blend::alpha_blending(),
				..Default::default()
		}).unwrap();
	}*/
	pub fn string_vertices(&mut self, display: &GlutinFacade, text: &str, pos: V, size: f32, color: MyColor, centered: Centered) -> Vec<GlyphVertex> {
		let (width, height) = display.get_framebuffer_dimensions();
		let (screen_width, screen_height) = (width as f32, height as f32);
		if height != self.last_screen_height {
			self.last_screen_height = height;
			self.cache.clear();
		}
		let size = size * screen_height / 400.;
		//let screen_width = target.get_dimensions().0;
		let text_width = (screen_width * (1. - pos.x)) as u32;
		let (glyphs, center_shift_x, center_shift_y) = layout_paragraph(&self.font, Scale::uniform(size * self.dpi_factor as f32), text_width, text);
		for glyph in &glyphs {
			self.cache.queue_glyph(0, glyph.clone());
		}

		let cache_tex = &self.cache_tex;
		self.cache.cache_queued(|rect, data| {
			cache_tex.main_level().write(glium::Rect {
				left: rect.min.x,
				bottom: rect.min.y,
				width: rect.width(),
				height: rect.height()
			}, glium::texture::RawImage2d {
				data: Cow::Borrowed(data),
				width: rect.width(),
				height: rect.height(),
				format: glium::texture::ClientFormat::U8
			});
		}).unwrap();

		let pos = V::new(pos.x, pos.y - 1.) * 2.;

		//let origin = point(/*0.9*/0., 0.);
		let mut origin = point(pos.x, pos.y);
		if centered.horz { origin.x += 2. * center_shift_x / screen_width; }
		if centered.vert { origin.y += 2. * center_shift_y / screen_height; }
		glyphs.iter().flat_map(|g| {
			if let Ok(Some((uv_rect, screen_rect))) = self.cache.rect_for(0, g) {
				let gl_rect = Rect {
					min: origin + (vector(screen_rect.min.x as f32 / screen_width - 0.5,
					                1.0 - screen_rect.min.y as f32 / screen_height - 0.5)) * 2.0,
					max: origin + (vector(screen_rect.max.x as f32 / screen_width - 0.5,
					                1.0 - screen_rect.max.y as f32 / screen_height - 0.5)) * 2.0
				};
				//arrayvec::ArrayVec::<[GlyphVertex; 6]>::from([
				vec![
					GlyphVertex {
						position: [gl_rect.min.x, gl_rect.max.y],
						tex_coords: [uv_rect.min.x, uv_rect.max.y],
						color: color
					},
					GlyphVertex {
						position: [gl_rect.min.x,  gl_rect.min.y],
						tex_coords: [uv_rect.min.x, uv_rect.min.y],
						color: color
					},
					GlyphVertex {
						position: [gl_rect.max.x,  gl_rect.min.y],
						tex_coords: [uv_rect.max.x, uv_rect.min.y],
						color: color
					},
					GlyphVertex {
						position: [gl_rect.max.x,  gl_rect.min.y],
						tex_coords: [uv_rect.max.x, uv_rect.min.y],
						color: color
					},
					GlyphVertex {
						position: [gl_rect.max.x, gl_rect.max.y],
						tex_coords: [uv_rect.max.x, uv_rect.max.y],
						color: color
					},
					GlyphVertex {
						position: [gl_rect.min.x, gl_rect.max.y],
						tex_coords: [uv_rect.min.x, uv_rect.max.y],
						color: color
					}]
			} else {
				//arrayvec::ArrayVec::new()
				vec![]
			}
		}).collect()
	}
	pub fn draw<T: Surface>(&self, display: &GlutinFacade, target: &mut T, vertices: &[GlyphVertex]) {
		let uniforms = uniform! {
			tex: self.cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
		};
		let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
		target.draw(&vertex_buffer,
			glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
			&self.program, &uniforms,
			&glium::DrawParameters {
				blend: glium::Blend::alpha_blending(),
				..Default::default()
		}).unwrap();
	}
}

pub struct Centered {
	horz: bool,
	vert: bool,
}
impl Centered {
	pub fn none() -> Centered { Centered { horz: false, vert: false } }
	pub fn horz() -> Centered { Centered { horz: true, vert: false } }
	pub fn vert() -> Centered { Centered { horz: false, vert: true } }
	pub fn both() -> Centered { Centered { horz: true, vert: true } }
}

/*trait RenderableQueue {
	type Renderable;
	fn render<T: Surface>(&self, display: &GlutinFacade, target: &mut T, r: &[Self::Renderable]);
}

impl<'a> RenderableQueue for MyFont<'a> {
	type Renderable = GlyphVertex;
	fn render<T: Surface>(&self, display: &GlutinFacade, target: &mut T, r: &[Self::Renderable]) {
		self.draw(display, target, r);
	}
}*/

/*pub struct QueueRenderable<T/*: Renderable*/> {
	rect: Rect,
	widget: T,
}*/

#[derive(Copy, Clone)]
pub struct GlyphVertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
	color: [f32; 4]
}
implement_vertex!(GlyphVertex, position, tex_coords, color);

fn layout_paragraph<'a>(font: &'a Font, scale: Scale, _width: u32, text: &str) -> (Vec<PositionedGlyph<'a>>, f32, f32) {
	use unicode_normalization::UnicodeNormalization;
	let mut result = Vec::new();
	let v_metrics = font.v_metrics(scale);
	//let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
	let advance_height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap) / 2.;
	let mut caret = point(0., /*v_metrics.ascent*/0.);
	let mut last_glyph_id = None;
	let mut max_width = 0.;
	let mut max_height = advance_height;
	//println!("! {:?} {}", v_metrics, advance_height);
	for c in text.nfc() {
		if c.is_control() {
			match c {
				'\r' => {},
				'\n' => {
					caret = point(0., caret.y + advance_height);
					max_height += advance_height;
				}
				_ => {}
			}
			continue;
		}
		let base_glyph = if let Some(glyph) = font.glyph(c) {
			glyph
		} else {
			continue;
		};
		if let Some(id) = last_glyph_id.take() {
			caret.x += font.pair_kerning(scale, id, base_glyph.id());
		}
		last_glyph_id = Some(base_glyph.id());
		let /*mut*/ glyph = base_glyph.scaled(scale).positioned(caret);
		/*if let Some(bb) = glyph.pixel_bounding_box() {
			if bb.max.x > width as i32 {
				caret = point(0., caret.y + advance_height);
				glyph = glyph.into_unpositioned().positioned(caret);
				last_glyph_id = None;
			}
		}*/
		caret.x += glyph.unpositioned().h_metrics().advance_width;
		result.push(glyph);
		max_width = max_width.max(caret.x);
	}
	(result, -max_width / 2., max_height / 2. - advance_height)
}

//fn string_pxl_width<'a>(font: &'a Font, scale: Scale, text: &str) -> f32 {}
