//! #Color
//!
//! A master class for color manipulation

pub mod color {
    use crate::instances::colors_info::colors_info::*;

    /// The color type enumeration
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum ColorType {
        /// The target color is 0 - 15
        Color16,
        /// The target color is 0 - 255
        Color255,
        /// The target color is a true color
        ColorTrue,
    }

    /// The structure for color information
    pub struct ColorFields {
        /// Plain sequence that describes only the color sequence
        pub plain_sequence         : String,
        /// Same as above, but enclosed
        pub plain_sequence_enclosed: String,
        /// VT sequence string to set the foreground color of the console to the parsed color
        pub vt_sequence_foreground : String,
        /// VT sequence string tk set the background color of the console to the parsed color
        pub vt_sequence_background : String,
        /// The red color level
        pub r                      : u8,
        /// The green color level
        pub g                      : u8,
        /// The blue color level
        pub b                      : u8,
        /// The hexadecimal representation of the color. Used in HTML.
        pub hex                    : String,
        /// The color type
        pub color_type             : ColorType,
    }
    
    /// Makes a color structure from the RGB values
    ///
    /// # Examples
    ///
    /// ```
    /// use colorseq::instances::color::*;
    /// let escape: char = char::from_u32(0x1b).unwrap();
    /// let result = color::make_color_from_rgb(64, 128, 255);
    /// assert_eq!(result.plain_sequence, "64;128;255");
    /// assert_eq!(result.plain_sequence_enclosed, "\"64;128;255\"");
    /// assert_eq!(result.vt_sequence_foreground, format!("{escape}[38;2;64;128;255m"));
    /// assert_eq!(result.vt_sequence_background, format!("{escape}[48;2;64;128;255m"));
    /// assert_eq!(result.r, 64);
    /// assert_eq!(result.g, 128);
    /// assert_eq!(result.b, 255);
    /// assert_eq!(result.hex, "#4080FF");
    /// assert_eq!(result.color_type, color::ColorType::ColorTrue);
    /// ```
    pub fn make_color_from_rgb(r: u8, g: u8, b: u8) -> ColorFields {
        let specifier = format!("{};{};{}", r, g, b);
        return make_color_from_specifier(specifier);
    }

    /// Makes a color structure from the color number according to the console data
    ///
    /// # Examples
    ///
    /// To use this function on color number larger than or equal to 16:
    ///
    /// ```
    /// use colorseq::instances::color::*;
    /// let escape: char = char::from_u32(0x1b).unwrap();
    /// let result = color::make_color_from_num(40);
    /// assert_eq!(result.plain_sequence, "0;215;0");
    /// assert_eq!(result.plain_sequence_enclosed, "\"0;215;0\"");
    /// assert_eq!(result.vt_sequence_foreground, format!("{escape}[38;2;0;215;0m"));
    /// assert_eq!(result.vt_sequence_background, format!("{escape}[48;2;0;215;0m"));
    /// assert_eq!(result.r, 0);
    /// assert_eq!(result.g, 215);
    /// assert_eq!(result.b, 0);
    /// assert_eq!(result.hex, "#00D700");
    /// assert_eq!(result.color_type, color::ColorType::Color255);
    /// ```
    ///
    /// To use this function on color number smaller than 16:
    ///
    /// ```
    /// use colorseq::instances::color::*;
    /// let escape: char = char::from_u32(0x1b).unwrap();
    /// let result = color::make_color_from_num(1);
    /// assert_eq!(result.plain_sequence, "128;0;0");
    /// assert_eq!(result.plain_sequence_enclosed, "\"128;0;0\"");
    /// assert_eq!(result.vt_sequence_foreground, format!("{escape}[38;2;128;0;0m"));
    /// assert_eq!(result.vt_sequence_background, format!("{escape}[48;2;128;0;0m"));
    /// assert_eq!(result.r, 128);
    /// assert_eq!(result.g, 0);
    /// assert_eq!(result.b, 0);
    /// assert_eq!(result.hex, "#800000");
    /// assert_eq!(result.color_type, color::ColorType::Color16);
    /// ```
    pub fn make_color_from_num(num: u8) -> ColorFields {
        let specifier = format!("{}", num);
        return make_color_from_specifier(specifier);
    }

    /// Initializes a color structure from the given color specifier either in the format of
    /// "[R];[G];[B]", "#RRGGBB", or "[ColorNumber]".
    ///
    /// # Examples
    ///
    /// ```
    /// use colorseq::instances::color::*;
    /// let escape: char = char::from_u32(0x1b).unwrap();
    /// let result = color::make_color_from_specifier(String::from("#4080FF"));
    /// assert_eq!(result.plain_sequence, "64;128;255");
    /// assert_eq!(result.plain_sequence_enclosed, "\"64;128;255\"");
    /// assert_eq!(result.vt_sequence_foreground, format!("{escape}[38;2;64;128;255m"));
    /// assert_eq!(result.vt_sequence_background, format!("{escape}[48;2;64;128;255m"));
    /// assert_eq!(result.r, 64);
    /// assert_eq!(result.g, 128);
    /// assert_eq!(result.b, 255);
    /// assert_eq!(result.hex, "#4080FF");
    /// assert_eq!(result.color_type, color::ColorType::ColorTrue);
    /// ```
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
