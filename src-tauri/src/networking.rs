use crate::xmlparser::parse_content;

pub async fn get_request(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;

    let headermap = (&response).headers();

    let header_value = &headermap["content-type"];
    let content_type = header_value.to_str().unwrap().to_owned();

    println!("CONTENT TYPE: {}", content_type);

    let content = response.text().await.unwrap();

    if content_type.contains("application/atom"){
        println!("Current feed is ATOM");
    } else if content_type.contains("application/xml") {
        parse_content(url, &content)
    } else {
        panic!("Current Feed Unknown: {:?}", content_type);
    }

    Ok(content.clone())
}
