use std::fs::{self, File};
use std::io::Write;

use tauri::async_runtime::block_on;

use super::networking;

enum FeedType {
    ATOM,
    RSS,
}

#[tauri::command]
pub fn add_new_rss_feed(url: &str) -> String {
    println!("Content= {url}");

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

fn get_feed_content(url: &str) {
    let rss_feed_content = block_on(networking::get_request(url));

    match rss_feed_content {
        Ok(value) => {
            let rss_feed = value;

            let file_path = "/home/chetan/Code/tauri-react/rss.xml";

            let mut file = File::options()
                .create(true)
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap();

            file.write_all((rss_feed.to_owned() + "\n").as_bytes())
                .unwrap();
        }
        Err(err) => {
            println!("Error occurred due to {err}");
        }
    }
}

#[tauri::command]
pub fn refresh_feed(name: &str) -> String {
    format!(
        "This command will be used to refresh a single feed {}",
        name
    )
}

#[tauri::command]
pub fn refresh_all_feeds() -> String {
    let file_path = "/home/chetan/Code/tauri-react/url.txt";

    let content = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");
    let lines = content.split("\n");

    for line in lines {
        get_feed_content(line);
    }

    format!("Refreshed all the feed")
}
