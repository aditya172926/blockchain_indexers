use ethers::types::U256;
use log::{debug, error, info, warn};
use crate::helpers::url::helper_url_data;
use crate::structs::index::MethodParam;
use crate::structs::meta::{self, Meta, MetaData};
use crate::structs::transactions::TransactionIndexed;

#[derive(Debug)]
struct LensPostMeta {
    profileId: U256,
    contentURI: String,
    collectModule: String,
    collectModuleData: String,
    referenceModule: String,
}

pub async fn handler_lens_post(transaction_indexed: &TransactionIndexed) -> Option<MetaData> {
    if transaction_indexed.method.name == "post" {
        let params_list = transaction_indexed.method.params[0].clone().into_tuple().unwrap();
        // info!("profileid {:?}, {:?}", params_list[0].clone().into_uint().unwrap(), params_list[0]);
        let meta_raw: LensPostMeta = LensPostMeta {
            profileId: params_list[0].clone().into_uint().unwrap(),
            contentURI: params_list[1].to_string(),
            collectModule: params_list[2].to_string(),
            collectModuleData: params_list[3].to_string(),
            referenceModule: params_list[4].to_string(),
        };

        info!("meta_raw -> {:?}", meta_raw);

        let mut image = String::from("https://i.seadn.io/gae/S67RadRtlIbTNk0UojZM-TEl4pybcblKyg3HxQHl0-JmxYZ2deLX-pK2Z89khCWHGeaXeYfE8Vxqj06YCUcqk0q1KWD9T997lGnGHw?auto=format&dpr=1&w=3840");
        let response = helper_url_data(&meta_raw.contentURI).await;
        let mut meta_id = String::new();
        let mut meta_image = String::new();
        let mut meta_title = String::new();
        match response {
            Ok(object) => {
                if object.status().is_success() {
                    let res = object.text().await.expect("Error in parsing object");
                    let ipfs_content: serde_json::Value =
                        serde_json::from_str(&res).expect("error in reading json format");
                    let metaId = ipfs_content["metadata_id"].to_string();
                    meta_id = metaId[1..metaId.len() - 1].to_string();
                    let metaImage = ipfs_content["image"].to_string();
                    meta_image = metaImage[1..metaImage.len() - 1].to_string();
                    let metaTitle: String = ipfs_content["content"].to_string();
                    meta_title = metaTitle[1..metaTitle.len() - 1].to_string();

                    // let mut ipfs_hashmap: HashMap<String, serde_json::Value> = HashMap::new();
                    // ipfs_hashmap.insert(String::from(&param.name), ipfs_content);
                    // metadata_list.insert(String::from("ipfs"), ipfs_hashmap);
                    // .insert(String::from(&param.name), ipfs_content);
                } else {
                    warn!("The response failed\n");
                }
            }
            Err(error) => {
                error!("Error in fetching response -> {:?}\n\n", error);
            }
        }
        let meta: Meta = Meta {
            id: Some(meta_id.clone()),
            owner: Some(transaction_indexed.transaction.from),
            title: Some(meta_title.clone()),
            image: Some(meta_image.clone()),
        };
        let meta_data: MetaData = MetaData { modified: meta };
        return Some(meta_data);
    } else {
        return None;
    }
}
