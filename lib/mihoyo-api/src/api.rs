use std::time::SystemTime;

use crate::{
    cache::Cache,
    store::Store,
    user::{ApiRawUser, ApiUser},
};
use reqwest::{Client, Error as ReqwestError};

pub struct Api {
    pub client: Client,
    cache: Cache,
    store: Store,
}

const USER_AGENT: &str = "MihoyoApi/0.1.0 (+https://artifacter.neody.land/)";

impl Api {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .expect("Failed to build reqwest client"),
            cache: Cache::new(),
            store: Store::new(),
        }
    }
    pub async fn asset(&self, uri: &str) -> Result<Vec<u8>, ReqwestError> {
        let cache_uri = format!("ui/{}", uri);
        match self.cache.get(&cache_uri).await {
            Ok((buf, _)) => Ok(buf),
            Err(_) => {
                let uri = format!(
                    "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/{}",
                    uri
                );
                let data = self.request(&uri).await?;
                let _ = self.cache.set(cache_uri, &data, SystemTime::now()).await;
                Ok(data)
            }
        }
    }
    async fn request(&self, uri: &str) -> Result<Vec<u8>, ReqwestError> {
        log::info!("requesting {}", uri);
        let request = self.client.get(uri);
        let body = request.send().await?;
        let body = body.error_for_status()?;
        let body = body.bytes().await?;
        Ok(body.to_vec())
    }
    pub fn get_store(&self) -> &Store {
        &self.store
    }
    async fn fetch_user(&self, uid: i32, lang: String) -> Result<ApiRawUser, Option<ReqwestError>> {
        let contents = self
            .request(&format!(
                "https://api.mihomo.me/sr_info_parsed/{}?lang={}",
                uid, lang
            ))
            .await?;
        let lastupdate = SystemTime::now();
        Ok(ApiRawUser::from_raw(contents, uid, lastupdate))
    }
    async fn set_cache(&self, data: &ApiRawUser) -> std::io::Result<()> {
        let now = SystemTime::now();
        self.cache
            .set(format!("user/{}", data.uid()), data.contents(), now)
            .await
    }
    async fn find_cache(&self, uid: i32) -> Option<ApiRawUser> {
        let (buf, modtime) = self.cache.get(format!("user/{}", uid)).await.ok()?;
        Some(ApiRawUser::from_raw(buf.to_vec(), uid, modtime))
    }
    pub async fn simple(&self, uid: i32, lang: String) -> Result<(ApiUser, bool), String> {
        match self.find_cache(uid).await {
            Some(cache) => {
                let data = cache.resolve()?;
                match self.reload(&data, lang).await {
                    Ok(u) => match u {
                        Some(new) => Ok((new, false)),
                        None => Ok((data, false)),
                    },
                    Err(e) => {
                        if e.contains("424") {
                            Ok((data, true))
                        } else {
                            Err(e)
                        }
                    }
                }
            }
            None => {
                let userdata = self.fetch_user(uid, lang).await;
                match userdata {
                    Ok(userdata) => {
                        let _ = self.set_cache(&userdata).await;
                        Ok((userdata.resolve()?, false))
                    }
                    Err(e) => Err(match e {
                        Some(e) => format!("{}", e),
                        None => String::from("unknown error"),
                    }),
                }
            }
        }
    }
    async fn reload(&self, data: &ApiUser, lang: String) -> Result<Option<ApiUser>, String> {
        let lastupdate = SystemTime::now();
        if data.reload_time() >= lastupdate {
            Ok(None)
        } else {
            let raw = self.fetch_user(data.uid(), lang).await;
            match raw {
                Ok(raw) => {
                    let _ = self.set_cache(&raw).await;
                    Ok(Some(raw.resolve()?))
                }
                Err(e) => Err(match e {
                    Some(e) => format!("{}", e),
                    None => String::from("unknown error"),
                }),
            }
        }
    }
}
