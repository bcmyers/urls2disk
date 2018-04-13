extern crate reqwest;
extern crate urls2disk;

use std::fs;
use std::path::Path;

use urls2disk::{wkhtmltopdf, ClientBuilder, Result, SimpleDocument, Url};

// This function will download Apple, Inc.'s annual reports for the years 2010 to 2017
// from the SEC's website to your disk.  It will download two copies of each annual
// report: one of just the raw html and another that has been converted to PDF.
fn run() -> Result<()> {
    // Create an output directory.
    let output_directory = Path::new("./data");
    if !output_directory.exists() {
        fs::create_dir_all(output_directory)?;
    }

    // Create a vector of urls we would like to download.
    // These urls represent the annual reports for Apple, Inc. from 2010 to 2017.
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

    // Turn the vector of urls into a vector of boxed Document trait objects (here we'll
    // be using the SimpleDocument struct as one possible implementer of the Document trait).
    // For this batch, we set the wkhtmltopdf option to false; so when we feed this list
    // to the Client it will just download the raw webpages in html format instead of
    // first converting them to PDF.
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

    // Turn the vector of urls into another vector of boxed Document trait objects
    // (to show off additional functionality).  This time we'll set the wkhtmltopdf
    // option to true; so when we feed this list to the Client it will first convert
    // the wepages to PDF before writing them to disk.
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

    // Combine our two vectors into one vector of Box<SimpleDocument>.
    let mut documents = [&html_documents[..], &pdf_documents[..]].concat();

    // Create the client.
    // Here, we're showing several customization options, but if you want to use
    // just the default settings, you could simply build the client with
    // `let client = ClientBuilder::default().build()?;`
    let client = ClientBuilder::default()
        .set_max_requests_per_second(9)
        .set_max_threads_cpu(4)
        .set_max_threads_io(50)
        .set_reqwest_client(reqwest::Client::new())
        .set_wkhtmltopdf_settings(vec![
            wkhtmltopdf::Setting::DisableExternalLinks(true),
            wkhtmltopdf::Setting::DisableJavascript(true),
        ])
        .build()?;

    // Let the client go. It will download and write to disk all the
    // documents while simultaneously respecting the 'requests per second' and
    // other limits we provided. If you already have the documents on disk,
    // the client will not redownload them.
    client.get_documents(&mut documents)?;

    // Note: Here, if you want to, you can now access the raw bytes of all the urls
    // you downloaded, since they are now stored on each SimpleDocument in addition
    // to being saved on your disk.
    Ok(())
}

fn main() {
    run().unwrap();
}
