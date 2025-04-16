use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, Serialize)]
pub struct PlayerStats {
    pub name: String,
    pub usertype: u8,
    pub userid: i64,
    pub score: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub mvps: i32,
    pub time: i32,
}

#[derive(Debug, Serialize)]
pub struct TrafficStats {
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub hour: u8,
    pub uploaded_bytes: i64,
    pub downloaded_bytes: i64,
    pub players: u8,
}

fn read_header<R: BufRead>(reader: &mut R) -> io::Result<String> {
    let mut header = String::new()
    ;reader.read_line(&mut header)?;
    Ok(header.trim_end().to_string())
}

fn read_line<R: Read>(reader: &mut BufReader<R>) -> io::Result<String> {
    let mut line = String::new();
    let mut byte = [0u8];
    loop {
        reader.read_exact(&mut byte)?;
        if byte[0] == b'\n' {
            break;
        }
        if byte[0] != b'\r' {
            line.push(byte[0] as char);
        }
    }
    Ok(line)
}

pub fn parse_userstats(file_path: &str) -> io::Result<Vec<PlayerStats>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let header = read_header(&mut reader)?;
    if header != "userstats steam" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid header: expected 'userstats steam'"));
    }

    let mut players = Vec::new();
    while let Ok(name) = read_line(&mut reader) {
        if name.trim().is_empty() {
            break;
        }

        let usertype = reader.read_u8()?;
        let userid = reader.read_i64::<LittleEndian>()?;
        let score = reader.read_i32::<LittleEndian>()?;
        let kills = reader.read_i32::<LittleEndian>()?;
        let deaths = reader.read_i32::<LittleEndian>()?;
        let assists = reader.read_i32::<LittleEndian>()?;
        let mvps = reader.read_i32::<LittleEndian>()?;
        let time = reader.read_i32::<LittleEndian>()?;

        players.push(PlayerStats {
            name,
            usertype,
            userid,
            score,
            kills,
            deaths,
            assists,
            mvps,
            time,
        });
    }

    Ok(players)
}

pub fn parse_stats(file_path: &str) -> io::Result<Vec<TrafficStats>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let header = read_header(&mut reader)?;
    if header != "stats alpha" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid header: expected 'stats alpha'"));
    }

    let mut traffic = Vec::new();
    loop {
        let day = match reader.read_u8() {
            Ok(d) => d,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };

        let month = reader.read_u8()?;
        let year = reader.read_u16::<LittleEndian>()?;
        let hour = reader.read_u8()?;
        let uploaded_bytes = reader.read_i64::<LittleEndian>()?;
        let downloaded_bytes = reader.read_i64::<LittleEndian>()?;
        let players = reader.read_u8()?;

        traffic.push(TrafficStats {
            day,
            month,
            year,
            hour,
            uploaded_bytes,
            downloaded_bytes,
            players,
        });
    }

    Ok(traffic)
}
