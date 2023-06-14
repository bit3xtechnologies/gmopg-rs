use chrono::Utc;
use clap::Parser;
use gmopg_rs::{
    arguments::{EntryTranArgs, ExecTranArgs, MemberArgs, TradedCardArgs},
    enums, CardService, CreditService,
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

    #[clap(name = "shop-id", long, env = "SHOP_ID")]
    shop_id: String,

    #[clap(name = "shop-pass", long, env = "SHOP_PASS")]
    shop_pass: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let config = ApiConfig::parse();
    tracing::info!("config: {:?}", config);
    let card_service = CardService::new(config.gmopg_url.as_str());
    let credit_service = CreditService::new(config.gmopg_url.as_str());

    // NOTE: every `order_id` need to be unique (max length: 27)
    let now = Utc::now();
    let order_id = format!("{}", now.format("%Y%m%d%H%M%s"));
    tracing::info!("order_id: {order_id}");

    // Step1. entry_tran
    let (access_id, access_pass) = match credit_service
        .entry_tran(EntryTranArgs {
            shop_id: config.shop_id.to_string(),
            shop_pass: config.shop_pass.to_string(),
            order_id: order_id.to_string(),
            job_cd: enums::JobCd::Auth,
            amount: 87,
            ..Default::default()
        })
        .await
    {
        Ok(res) => (res.access_id, res.access_pass),
        Err(err) => {
            tracing::error!("entry_tran error: {err}");
            panic!("entry_tran error!")
        }
    };

    // Step2. exec_tran with (access_id, access_pass) got from entry_tran
    match credit_service
        .exec_tran(ExecTranArgs {
            access_id,
            access_pass,
            order_id: order_id.to_string(),
            method: Some(enums::Method::Lump),
            card_no: Some("4111111111111111".to_string()),
            expire: Some("2810".to_string()),
            security_code: Some("123".to_string()),
            ..Default::default()
        })
        .await
    {
        Ok(res) => {
            tracing::info!("exec_tran response: {:?}", res);
        }
        Err(err) => {
            tracing::error!("exec_tran error: {err}");
            panic!("exec_tran error!")
        }
    }

    // member_id for test
    let member_id = Uuid::nil();

    // Step3. save_member
    let member_id = match card_service
        .save_member(MemberArgs {
            site_id: config.site_id.to_string(),
            site_pass: config.site_pass.to_string(),
            member_id: member_id.to_string(),
            member_name: Some("Organization Name".to_string()),
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

    // Step4. save traded_card
    match card_service
        .traded_card(TradedCardArgs {
            shop_id: config.shop_id,
            shop_pass: config.shop_pass,
            order_id: order_id.to_string(),
            site_id: config.site_id,
            site_pass: config.site_pass,
            member_id,
            card_name: Some("Test Visa Card".to_string()),
            holder_name: Some("Test Holder Name".to_string()),
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
