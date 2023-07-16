pub mod colors_info {
    use serde_json::Value;

    pub struct ColorInfo {
        pub color_id: u8,
        pub rgb     : ColorRgbInfo,
    }

    pub struct ColorRgbInfo {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    pub fn get_color_info(num: u8) -> ColorInfo {
        // Get the parsed JSON info
        let color_data_json = include_str!("../resources/colors_data.json");
        let color_data_val: Vec<Value> = serde_json::from_str(color_data_json).unwrap();
        
        // Now, deal with enumerating the objects that hold color data
        let mut color_info: ColorInfo = ColorInfo {
            color_id: 0,
            rgb:      ColorRgbInfo {
                r: 0,
                g: 0,
                b: 0,
            }
        };
        for color_data_val_obj in color_data_val {
            let color_data = color_data_val_obj.as_object().unwrap();
            let color_id: u8 = color_data["colorId"].as_u64().unwrap().try_into().unwrap();
            if color_id == num {
                let color_rgb: ColorRgbInfo = ColorRgbInfo {
                    r: color_data["rgb"]["r"].as_u64().unwrap().try_into().unwrap(),
                    g: color_data["rgb"]["g"].as_u64().unwrap().try_into().unwrap(),
                    b: color_data["rgb"]["b"].as_u64().unwrap().try_into().unwrap(),
                };
                color_info = ColorInfo {
                    color_id: color_id,
                    rgb:      color_rgb,
                }
            } 
        }

        // Return the info
        return color_info;
    }
}
