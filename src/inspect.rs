//use std::fs;
use reqwest::blocking;
use regex::Regex;

pub fn handle_inspect(){
    let mut hash = "71feca15ba2b64c929da81acc6c1be0310e97c71aca0b54d2f6f5adfd0b93dc3".to_string();
    println!("Hash: {}", hash);
    while !hash.is_empty() {
        let url = format!("https://dl.zxteam.net/stratum/images/{}/MANIFEST", hash);

        if let Ok(response) = blocking::get(&url) {
            if let Ok(body) = response.text() {
                let re = Regex::new(r"BASE: (\w+)").ok();
                
                if let Some(captures) = re.and_then(|re| re.captures(&body)) {
                    if let Some(base_hash) = captures.get(1) {
                        let base_hash_str = base_hash.as_str().to_string();
                    
                        println!("Base hash: {}", base_hash_str);
                        hash = base_hash_str;
                    }
                }
            }
        }
        // let root_folder = "strarum";
        // if !fs::metadata(root_folder).is_ok() {
        //     if let Err(err) = fs::create_dir(root_folder) {
        //         eprintln!("Failed to create root folder: {}", err);
        //     }
        // }

        // let user_folder = format!("{}/{}", root_folder, hash);

        // if !fs::metadata(&user_folder).is_ok() {
        //     if let Err(err) = fs::create_dir(&user_folder) {
        //         eprintln!("Failed to create user folder: {}", err);
        //     }
        // }

        // let _file_path = format!("{}/MANIFEST", user_folder);
    }
}
