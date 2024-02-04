use crate::{db::{
    get_channel_id, get_image_id, insert_into_channel,insert_into_item
}, models::parse_channel};

pub fn parse_content(url: &str, content: &str) {

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
                        insert_into_channel(&url, &channel, image_id).unwrap();
                        channel_id = get_channel_id(&channel).unwrap_or(-1 as i64);
                    }
                    for item in channel.get_items() {
                        insert_into_item(item, channel_id).unwrap();
                    }
                }
            });
        }
        Err(error) => {
            println!("Error : {:?}", error);
        }
    };
}
