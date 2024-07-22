use egui::ColorImage;
use image::{DynamicImage,GenericImageView, GenericImage, EncodableLayout};
pub struct ImageWidgetExtender{original_image : DynamicImage, edited_image : DynamicImage}


    impl ImageWidgetExtender{

        pub fn new(original_image : DynamicImage) -> ImageWidgetExtender{
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
                self.edited_image.put_pixel(pos_x, pos_y, rgba);
            }
            self.edited_image.clone()
        }

        pub fn change_image_brightness(&mut self, brightness : i16) -> DynamicImage{
            for pixel in self.original_image.pixels(){

                let mut rgba = pixel.2;
                if brightness > 0{
                    rgba[0] = rgba[0].saturating_add(brightness as u8);
                    rgba[1] = rgba[1].saturating_add(brightness as u8);
                    rgba[2] = rgba[2].saturating_add(brightness as u8);
                }
                else{
                    let signed_brightness = brightness.abs() as u8;
                    rgba[0] = rgba[0].saturating_sub(signed_brightness);
                    rgba[1] = rgba[1].saturating_sub(signed_brightness);
                    rgba[2] = rgba[2].saturating_sub(signed_brightness);
                }
                self.edited_image.put_pixel(pixel.0, pixel.1, rgba);
            }
            self.edited_image.clone()
        } 

        pub fn create_canvas_dynamic_image() -> DynamicImage{
            let dyn_img = DynamicImage::new(500,500,image::ColorType::Rgb8);
            dyn_img
        }

        pub fn dynamic_image_to_color_image(dyn_img : DynamicImage) -> ColorImage{
            let color_image = match &dyn_img {
                DynamicImage::ImageRgb8(image) => {
                    egui::ColorImage::from_rgb(
                        [image.width() as usize,image.height() as usize],
                        image.as_bytes(),
                    )
                },
                other => {
                    let image = other.to_rgba8();
                    egui::ColorImage::from_rgba_unmultiplied(
                    [image.width() as usize, image.height() as usize],
                        image.as_bytes()
                    )
                },
            };
            color_image
        }
    }