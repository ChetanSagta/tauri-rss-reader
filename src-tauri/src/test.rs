use std::fs;
use super::xmlparser;

#[cfg(test)]

//#[test]
//fn refresh_all_url_feeds_test(){
//    use super::commands;
//    assert_eq!(commands::refresh_all_feeds(), "Refreshed all the feed");
//}

#[test]
fn parse_xml_feed(){

    let content = fs::read_to_string("../rss.xml").unwrap();
    xmlparser::parse_content("https://rss.nytimes.com/services/xml/rss/nyt/World.xml", &content);


}

