use serde::Deserialize;
use isahc::prelude::*;

const GEOCODE_API_KEY: &str = "dcaab990-5a96-11eb-a3dc-95ccdabef212";
const GEOCODE_API_URL: &str = "https://app.geocodeapi.io/api";
const GEOCODE_API_VERSION: &str = "v1";

#[derive(Deserialize)]
pub struct LocationPoint {
    pub lat: f32,
    pub lon: f32,
    pub location: String,
}

fn base_url() -> String {
    return format!("{}/{}", 
        GEOCODE_API_URL, 
        GEOCODE_API_VERSION);
}

pub fn get_location_data(search: &str) -> Option<LocationPoint> {
    let url = format!("{}/search?apikey={}&text={}",
        base_url(),
        GEOCODE_API_KEY,
        search);

    println!("location get {}", url);
    let response = isahc::get(url);
    if let Some(mut body) = response.ok() {
        let text = body.text().unwrap();
        let data: serde_json::Value = serde_json::from_str(&text)
            .expect("Unable to deserialize location data into json");

        println!("location got: {}", data);

        let features = data["features"].as_array()
            .expect("Location data did not contain features");
        
        let exact_match = features.iter()
            .find(|f| f["properties"]["match_type"] == "exact");
        
        if let Some(feature) = exact_match {
            let properties = &feature["property"];
            let coords = &feature["geometry"]["coordinates"].as_array()
                .expect("Unable to find feature coordinates");
            let lon = coords[0].as_f64().unwrap();
            let lat = coords[1].as_f64().unwrap();

            Some(LocationPoint {
                lat: lat as f32, 
                lon: lon as f32,
                location: properties["label"].to_string(),
            })
        } else {
            None
        }
    } else {
        None
    }
}
