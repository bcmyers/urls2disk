extern crate pdf_downloader;
extern crate url;

use std::fs;
use std::path::Path;
use std::time::Instant;

use pdf_downloader::{Client, Result, SimpleDocument, Url};

const NO_OF_URLS: usize = 100;

fn elapsed(instant: &Instant) -> f32 {
    let elapsed = instant.elapsed();
    (elapsed.as_secs() as f32) + (elapsed.subsec_nanos() as f32 / 1_000_000_000f32)
}

fn setup<P: AsRef<Path>>(data_directory: P) -> Result<Client> {
    if data_directory.as_ref().exists() {
        fs::remove_dir_all(&data_directory)?;
    }
    fs::create_dir_all(&data_directory)?;
    let client = Client::default()?;
    Ok(client)
}

#[test]
fn test_client_html() {
    let data_directory = Path::new("./tests/test_output/test_client_html");
    let client = setup(&data_directory).unwrap();
    let mut documents = (0..NO_OF_URLS)
        .map(|i| {
            let document = SimpleDocument::new(
                data_directory.join(format!("test{}.html", i)),
                Url::parse("https://www.sec.gov/Archives/edgar/data/320193/000032019318000007/a10-qq1201812302017.htm").unwrap(),
                false,
            );
            Box::new(document)
        })
        .collect::<Vec<Box<SimpleDocument>>>();
    let start = Instant::now();
    client.get_documents(&mut documents).unwrap();
    let elapsed = elapsed(&start);
    let requests_per_second = documents.len() as f32 / elapsed;
    println!("requests per second: {}", requests_per_second);
    assert!(requests_per_second < 10 as f32);
}

#[test]
fn test_client_wkhtmltopdf() {
    let data_directory = Path::new("./tests/test_output/test_client_wkhtmltopdf");
    let client = setup(&data_directory).unwrap();
    let mut documents = (0..NO_OF_URLS)
        .map(|i| {
            let wkhtmltopdf = i % 10 == 0;
            let document = SimpleDocument::new(
                data_directory.join(
                    format!("test{}{}", i, if wkhtmltopdf {".pdf"} else {".html"})),
                Url::parse("https://www.sec.gov/Archives/edgar/data/320193/000032019318000007/a10-qq1201812302017.htm").unwrap(),
                wkhtmltopdf,
            );
            Box::new(document)
        })
        .collect::<Vec<Box<SimpleDocument>>>();
    let start = Instant::now();
    client.get_documents(&mut documents).unwrap();
    let elapsed = elapsed(&start);
    let requests_per_second = documents.len() as f32 / elapsed;
    println!("requests per second: {}", requests_per_second);
    assert!(requests_per_second < 10 as f32);
}
