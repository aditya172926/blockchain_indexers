use crate::structs::index::MethodParam;
use crate::structs::meta::{self, Meta, MetaData};
use crate::structs::transactions::TransactionIndexed;
#[derive(Debug)]
struct PoapMeta {
    from: String,
    to: String,
    tokenId: String,
}

// pub async fn handler_poap_ethereum(transaction_indexed: &TransactionIndexed) -> Option<MetaData> {
//     if transaction_indexed.method.name == "transferFrom"
//         || transaction_indexed.method.name == "safeTransferFrom"
//     {
//         let meta_raw: PoapMeta = PoapMeta {
//             from: transaction_indexed.method.params[0].value.clone(),
//             to: transaction_indexed.method.params[1].value.clone(),
//             tokenId: transaction_indexed.method.params[2].value.clone(),
//         };
//         let mut image = String::new();

//         let meta: Meta = Meta {
//             id: Some(meta_raw.tokenId.clone()),
//             owner: Some(meta_raw.to.clone()),
//             title: Some(meta_raw.tokenId.clone()),
//             image: Some(image),
//         };
//         let meta_data: MetaData = MetaData { modified: meta };
//         Some(meta_data)
//     } else {
//         None
//     }
// }
