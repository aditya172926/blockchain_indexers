use ethers::types::H160;
use std::collections::HashMap;
use crate::structs::extract::Schema;
use crate::structs::meta::{Meta, MetaIndexed, MetaResult};
use crate::structs::transactions::TransactionIndexed;
#[derive(Debug)]
struct UdMeta {
    from: H160,
    to: H160,
    tokenId: String,
}

pub async fn handler(from: H160, to: H160, tokenId: String, slug: String) -> Option<MetaIndexed> {
    let meta_raw: UdMeta = UdMeta {
        from,
        to,
        tokenId: tokenId.clone(),
    };
    let mut raw_data: HashMap<String, String> = HashMap::from([
        (String::from("from"), format!("0x{:x}", meta_raw.from)),
        (String::from("to"), format!("0x{:x}", meta_raw.to)),
        (String::from("tokenId"), meta_raw.tokenId),
    ]);

    let mut image = String::from("");
    let meta_modified: Meta = Meta {
        id: Some(tokenId.clone()),
        owner: Some(meta_raw.to),
        title: Some(tokenId.clone()),
        image: Some(image),
        content: Some("".to_string()),
    };

    let meta_indexed = MetaIndexed {
        owner: to,
        id: tokenId,
        slug,
        raw: raw_data,
        modified: Some(meta_modified),
        createdAt: "".to_string(),
        updatedAt: "".to_string(),
    };

    // println!("\n\n\nMeta indexed {:?} \n\n\n", meta);

    return Some(meta_indexed);
}

pub async fn handler_ud(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult> {
    let transaction_indexed_methods = transaction_indexed.method.clone().unwrap();
    if transaction_indexed_methods.name == "transferFrom"
        || transaction_indexed_methods.name == "safeTransferFrom"
    {
        let meta_data = handler(
            transaction_indexed_methods.params[0]
                .clone()
                .into_address()
                .unwrap(),
            transaction_indexed_methods.params[1]
                .clone()
                .into_address()
                .unwrap(),
            transaction_indexed_methods.params[2].to_string(),
            schema.slug.clone(),
        )
        .await;

        let result: MetaResult = MetaResult {
            id: transaction_indexed_methods.params[2].to_string(),
            owner: transaction_indexed_methods.params[0].to_string(),
            slug: schema.slug.clone(),
            insert: meta_data,
            update: None,
            source: transaction_indexed.clone(),
        };
        return Some(result);
    } else if transaction_indexed_methods.name == "setOwner" {
        let update_obj: HashMap<String, String> = HashMap::from([
            (
                String::from("document.owner"),
                format!("0x{}", transaction_indexed_methods.params[0]),
            ),
            (
                String::from("document.raw.to"),
                format!("0x{}", transaction_indexed_methods.params[0]),
            ),
            (
                String::from("document.modified.owner"),
                format!("0x{}", transaction_indexed_methods.params[0]),
            ),
        ]);
        let result = MetaResult {
            id: transaction_indexed_methods.params[1].to_string(),
            owner: format!("0x{}", transaction_indexed_methods.params[0]),
            slug: schema.slug.clone(),
            insert: None,
            update: Some(update_obj),
            source: transaction_indexed.clone(),
        };
        return Some(result);
    } else {
        return None;
    }
}
