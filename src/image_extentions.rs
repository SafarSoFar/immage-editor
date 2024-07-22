use image::{DynamicImage,GenericImageView, GenericImage};
pub struct DynamicImageExtender{original_image : DynamicImage, edited_image : DynamicImage}


    impl DynamicImageExtender{

        pub fn new(original_image : DynamicImage) -> DynamicImageExtender{
            Self{original_image : original_image.clone(), edited_image : original_image}
        }


        ///rbg levels are i16 because of main application sliders negative values usage from -255 to 255
        pub fn change_color_level(&mut self, r_level : i16, g_level : i16, b_level : i16) -> DynamicImage{


            //Temporary check until an image cut tool isn' implemented 
            assert_eq!(self.original_image.height(), self.edited_image.height());
            assert_eq!(self.original_image.width(), self.edited_image.width());

            for pixel in self.original_image.pixels(){
                let pos_x = pixel.0;
                let pos_y = pixel.1;
                let mut rgba = pixel.2;

                if r_level > 0{
                    rgba[0] = rgba[0].saturating_add(r_level as u8);
                }
                else{
                    rgba[0] = rgba[0].saturating_sub(r_level.abs() as u8);
                }
                if g_level > 0{
                    rgba[1] = rgba[1].saturating_add(g_level as u8);
                }
                else{

                    rgba[1] = rgba[1].saturating_sub(g_level.abs() as u8);
                }
                if b_level > 0{
                    rgba[2] = rgba[2].saturating_add(b_level as u8);
                }
                else{
                    rgba[2] = rgba[2].saturating_sub(b_level.abs() as u8);
                }
                //rgba[1] = (rgba[1] as i16 + g_level as i16) as u8;
                //rgba[2] = (rgba[2] as i16 + b_level as i16) as u8;
                //rgba[1] = rgba[1].wrapping_add(g_level);
                //rgba[2] = rgba[2].wrapping_add(b_level);
                self.edited_image.put_pixel(pos_x, pos_y, rgba);
            }
            self.edited_image.clone()
        }

        //p
    }