use clap::Parser;
use gmopg_rs::{arguments::SearchCardArgs, CardService};
use uuid::Uuid;

#[derive(Clone, Debug, Parser)]
pub struct ApiConfig {
    #[clap(
        name = "gmopg-url",
        long,
        env = "GMOPG_URL",
        default_value = "http://127.0.0.1:8080"
    )]
    gmopg_url: url::Url,

    #[clap(name = "site-id", long, env = "SITE_ID")]
    site_id: String,

    #[clap(name = "site-pass", long, env = "SITE_PASS")]
    site_pass: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let config = ApiConfig::parse();
    tracing::info!("config: {:?}", config);
    let card_service = CardService::new(config.gmopg_url.as_str());

    // member_id for test
    let member_id = Uuid::nil();

    // search_card
    match card_service
        .search_card(SearchCardArgs {
            site_id: config.site_id,
            site_pass: config.site_pass,
            member_id: member_id.to_string(),
            ..Default::default()
        })
        .await
    {
        Ok(res) => {
            tracing::info!("search_card response: {:?}", res);
        }
        Err(err) => {
            // ErrInfo=E01240002 => The specified card does not exist.
            // ErrInfo=E01390002 => The member with the specified site ID and member ID does not exist.
            tracing::error!("search_card error: {err}");
            panic!("search_card error!")
        }
    };

    Ok(())
}
