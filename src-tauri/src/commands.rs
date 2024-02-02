use std::fs::{self, File};
use std::io::Write;

use tauri::async_runtime::block_on;

use crate::db;
use crate::models::{Channel, Item};

use super::networking;

//enum FeedType {
//    ATOM,
//    RSS,
//}

#[tauri::command]
pub fn add_new_rss_feed(url: &str) -> String {
    let file_path = "/home/chetan/Code/tauri-react/url.txt";

    let mut file = File::options()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    file.write_all((url.to_owned() + "\n").as_bytes()).unwrap();

    format!("Successfully Added to URL")
}

#[tauri::command]
pub fn refresh_feed(name: &str) {
    let data = db::get_url_from_channel(name);

    match data {
        Ok(success) => {
            block_on(networking::get_request(&success));
        }
        Err(error) => {
            println!("Error : {}", error);
        }
    }
}

#[tauri::command]
pub fn refresh_all_feeds() -> String {
    let file_path = "/home/chetan/Code/tauri-react/url.txt";

    let content = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");
    let lines = content.split("\n");

    for line in lines {
        println!("DB Refresh: {}", line)
        //refresh_feed(line);
    }

    format!("Refreshed all the feed")
}

#[tauri::command]
pub fn get_all_feed_names_from_file() -> Result<Vec<String>, String> {
    let file_path = "/home/chetan/Code/tauri-react/url.txt";

    let content = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");
    let lines = content.split("\n");
    let lines_vec: Vec<String> = Vec::new();
    for line in lines {
        println!("{}",line)
        //block_on(networking::get_request(line));
    }
    return Ok(lines_vec);
}

#[tauri::command]
pub fn get_all_feed_names() -> Result<Vec<Channel>, String> {
    let channels = db::get_all_channels();

    match channels {
        Ok(success) => {
            return Ok(success);
        }
        Err(error) => {
            println!("Error : {}", error);
        }
    }
    Ok(Vec::new())
}

#[tauri::command]
pub fn get_feed_data(url: String) -> Result<Vec<Item>, String> {
    let data = db::get_items(url.as_str());

    match data {
        Ok(success) => {
            return Ok(success);
        }
        Err(error) => {
            println!("Error : {}", error);
        }
    }
    Ok(Vec::new())
}
