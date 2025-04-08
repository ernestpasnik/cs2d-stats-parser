use clap::{Arg, Command};
use std::path::Path;
use std::time::Instant;
use parser::parse_userstats;
use std::process;

mod parser;
mod output;

fn main() {
    let matches = Command::new("CS2D Stats Parser")
        .version("1.0.0")
        .author("Ernest Paśnik <https://github.com/ernestpasnik/cs2d-stats-parser>")
        .about("Parse CS2D stats and generate reports in HTML, JSON, or CSV format.")
        .arg(Arg::new("folder")
            .help("Path to the folder containing 'userstats.dat'")
            .index(1)
            .required(true))
        .arg(Arg::new("output")
            .index(2)
            .required(true)
            .help("Output file (filename with extension html, json, or csv)"))
        .arg(Arg::new("sort")
            .short('s')
            .long("sort")
            .value_parser(clap::value_parser!(u8).range(0..=2))
            .default_value("1")
            .help("Sort leaderboard:\n0 = score+kills-deaths\n1 = assists+kills-deaths\n2 = score+assists+deaths"))
        .arg(Arg::new("limit")
            .short('l')
            .long("limit")
            .value_parser(clap::value_parser!(u32).range(1..=100000))
            .default_value("100")
            .help("Limit players in the generated output"))
        .arg(Arg::new("title")
            .short('t')
            .long("title")
            .value_parser(clap::value_parser!(String))
            .default_value("CS2D Server")
            .help("Title to display in the HTML report"))
        .get_matches();

    let folder = matches.get_one::<String>("folder").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let sort = *matches.get_one::<u8>("sort").unwrap();
    let limit = *matches.get_one::<u32>("limit").unwrap();
    let title = matches.get_one::<String>("title").unwrap();

    if !Path::new(folder).is_dir() {
        eprintln!("Error: '{}' is not a valid folder.", folder);
        process::exit(1);
    }

    let userstats_path = Path::new(folder).join("userstats.dat");
    if !userstats_path.exists() {
        eprintln!("Error: 'userstats.dat' not found in '{}'.", folder);
        process::exit(1);
    }

    let ext = Path::new(output)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown");

    if !["html", "json", "csv"].contains(&ext) {
        eprintln!("Error: Invalid output format. Use html, json, or csv.");
        process::exit(1);
    }

    let start_time = Instant::now();

    match parse_userstats(userstats_path.to_str().unwrap()) {
        Ok(mut players) => {
            match sort {
                0 => players.sort_by_key(|p| -(p.score + p.kills - p.deaths)),
                1 => players.sort_by_key(|p| -(p.assists + p.kills - p.deaths)),
                2 => players.sort_by_key(|p| -(p.score + p.assists + p.deaths)),
                _ => {}
            }

            let limit = limit.min(players.len() as u32) as usize;
            players.truncate(limit);

            match ext {
                "html" => {
                    if let Err(e) = output::write_html(&players, output, title) {
                        eprintln!("Error writing HTML: {}", e);
                        process::exit(1);
                    }
                }
                "json" => {
                    if let Err(e) = output::write_json(&players, output) {
                        eprintln!("Error writing JSON: {}", e);
                        process::exit(1);
                    }
                }
                "csv" => {
                    if let Err(e) = output::write_csv(&players, output) {
                        eprintln!("Error writing CSV: {}", e);
                        process::exit(1);
                    }
                }
                _ => {}
            }

            println!("Saved to '{}' (Processed in {} ms)", output, start_time.elapsed().as_millis());
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error parsing userstats: {}", e);
            process::exit(1);
        }
    }
}
