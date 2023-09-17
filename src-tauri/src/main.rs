mod commands;
mod networking;
mod xmlparser;
#[cfg(test)]
mod test;
mod db;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::add_new_rss_feed, commands::refresh_all_feeds, commands::refresh_feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
