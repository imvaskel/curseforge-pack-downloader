use std::{error::Error, io::stdin, time::Duration};

use clap::{App, Arg};
use curseforge_pack_downloader::{
    models::{LatestFile, PackProject},
    url, utils,
};
fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Curseforge Pack Downloader")
        .version("1.0")
        .author("ImVaskel (contact@vaskel.xyz)")
        .about("Downloads server packs from curseforge.")
        .arg(
            Arg::with_name("search")
                .short("s")
                .long("search")
                .value_name("SEARCH")
                .help("The search term.")
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

    let outfile = matches.value_of("output");

    let timeout: u64 = matches
        .value_of("timeout")
        .unwrap_or("60")
        .parse()
        .expect("Timeout was not a valid unsigned integer.");

    let search = matches.value_of("search").unwrap();

    let max: i32 = matches
        .value_of("max")
        .unwrap_or("1")
        .parse()
        .expect("Failed to parse max value.");

    let client = url::Client::with_timeout(Duration::from_secs(timeout)).unwrap();

    let res = client.search_packs(search, max as u32).unwrap();
    let pack: &PackProject;

    if res.len() > 1 {
        let mut input = String::new();

        println!("Please pick the pack you want.");

        for (index, option) in res.iter().enumerate() {
            println!("{}: {}", index + 1, option.name);
        }

        loop {
            stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            let choice: u32 = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Failed to convert to integer.");
                    continue;
                }
            };

            match res.get((choice - 1) as usize) {
                Some(n) => {
                    pack = n;
                    break;
                }
                None => (),
            }
        }
    } else if res.len() == 1 {
        pack = res.get(0).unwrap();
    } else {
        panic!("Error: There were no packs matching the search term.");
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

    let url = client.get_server_pack_url(pack.id as u32, file).unwrap();
    println!(
        "Downloading server file `{}` from `{}`",
        file.file_name, url
    );
    utils::download_server_pack(client, &url[..], outfile).expect("Error downloading file");

    Ok(())
}
