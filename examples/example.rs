extern crate num_cpus;
extern crate pdf_downloader;
extern crate reqwest;

use std::fs;
use std::path::Path;

use pdf_downloader::{Client, Result, SimpleDocument, Url};

fn run() -> Result<()> {
    // Create an output directory
    let output_directory = Path::new("./data");
    if !output_directory.exists() {
        fs::create_dir_all(output_directory)?;
    }

    // Create a vector of urls we would like to download
    let base = "https://www.sec.gov/Archives/edgar/data/";
    let urls = vec![
        "320193/000119312510238044/d10k.htm",
        "320193/000119312511282113/d220209d10k.htm",
        "320193/000119312512444068/d411355d10k.htm",
        "320193/000119312513416534/d590790d10k.htm",
        "320193/000119312514383437/d783162d10k.htm",
        "320193/000119312515356351/d17062d10k.htm",
        "320193/000162828016020309/a201610-k9242016.htm",
        "320193/000032019317000070/a10-k20179302017.htm",
    ].iter()
        .map(|stem| format!("{}{}", &base, stem))
        .collect::<Vec<String>>();

    // Turn the vector of urls into a vector of Box<SimpleDocument> that we can
    // feed to Client. This version sets the wkhtmltopdf option to false; so when
    // we feed this list to Client it will just download the raw webpages in
    // html format instead of first converting them to PDF
    let html_documents = urls.iter()
        .enumerate()
        .map(|(i, url_string)| {
            let filename = format!("Apple 10-K {}.html", i + 2010);
            let path = output_directory.join(&filename);
            let url = url_string.parse::<Url>()?;
            let wkhtmltopdf = false;
            let document = SimpleDocument::new(path, url, wkhtmltopdf);
            Ok(Box::new(document))
        })
        .collect::<Result<Vec<Box<SimpleDocument>>>>()?;

    // Turn the vector of urls into a vector of Box<SimpleDocument> that we can
    // feed to Client. This version sets the wkhtmltopdf option to true; so when we
    // feed this list to Client it will use wkhtmltopdf to convert the webpages
    // into PDF before writing them to disk
    let pdf_documents = urls.iter()
        .enumerate()
        .map(|(i, url_string)| {
            let filename = format!("Apple 10-K {}.pdf", i + 2010);
            let path = output_directory.join(&filename);
            let url = url_string.parse::<Url>()?;
            let wkhtmltopdf = true;
            let document = SimpleDocument::new(path, url, wkhtmltopdf);
            Ok(Box::new(document))
        })
        .collect::<Result<Vec<Box<SimpleDocument>>>>()?;

    // Combine our two vectors into one vector of Box<SimpleDocument>
    let mut documents = [&html_documents[..], &pdf_documents[..]].concat();

    // Create the client manually. A much simpler way of doing this would be
    // to use Client::default() instead
    let max_requests_per_second = 10;
    let max_threads_cpu = num_cpus::get();
    let max_threads_io = 100;
    let reqwest_client = reqwest::ClientBuilder::new()
        .gzip(false)
        .timeout(None)
        .build()?;
    let wkhtmltopdf_zoom = "3.5";
    let client = Client::new(
        max_requests_per_second,
        max_threads_cpu,
        max_threads_io,
        reqwest_client,
        wkhtmltopdf_zoom,
    );

    // Set the client off and running. It will download and write to disk all the
    // documents while simultaneously respecting the 'requests per second' and
    // other limits we provided
    client.get_documents(&mut documents)?;

    // Note: Here, if you want to, you can now access the raw bytes of all the urls
    // you downloaded, since they are now stored on each SimpleDocument in addition
    // to being saved on your disk

    Ok(())
}

fn main() {
    run().unwrap();
}
