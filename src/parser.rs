use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{self, BufReader, Read};
use serde::Serialize;

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

fn read_line<R: Read>(reader: &mut BufReader<R>) -> io::Result<String> {
    let mut line = String::new();
    
    loop {
        let mut byte = [0u8];
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

pub fn parse_userstats(file_path: &str) -> Result<Vec<PlayerStats>, std::io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut header_line = Vec::new();
    loop {
        let mut byte = [0u8];
        reader.read_exact(&mut byte)?;
        if byte[0] == b'\n' {
            break;
        }
        header_line.push(byte[0]);
    }

    let header = String::from_utf8_lossy(&header_line);
    if header.trim() != "userstats steam" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid file header: expected 'userstats steam'",
        ));
    }

    let mut players = Vec::new();

    while let Ok(name) = read_line(&mut reader) {
        if name.trim().is_empty() {
            break;
        }

        let usertype = reader.read_u8().unwrap_or(0);
        let userid = reader.read_i64::<LittleEndian>().unwrap_or(0);
        let score = reader.read_i32::<LittleEndian>().unwrap_or(0);
        let kills = reader.read_i32::<LittleEndian>().unwrap_or(0);
        let deaths = reader.read_i32::<LittleEndian>().unwrap_or(0);
        let assists = reader.read_i32::<LittleEndian>().unwrap_or(0);
        let mvps = reader.read_i32::<LittleEndian>().unwrap_or(0);
        let time = reader.read_i32::<LittleEndian>().unwrap_or(0);

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
