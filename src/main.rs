use clap::{Arg, Command};
use std::path::Path;
use std::time::Instant;
use parser::parse_userstats;
use std::process;
use notify::{Watcher, RecursiveMode, recommended_watcher, EventKind};
use std::sync::mpsc::channel;
use std::time::Duration;

mod parser;
mod output;

fn generate_output(
    userstats_path: &Path,
    output: &str,
    ext: &str,
    sort: u8,
    limit: u32,
    title: &str,
    pretty: bool,
) {
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

            let result = match ext {
                "html" => output::write_html(&players, output, title),
                "json" => output::write_json(&players, output, pretty),
                "csv"  => output::write_csv(&players, output),
                "md"   => output::write_md(&players, output, title),
                "xml"  => output::write_xml(&players, output),
                _ => unreachable!(),
            };

            if let Err(e) = result {
                eprintln!("Error writing {}: {}", ext.to_uppercase(), e);
                return;
            }

            println!("Generated '{}' within {} ms", output, start_time.elapsed().as_millis());
        }
        Err(e) => eprintln!("Error parsing userstats: {}", e),
    }
}

fn main() {
    let matches = Command::new("CS2D Stats Parser")
        .version("3.0.1")
        .author("Ernest Paśnik <https://github.com/ernestpasnik/cs2d-stats-parser>")
        .about("This tool parses CS2D stats and exports them as HTML, JSON, CSV, Markdown, or XML.")
        .arg(Arg::new("folder")
            .help("Path to the folder containing 'userstats.dat'")
            .index(1)
            .required(true))
        .arg(Arg::new("output")
            .index(2)
            .required(true)
            .help("Output file (must end with .html, .json, .csv, .md, or .xml)"))
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
            .help("Title to display in the HTML/Markdown report"))
        .arg(Arg::new("pretty-print")
            .short('p')
            .long("pretty-print")
            .help("Enable pretty-printing for JSON output to improve readability")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("watch")
            .short('w')
            .long("watch")
            .help("Monitor 'userstats.dat' for changes and regenerate output when modified")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let folder = matches.get_one::<String>("folder").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let sort = *matches.get_one::<u8>("sort").unwrap();
    let limit = *matches.get_one::<u32>("limit").unwrap();
    let title = matches.get_one::<String>("title").unwrap();
    let pretty = matches.get_flag("pretty-print");
    let watch = matches.get_flag("watch");

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

    let supported_formats = ["html", "json", "csv", "md", "xml"];
    if !supported_formats.contains(&ext) {
        eprintln!(
            "Error: Invalid output format '{}'. Supported formats: {}.",
            ext,
            supported_formats.join(", ")
        );
        process::exit(1);
    }

    generate_output(&userstats_path, output, ext, sort, limit, title, pretty);

    if watch {
        println!("Monitoring '{}' for changes...", userstats_path.display());
        let (tx, rx) = channel();

        let mut watcher = recommended_watcher(tx).expect("Failed to create file watcher");
        watcher
            .watch(&userstats_path, RecursiveMode::NonRecursive)
            .expect("Failed to watch file");

        loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(Ok(event)) => {
                    if let EventKind::Modify(_) = event.kind {
                        println!("File change detected. Regenerating output...");
                        generate_output(&userstats_path, output, ext, sort, limit, title, pretty);
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Watcher error: {}", e);
                }
                Err(_) => {}
            }
        }
    }
}
