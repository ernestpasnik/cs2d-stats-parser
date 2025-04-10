use std::fs::File;
use std::io::{self, Write};
use std::vec::Vec;
use crate::parser::PlayerStats;
use chrono::Local;

pub fn write_json(players: &Vec<PlayerStats>, output_path: &str) -> io::Result<()> {
    let file = File::create(output_path)?;
    serde_json::to_writer(file, players)?;
    Ok(())
}

pub fn write_csv(players: &Vec<PlayerStats>, output_path: &str) -> io::Result<()> {
    let file = File::create(output_path)?;
    let mut writer = csv::Writer::from_writer(file);

    writer.write_record(&[
        "name", "usertype", "userid", "score", "kills", "deaths", "assists", "mvps", "time",
    ])?;

    for player in players {
        writer.write_record(&[
            &player.name,
            &player.usertype.to_string(),
            &player.userid.to_string(),
            &player.score.to_string(),
            &player.kills.to_string(),
            &player.deaths.to_string(),
            &player.assists.to_string(),
            &player.mvps.to_string(),
            &player.time.to_string(),
        ])?;
    }

    Ok(())
}

pub fn write_html(players: &Vec<PlayerStats>, output_path: &str, title: &str) -> io::Result<()> {
    let mut file = File::create(output_path)?;

    let formatted_time = Local::now().format("%B %e, %Y at %H:%M:%S").to_string();

    let mut html = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <meta name="color-scheme" content="light dark">
            <title>{}</title>
            <link rel="icon" href="http://cs2d.com/favicon.ico" type="image/x-icon">
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css">
            <style>
                * {{
                    font-size: 100%;
                }}
                tr, td {{
                    white-space: nowrap;
                }}
                .container {{
                    max-width: 1000px;
                }}
                .sortable thead th:not(.no-sort) {{ cursor: pointer; }}
                .sortable thead th:not(.no-sort)::after, .sortable thead th:not(.no-sort)::before {{
                    transition: color 0.1s ease-in-out;
                    vertical-align: baseline;
                    color: transparent;
                }}
                .sortable thead th:not(.no-sort)::after {{ margin-left: 3px; content: "▸"; }}
                .sortable thead th:not(.no-sort):hover::after {{ color: inherit; }}
                .sortable thead th:not(.no-sort)[aria-sort=descending]::after {{
                    color: inherit;
                    content: "▾";
                }}
                .sortable thead th:not(.no-sort)[aria-sort=ascending]::after {{
                    color: inherit;
                    content: "▴";
                }}
                .sortable thead th:not(.no-sort).indicator-left::after {{ content: ""; }}
                .sortable thead th:not(.no-sort).indicator-left::before {{
                    margin-right: 3px;
                    content: "▸";
                }}
                .sortable thead th:not(.no-sort).indicator-left:hover::before {{ color: inherit; }}
                .sortable thead th:not(.no-sort).indicator-left[aria-sort=descending]::before {{
                    color: inherit;
                    content: "▾";
                }}
                .sortable thead th:not(.no-sort).indicator-left[aria-sort=ascending]::before {{
                    color: inherit;
                    content: "▴";
                }}
            </style>
        </head>
        <body class="container">
            <header>
                <hgroup>
                    <h1>{}</h1>
                    <p>Last updated on {}</p>
                </hgroup>
            </header>
            <main>
                <div class="overflow-auto">
                    <table class="sortable striped">
                        <thead>
                            <tr>
                                <th scope="col" aria-sort="ascending">#</th>
                                <th scope="col">Player</th>
                                <th scope="col">K/D</th>
                                <th scope="col">K</th>
                                <th scope="col">A</th>
                                <th scope="col">D</th>
                                <th scope="col">⭐</th>
                                <th scope="col">⌚</th>
                            </tr>
                        </thead>
                        <tbody>"#,
        title, title, formatted_time
    );

    for (i, p) in players.iter().enumerate() {
        let profile_url = if p.usertype == 1 {
            format!("https://steamcommunity.com/profiles/{}", p.userid)
        } else {
            format!("https://unrealsoftware.de/profile.php?userid={}", p.userid)
        };

        let kd_ratio = if p.deaths > 0 {
            p.kills as f32 / p.deaths as f32
        } else {
            p.kills as f32
        };

        html.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <th scope="row">
                    <a href="{}" target="_blank">{}</a>
                </th>
                <td>{:.2}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td data-sort="{}">{}</td>
            </tr>"#,
            i + 1,
            profile_url,
            sanitize_html(&p.name),
            kd_ratio,
            p.kills,
            p.assists,
            p.deaths,
            p.mvps,
            p.time,
            format_time(p.time.try_into().unwrap())
        ));
    }

    html.push_str(r#"</tbody></table></div></main><script src="https://cdn.jsdelivr.net/gh/tofsjonas/sortable@latest/dist/sortable.min.js"></script></body></html>"#);

    let mut minified_html = String::with_capacity(html.len());
    let mut last_was_space = false;
    let mut prev_char = '\0';
    
    for c in html.chars() {
        if c.is_whitespace() {
            last_was_space = true;
            continue;
        }
    
        if last_was_space {
            if !(prev_char == '>' && c == '<') {
                minified_html.push(' ');
            }
            last_was_space = false;
        }
    
        minified_html.push(c);
        prev_char = c;
    }
    write!(file, "{}", minified_html.trim())?;
    
    Ok(())
}


fn format_time(s: i32) -> String {
    let days = s / 86400;
    let hours = (s % 86400) / 3600;
    let minutes = (s % 3600) / 60;
    let seconds = s % 60;

    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

fn sanitize_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('\n', " ")
        .replace('\r', " ")
        .trim()
        .to_string()
}
