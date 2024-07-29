use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio;

use url::Url;

const BASE_URL: &str = "http://kaukkari.frakt.io:8080/";
fn create_url(path: &str) -> String {
    let url = Url::parse(BASE_URL).unwrap();
    let url = url.join(&path).unwrap();
    url.as_str().to_string()
}

// Amplifier

#[derive(Serialize, Deserialize)]
struct AmplifierStatusResponseResponse {
    result: AmplifierStatusResponseResult,
}

#[derive(Serialize, Deserialize)]
struct AmplifierStatusResponseResult {
    data: AmplifierStatusResponseData,
}

#[derive(Serialize, Deserialize)]
struct AmplifierStatusResponseData {
    amplifier: AmplifierStatusResponseAmplifier,
}

#[derive(Serialize, Deserialize)]
pub struct AmplifierStatusResponseVolume {
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

#[derive(Serialize, Deserialize)]
pub struct AmplifierStatusResponseAmplifier {
    pub powered: bool,
    pub volume: AmplifierStatusResponseVolume,
}

pub struct AmplifierState {
    pub powered: bool,
    pub volume: AmplifierVolume,
}

pub struct AmplifierVolume {
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

pub async fn api_amplifier_state() -> Result<AmplifierState, reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    let state: AmplifierStatusResponseResponse = client
        .get(create_url("state.get"))
        .timeout(tokio::time::Duration::from_secs(2))
        .send()
        .await?
        .json()
        .await?;

    Ok(AmplifierState {
        powered: state.result.data.amplifier.powered,
        volume: AmplifierVolume {
            min: state.result.data.amplifier.volume.min,
            max: state.result.data.amplifier.volume.max,
            value: state.result.data.amplifier.volume.value,
        },
    })
}

// Power

pub async fn api_on() -> Result<(), reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    client
        .post(create_url("amplifier.powerToggle"))
        .timeout(tokio::time::Duration::from_secs(2))
        .send()
        .await?;

    Ok(())
}

pub async fn api_off() -> Result<(), reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    client
        .post(create_url("amplifier.powerToggle"))
        .timeout(tokio::time::Duration::from_secs(2))
        .send()
        .await?;

    Ok(())
}

// Input

pub enum Input {
    AppleTV,
    Chromecast,
}

fn enum_to_input(input: &Input) -> &str {
    match input {
        Input::AppleTV => "DB",
        Input::Chromecast => "DVD",
    }
}

pub async fn api_input(input: &Input) -> Result<(), reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    let payload = &json!({ "input": enum_to_input(input) });

    client
        .post(create_url("amplifier.input"))
        .json(payload)
        .timeout(tokio::time::Duration::from_secs(2))
        .send()
        .await?;

    Ok(())
}

// Volume

pub async fn api_volume_set(volume: &f32) -> Result<(), reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    let payload = &json!({ "volume": volume });

    client
        .post(create_url("amplifier.volume"))
        .json(&payload)
        .timeout(tokio::time::Duration::from_secs(2))
        .send()
        .await?;

    Ok(())
}

// Accessories

pub async fn api_starry_sky() -> Result<(), reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    client
        .post(create_url("starrySky.powerToggle"))
        .timeout(tokio::time::Duration::from_secs(2))
        .send()
        .await?;

    Ok(())
}

pub async fn api_disco() -> Result<(), reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    client
        .post(create_url("disco.powerToggle"))
        .timeout(tokio::time::Duration::from_secs(2))
        .send()
        .await?;

    Ok(())
}

pub async fn api_fridge() -> Result<(), reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    client
        .post(create_url("fridge.powerToggle"))
        .timeout(tokio::time::Duration::from_secs(2))
        .send()
        .await?;

    Ok(())
}
