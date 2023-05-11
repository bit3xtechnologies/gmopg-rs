use std::fmt::Debug;

use http::StatusCode;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use snafu::ResultExt;

use crate::arguments::{DeleteMemberArgs, MemberArgs, SaveCardArgs};
use crate::error::{ReqwestClientSnafu, UrlDecodeSnafu};
use crate::response::{MemberIDResponse, SaveCardResponse, SearchMemberResponse};
use crate::Error;

#[derive(Debug, Clone)]
pub struct CardService {
    client: Client,
    base_url: String,
}

impl CardService {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn save_member(&self, args: MemberArgs) -> Result<MemberIDResponse, Error> {
        self.post("SaveMember", args).await
    }

    pub async fn update_member(&self, args: MemberArgs) -> Result<MemberIDResponse, Error> {
        self.post("UpdateMember", args).await
    }

    pub async fn delete_member(&self, args: DeleteMemberArgs) -> Result<MemberIDResponse, Error> {
        self.post("DeleteMember", args).await
    }

    pub async fn search_member(&self, args: MemberArgs) -> Result<SearchMemberResponse, Error> {
        self.post("SearchMember", args).await
    }

    pub async fn save_card(&self, args: SaveCardArgs) -> Result<SaveCardResponse, Error> {
        self.post("SaveCard", args).await
    }

    async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        action: &str,
        args: T,
    ) -> Result<R, Error> {
        let url = format!("{}/payment/{}.idPass", self.base_url, action);
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
