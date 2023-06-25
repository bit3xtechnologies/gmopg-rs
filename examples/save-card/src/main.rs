use clap::Parser;
use gmopg_rs::{
    arguments::{MemberArgs, SaveCardArgs},
    CardService,
};
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

    // Step1. save_member
    let member_id = match card_service
        .save_member(MemberArgs {
            site_id: config.site_id.to_string(),
            site_pass: config.site_pass.to_string(),
            member_id: member_id.to_string(),
            member_name: Some("Test Organization Name".to_string()),
        })
        .await
    {
        Ok(res) => res.member_id,
        Err(err) => {
            if let gmopg_rs::Error::Gmopg { text } = err {
                if text.contains("E01390010") {
                    // ErrInfo=E01390010 => A member with the specified site ID and member ID already exists.
                    member_id.to_string()
                } else {
                    tracing::error!("save_member gmopg error: {text}");
                    panic!("save_member gmopg error!")
                }
            } else {
                tracing::error!("save_member error: {err}");
                panic!("save_member error!")
            }
        }
    };

    // Step2. save_card
    match card_service
        .save_card(SaveCardArgs {
            site_id: config.site_id,
            site_pass: config.site_pass,
            member_id,
            card_no: Some("4111111111111111".to_string()),
            expire: Some("2810".to_string()),
            security_code: Some("123".to_string()),
            ..Default::default()
        })
        .await
    {
        Ok(res) => {
            tracing::info!("save_card response: {:?}", res);
        }
        Err(err) => {
            tracing::error!("save_card error: {err}");
            panic!("save_card error!")
        }
    }

    Ok(())
}
