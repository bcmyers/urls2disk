use std::sync::Arc;

use num_cpus;
use reqwest;

use client::Client;
use error::Result;
use semaphore::Semaphore;
use wkhtmltopdf;

/// A `ClientBuilder` can be used to create a `Client` with custom configuration.
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    max_requests_per_second: usize,
    max_threads_cpu: usize,
    max_threads_io: usize,
    reqwest_client: Option<reqwest::Client>,
    wkhtmltopdf_settings: wkhtmltopdf::Settings,
}

impl Default for ClientBuilder {
    /// Creates a `ClientBuilder` with the following default settings:
    /// * `max_requests_per_second` = `10`
    /// * `max_threads_cpu` = number of logical cores on your machine
    /// * `max_threads_io` = `100`
    /// * `reqwest_client` = default `reqwest::Client` plus `gzip` set to `false` and `timeout` set to `None`
    /// * `wkhtmltopdf_zoom` = `"3.5"` on macOS and `"1.0"` on any other system
    fn default() -> ClientBuilder {
        ClientBuilder {
            max_requests_per_second: 10,
            max_threads_cpu: num_cpus::get(),
            max_threads_io: 100,
            reqwest_client: None,
            wkhtmltopdf_settings: wkhtmltopdf::Settings::default(),
        }
    }
}

impl ClientBuilder {
    /// Set the maximum number of requests per second.
    pub fn set_max_requests_per_second(mut self, max_requests_per_second: usize) -> ClientBuilder {
        self.max_requests_per_second = max_requests_per_second;
        self
    }

    /// Set the maximum number of cpu threads (those used for PDF conversion).
    pub fn set_max_threads_cpu(mut self, max_threads_cpu: usize) -> ClientBuilder {
        self.max_threads_cpu = max_threads_cpu;
        self
    }

    /// Set the maximum number of io threads (those used for downloading bytes).
    pub fn set_max_threads_io(mut self, max_threads_io: usize) -> ClientBuilder {
        self.max_threads_io = max_threads_io;
        self
    }

    /// Provide your own customized `reqwest::Client`.
    pub fn set_reqwest_client(mut self, reqwest_client: reqwest::Client) -> ClientBuilder {
        self.reqwest_client = Some(reqwest_client);
        self
    }

    /// Set wkhtmltopdf setting.
    pub fn set_wkhtmltopdf_setting(mut self, setting: wkhtmltopdf::Setting) -> ClientBuilder {
        self.wkhtmltopdf_settings.set(setting);
        self
    }

    /// Set wkhtmltopdf settings based on provided `Vec` of `wkhtmltopdf::Setting`.
    pub fn set_wkhtmltopdf_settings(mut self, settings: Vec<wkhtmltopdf::Setting>) -> ClientBuilder {
        for setting in settings {
            self.wkhtmltopdf_settings.set(setting);
        }
        self
    }

    /// Returns a `Client` that uses this `ClientBuilder` configuration.
    pub fn build(self) -> Result<Client> {
        let reqwest_client = match self.reqwest_client {
            Some(reqwest_client) => reqwest_client,
            None => reqwest::ClientBuilder::new()
                .gzip(false)
                .timeout(None)
                .build()?,
        };
        let semaphore = Semaphore::new(
            self.max_requests_per_second,
            self.max_threads_cpu,
            self.max_threads_io,
        );
        Ok(Client {
            inner: reqwest_client,
            semaphore: Arc::new(semaphore),
            wkhtmltopdf_settings: self.wkhtmltopdf_settings,
        })
    }
}
