fn main() {
    let _api_token = std::env::var("API_TOKEN").expect("expected there to be an API token");
    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();
    dbg!(args);
}
