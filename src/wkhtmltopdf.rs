//! wkhtmltopdf settings

use std::time::Duration;

use utils::duration_to_millis;

cfg_if! {
    if #[cfg(target_os = "macos")] {
        fn default_zoom() -> f32 {3.5}
    } else {
        fn default_zoom() -> f32 {1.0}
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Settings {
    disable_external_links: bool,
    disable_javascript: bool,
    enable_forms: bool,
    dpi: usize,
    grayscale: bool,
    image_dpi: usize,
    image_quality: usize,
    low_quality: bool,
    javascript_delay: Duration,
    margin_bottom: String,
    margin_left: String,
    margin_right: String,
    margin_top: String,
    no_background: bool,
    no_images: bool,
    no_pdf_compression: bool,
    orientation: Orientation,
    page_size: PageSize,
    zoom: f32,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            disable_external_links: false,
            disable_javascript: false,
            enable_forms: false,
            dpi: 96,
            grayscale: false,
            image_dpi: 600,
            image_quality: 94,
            low_quality: false,
            javascript_delay: Duration::from_millis(200),
            margin_bottom: String::from("0.5in"),
            margin_left: String::from("0.5in"),
            margin_right: String::from("0.5in"),
            margin_top: String::from("0.5in"),
            no_background: false,
            no_images: false,
            no_pdf_compression: false,
            orientation: Orientation::Portrait,
            page_size: PageSize::Letter,
            zoom: default_zoom(),
        }
    }
}

impl Settings {
    pub(crate) fn to_arguments(&self) -> Vec<String> {
        let mut arguments = Vec::new();
        if self.disable_external_links {
            arguments.push("--disable-external-links".to_string());
        }
        if self.disable_javascript {
            arguments.push("--disable-javascript".to_string());
        }
        if self.enable_forms {
            arguments.push("--enable-forms".to_string());
        }
        arguments.extend_from_slice(&["--dpi".to_string(), self.dpi.to_string()]);
        if self.grayscale {
            arguments.push("--grayscale".to_string());
        }
        arguments.extend_from_slice(&["--image-dpi".to_string(), self.image_dpi.to_string()]);
        arguments.extend_from_slice(&["--image-quality".to_string(), self.image_quality.to_string()]);
        if self.low_quality {
            arguments.push("--low-quality".to_string());
        }
        arguments.extend_from_slice(&["--javascript-delay".to_string(), duration_to_millis(self.javascript_delay).to_string()]);
        arguments.extend_from_slice(&["--margin-bottom".to_string(), self.margin_bottom.clone()]);
        arguments.extend_from_slice(&["--margin-left".to_string(), self.margin_left.clone()]);
        arguments.extend_from_slice(&["--margin-right".to_string(), self.margin_right.clone()]);
        arguments.extend_from_slice(&["--margin-top".to_string(), self.margin_top.clone()]);
        if self.no_background {
            arguments.push("--no-background".to_string());
        }
        if self.no_images {
            arguments.push("--no-images".to_string());
        }
        if self.no_pdf_compression {
            arguments.push("--no-pdf-compression".to_string());
        }
        arguments.extend_from_slice(&["--orientation".to_string(), self.orientation.clone().into()]);
        arguments.extend_from_slice(&["--page-size".to_string(), self.page_size.clone().into()]);
        arguments.extend_from_slice(&["--zoom".to_string(), format!("{:.2}", self.zoom)]);
        arguments
    }
    pub(crate) fn set(&mut self, setting: Setting) {
        use self::Setting::*;
        match setting {
            DisableExternalLinks(v) => self.disable_external_links = v,
            DisableJavascript(v) => self.disable_javascript = v,
            EnableForms(v) => self.enable_forms = v,
            Dpi(v) => self.dpi = v,
            Grayscale(v) => self.grayscale = v,
            ImageDpi(v) => self.image_dpi = v,
            ImageQuality(v) => self.image_quality = v,
            LowQuality(v) => self.low_quality = v,
            JavascriptDelay(v) => self.javascript_delay = v,
            MarginBottom(v) => self.margin_bottom = v,
            MarginLeft(v) => self.margin_left = v,
            MarginRight(v) => self.margin_right = v,
            MarginTop(v) => self.margin_top = v,
            NoBackground(v) => self.no_background = v,
            NoImages(v) => self.no_images = v,
            NoPdfCompression(v) => self.no_pdf_compression = v,
            Orientation(v) => self.orientation = v,
            PageSize(v) => self.page_size = v,
            Zoom(v) => self.zoom = v,
        };
    }
}

