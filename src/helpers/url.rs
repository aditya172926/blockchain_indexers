use reqwest::get;

pub async fn helper_url_data(param: &str) -> Result<reqwest::Response, reqwest::Error> {
    let mut query = String::new();
    if param.starts_with("ar://") {
        let arweave_id = &param[5..param.len()];
        query = "https://arweave.net/".to_string() + arweave_id;
    } else if param.starts_with("ipfs://") {
        let ipfs_cid = &param[7..param.len()];
        query = "https://ipfs.io/ipfs/".to_string() + ipfs_cid;
    } else if param.starts_with("https://") {
        query = String::from(param);
    }

    let response = get(query).await;
    response
}
