use std::{fs::File, io::Cursor, path::PathBuf};

use crate::url::Client;

pub fn download_server_pack(
    client: Client,
    url: &str,
    file_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = match file_name {
        Some(name) => name.to_owned(),
        None => format!("./{}", url.split("/").last().unwrap()),
    };

    let res = client.download_server_pack(url).unwrap();

    let path = PathBuf::from(file_name);
    let mut file = File::create(path).unwrap();
    let mut content = Cursor::new(res);

    std::io::copy(&mut content, &mut file)?;

    Ok(())
}