/// A wkhtmltopdf setting, i.e. `DisableExternalLinks(false)`, `DisableJavascript(false)`, `Dpi(96)`, etc.
#[cfg_attr(nightly, feature(non_exhaustive))]
#[derive(Clone, Debug)]
pub enum Setting {
    /// Do not make links to remote web pages (default is `false`)
    DisableExternalLinks(bool),
    /// Do not allow web pages to run javascript (default is `false`)
    DisableJavascript(bool),
    /// Turn HTML form fields into pdf form fields (default is `false`)
    EnableForms(bool),
    /// Change the dpi explicitly (this has noeffect on X11 based systems) (default is `96`)
    Dpi(usize),
    /// PDF will be generated in grayscale (default is `false`)
    Grayscale(bool),
    /// When embedding images scale them down to this dpi (default is `600`)
    ImageDpi(usize),
    /// When jpeg compressing images use this quality (default is `94`)
    ImageQuality(usize),
    /// Generates lower quality pdf/ps (default is `false`)
    LowQuality(bool),
    /// Wait some milliseconds for javascript finish (default is 200 milliseconds)
    JavascriptDelay(Duration),
    /// Set the page bottom margin (default is `String::from("0.5in")`)
    MarginBottom(String),
    /// Set the page left margin (default is `String::from("0.5in")`)
    MarginLeft(String),
    /// Set the page right margin (default is `String::from("0.5in")`)
    MarginRight(String),
    /// Set the page top margin (default is `String::from("0.5in")`)
    MarginTop(String),
    /// Do not print background (default is `false`)
    NoBackground(bool),
    /// Do not load or print images (default is `false`)
    NoImages(bool),
    /// Do not use lossless compression on pdf objects (default is `false`)
    NoPdfCompression(bool),
    /// Set orientation to Landscape or Portrait (default is `Orientation::Portrait`)
    Orientation(Orientation),
    /// Set paper size to: A4, Letter, etc. (default is `PageSize::Letter`)
    PageSize(PageSize),
    /// Use this zoom factor (default is `3.5` on macOS and `1.0` on other systems)
    Zoom(f32),
}

/// An orientation, i.e. `Landscape` or `Portrait`
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Orientation {
    Landscape,
    Portrait,
}

impl From<Orientation> for String {
    fn from(orientation: Orientation) -> String {
        use self::Orientation::*;
        match orientation {
            Landscape => "Landscape".to_string(),
            Portrait => "Portrait".to_string(),
        }
    }
}

/// A paper size, i.e. `A4`, `Legal`, `Letter`, etc.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PageSize {
    /// 841 x 1189 mm
    A0,
    /// 594 x 841 mm
    A1,
    /// 420 x 594 mm
    A2,
    /// 297 x 420 mm
    A3,
    /// 210 x 297 mm
    A4,
    /// 148 x 210 mm
    A5,
    /// 105 x 148 mm
    A6,
    /// 74 x 105 mm
    A7,
    /// 52 x 74 mm
    A8,
    /// 37 x 52 mm
    A9,
    /// 1000 x 1414 mm
    B0,
    /// 707 x 1000 mm
    B1,
    /// 500 x 707 mm
    B2,
    /// 353 x 500 mm
    B3,
    /// 250 x 353 mm
    B4,
    /// 176 x 250 mm
    B5,
    /// 125 x 176 mm
    B6,
    /// 88 x 125 mm
    B7,
    /// 62 x 88 mm
    B8,
    /// 33 x 62 mm
    B9,
    /// 31 x 44 mm
    B10,
    /// 163 x 229 mm
    C5E,
    /// 4.125 x 9.5 inches (U.S. Common 10 Envelope)
    Comm10E,
    /// 110 x 220 mm
    DLE,
    /// 7.5 x 10.0 inches
    Executive,
    /// 8.0 x 13.0 inches
    Folio,
    /// 17.0 x 11.0 inches
    Ledger,
    /// 8.5 x 14.0 inches
    Legal,
    /// 8.5 x 11.0 inches
    Letter,
    /// 11.0 x 17.0 inches
    Tabloid,
}

impl From<PageSize> for String {
    fn from(page_size: PageSize) -> String {
        use self::PageSize::*;
        match page_size {
            A0 => "A0".to_string(),
            A1 => "A1".to_string(),
            A2 => "A2".to_string(),
            A3 => "A3".to_string(),
            A4 => "A4".to_string(),
            A5 => "A5".to_string(),
            A6 => "A6".to_string(),
            A7 => "A7".to_string(),
            A8 => "A8".to_string(),
            A9 => "A9".to_string(),
            B0 => "B0".to_string(),
            B1 => "B1".to_string(),
            B2 => "B2".to_string(),
            B3 => "B3".to_string(),
            B4 => "B4".to_string(),
            B5 => "B5".to_string(),
            B6 => "B6".to_string(),
            B7 => "B7".to_string(),
            B8 => "B8".to_string(),
            B9 => "B9".to_string(),
            B10 => "B10".to_string(),
            C5E => "C5E".to_string(),
            Comm10E => "Comm10E".to_string(),
            DLE => "DLE".to_string(),
            Executive => "Executive".to_string(),
            Folio => "Folio".to_string(),
            Ledger => "Ledgar".to_string(),
            Legal => "Legal".to_string(),
            Letter => "Letter".to_string(),
            Tabloid => "Tabloid".to_string(),
        }
    }
}
