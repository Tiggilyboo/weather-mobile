use serde::Deserialize;
use isahc::prelude::*;

const GEOCODE_API_KEY: &str = "dcaab990-5a96-11eb-a3dc-95ccdabef212";
const GEOCODE_API_URL: &str = "https://app.geocodeapi.io/api";
const GEOCODE_API_VERSION: &str = "v1";

#[derive(Deserialize, Clone)]
pub struct LocationPoint {
    pub lat: f32,
    pub lon: f32,
    pub location: String,
}

pub async fn search_locations(search: &str) -> Option<Vec<LocationPoint>> {
    if let Some(data) = get_location_data(search).await {
        let locations = parse_to_location_points(data);
        Some(locations)
    } else {
        None
    }
}

pub async fn search_locations_exact(search: &str) -> Option<LocationPoint> {
    if let Some(data) = get_location_data(search).await {
        let features = data["features"].as_array()
            .expect("Location data did not contain features");
        let exact_match = features.iter()
            .find(|f| f["properties"]["match_type"] == "exact");

        if let Some(location) = exact_match {
            Some(parse_feature_to_location_point(location.clone()))
        } else {
            None
        }
            
    } else {
        None
    }
}

fn base_url() -> String {
    return format!("{}/{}", 
        GEOCODE_API_URL, 
        GEOCODE_API_VERSION);
}

fn parse_feature_to_location_point(feature: serde_json::Value) -> LocationPoint {
    let properties = &feature["properties"];
    let coords = &feature["geometry"]["coordinates"].as_array()
        .expect("Unable to find feature coordinates");
    let lon = coords[0].as_f64().unwrap();
    let lat = coords[1].as_f64().unwrap();
    let location = String::from(properties["label"].as_str().unwrap());

    LocationPoint {
        lat: lat as f32, 
        lon: lon as f32,
        location,
    }
}

fn parse_to_location_points(value: serde_json::Value) -> Vec<LocationPoint> {
    let features = value["features"].as_array()
        .expect("Location data did not contain features");
    
    let locations = features.iter()
            .map(|f| parse_feature_to_location_point(f.clone()))
            .collect::<Vec<_>>();

    locations
}

async fn get_location_data(search: &str) -> Option<serde_json::Value> {
    let url = format!("{}/search?apikey={}&text={}",
        base_url(),
        GEOCODE_API_KEY,
        search);

    let response = isahc::get_async(url).await;

    if let Some(mut body) = response.ok() {
        let text = body.text().await;

        if let Ok(text) = text {
            let data: serde_json::Value = serde_json::from_str(&text)
                    .expect("Unable to deserialize location data into json");
            Some(data)
        } else {
            panic!("Unable to get text data from 200 response");
        }
    } else {
        println!("get_location_data: did not get ok response!");
        None
    }
}

