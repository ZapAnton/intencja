use std::{fs, io::Error};

pub fn get_text_data(dir: &str, filename: &str) -> Result<String, Error> {
    let data_path = format!("data/text/{}/{}.txt", dir, filename);

    Ok(fs::read_to_string(&data_path)?)
}
