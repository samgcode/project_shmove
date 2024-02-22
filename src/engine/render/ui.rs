use wgpu::{CommandEncoder, Device, SurfaceConfiguration, TextureView};
use wgpu_glyph::{ab_glyph, Section, Text};

pub use text::TextObject;

use crate::engine::Color;

pub mod text;

const FONT_BYTES: &[u8] = include_bytes!("../../../res/font/PressStart2P-Regular.ttf");

pub struct UIState {
  glyph_brush: wgpu_glyph::GlyphBrush<()>,
  staging_belt: wgpu::util::StagingBelt,
}

impl UIState {
  pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
    let font = ab_glyph::FontArc::try_from_slice(FONT_BYTES).unwrap();
    let glyph_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font).build(device, config.format);
    let staging_belt = wgpu::util::StagingBelt::new(1024);

    Self {
      glyph_brush,
      staging_belt,
    }
  }

  pub fn render(
    &mut self,
    device: &Device,
    encoder: &mut CommandEncoder,
    view: &TextureView,
    config: &SurfaceConfiguration,
  ) {
    self
      .glyph_brush
      .draw_queued(
        device,
        &mut self.staging_belt,
        encoder,
        &view,
        config.width,
        config.height,
      )
      .unwrap();

    self.staging_belt.finish();
  }

  pub fn draw_text(&mut self, text_objects: Vec<&TextObject>) {
    for text in text_objects.iter() {
      if text.enabled {
        self.draw_text_object(text, text.shadowed);
      }
    }
  }

  fn draw_text_object(&mut self, text: &TextObject, shadowed: bool) {
    if shadowed {
      let mut shadow = text.clone();
      shadow.position.x += 2.0;
      shadow.position.y += 2.0;
      shadow.color = Color::from_inverted(&text.color);
      self.draw_text_object(&shadow, false);
    }
    let layout = wgpu_glyph::Layout::default().h_align(if text.centered {
      wgpu_glyph::HorizontalAlign::Center
    } else {
      wgpu_glyph::HorizontalAlign::Left
    });

    let section = Section {
      screen_position: text.position.into(),
      bounds: text.bounds.into(),
      layout,
      ..Section::default()
    }
    .add_text(
      Text::new(&text.text)
        .with_color(text.color.to_vec4())
        .with_scale(text.size),
    );

    self.glyph_brush.queue(section);
  }
}
