use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("reqwest client error `{source}`"))]
    ReqwestClient { source: reqwest::Error },

    #[snafu(display("http response error with status code `{status_code}`"))]
    HttpResponse { status_code: String },

    #[snafu(display("url decode error `{source}`"))]
    UrlDecode { source: serde_urlencoded::de::Error },

    #[snafu(display("gmopg error `{text}`"))]
    Gmopg { text: String },
}
