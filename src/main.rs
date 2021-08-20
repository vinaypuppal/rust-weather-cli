fn main() {
    let token = std::env::var("API_TOKEN").expect("expected there to be an API token");
    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", token), ("keyword", args)])
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36")
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");
    dbg!(response);
}
