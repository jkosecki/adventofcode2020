use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


pub async fn download_file<T: reqwest::IntoUrl>(url: T) -> reqwest::Result<String> {
    let body = reqwest::blocking::get(url)?.text()?;

    return Ok(body);
}


pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn iter_from_file(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    BufReader::new(File::open(filename).expect("Can't open the file")).lines().map(|res| res.expect("Can't load line of a file"))
}
