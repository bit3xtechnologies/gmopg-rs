use crate::arguments::{
    AlterTranArgs, ChangeTranArgs, EntryTranArgs, ExecTranArgs, SearchTradeArgs,
};
use crate::error::{Error, ReqwestClientSnafu, UrlDecodeSnafu};
use crate::response::{
    AlterTranResponse, ChangeTranResponse, EntryTranResponse, ExecTranResponse, SearchTradeResponse,
};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use snafu::ResultExt;

#[derive(Debug, Clone)]
pub struct CreditService {
    client: reqwest::Client,
    base_url: String,
}

impl CreditService {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn entry_tran(&self, args: EntryTranArgs) -> Result<EntryTranResponse, Error> {
        self.post("EntryTran.idPass", args).await
    }

    pub async fn exec_tran(&self, args: ExecTranArgs) -> Result<ExecTranResponse, Error> {
        self.post("ExecTran.idPass", args).await
    }

    pub async fn alter_tran(&self, args: AlterTranArgs) -> Result<AlterTranResponse, Error> {
        self.post("AlterTran.idPass", args).await
    }

    pub async fn search_trade(&self, args: SearchTradeArgs) -> Result<SearchTradeResponse, Error> {
        self.post("SearchTrade.idPass", args).await
    }

    pub async fn change_tran(&self, args: ChangeTranArgs) -> Result<ChangeTranResponse, Error> {
        self.post("ChangeTran.idPass", args).await
    }

    // pub async fn search_card_detail(&self, args: SearchCardDetailArgs) -> Result<SearchCardDetailResult, reqwest::Error> {
    //     self.post("SearchCardDetail.idPass", args).await
    // }

    async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        action: &str,
        args: T,
    ) -> Result<R, Error> {
        let url = format!("{}/payment/{}", self.base_url, action);
        let res = self
            .client
            .post(&url)
            .form(&args)
            .send()
            .await
            .context(ReqwestClientSnafu)?;

        if res.status() != StatusCode::OK {
            return Err(Error::HttpResponse {
                status_code: res.status().to_string(),
            });
        }

        let text = res.text().await.context(ReqwestClientSnafu)?;
        if text.starts_with("ErrCode") {
            // error happened
            tracing::error!("gmopg error: {}", text.to_string());
            return Err(Error::Gmopg {
                text: text.to_string(),
            });
        }

        Ok(serde_urlencoded::from_str(&text).context(UrlDecodeSnafu)?)
    }
}
