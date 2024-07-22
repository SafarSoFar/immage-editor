#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

//use std::arch::x86_64;
use std::{fs::File, io::Read};

use eframe::egui;
use egui::{pos2, Align2, Color32, ColorImage, Image, ImageData, Pos2, Rounding, Stroke, TextureOptions, Vec2};
use image::io::Reader;
use image::{GenericImage, GenericImageView, Rgba};
use image::{DynamicImage, EncodableLayout, ImageBuffer};
use std::io::Error;

mod image_extentions;
use image_extentions::{ImageWidgetExtender, ImageEffectHandler};

const WINDOW_SIZE : Vec2 = Vec2{x: 800.0, y :500.0};

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };  

    // Our application state:
    let mut r_level : i16 = 0;
    let mut g_level : i16 = 0;
    let mut b_level : i16 = 0;
    let mut brightness : i16 = 0;


    // required image initializatoin on set up stage to prevent freezes
    let original_dynamic_image = img_path_to_dyn_image("forest.jpg");
    
    let mut image_effect_handler = ImageEffectHandler::new(original_dynamic_image.clone());

    //let color_image = img_path_to_color_image("forest.jpg").unwrap();
    let mut color_image = ImageWidgetExtender::dynamic_image_to_color_image(original_dynamic_image.clone());
    
    
    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| -> Result<(), Error> {
            
            // Required in order to use images in egui 
            egui_extras::install_image_loaders(ctx);
            
            
            let mut edited_dynamic_image = original_dynamic_image.clone(); 
            ui.heading("I'm mage editor");
            let r_level_slider = ui.add(egui::Slider::new(&mut r_level, -255..=255).text("Red level"));
            let b_level_slider = ui.add(egui::Slider::new(&mut g_level, -255..=255).text("Green level"));
            let g_level_slider = ui.add(egui::Slider::new(&mut b_level, -255..=255).text("Blue level"));
            let brightness_slider = ui.add(egui::Slider::new(&mut brightness, -255..=255).text("Brightness"));

            if r_level_slider.drag_stopped() || b_level_slider.drag_stopped() || g_level_slider.drag_stopped(){
                edited_dynamic_image = image_effect_handler.change_color_level(r_level, g_level, b_level);
                color_image = ImageWidgetExtender::dynamic_image_to_color_image(edited_dynamic_image);
            }
            
            if brightness_slider.drag_stopped(){
                edited_dynamic_image = image_effect_handler.change_image_brightness(brightness); 
                color_image = ImageWidgetExtender::dynamic_image_to_color_image(edited_dynamic_image);
            }
            
            
            let image_view_texture_handle =  ctx.load_texture("main_image", color_image.clone(), TextureOptions::LINEAR);
            let sized_image = egui::load::SizedTexture::new(image_view_texture_handle.id(), egui::vec2(color_image.size[0] as f32, color_image.size[1] as f32));
            let image = egui::Image::from_texture(sized_image).max_size(egui::Vec2{x: 300.0, y : 300.0});
            ui.add(image);
            //let (image_rectangle, _response) = ui.allocate_at_least(Vec2::new(350.0,350.0), egui::Sense::click_and_drag());
            //Align2::anchor_rect(Align2::RIGHT_BOTTOM, image_rectangle);
            /*ui.painter().rect(
                image_rectangle,
                90.0,
                Color32::from_gray(64),
                Stroke::new(1.0, Color32::WHITE), 
            );*/
            //image.paint_at(ui, image_rectangle);
            
            
            Ok(())
        });
    })
}



pub fn img_path_to_dyn_image(img_path : &str) -> DynamicImage{
    let dyn_img = Reader::open(&img_path).expect("couldn't open image").decode().expect("couldn't decode image");
    dyn_img
    

}

