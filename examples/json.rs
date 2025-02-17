use isahc::prelude::*;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct ShoutCloudRequest {
    input: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct ShoutCloudResponse {
    input: String,
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = ShoutCloudRequest {
        input: "hello world".into(),
    };

    let response = Request::post("HTTP://API.SHOUTCLOUD.IO/V1/SHOUT")
        .header("content-type", "application/json")
        .body(serde_json::to_vec(&request)?)?
        .send()?
        .json::<ShoutCloudResponse>()?;

    println!("Response: {:?}", response);

    Ok(())
}
