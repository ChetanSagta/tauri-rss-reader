pub async fn get_request(url: &str) -> Result<String,reqwest::Error>{
    let response = reqwest::get(url).await.unwrap();
    let headermap = (&response).headers();

    let header_value= &headermap["content-type"];
    let content_type = header_value.to_str().unwrap().to_owned();
    
    let content = response.text().await.unwrap();

    if content_type == "application/atom"{
        println!("Current feed is ATOM");
    }
    else if content_type == "application/xml"{
        println!("Current feed is RSS");
    }
    else{
        panic!("Current Feed Unknown: {:?}", content_type);
    }

    Ok(content.clone())
}
