use std::fs;
use crate::xmlparser;

#[cfg(test)]

#[test]
fn parse_xml_feed(){

    let content = fs::read_to_string("/home/chetan/Code/tauri-react/url.txt").unwrap();

    let parsed_content:String = "".to_string();
    xmlparser::parse_content(content.as_str(), &parsed_content.as_str())

}

