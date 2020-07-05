use std::{io, io::Write};
use std::{fs, fs::File};
use std::path::{Path, PathBuf};

use crate::cli::CliError;
use crate::data::{Symbol, Option};
use chrono::{Date, Datelike, FixedOffset, Local, TimeZone};
use ftp::FtpStream;

const SYMBOLS_DIRECTORY: &str = "SymbolDirectory";
const SYMBOLS_FILENAME: &str = "nasdaqlisted.txt";
// const SYMBOLS_FILENAME: &str = "otherlisted.txt";
const OPTIONS_FILENAME: &str = "options.txt";

fn est() -> FixedOffset {
    chrono::FixedOffset::west(5 * 3600)
}

pub trait Downloadable { 
    fn filename() -> &'static str;
    fn filepath() -> PathBuf;
}

impl Downloadable for Symbol { 
    fn filename() -> &'static str { SYMBOLS_FILENAME }
    fn filepath() -> PathBuf { Path::new(SYMBOLS_DIRECTORY).join(SYMBOLS_FILENAME) }   
}

impl Downloadable for Option { 
    fn filename() -> &'static str { OPTIONS_FILENAME }
    fn filepath() -> PathBuf { Path::new(SYMBOLS_DIRECTORY).join(OPTIONS_FILENAME) }   
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::InitError { msg: err.to_string() }
    }
}

pub fn load<T: Downloadable + std::str::FromStr>() -> Result<Vec<T>, CliError> {
    let data = read_nasdaq_file::<T>();
    if data.is_err() {
        create_dir_if_necessary()?;
        return refresh_file_from_remote()
    }
    let (result, date) = data?;
    if is_outdated(date) {
        refresh_file_from_remote()  
    } else {
        Ok(result)
    }
}

// TODO: Pass in an Arc<Mutex<FtpStream>>
fn refresh_file_from_remote<T: Downloadable + std::str::FromStr>() -> Result<Vec<T>, CliError> {
    let mut ftp_stream = create_ftp_stream()
        .map_err(|err| CliError::InitError{ msg: err.to_string() })?;
    fetch_and_write_nasdaq_file::<T>(&mut ftp_stream)?;
    ftp_stream.quit().unwrap();
    let (result, _) = read_nasdaq_file::<T>()?;
    Ok(result)
}

fn create_ftp_stream() -> Result<ftp::FtpStream, ftp::FtpError> { 
    let mut ftp_stream =  FtpStream::connect("206.200.251.105:21")?;
    ftp_stream.login("anonymous", "anonymous")?;
    ftp_stream.cwd(SYMBOLS_DIRECTORY)?;
    Ok(ftp_stream)
}

fn read_nasdaq_file<T: std::str::FromStr + Downloadable>() -> Result<(Vec<T>, Date<FixedOffset>), io::Error> {
    let contents = fs::read_to_string(T::filepath())?;
    let lines: Vec<&str> = contents.split("\n").collect();
    // Remove the first line (header) and last line (empty newline)
    let mut lines = lines[1..lines.len() - 1].to_vec();
    let last_line = lines.pop().unwrap();
    let file_creation_date = get_file_creation_date(last_line);
    let result: Vec<T> = lines
        .into_iter()
        .map(|line| line.parse::<T>())
        .filter_map(Result::ok)
        .collect();
    Ok((result, file_creation_date))
}

fn fetch_and_write_nasdaq_file<T: Downloadable>(ftp_stream: &mut FtpStream) -> Result<(), io::Error> {
    let bytes = fetch_remote_file::<T>(ftp_stream)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
    write_remote_file::<T>(&bytes[..])
}

fn create_dir_if_necessary() -> Result<(), io::Error> {
    fs::create_dir_all(SYMBOLS_DIRECTORY)
}

fn fetch_remote_file<T: Downloadable>(ftp_stream: &mut FtpStream) -> Result<Vec<u8>, ftp::FtpError> {
    let remote_file = ftp_stream.simple_retr(T::filename())?;
    let bytes = remote_file.into_inner();
    Ok(bytes)
}   

// TODO: Pass in Logger
fn write_remote_file<T: Downloadable>(bytes: &[u8]) -> Result<(), std::io::Error> {
    // Write to filesystem
    let mut buffer = File::create(T::filepath())?;
    buffer.write_all(&bytes)?;
    Ok(())
}

fn get_file_creation_date(line: &str) -> Date<FixedOffset> { 

    let start_index = "File Creation Time: ".len();
    let end_index = line.find("|");

    // If we can't parse the file creation date, just assume it's today's date.symbols
    // That means the file won't be refreshed. Is this bad?
    if let None = end_index {
        let local_date = chrono::offset::Local::today().naive_local();
        est().from_local_date(&local_date).unwrap()
    } else {
        let segment = line[start_index..end_index.unwrap()].to_string();
        let month_component = &segment[..2];
        let day_component = &segment[2..4];
        let year_component = &segment[4..8];
        est().ymd(year_component.parse().unwrap(),
                  month_component.parse().unwrap(), 
                  day_component.parse().unwrap())
    }  
}

fn is_outdated(creation_date: Date<FixedOffset>) -> bool {
    let today = chrono::offset::Local::today();
    if today.weekday().number_from_monday() > 5 || is_market_holiday(today) {
        false
    } else {
        creation_date != chrono::offset::Local::today()
    }
}

fn is_market_holiday(date: Date<Local>) -> bool { 
    if date.month() == 7 && date.day() == 3 && date.year() == 2020 {
        true
    } else {
        false
    }
}