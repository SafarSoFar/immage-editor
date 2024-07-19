#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{fs::File, io::Read};

use eframe::egui;
use egui::{Image, ImageData};
use image::{DynamicImage, EncodableLayout};
use std::io::Error;

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| -> Result<(), Error> {
            
            // Required in order to use images in egui 
            egui_extras::install_image_loaders(ctx);

            ui.heading("I'm mage editor");
            ui.horizontal(|ui| {
                let name_label = ui.label("binary threshold text");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=255).text("binary threshold slider"));
            if ui.button("Increment").clicked(){
                age += 1;
            }

            //ui.label(format!("Hello '{name}', age {age}"));

            let mut image_buf = vec![];
            File::open("../forest.jpg")?.read_to_end(&mut image_buf)?;
            //let img = Image::from_bytes("hehe", &image_buf[..]);
            //ui.image(egui::include_image!("../forest.jpg"));
            //ui.add(egui::Image::new(egui::include_image!("../forest.jpg")).max_size(egui::Vec2 { x: (100.0), y: (100.0) }));
            Ok(())
        });
    })
}

/*pub fn convert_image(){
    let image = egui::Image::new(egui::include_image!("../forest.jpg"));
    let color_image = match &image {
        DynamicImage::ImageRgb8(image) => {
            egui::ColorImage::from_rgb(
                [image.width() as usize,image.height() as usize],
                image.as_bytes(),
            )
        },
        other => {
            let image = other.to_rgba8();
        }
    };
}*/

