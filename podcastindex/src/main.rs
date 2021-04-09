use std::time::{SystemTime, UNIX_EPOCH};
use sha1::{Sha1, Digest};
use urlencoding;
use reqwest::header;


fn main() {
    println!("Running...");

    //##: ======== Required values ======== 
	//##: WARNING: don't publish these to public repositories or in public places!
	//##: NOTE: values below are sample values, to get your own values go to https://api.podcastindex.org 
    let api_key: &str = "ABCDEFG";
    let api_secret: &str = "12345678abcdefg";
    let api_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time mismatch.").as_secs().to_string();


    //##: Create the authorization token
    //##: The auth token is a built by creating an sha1 hash of the key, secret and current time (as a string)
    //##: concatenated together. The hash is a lowercase string.
    let data4hash: String = format!("{}{}{}", api_key, api_secret, api_time);
    println!("Data to hash: [{}]", data4hash);
    let mut hasher = Sha1::new();
    hasher.update(data4hash);
    let authorization_token = hasher.finalize();
    let api_hash: String = format!("{:X}", authorization_token).to_lowercase();
    println!("Hash String: [{}]", api_hash);


    //##: Set up the parameters and the api endpoint url to call and make sure all params are
    //##: url encoded before sending.
    let query: String = "bastiat".to_string();
    let url: String = format!("https://api.podcastindex.org/api/1.0/search/byterm?q={}", urlencoding::encode(query.as_str()));


    //##: Build the query with the required headers
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("Rust-podcastindex-org-example/v1.0"));
    headers.insert("X-Auth-Date", header::HeaderValue::from_str(api_time.as_str()).unwrap());
    headers.insert("X-Auth-Key", header::HeaderValue::from_static(api_key));
    headers.insert("Authorization", header::HeaderValue::from_str(api_hash.as_str()).unwrap());
    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();


    //##: Send the request and display the results or the error
    let res = client.get(url.as_str()).send();
    match res {
        Ok(res) => {
            println!("Response Status: [{}]", res.status());
            println!("Response Body: {}", res.text().unwrap());
        },
        Err(e) => {
            eprintln!("Error: [{}]", e);
        }
    }


    //##: Finished
    ()
}