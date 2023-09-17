use roxmltree::Node;

use crate::db::{
    get_channel_id, get_image_id, insert_into_channel, insert_into_image, insert_into_item,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Image<'a> {
    title: &'a str,
    link: &'a str,
    url: &'a str,
}

impl Image<'_> {
    // pub fn new(&mut self) {
    //     self.title = "";
    //     self.link= "";
    //     self.url = "";
    // }

    pub fn get_title(&self) -> &str {
        self.title
    }
    pub fn get_link(&self) -> &str {
        self.link
    }
    pub fn get_url(&self) -> &str {
        self.url
    }
}

#[derive(Debug, Default, Clone)]
pub struct Item {
    title: String,
    link: String,
    description: String,
    creator: String,
    pub_date: i64,
    category: Vec<String>,
    channel_id: i64,
}

impl Item {
    pub fn new(&mut self) {
        self.title = String::new();
        self.link = String::new();
        self.description = String::new();
        self.creator = String::new();
        self.pub_date = -1;
        self.category = Vec::new();
        self.channel_id = -1;
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
    pub fn get_category(&self) -> Vec<String> {
        return self.category.to_owned();
    }
}

#[derive(Default)]
pub struct Channel<'a> {
    title: String,
    link: String,
    description: String,
    last_build_date: i64,
    publish_date: i64,
    image: Image<'a>,
    items: Vec<Item>,
}

impl Channel<'_> {
    // pub fn new(&mut self) {
    //     self.title = String::new();
    //     self.link = String::new();
    //     self.description = String::new();
    //     self.last_build_date = -1;
    //     self.publish_date = -1;
    //     self.image = Image {
    //         title: "",
    //         link: "",
    //         url: "",
    //     };
    //     self.items = Vec::new();
    // }

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
    let mut count = 0;
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
                publish_date = chrono::DateTime::parse_from_str(
                    child.text().unwrap(),
                    "%a, %d %b %Y %H:%M:%S %z",
                )
                .unwrap()
                .timestamp();
            }
            "category" => {
                categories.push(child.text().unwrap_or("").to_string());
            }
            _ => {}
        }
    }
    return Item {
        channel_id: -1,
        title: title.to_string(),
        link: link.to_string(),
        description: description.to_string(),
        creator: creator.to_string(),
        pub_date: publish_date,
        category: categories,
    };
}

fn parse_image<'a>(node: &Node<'a, 'a>) -> Image<'a> {
    //let mut image = Image::default();
    //let child = node.first_child().unwrap();
    let mut title = "";
    let mut link= "";
    let mut url = "";
    for child in node.children() {
        let name = child.tag_name().name();
        match name {
            "title" => {
                title = child.text().unwrap();
            }
            "link" => {
                link= child.text().unwrap();
            }
            "url" => {
                url = child.text().unwrap();
            }
            _ => {}
        }
    }
    let image = Image {
        title: title,
        link: link,
        url: url,
    };
    //print!("Title: {title}, Text: {text}, Url: {url}");
    image
}

fn parse_channel<'a>(node: Node<'a, 'a>) -> Channel<'a> {
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
                last_build_date = chrono::DateTime::parse_from_str(
                    child.text().unwrap(),
                    "%a, %d %b %Y %H:%M:%S %z",
                )
                .unwrap()
                .timestamp();
            }
            "pubDate" => {
                pub_date = chrono::DateTime::parse_from_str(
                    child.text().unwrap(),
                    "%a, %d %b %Y %H:%M:%S %z",
                )
                .unwrap()
                .timestamp();
            }
            "image" => {
                image = parse_image(&child);
                let image_index = get_image_id(&image).unwrap();
                println!("Image Index: {}", image_index);
                insert_into_image(image).unwrap();
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
        last_build_date: last_build_date,
        publish_date: pub_date,
        image: image,
        items: items,
    };
    channel
}

pub fn parse_content(content: &str) {
    let doc = roxmltree::Document::parse(content);

    match doc {
        Ok(document) => {
            let root = document.root_element();
            root.children().for_each(|node| {
                let name = node.tag_name().name();
                if name == "channel" {
                    let channel = parse_channel(node);
                    let image_id = get_image_id(&channel.get_image()).unwrap();
                    let mut channel_id = get_channel_id(&channel).unwrap_or(-1 as i64);
                    if channel_id == -1 {
                        insert_into_channel(&channel, image_id).unwrap();
                        channel_id = get_channel_id(&channel).unwrap_or(-1 as i64);
                    }
                    for item in channel.get_items() {
                        let result = insert_into_item(item, channel_id).unwrap();
                    }
                }
            });
        }
        Err(error) => {
            println!("Error : {:?}", error);
        }
    };
}
