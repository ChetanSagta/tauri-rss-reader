use sqlite::{self, State};

use crate::models::{Channel, Image, Item};

pub fn get_channel_id(channel: &Channel) -> Result<i64, sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "select rowid from channel where link = ?";
    let mut statement = connection.prepare(query).unwrap();
    println!("Link: {}", channel.get_link());
    statement.bind((1, channel.get_link()))?;
    while let State::Row = statement.next().unwrap() {
        let row_id = statement.read::<i64, _>("rowid").unwrap();
        return Ok(row_id);
    }
    Ok(-1 as i64)
}

pub fn get_image_id(image: &Image) -> Result<i64, sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "select rowid , * from image where url = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, image.get_url().as_str()))?;
    while let State::Row = statement.next().unwrap() {
        let row_id = statement.read::<i64, _>("rowid").unwrap();
        return Ok(row_id);
    }
    Ok(-1 as i64)
}

pub fn insert_into_channel(
    url: &str,
    channel: &Channel,
    image_id: i64,
) -> Result<(), sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query =
        "insert into channel values (:url, :title, :link , :description, :last_build_date, :image_id, :pub_date)";
    let mut statement = connection.prepare(query)?;
    statement.bind((":url", url))?;
    statement.bind((":title", channel.get_title()))?;
    statement.bind((":link", channel.get_link()))?;
    statement.bind((":description", channel.get_description()))?;
    statement.bind((":last_build_date", channel.get_last_build_date()))?;
    statement.bind((":image_id", image_id))?;
    statement.bind((":pub_date", channel.get_pub_date()))?;
    let state = statement.next();
    match state {
        Ok(_state) => {
            while _state != State::Done {
                statement.next().unwrap();
            }
        }
        Err(error) => {
            println!("Error : {error}");
        }
    }
    Ok(())
}

pub fn insert_into_image(image: &Image) -> Result<(), sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "insert into image values (:title, :link, :url)";
    let mut statement = connection.prepare(query)?;
    statement.bind((":title", image.get_title().as_str()))?;
    statement.bind((":link", image.get_link().as_str()))?;
    statement.bind((":url", image.get_url().as_str()))?;
    let state = statement.next();
    match state {
        Ok(_state) => {
            while _state != State::Done {
                statement.next().unwrap();
            }
        }
        Err(error) => {
            println!("Error : {error}");
        }
    }
    Ok(())
}

pub fn insert_into_item(item: Item, channelid: i64) -> Result<(), sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "insert into item values(:title, :link, :description, :creator, :pub_date, :category, :channel_id)";
    let mut statement = connection.prepare(query)?;
    statement.bind((":title", item.get_title()))?;
    statement.bind((":link", item.get_link()))?;
    statement.bind((":description", item.get_description()))?;
    statement.bind((":creator", item.get_creator()))?;
    statement.bind((":pub_date", item.get_pub_date()))?;
    statement.bind((":category", item.get_category().as_str()))?;
    statement.bind((":channel_id", channelid))?;
    let state = statement.next();
    match state {
        Ok(_state) => {
            while _state != State::Done {
                statement.next().unwrap();
            }
            return Ok(());
        }
        Err(error) => {
            println!("Error : {error}");
        }
    }
    Ok(())
}

pub fn get_channel_info(channel_name: &String) -> Result<Option<Channel>, sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "select * from channel where name like '%:channel_name%'";
    let mut statement = connection.prepare(query)?;
    statement.bind((":channel_name", channel_name.as_str()))?;
    while let Ok(State::Row) = statement.next() {
        let title = statement.read::<String, _>("title").unwrap();
        let link = statement.read::<String, _>("link").unwrap();
        let description = statement.read::<String, _>("description").unwrap();
        let last_build_date = statement.read::<i64, _>("last_build_date").unwrap();
        let publish_date = statement.read::<i64, _>("publish_date").unwrap();

        let channel = Channel::new(
            title,
            link,
            description,
            last_build_date,
            publish_date,
            Image::new("".to_string(), "".to_string(), "".to_string()),
            Vec::new(),
        );
        return Ok(Some(channel));
    }
    Ok(None)
}

pub fn get_items(url: &str) -> Result<Vec<Item>, sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();

    let query =
        "select i.* from item as i , channel as ch where i.channel_id  = ch.rowid and ch.link= ?";
    let mut statement = connection.prepare(query)?;
    statement.bind((1, url))?;

    let mut vector: Vec<Item> = Vec::new();

    while let State::Row = statement.next().unwrap() {
        let title = statement.read::<String, _>("title").unwrap();
        let link = statement.read::<String, _>("link").unwrap();
        let description = statement.read::<String, _>("description").unwrap();
        let last_build_date = statement.read::<String, _>("creator").unwrap();
        let publish_date = statement.read::<i64, _>("pub_date").unwrap();
        let category = statement.read::<String, _>("category").unwrap();
        let channel_id = statement.read::<i64, _>("channel_id").unwrap();

        let item = Item::new(
            title,
            link,
            description,
            last_build_date,
            publish_date,
            channel_id,
            category,
        );
        vector.push(item);
    }
    Ok(vector)
}

pub fn get_all_channels() -> Result<Vec<Channel>, sqlite::Error>{

    let connection = sqlite::open("db.sqlite").unwrap();

    let query = "select * from channel";

    let mut statement = connection.prepare(query)?;
    let mut vector : Vec<Channel> = Vec::new();

    while let State::Row = statement.next().unwrap(){
        let link= statement.read::<String, _>("link").unwrap();
        let title = statement.read::<String, _>("title").unwrap();

        let channel = Channel{
            title,
            link,
            ..Default::default()
    
        };

        vector.push(channel);
    }

    Ok(vector)
}

pub fn get_url_from_channel(link: &str) -> Result<String, sqlite::Error>{

    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "select url from channel where link = ?";

    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, link))?;
    
    if let State::Row = statement.next().unwrap(){
        return Ok(statement.read::<String, _>("url").unwrap());
    }
    else {
        return Ok("".to_string());
    }
}
