pub mod instances;

#[cfg(test)]
mod tests {
    use crate::instances::color::*;

    #[test]
    fn parse_color_sequence_from_rgb() {
        let escape: char = char::from_u32(0x1b).unwrap();
        let result = color::make_color_from_rgb(64, 128, 255);
        assert_eq!(result.plain_sequence, "64;128;255");
        assert_eq!(result.plain_sequence_enclosed, "\"64;128;255\"");
        assert_eq!(result.vt_sequence_foreground, format!("{escape}[38;2;64;128;255m"));
        assert_eq!(result.vt_sequence_background, format!("{escape}[48;2;64;128;255m"));
        assert_eq!(result.r, 64);
        assert_eq!(result.g, 128);
        assert_eq!(result.b, 255);
        assert_eq!(result.hex, "#4080FF");
        assert_eq!(result.color_type, color::ColorType::ColorTrue);
    }

    #[test]
    fn parse_color_sequence_from_num_255() {
        let escape: char = char::from_u32(0x1b).unwrap();
        let result = color::make_color_from_num(40);
        assert_eq!(result.plain_sequence, "0;215;0");
        assert_eq!(result.plain_sequence_enclosed, "\"0;215;0\"");
        assert_eq!(result.vt_sequence_foreground, format!("{escape}[38;2;0;215;0m"));
        assert_eq!(result.vt_sequence_background, format!("{escape}[48;2;0;215;0m"));
        assert_eq!(result.r, 0);
        assert_eq!(result.g, 215);
        assert_eq!(result.b, 0);
        assert_eq!(result.hex, "#00D700");
        assert_eq!(result.color_type, color::ColorType::Color255);
    }

    #[test]
    fn parse_color_sequence_from_num_16() {
        let escape: char = char::from_u32(0x1b).unwrap();
        let result = color::make_color_from_num(1);
        assert_eq!(result.plain_sequence, "128;0;0");
        assert_eq!(result.plain_sequence_enclosed, "\"128;0;0\"");
        assert_eq!(result.vt_sequence_foreground, format!("{escape}[38;2;128;0;0m"));
        assert_eq!(result.vt_sequence_background, format!("{escape}[48;2;128;0;0m"));
        assert_eq!(result.r, 128);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 0);
        assert_eq!(result.hex, "#800000");
        assert_eq!(result.color_type, color::ColorType::Color16);
    }

    #[test]
    fn parse_color_sequence_from_hex_code() {
        let escape: char = char::from_u32(0x1b).unwrap();
        let result = color::make_color_from_specifier(String::from("#4080FF"));
        assert_eq!(result.plain_sequence, "64;128;255");
        assert_eq!(result.plain_sequence_enclosed, "\"64;128;255\"");
        assert_eq!(result.vt_sequence_foreground, format!("{escape}[38;2;64;128;255m"));
        assert_eq!(result.vt_sequence_background, format!("{escape}[48;2;64;128;255m"));
        assert_eq!(result.r, 64);
        assert_eq!(result.g, 128);
        assert_eq!(result.b, 255);
        assert_eq!(result.hex, "#4080FF");
        assert_eq!(result.color_type, color::ColorType::ColorTrue);
    }
}
