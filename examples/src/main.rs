use chrono::Utc;
use clap::Parser;
use gmopg_rs::{
    arguments::{EntryTranArgs, ExecTranArgs},
    enums, CreditService,
};

#[derive(Clone, Debug, Parser)]
pub struct ApiConfig {
    #[clap(
        name = "gmopg-url",
        long,
        env = "GMOPG_URL",
        default_value = "http://127.0.0.1:8080"
    )]
    gmopg_url: url::Url,

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
    let credit_service = CreditService::new(config.gmopg_url.as_str());

    // NOTE: every `order_id` need to be unique (max length: 27)
    let now = Utc::now();
    let order_id = format!("{}", now.format("%Y%m%d%H%M%s"));
    tracing::info!("order_id: {order_id}");

    // Step1. entry_tran
    let (access_id, access_pass) = match credit_service
        .entry_tran(EntryTranArgs {
            shop_id: config.shop_id,
            shop_pass: config.shop_pass,
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

    Ok(())
}
