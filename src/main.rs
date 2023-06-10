#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Write;
use std::path::Path;
use curl::{easy::Easy};
use eframe::egui;
use curl_sys;
use egui::{Color32, TextStyle};
use std::panic;
use std::error::Error;

pub type Result<T, E = dyn Error> = std::result::Result<T, E>;

/*
pub struct Error{
    pub code: curl_sys::CURLcode,
    pub extra: Option<Box<str>>,
}*/

fn main() -> Result<(), eframe::Error>  {

    //defining the color used


    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut name = "exemple.png".to_owned();
    let mut path = "/path/to/storage".to_owned();
    let mut url = "https://site.com/image.png".to_owned();
    let mut resultat = String::new();
    let mut result_label_color = Color32::from_rgb(255, 255, 255);

    eframe::run_simple_native("Curl 0.0.2", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("The download service of your dreams");
            ui.horizontal(|ui| {
                let name_label = ui.label("file name :");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let path_label = ui.label("file path :");
                ui.text_edit_singleline(&mut path)
                    .labelled_by(path_label.id);
            });
            ui.horizontal(|ui| {
                let url_label = ui.label("file url  :");
                ui.text_edit_singleline(&mut url)
                    .labelled_by(url_label.id);
            });

            
            ui.colored_label(result_label_color ,resultat.clone());

            if ui.button("Download").clicked() {
                let result = panic::catch_unwind(||{
                    download(name.clone(), url.clone(), path.clone());
                });
                match result {
                    Ok(()) => {
                        resultat = "success".to_owned();
                        result_label_color = Color32::from_rgb(0, 255, 0)},
                    Err(error) => {
                        result_label_color = Color32::from_rgb(255, 0, 0);
                        if let Some(err) = error.downcast_ref::<Box<dyn Error>>() {
                            resultat = err.as_ref().to_string();
                        } else {
                            resultat = "the url or the path might be unusable".to_owned();
                        }
                    }
                }
            }
        });
    })
}

fn download(name: String, url: String, path: String) {
    let wanted_name = name;
    let wanted_file = url;
    let binding = &(path.trim().to_owned() + wanted_name.trim());
    let path_name = Path::new(binding);

    let mut file = File::create(path_name).expect("Unable to create the output file");

    let mut easy = Easy::new();
    println!("Fetching from: {}", &wanted_file);
    easy.url(&wanted_file.trim()).unwrap();
    easy.follow_location(true).unwrap();

    easy.write_function(move |data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();

    easy.perform().unwrap();
}
