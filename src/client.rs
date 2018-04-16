use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam;
use reqwest::{self, StatusCode};
use url::Url;

use document::Document;
use error::{Error, Result};
use semaphore::Semaphore;
use wkhtmltopdf;

/// A `Client` downloads and writes to disk a slice of boxed objects
/// implementing `Document`. It does this in parallel to maximize efficiency,
/// but will never exceed the maximum number of requests per second provided by
/// the user nor the maximum number of threads provided.  Additionally, if the
/// object implemeting `Document` returns `true` from its `wkhtmltopdf()` method,
/// the `Client` will use `wkhtmltopdf` to convert what it downloads to PDF before
/// writing it to disk.
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) inner: reqwest::Client,
    pub(crate) semaphore: Arc<Semaphore>,
    pub(crate) wkhtmltopdf_settings: wkhtmltopdf::Settings,
}

impl Client {
    /// Downloads documents and writes them to disk. If the document already
    /// exists on disk `get_documents` will not redownload it
    pub fn get_documents<D>(&self, documents: &mut [Box<D>]) -> Result<()>
    where
        D: Document + Send,
    {
        let results = crossbeam::scope(|scope| {
            let (s1, r1) = channel();
            let (s2, r2) = channel();

            let semaphore = (self.semaphore).clone();
            scope.spawn(move || {
                loop {
                    thread::sleep(Duration::from_millis(1000));
                    semaphore.reset_requests();
                    match r1.try_recv() {
                        Ok(_) | Err(TryRecvError::Disconnected) => break,
                        Err(TryRecvError::Empty) => (),
                    }
                }
            });

            documents.sort_by(|a, b| a.wkhtmltopdf().cmp(&b.wkhtmltopdf()));

            let mut children = Vec::new();
            for document in documents.iter_mut() {
                let path = PathBuf::from(document.path());
                let url = document.url().clone();
                let wkhtmltopdf = document.wkhtmltopdf();
                if path.exists() {
                    let result = File::open(path).map_err(Error::from).and_then(|file| {
                        let mut reader = BufReader::new(file);
                        let mut bytes = Vec::new();
                        reader.read_to_end(&mut bytes)?;
                        trace!("processed {:?}", &url);
                        (*document).set_bytes(Some(bytes));
                        Ok::<_, Error>(())
                    });
                    s2.send(result).unwrap();
                    continue;
                }

                let client = self.clone();
                let s2 = s2.clone();
                self.semaphore.increment_requests();
                if wkhtmltopdf {
                    self.semaphore.increment_threads_cpu();
                    let child = scope.spawn(move || {
                        let result = self.get_pdf(&path, &url).and_then(|bytes| {
                            document.set_bytes(Some(bytes));
                            info!("downloaded {:?}", &url);
                            Ok::<_, Error>(())
                        });
                        s2.send(result).unwrap();
                        client.semaphore.decrement_threads_cpu();
                    });
                    children.push(child);
                } else {
                    self.semaphore.increment_threads_io();
                    let child = scope.spawn(move || {
                        let result = client.get_url(&url);
                        let result = result.and_then(|bytes| {
                            let file = File::create(&path)?;
                            let mut writer = BufWriter::new(file);
                            writer.write_all(&bytes)?;
                            info!("downloaded {:?}", &url);
                            document.set_bytes(Some(bytes));
                            Ok::<_, Error>(())
                        });
                        s2.send(result).unwrap();
                        client.semaphore.decrement_threads_io();
                    });
                    children.push(child);
                }
            }
            let mut results = Vec::new();
            for _ in children {
                let result = r2.recv().unwrap();
                results.push(result);
            }

            s1.send(()).unwrap();
            results
        });
        for result in results {
            result?;
        }
        Ok(())
    }

    fn get_url(&self, url: &Url) -> Result<Vec<u8>> {
        let mut response = self.inner.get(url.clone()).send()?;
        match response.status() {
            StatusCode::Ok => (),
            status => bail!(format_err!("response status: {}", status)),
        }
        let mut bytes = Vec::new();
        response.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    fn get_pdf<P: AsRef<Path>>(&self, path: P, url: &Url) -> Result<Vec<u8>> {
        let mut arguments = self.wkhtmltopdf_settings.to_arguments();
        arguments.push(url.to_string());
        arguments.push(
            path.as_ref()
                .to_str()
                .ok_or_else(|| format_err!("failed to parse path: {:?}", path.as_ref()))?
                .to_string(),
        );
        let mut process = Command::new("wkhtmltopdf")
            .args(&arguments)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()?;
        let exit_status = process.wait()?;
        if !exit_status.success() {
            match exit_status.code() {
                Some(code) => bail!("process failed with exit code {}", code),
                None => bail!("process failed with no exit code"),
            }
        }
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}
