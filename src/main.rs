#![windows_subsystem = "windows"]

use std::thread;
use std::time;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::fs;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use std::fs::OpenOptions;

use simplelog::*;
use log::*;

use serde_json::{Value};

const SETTINGSFILE: &str = "settings.txt";
const ONE_HOUR: time::Duration = time::Duration::from_secs(3600);
const VERSIONSTRING: &str = "1.0.0";
const NINTENDO_URL: &str = "https://app.splatoon2.nintendo.net/api/schedules";

fn main() {
    // set up logging

    // create log file if necessary
    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("logs/ping_log.txt")
        .unwrap();

    fs::create_dir_all("logs").unwrap();
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
        ]
    ).unwrap();

    info!("Starting splatnet_ping, Version {}", VERSIONSTRING);

    loop{
        let token = read_token(SETTINGSFILE);

        if let Ok(token) = token { // if io error occurs here, we retry in an hour
            let _ = ping_token(&token); // we don't need the response value
        } else {
            warn!("The settings file could not be accessed due to an IO error.");
        }

        // sleep for one hour
        info!("Initiated sleep, next iteration to be performed in one hour.");
        thread::sleep(ONE_HOUR);
    }
}


fn read_token(settings_file: &str) -> io::Result<String>{

    if !Path::new(settings_file).exists() {
        error!("Settings file not found.");
        panic!("Settings file not found.");
    }

    let file = File::open(settings_file)?;
    let reader = BufReader::new(file);

    if let Ok(settings_json) = serde_json::from_reader(reader){
        if let Value::Object(json_map) = settings_json { // json tree should start with an object
            // json_map should contain an element called "iksm-session"
            if let Some(token_obj) = json_map.get("iksm-session"){
                if let Value::String(token) = token_obj{
                    info!("Read token {}", token);
                    Ok(token.to_string())
                }else {
                    error!("Could not find token information in the settings file. (Invalid data format for iksm-session)");
                    panic!("Could not find token information in the settings file. (Invalid data format for iksm-session)");
                }
            } else {
                error!("Could not find token information in the settings file. (Entry 'iksm-session' not found)");
                panic!("Could not find token information in the settings file. (Entry 'iksm-session' not found)");
            }

        } else {
            error!("Could not find token information in the settings file. (Invalid JSON structure)");
            panic!("Could not find token information in the settings file. (Invalid JSON structure)");
        }
    } else {
        error!("Could not find token information in the settings file. (Invalid JSON structure)");
        panic!("Could not find token information in the settings file. (Invalid JSON structure)");
    }
}


// sends a message to the Splatnet server using the provided token
fn ping_token(token: &str) -> Result<reqwest::blocking::Response, reqwest::Error>{
    let client = reqwest::blocking::Client::new();
    let mut headers = HeaderMap::new();
    let cookie_body = format!("iksm_session={}", token);
    headers.insert(COOKIE,
        HeaderValue::from_str(&cookie_body).unwrap());

    let response = client.get(NINTENDO_URL)
        .headers(headers)
        .send();
    
    if let Ok(success_response) = &response{ 
        info!("Sent ping to SplatNet with token {} and received response code {}.",
            token,
            success_response.status());
    } else {
        error!("Failed to send ping, retrying in the next iteration.");
    }

    response
}


#[cfg(test)]
mod tests {

    use super::*;
    use reqwest::StatusCode;

    #[test]
    fn test_read_token_valid(){
        assert_eq!(
            read_token(SETTINGSFILE).unwrap(),
            "s0f338ea73ba798a7b7feb5b6b32d89ff990eea9"
        );
    }

    #[test]
    #[should_panic(expected = "Settings file not found.")]
    fn test_read_token_file_not_found(){
        read_token("invalidfile").unwrap();
    }

    #[test]
    #[should_panic(expected = "Could not find token information in the settings file. (Entry 'iksm-session' not found)")]
    fn test_read_token_token_not_found(){
        read_token("settings_notoken.txt").unwrap();
    }

    #[test]
    #[should_panic(expected = "Could not find token information in the settings file. (Invalid JSON structure)")]
    fn test_read_token_not_json(){
        read_token("settings_nojson.txt").unwrap();
    }

    #[test]
    fn test_ping(){
        assert_eq!(ping_token("invalid").unwrap().status(), StatusCode::FORBIDDEN);
    }
}