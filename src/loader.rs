use runner::TextureId;
use error::{self,LoadShaderError};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use gfx;
use image;

pub static DEFAULT_VERT_SRC_BUF: &'static [u8] = include_bytes!("./shaders/elevated.vert");
pub static DEFAULT_FRAG_SRC_STR: &'static str  = include_str!("./shaders/elevated.frag");

pub static DEFAULT_TEXTURE0_BUF: &'static [u8] = include_bytes!("./textures/channel0.png");
pub static DEFAULT_TEXTURE1_BUF: &'static [u8] = include_bytes!("./textures/channel1.jpg");
pub static DEFAULT_TEXTURE2_BUF: &'static [u8] = include_bytes!("./textures/channel2.jpg");
pub static DEFAULT_TEXTURE3_BUF: &'static [u8] = include_bytes!("./textures/channel3.jpg");

const PREFIX: &str = "
    #version 150 core

    uniform float     iGlobalTime;
    uniform float     iTime;
    uniform vec3      iResolution;
    uniform vec4      iMouse;
    uniform int       iFrame;
    uniform sampler2D iChannel0;
    uniform sampler2D iChannel1;
    uniform sampler2D iChannel2;
    uniform sampler2D iChannel3;

    in vec2 fragCoord;
    out vec4 fragColor;
";

// Fragment shader suffix
const SUFFIX: &str = "
    void main() {
        mainImage(fragColor, fragCoord);
    }
";
pub fn format_shader_src(src: &str) -> Vec<u8> {
    format!("{}\n{}\n{}", PREFIX, src, SUFFIX).into_bytes()
}
pub fn load_fragment_shader() -> Vec<u8> {
    format_shader_src(&DEFAULT_FRAG_SRC_STR)
}
pub fn load_vertex_shader() -> Vec<u8> {
    DEFAULT_VERT_SRC_BUF.to_vec()
}
pub fn load_texture<F, R>(id: &TextureId, texpath: &str, factory: &mut F) ->
        error::Result<gfx::handle::ShaderResourceView<R, [f32; 4]>>
    where F: gfx::Factory<R>,
          R: gfx::Resources
{
    use gfx::format::Rgba8;
    use gfx::texture::Mipmap;

    let default_buf = if texpath.is_empty() {
        None
    } else {
        match *id {
            TextureId::ZERO  => Some(DEFAULT_TEXTURE0_BUF),
            TextureId::ONE   => Some(DEFAULT_TEXTURE1_BUF),
            TextureId::TWO   => Some(DEFAULT_TEXTURE2_BUF),
            TextureId::THREE => Some(DEFAULT_TEXTURE3_BUF),
        }
    };

    let img = if let Some(default_buf) = default_buf {
        image::load_from_memory(default_buf)?.flipv().to_rgba()
    } else {
        image::open(&texpath.clone())?.flipv().to_rgba()
    };

    let (w, h) = img.dimensions();
    let kind = gfx::texture::Kind::D2(w as u16, h as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, Mipmap::Allocated, &[&img])?;
    Ok(view)
}