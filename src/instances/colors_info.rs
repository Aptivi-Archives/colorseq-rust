//! Colors Info
//!
//! This module allows you to directly get information from the console color number from 0 to 255.
pub mod colors_info {
    use serde_json::Value;

    /// Color information
    pub struct ColorInfo {
        /// Color ID. It stores the console color number.
        pub color_id: u8,
        /// Color RGB information
        pub rgb     : ColorRgbInfo,
    }

    /// Color RGB information
    pub struct ColorRgbInfo {
        /// The red color level
        pub r: u8,
        /// The green color level
        pub g: u8,
        /// The blue color level
        pub b: u8,
    }

    /// Gets the color information from a specified color number
    ///
    /// # Examples
    ///
    /// ```
    /// use colorseq::instances::colors_info::colors_info::*;
    /// let spec_info: ColorInfo = get_color_info(45);
    /// assert_eq!(spec_info.rgb.r, 0);
    /// assert_eq!(spec_info.rgb.g, 215);
    /// assert_eq!(spec_info.rgb.b, 255);
    /// assert_eq!(spec_info.color_id, 45);
    /// ```
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
