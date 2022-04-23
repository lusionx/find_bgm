use anyhow::Result;
use kuchiki::traits::*;
use reqwest::{Client, Url};

use tracing::info;

use crate::prop::{scan_node, select_attributes, serialize_text};

pub struct ComiCat {
    words: Vec<String>,
}

static ROOT: &'static str = "http://www.comicat.org/search.php";

impl ComiCat {
    pub fn new(words: &Vec<String>) -> ComiCat {
        return ComiCat {
            words: words.to_vec(),
        };
    }

    fn build_url(&self) -> Url {
        let url = Url::parse_with_params(ROOT, &[("keyword", self.words.join(" "))]).unwrap();
        url
    }

    pub async fn fetch_html(&self) -> Result<String> {
        let client = Client::new();
        let req = client.get(self.build_url());

        let res = req.send().await?.text().await?;

        Ok(res)
    }

    pub async fn find(&self) -> Result<()> {
        let html = self.fetch_html().await?;

        let document = kuchiki::parse_html()
            .from_utf8()
            .read_from(&mut html.as_bytes())?;
        let selectert = "#data_list a[target]";

        let mut res: Vec<(String, String)> = Vec::new();

        scan_node(&document, selectert, |node| {
            if let Some(href) = select_attributes(node, "href") {
                res.push((href, serialize_text(node)))
            }
            let mut content: Vec<u8> = Vec::new();
            node.serialize(&mut content).unwrap();
            info!("LINE {}", std::str::from_utf8(&content).unwrap());
        });

        info!("{:?}", res);
        Ok(())
    }
}
