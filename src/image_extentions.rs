use image::{DynamicImage,GenericImageView, GenericImage};
pub struct DynamicImageExtender;
    impl DynamicImageExtender{
        pub fn change_color_level(dyn_img : DynamicImage, r_level : i16, g_level : i16, b_level : i16) -> DynamicImage{
            let mut new_dyn_img : DynamicImage = dyn_img.clone();
            for pixel in dyn_img.pixels(){
                let pos_x = pixel.0;
                let pos_y = pixel.1;
                let mut rgba = pixel.2;
                rgba[0] = (rgba[0] as i16 + r_level as i16) as u8;
                rgba[1] = (rgba[1] as i16 + g_level as i16) as u8;
                rgba[2] = (rgba[2] as i16 + b_level as i16) as u8;
                //rgba[1] = rgba[1].wrapping_add(g_level);
                //rgba[2] = rgba[2].wrapping_add(b_level);
                new_dyn_img.put_pixel(pos_x, pos_y, rgba);
            }
            new_dyn_img
        }
    }