use sqlite::{self, State};

use crate::xmlparser::{Channel, Image, Item};

pub fn get_channel_id(channel: &Channel) -> Result<i64, sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "select id from channel where link like '%:link%'";
    let mut statement = connection.prepare(query)?;
    statement.bind((":link", channel.get_link()))?;
    // let channel_index = connection.execute(query).unwrap();
    // let value = statement.read(channel_index);
    let channel_index = statement.read("id").unwrap();
    Ok(channel_index)
}

pub fn get_image_id(image: &Image) -> Result<i64, sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "select rowid from image where url = :url";
    let mut statement = connection.prepare(query)?;
    statement.bind((":url", image.get_url()))?;
    let value = statement.read("rowid").unwrap();
    Ok(value)
}

pub fn insert_into_channel(channel: &Channel, image_id: i64) -> Result<(), sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query =
        "insert into channel values (:title, :link , :description, :last_build_date, :image_id, :pub_date)";
    let mut statement = connection.prepare(query)?;
    statement.bind((":title", channel.get_title()))?;
    statement.bind((":link", channel.get_link()))?;
    statement.bind((":description", channel.get_description()))?;
    statement.bind((":last_build_date", channel.get_last_build_date()))?;
    statement.bind((":image_id", image_id))?;
    statement.bind((":pub_date", channel.get_pub_date()))?;
    let state = statement.next();
    match state {
        Ok(_state) => {
            statement.next().unwrap();
        }
        Err(error) => {
            println!("Error : {error}");
        }
    }
    Ok(())
}

pub fn insert_into_image(image: Image) -> Result<(), sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "insert into image values (:title, :link, :url)";
    let mut statement = connection.prepare(query)?;
    statement.bind((":title", image.get_title()))?;
    statement.bind((":link", image.get_link()))?;
    statement.bind((":url", image.get_url()))?;
    let state = statement.next();
    match state {
        Ok(_state) => {
            statement.next().unwrap();
        }
        Err(error) => {
            println!("Error : {error}");
        }
    }
    Ok(())
}

pub fn insert_into_item(item: Item, channelid: i64) -> Result<(), sqlite::Error> {
    let connection = sqlite::open("db.sqlite").unwrap();
    let query = "INSERT INTO item VALUES (:title, :link, :description, :creator, :pub_date, :category, :channel_id)";
    let mut statement = connection.prepare(query)?;
    statement.bind((":title", item.get_title()))?;
    statement.bind((":link", item.get_link()))?;
    statement.bind((":description", item.get_description()))?;
    statement.bind((":creator", item.get_creator()))?;
    statement.bind((":pub_date", item.get_pub_date()))?;
    statement.bind((":category", item.get_creator()))?;
    statement.bind((":channel_id", channelid))?;
    let state = statement.next();
    match state {
        Ok(_state) => {
            statement.next().unwrap();
        }
        Err(error) => {
            println!("Error : {error}");
        }
    }
    Ok(())
}
