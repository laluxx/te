use ab_glyph::{Font, FontArc, point}; // Glyph
use std::ffi::c_void;

pub struct FontAtlas {
    pub texture_id: u32,
    pub atlas_data: Vec<u8>,
    pub atlas_width: usize,
    pub atlas_height: usize,
}

impl FontAtlas {
    pub fn new(font_data: &'static [u8], scale: f32, atlas_width: usize, atlas_height: usize) -> Self {
        let font = FontArc::try_from_slice(font_data).expect("Error loading font");
        let mut atlas_data = vec![0; atlas_width * atlas_height];

        // Placeholder for starting position in the atlas
        let mut cursor = point(0.0, 0.0);
        let advance_height = 30.0; // Adjust based on your scale and font

        for character in 32..127u8 { // ASCII range
            let glyph = font.glyph_id(character as char).with_scale_and_position(scale, cursor);
            
            if let Some(outlined_glyph) = font.outline_glyph(glyph) {
                let bounds = outlined_glyph.px_bounds();
                outlined_glyph.draw(|x, y, coverage| {
                    let atlas_x = x + bounds.min.x as u32 + cursor.x as u32;
                    let atlas_y = y + bounds.min.y as u32 + cursor.y as u32;
                    if atlas_x < atlas_width as u32 && atlas_y < atlas_height as u32 {
                        let index = atlas_x as usize + atlas_y as usize * atlas_width;
                        atlas_data[index] = (coverage * 255.0) as u8;
                    }
                });

                cursor.x += bounds.width() + 5.0; // Add some padding
                if cursor.x >= atlas_width as f32 {
                    cursor.x = 0.0;
                    cursor.y += advance_height;
                }
            }
        }

        // Create an OpenGL texture
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32, atlas_width as i32, atlas_height as i32, 0, gl::RED, gl::UNSIGNED_BYTE, atlas_data.as_ptr() as *const c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        FontAtlas {
            texture_id,
            atlas_data,
            atlas_width,
            atlas_height,
        }
    }
}
