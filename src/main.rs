use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use std::path::Path;
use curl::easy::Easy;
use std::cmp::PartialEq;
use egui::Context;
use egui::Ui::ui;

enum simpleEnum {
    rouge, 
    bleu, 
    vert,
}

fn main() {
    ui.label("This is a label");
    ui.hyperlink("https://github.com/emilk/egui");
    ui.text_edit_singleline(&mut "hello".to_owned());
    if ui.button("Click me").clicked() { }
    ui.add(egui::Slider::new(&mut 42, 0.0..=100.0));
    ui.add(egui::DragValue::new(&mut 50));

    ui.checkbox(&mut true, "Checkbox");

    #[derive(PartialEq)]
    enum Enum { First, Second, Third }
    ui.horizontal(|ui| {
        ui.radio_value(&mut simpleEnum::bleu, Enum::First, "First");
        ui.radio_value(&mut simpleEnum::rouge, Enum::Second, "Second");
        ui.radio_value(&mut simpleEnum::vert, Enum::Third, "Third");
    });

    ui.separator();

    ui.collapsing("Click to see what is hidden!", |ui| {
        ui.label("Not much, as it turns out");
    });
    }


    fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
        // Put the buttons and label on the same row:
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                *counter -= 1;
            }
            ui.label(counter.to_string());
            if ui.button("+").clicked() {
                *counter += 1;
            }
        });
}

fn download(name: String, file: String, path: String) {
    let wanted_name = name;
    let wanted_file = file;
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
    println!("File downloaded successfully!");
}
