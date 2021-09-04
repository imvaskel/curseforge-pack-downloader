#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{stdin, Write};
use std::process::exit;
use std::{cmp::min, time::Duration};

use clap::{App, Arg};
use curseforge_pack_downloader::client::Client;
use curseforge_pack_downloader::models::{LatestFile, PackProject};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

async fn download_file(client: &reqwest::Client, url: &str, path: &str) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to get server pack from `{}`", url)))?;

    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from `{}`", &url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", &url));

    let mut file = File::create(path).expect(&format!("Failed to create file at path `{}`", path));
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Curseforge Pack Downloader")
        .version("1.2")
        .author("Vaskel (contact@vaskel.xyz)")
        .about("Downloads server packs from curseforge.")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .value_name("QUERY")
                .help("The search query.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("The output file. This will default to a parsed name from the download url.")
                .required(false)
                .value_name("OUTPUT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .required(false)
                .takes_value(true)
                .default_value("60")
                .help("The timeout for the request client to use, in seconds.")
                .value_name("TIMEOUT"),
        )
        .arg(
            Arg::with_name("max")
                .short("m")
                .help("Max amount of search results to show.")
                .long("max")
                .value_name("max")
                .required(false)
                .default_value("1")
                .takes_value(true),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let timeout: u64 = value_t!(matches.value_of("timeout"), u64).unwrap_or(60);
    let max: u64 = value_t!(matches.value_of("max"), u64).unwrap_or(1);

    let client = Client::with_timeout(Duration::from_secs(timeout)).unwrap();

    let packs = client.search_packs(query, max).await?;
    let pack: &PackProject;

    if packs.len() > 1 {
        let mut input = String::new();

        println!("Please pick the pack you want.");

        for (index, option) in packs.iter().enumerate() {
            println!("{}: {}", index + 1, option.name);
        }

        loop {
            stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            let choice: u64 = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid input, please try again.");
                    continue;
                }
            };

            match packs.get((choice - 1) as usize) {
                Some(n) => {
                    pack = n;
                    break;
                }
                None => (),
            }
        }
    } else if packs.len() == 1 {
        pack = packs.get(0).unwrap();
    } else {
        eprintln!("There were no packs matching the search term.");
        exit(1);
    }

    println!("Please pick the client pack you would like the server pack for.");

    for (index, file) in pack.latest_files.iter().enumerate() {
        println!("{}: {}", index + 1, file.file_name);
    }

    let mut input = String::new();

    let file: &LatestFile;

    loop {
        stdin()
            .read_line(&mut input)
            .expect("Failed to recieve input.");

        let choice: u32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to convert to integer.");
                continue;
            }
        };

        match pack.latest_files.get((choice - 1) as usize) {
            Some(n) => {
                file = n;
                break;
            }
            None => (),
        }
    }

    let file_id = match file.server_pack_file_id {
        Some(n) => n,
        None => {
            println!("Pack version does not have a server file attached.");
            return Ok(());
        }
    };

    let url = client.get_download_url(pack.id, file_id).await?;
    let file_name = match matches.value_of("output") {
        Some(n) => n.to_owned(),
        None => format!("./{}", url.split("/").last().unwrap()),
    };

    println!("Downloading server file `{}` from `{}`.", file_name, url);

    download_file(&client.http, &url, &file_name).await?;

    Ok(())
}
