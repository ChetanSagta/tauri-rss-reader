use regex::Regex;

pub fn date_parser(date: &str) -> i64 {
    println!("Date: {}",date);
    chrono::DateTime::parse_from_str(date, "%a, %d %b %Y %H:%M:%S %z")
        .unwrap()
        .timestamp()
}

pub fn strip_cdata(data: &str) -> String{
    println!("Data: {}",data);

    let cdata_pattern = r"<!\[CDATA\[(.*?)\]\]>";

    if data.contains("CDATA") {
        let re = Regex::new(cdata_pattern).unwrap();

        let haystack = re.captures(data).unwrap();

        println!("Haystack: {:?}", haystack);

        for (_, [line]) in re.captures_iter(data).map(|c| c.extract()) {
            return line.to_string().trim().to_string();
        }
    }

    return data.to_string().trim().to_string();
}
