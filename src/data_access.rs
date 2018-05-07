use std::{fs::File, io::{Error, Read}, path::Path};

pub fn get_text_data(dir: &str, filename: &str) -> Result<String, Error> {
    let mut file_content = String::new();

    let data_path_str = format!("data/text/{}/{}.txt", dir, filename);

    let data_path = Path::new(&data_path_str);

    let mut data_file = File::open(&data_path)?;

    data_file.read_to_string(&mut file_content)?;

    Ok(file_content)
}
