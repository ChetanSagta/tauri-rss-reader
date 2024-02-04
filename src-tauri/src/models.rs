use roxmltree::Node;
use serde::{Deserialize, Serialize};

use crate::db::insert_into_image;
use crate::util::{self, date_parser};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Image {
    title: String,
    link: String,
    url: String,
}

impl Image {
    pub fn new(title: String, link: String, url: String) -> Image {
        Self { title, link, url }
    }

    pub fn get_title(&self) -> String {
        self.title.to_owned()
    }
    pub fn get_link(&self) -> String {
        self.link.to_owned()
    }
    pub fn get_url(&self) -> String {
        self.url.to_owned()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Item {
    title: String,
    link: String,
    description: String,
    creator: String,
    pub_date: i64,
    category: String,
    channel_id: i64,
}

impl Item {
    pub fn new(
        title: String,
        link: String,
        description: String,
        creator: String,
        pub_date: i64,
        channel_id: i64,
        category: String,
    ) -> Self {
        Self {
            title,
            link,
            description,
            creator,
            pub_date,
            channel_id,
            category,
        }
    }

    pub fn get_title(&self) -> &str {
        return self.title.as_str();
    }
    pub fn get_link(&self) -> &str {
        return self.link.as_str();
    }
    pub fn get_description(&self) -> &str {
        return self.description.as_str();
    }
    pub fn get_creator(&self) -> &str {
        return self.creator.as_str();
    }
    pub fn get_pub_date(&self) -> i64 {
        return self.pub_date;
    }
    pub fn get_category(&self) -> String {
        return self.category.to_owned();
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub last_build_date: i64,
    pub publish_date: i64,
    pub image: Image,
    pub items: Vec<Item>,
}

impl Channel {
    pub fn new(
        title: String,
        link: String,
        description: String,
        last_build_date: i64,
        publish_date: i64,
        image: Image,
        items: Vec<Item>,
    ) -> Self {
        Self {
            title,
            link,
            description,
            last_build_date,
            publish_date,
            image: Image {
                title: "".to_string(),
                link: "".to_string(),
                url: "".to_string(),
            },
            items: Vec::new(),
        }
    }

    pub fn get_title(&self) -> &str {
        return self.title.as_str();
    }
    pub fn get_link(&self) -> &str {
        return self.link.as_str();
    }
    pub fn get_description(&self) -> &str {
        return self.description.as_str();
    }
    pub fn get_last_build_date(&self) -> i64 {
        return self.last_build_date;
    }

    pub fn get_image(&self) -> Image {
        return self.image.clone();
    }

    pub fn get_items(&self) -> Vec<Item> {
        return self.items.clone();
    }

    pub fn get_pub_date(&self) -> i64 {
        return self.publish_date;
    }
}

fn parse_items(node: Node) -> Item {
    let mut title = "";
    let mut link = "";
    let mut description = "";
    let mut creator = "";
    let mut publish_date: i64 = 0;
    let mut categories: Vec<String> = Vec::new();
    //let mut count = 0;
    for child in node.children() {
        let name = child.tag_name().name();
        match name {
            "title" => {
                title = child.text().unwrap();
            }
            "link" => {
                let namespace = child.tag_name().namespace();
                match namespace {
                    Some(namespace) => {
                        if namespace == "atom" {
                            link = child.attribute("href").unwrap();
                        }
                    }
                    None => {
                        link = child.text().unwrap();
                    }
                }
            }
            "description" => {
                description = child.text().unwrap();
            }
            "creator" => {
                creator = child.text().unwrap();
            }
            "pubDate" => {
                publish_date = util::date_parser(util::strip_cdata(child.text().unwrap()).as_str());
            }
            "category" => {
                categories.push(child.text().unwrap_or("").to_string());
            }
            _ => {}
        }
    }
    let mut cat = "".to_string();
    for category in categories {
        cat.push_str(category.as_str());
        cat.push_str("|");
    }
    return Item {
        title: title.to_string(),
        link: link.to_string(),
        description: description.to_string(),
        creator: creator.to_string(),
        pub_date: publish_date,
        category: cat,
        channel_id: -1,
    };
}

fn parse_image<'a>(node: &Node<'a, 'a>) -> Image {
    let mut title = "";
    let mut link = "";
    let mut url = "";
    for child in node.children() {
        let name = child.tag_name().name();
        match name {
            "title" => {
                title = child.text().unwrap();
            }
            "link" => {
                link = child.text().unwrap();
            }
            "url" => {
                url = child.text().unwrap();
            }
            _ => {}
        }
    }
    let image = Image {
        title: title.to_string(),
        link: link.to_string(),
        url: url.to_string(),
    };
    image
}

pub fn parse_channel<'a>(node: Node<'a, 'a>) -> Channel {
    let mut title = "";
    let mut link = "";
    let mut description = "";
    let mut last_build_date: i64 = 0;
    let mut pub_date: i64 = 0;
    let mut image: Image = Default::default();
    let mut items: Vec<Item> = Vec::new();
    let mut image_id: i64;
    for child in node.children() {
        let name = child.tag_name().name();
        if let Some(_) = child.tag_name().namespace() {
            continue;
        }
        match name {
            "title" => {
                title = child.text().unwrap();
            }
            "link" => {
                link = child.text().unwrap_or("");
            }
            "description" => {
                description = child.text().unwrap_or("");
            }
            "lastBuildDate" => {
                last_build_date =
                    util::date_parser(util::strip_cdata(child.text().unwrap()).as_str());
            }
            "pubDate" => {
                pub_date =
                    util::date_parser(util::strip_cdata(child.text().unwrap()).as_str());
            }
            "image" => {
                image = parse_image(&child);
                //let image_index = get_image_id(&image).unwrap();
                insert_into_image(&image).unwrap();
            }
            "item" => {
                items.push(parse_items(child));
            }
            _ => {}
        }
    }
    let channel = Channel {
        title: title.to_string(),
        link: link.to_string(),
        description: description.to_string(),
        last_build_date,
        publish_date: pub_date,
        image,
        items,
    };
    channel
}
