pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let int_value = u32::from_str_radix(hex, 16).unwrap();

        let r = ((int_value >> 16) & 255) as f32 / 255.0;
        let g = ((int_value >> 8) & 255) as f32 / 255.0;
        let b = (int_value & 255) as f32 / 255.0;
        let a = 1.0; // Default to fully opaque if not using alpha in the hex

        Color { r, g, b, a }
    }
}
