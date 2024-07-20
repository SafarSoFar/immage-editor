#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::arch::x86_64;
use std::{fs::File, io::Read};

use eframe::egui;
use egui::{ColorImage, Image, ImageData, TextureOptions, Vec2};
use image::io::Reader;
use image::{GenericImage, GenericImageView, Rgba};
use image::{DynamicImage, EncodableLayout, ImageBuffer};
use std::io::Error;

const WINDOW_SIZE : Vec2 = Vec2{x: 800.0, y :500.0};

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };  

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;
    
    // required to use on init state to prevent freezes
    let mut main_dynamic_image = img_path_to_dyn_image("forest.jpg");
    
    //let color_image = img_path_to_color_image("forest.jpg").unwrap();
    let mut color_image = dynamic_image_to_color_image(main_dynamic_image.clone());
    
    
    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| -> Result<(), Error> {
            
            // Required in order to use images in egui 
            egui_extras::install_image_loaders(ctx);
            
            
            let mut r_level : u8 = 0;
            let mut g_level : u8 = 0;
            let mut b_level : u8 = 0;
            
            ui.heading("I'm mage editor");
            let r_level_slider = ui.add(egui::Slider::new(&mut r_level, 0..=255).text("Red level"));
            let b_level_slider = ui.add(egui::Slider::new(&mut g_level, 0..=255).text("Green level"));
            let g_level_slider = ui.add(egui::Slider::new(&mut b_level, 0..=255).text("Blue level"));
            
            if r_level_slider.drag_stopped() || b_level_slider.drag_stopped() || g_level_slider.drag_stopped(){
                main_dynamic_image = DynamicImageChanger::change_color_level(main_dynamic_image.clone(), r_level, g_level, b_level);
                color_image = dynamic_image_to_color_image(main_dynamic_image.clone());
            }

            ui.add(egui::Slider::new(&mut age, 0..=255).text("binary threshold"));
            if ui.button("Increment").clicked(){
                age += 1;
            }
            
            
            let image_view_texture_handle =  ctx.load_texture("main_image", color_image.clone(), TextureOptions::LINEAR);
            let sized_image = egui::load::SizedTexture::new(image_view_texture_handle.id(), egui::vec2(color_image.size[0] as f32, color_image.size[1] as f32));
            let image = egui::Image::from_texture(sized_image).max_size(egui::Vec2{x: 100.0, y : 100.0});
            ui.label(format!("Hello '{name}', age {age}"));
            ui.add(image);
            
            Ok(())
        });
    })
}




struct DynamicImageChanger;

impl DynamicImageChanger{
    pub fn change_color_level(dyn_img : DynamicImage, r_level : u8, g_level : u8, b_level : u8) -> DynamicImage{
        let mut new_dyn_img : DynamicImage = dyn_img.clone();
        for pixel in dyn_img.pixels(){
            let pos_x = pixel.0;
            let pos_y = pixel.1;
            let mut rgba = pixel.2;
            rgba[0] = rgba[0].wrapping_sub(r_level);
            rgba[1] = rgba[1].wrapping_sub(g_level);
            rgba[2] = rgba[2].wrapping_sub(b_level);
            new_dyn_img.put_pixel(pos_x, pos_y, rgba);
        }
        new_dyn_img
    }
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

pub fn img_path_to_dyn_image(img_path : &str) -> DynamicImage{
    let dyn_img = Reader::open(&img_path).expect("couldn't open image").decode().expect("couldn't decode image");
    dyn_img
    

}

