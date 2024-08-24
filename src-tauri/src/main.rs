#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::command;
use reqwest;

#[command]
async fn get_weather(city: String) -> Result<String, String> {
    let api_key = "API-KEY";
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}",
        api_key, city
    );

    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<serde_json::Value>().await {
                Ok(weather_data) => {
                    let location = weather_data["location"]["name"].as_str().unwrap_or("Unknown Location");
                    let country = weather_data["location"]["country"].as_str().unwrap_or("");
                    let temperature = weather_data["current"]["temp_c"].as_f64().unwrap_or(0.0);
                    let description = weather_data["current"]["condition"]["text"].as_str().unwrap_or("No description").to_string();
                    let icon_url = weather_data["current"]["condition"]["icon"].as_str().unwrap_or("");

                    let wind_speed = weather_data["current"]["wind_kph"].as_f64().unwrap_or(0.0);
                    let humidity = weather_data["current"]["humidity"].as_u64().unwrap_or(0);
                    let pressure = weather_data["current"]["pressure_mb"].as_f64().unwrap_or(0.0);
                    let visibility = weather_data["current"]["vis_km"].as_f64().unwrap_or(0.0);

                    let weather_info = format!(
                        "<h2>{}, {}</h2>
                        <p class='temperature'><img src='{}' alt='Weather Icon'> {}°C, {}</p>
                        <p><strong>Wind:</strong> {} kph</p>
                        <p><strong>Humidity:</strong> {}%</p>
                        <p><strong>Pressure:</strong> {} mb</p>
                        <p><strong>Visibility:</strong> {} km</p>",
                        location, country, icon_url, temperature, description, wind_speed, humidity, pressure, visibility
                    );

                    Ok(weather_info)
                }
                Err(err) => Err(format!("Failed to parse weather data: {}", err)),
            }
        }
        Err(err) => Err(format!("Failed to fetch weather data: {}", err)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_weather])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
