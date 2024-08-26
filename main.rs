// ANDROID | TERMUX : apt install pkg-config

use download_rs::sync_download::Download;

use inquire::Text;

extern crate rustydav;
use rustydav::client;

use terminal_color_builder::OutputFormatter as tcb;

extern crate chrono;
use chrono::{DateTime, Local};

use csv::ReaderBuilder;

use rand::Rng;

use std::error::Error;
use std::fs;
use std::fs::read_dir;
use std::fs::File;

use std::path::PathBuf;
use zippylib::create_zip_archive;
use simple_zip::zip::Decompress;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    
    println!(
        "{}",
        tcb::new()
        .bg().magenta()
        .fg().green()
        .text_str("\n\t\t\t\t\t\t\tMusic Chooser")
        .print() // render to string
    );
        
    
    let ip_adress = Text::new("\nWhat is your IP server adress?")
        .with_default("http://")
        .with_initial_value("127.0.0.1")
        .with_help_message("by RUBINO Marc")
        .prompt();

    let ip_adress = format!("http://{}:8080/music.zip", ip_adress.unwrap());
    println!("OK, using: {}", ip_adress);

    let webdav_client = client::Client::init("", "");

    let file_existence = webdav_client.list(&ip_adress, "0");
    let status = match file_existence {
        Ok(_) => {
            let file_existence = file_existence.unwrap();
            let _mess = if !file_existence.status().is_success() {
                println!("file not found on the network");
                "CREATE"
            } else {
                println!("file found on the network");
                "DOWNLOAD"
            };
            _mess
        }
        Err(_) => {
            println!("server unreachable... using file locally...");
            "LOCAL"
        }
    };

    match status {
        "DOWNLOAD" => {
            let download = Download::new(&ip_adress, Some("music.zip"), None);
            match download.download() {
                Ok(_) => println!("Downloadind DB from {}", &ip_adress),
                Err(e) => println!("{:?}", e),
            }
            Decompress::local_str(&"music.zip");
        }

        _ => {
            if !fs::metadata("music.lst").is_ok() {
                let _ = write_csv(list_dir());
            }
        }
    }

    let metadata = fs::metadata("music.lst").unwrap().modified();
    let date: DateTime<Local> = metadata.unwrap().into();
    println!(
        "\n{}",
        tcb::new()
            .fg()
            .blue()
            .text_str(&*format!("DB last modified: {}", &date.to_string()[0..10]))
            .print()
    );

    let list = read_csv();
    let list_length = list.as_ref().expect("PROBLEM!").len() as usize;
    let rand_item = rand::thread_rng().gen_range(0..list_length);
        println!(
        "\nYour daily MUSIC album to listen to: {}",
        tcb::new()
            .fg()
            .red()
            .text_str(&list.unwrap()[rand_item])
            .print() // render to string
    );

    let _ = cut_list(rand_item as u32);

    match status {
        "LOCAL" => {
            // DO NOTHING
        }
        _ => {
            let files = vec![PathBuf::from("music.lst")];
            let output_path = PathBuf::from("music.zip");
            create_zip_archive(&files, output_path).expect("ZIP archive creation failed");
            let _ = webdav_client.put(std::fs::File::open("music.zip").unwrap(), &ip_adress);
            let _ = fs::remove_file("music.lst");
            let _ = fs::remove_file("music.zip");
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn list_dir() -> Vec<String> {
    let mut count = 0;
    let mut directories_list: Vec<String> = vec![];
    for entry_res in read_dir(".").unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();
        if !file_name.starts_with(".") && entry.file_type().unwrap().is_dir() {
            directories_list.push(file_name.to_string());
            count += 1;
        }
    }
    if count == 0 {    println!(
        "{}",
        tcb::new()
        .bg().red()
        .fg().black()
        .text_str( "PLEASE copy this program into your MUSIC drive.")
        .print()
    );
  
        panic!("EXITING . . .");}
    return directories_list;
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn write_csv(list: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = File::create("music.lst")?;
    let mut writer = csv::Writer::from_writer(file);
    for n in 0..list.len() {
        let _ = writer.write_record(&[list[n].clone()]);
    }
    let _ = writer.flush()?;
    return Ok(());
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn read_csv() -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("music.lst")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut list: Vec<String> = Vec::new();
    for element in reader.records() {
        let record = element?;
        list.push(record[0].to_string());
    }
    return Ok(list);
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn cut_list(item: u32) -> Result<(), Box<dyn Error>> {
    let file = File::open("music.lst")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut list: Vec<String> = Vec::new();
    let mut entry = 0;
    for element in reader.records() {
        let record = element?;
        if entry != item {
            list.push(record[0].to_string());
        }
        entry += 1;
    }


    if entry > 1 {
        let _ = write_csv(list);
    } else {
        println!(
            "{}",
            tcb::new()
                .fg()
                .yellow()
                .text_str(&format!("\n{} \n{}", "GREAT JOB, you've listening to all of your MUSIC playlist !!!" , "Indexing from local folders..."))
                .print()
        );

        let dirs = list_dir();
        let _ = write_csv(dirs);
    }
    return Ok(());
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
