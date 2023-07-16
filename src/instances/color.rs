pub mod color {
    use crate::instances::colors_info::colors_info::*;

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum ColorType {
        Color16,
        Color255,
        ColorTrue,
    }

    pub struct ColorFields {
        pub plain_sequence         : String,
        pub plain_sequence_enclosed: String,
        pub vt_sequence_foreground : String,
        pub vt_sequence_background : String,
        pub r                      : u8,
        pub g                      : u8,
        pub b                      : u8,
        pub hex                    : String,
        pub color_type             : ColorType,
    }
    
    pub fn make_color_from_rgb(r: u8, g: u8, b: u8) -> ColorFields {
        let specifier = format!("{};{};{}", r, g, b);
        return make_color_from_specifier(specifier);
    }

    pub fn make_color_from_num(num: u8) -> ColorFields {
        let specifier = format!("{}", num);
        return make_color_from_specifier(specifier);
    }

    pub fn make_color_from_specifier(specifier: String) -> ColorFields {
        // Some essential variables to install to the struct
        let mut plain_seq          = String::new();
        let mut plain_seq_enclosed = String::new();
        let mut vt_seq_fg          = String::new();
        let mut vt_seq_bg          = String::new();
        let mut r                  = 0;
        let mut g                  = 0;
        let mut b                  = 0;
        let mut hex                = String::new();
        let mut color_type         = ColorType::Color16;
        let escape : char          = char::from_u32(0x1b).unwrap();

        // Remove stray double quotes
        let spec = specifier.replace("\"", "");

        // Now, parse the output. First, start with the semicolon separator
        // for the three color values.
        if spec.contains(';') {
            // Split the VT sequence into three parts
            let spec_vec: Vec<&str> = spec.split(';').collect();
            if spec_vec.len() == 3 {
                // Get the RGB values
                let spec_vec_r: u8 = spec_vec[0].parse().unwrap();
                let spec_vec_g: u8 = spec_vec[1].parse().unwrap();
                let spec_vec_b: u8 = spec_vec[2].parse().unwrap();

                // Install the values
                plain_seq = format!("{spec_vec_r};{spec_vec_g};{spec_vec_b}");
                plain_seq_enclosed = format!("\"{plain_seq}\"");
                vt_seq_fg = format!("{escape}[38;2;{plain_seq}m");
                vt_seq_bg = format!("{escape}[48;2;{plain_seq}m");
                r = spec_vec_r;
                g = spec_vec_g;
                b = spec_vec_b;
                hex = format!("#{r:02X}{g:02X}{b:02X}");
                color_type = ColorType::ColorTrue;
            }
        }

        // If the string contains numbers
        else if spec.chars().all(char::is_numeric) {
            // Get the color info
            let spec_num: u8 = spec.parse().unwrap();
            let spec_info: ColorInfo = get_color_info(spec_num);

            // Install the values
            plain_seq = format!("{};{};{}",
                                spec_info.rgb.r,
                                spec_info.rgb.g,
                                spec_info.rgb.b);
            plain_seq_enclosed = format!("\"{plain_seq}\"");
            vt_seq_fg = format!("{escape}[38;2;{plain_seq}m");
            vt_seq_bg = format!("{escape}[48;2;{plain_seq}m");
            r = spec_info.rgb.r;
            g = spec_info.rgb.g;
            b = spec_info.rgb.b;
            hex = format!("#{r:02X}{g:02X}{b:02X}");
            if spec_num > 15 {
                color_type = ColorType::Color255;
            } else {
                color_type = ColorType::Color16;
            }
        }

        // If the string starts with the hashtag symbol to denote the hex
        // code for the specified color
        else if spec.starts_with('#') {
            // Get the color info from the hex code
            let spec_hexes = &spec[1..];
            let spec_decimal: u32 = u32::from_str_radix(spec_hexes, 16).unwrap();

            // Convert the RGB values to numbers
            let spec_r = (spec_decimal & 0xFF0000) >> 0x10;
            let spec_g = (spec_decimal & 0x00FF00) >> 0x08;
            let spec_b =  spec_decimal & 0x0000FF;

            // Install the values
            plain_seq = format!("{spec_r};{spec_g};{spec_b}");
            plain_seq_enclosed = format!("\"{plain_seq}\"");
            vt_seq_fg = format!("{escape}[38;2;{plain_seq}m");
            vt_seq_bg = format!("{escape}[48;2;{plain_seq}m");
            r = spec_r.try_into().unwrap();
            g = spec_g.try_into().unwrap();
            b = spec_b.try_into().unwrap();
            hex = format!("#{r:02X}{g:02X}{b:02X}");
            color_type = ColorType::ColorTrue;
        }

        // Return the color
        return ColorFields {
            plain_sequence         : plain_seq,
            plain_sequence_enclosed: plain_seq_enclosed,
            vt_sequence_foreground : vt_seq_fg,
            vt_sequence_background : vt_seq_bg,
            r                      : r,
            g                      : g,
            b                      : b,
            hex                    : hex,
            color_type             : color_type,
        }
    }
}
