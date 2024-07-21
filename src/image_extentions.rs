use image::{DynamicImage,GenericImageView, GenericImage};
pub struct DynamicImageExtender;
    impl DynamicImageExtender{
        ///rbg levels are i16 because of main application sliders negative values usage from -255 to 255
        pub fn change_color_level(dyn_img : DynamicImage, r_level : i16, g_level : i16, b_level : i16) -> DynamicImage{
            let mut new_dyn_img : DynamicImage = dyn_img.clone();
            for pixel in dyn_img.pixels(){
                let pos_x = pixel.0;
                let pos_y = pixel.1;
                let mut rgba = pixel.2;

                if r_level > 0{
                    rgba[0] = rgba[0].saturating_add(r_level as u8);
                }
                else{

                    rgba[0] = rgba[0].saturating_sub(r_level as u8);
                }
                if g_level > 0{
                    rgba[1] = rgba[1].saturating_add(g_level as u8);
                }
                else{

                    rgba[1] = rgba[1].saturating_sub(g_level as u8);
                }
                if b_level > 0{
                    rgba[2] = rgba[2].saturating_add(b_level as u8);
                }
                else{
                    rgba[2] = rgba[2].saturating_sub(b_level as u8);
                }
                //rgba[1] = (rgba[1] as i16 + g_level as i16) as u8;
                //rgba[2] = (rgba[2] as i16 + b_level as i16) as u8;
                //rgba[1] = rgba[1].wrapping_add(g_level);
                //rgba[2] = rgba[2].wrapping_add(b_level);
                new_dyn_img.put_pixel(pos_x, pos_y, rgba);
            }
            new_dyn_img
        }
    }