use std::{collections::HashMap, process::exit};

use ethers::types::H160;
use log::info;

use crate::{
    handlers::{
        ens_ethereum::handler::{
            handler_event_register_ens_by_base, handler_event_register_ens_by_controller,
            handler_event_register_ens_by_controller_old, handler_event_renew_ens_by_base,
            handler_event_renew_ens_by_controller, handler_event_transfer_ens_by_base,
        },
        poap_ethereum::handler::handler_transfer_poap,
    },
    structs::{
        contracts::ContractIndexed,
        extract::Schema,
        transactions::{TransactionEvent, TransactionIndexed},
    },
};

pub async fn load_ens_event(
    schema: &Schema,
    contracts: &mut Vec<ContractIndexed>,
    transaction_indexed: &TransactionIndexed,
) -> Option<HashMap<String, String>> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();

    for event in transaction_indexed.events.as_ref().unwrap() {
        if schema.slug == "ens_ethereum"
            && format!("0x{:x}", event.topic0)
                == "0x69e37f151eb98a09618ddaa80c8cfaf1ce5996867c489f45b555b412271ebf27"
        {
            // ENSRegisterController
            handler_event_register_ens_by_controller(&mut meta_raw, &event).await;
        }

        if schema.slug == "ens_ethereum"
            && format!("0x{:x}", event.topic0)
                == "0xb3d987963d01b2f68493b4bdb130988f157ea43070d4ad840fee0466ed9370d9"
        {
            //ENSRegisterBase
            handler_event_register_ens_by_base(&mut meta_raw, &event).await;
        }

        if schema.slug == "ens_ethereum"
            && format!("0x{:x}", event.topic0)
                == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        {
            //ENSTransferBase
            handler_event_transfer_ens_by_base(&mut meta_raw, event.clone()).await;
        }

        if schema.slug == "ens_ethereum"
            && format!("0x{:x}", event.topic0)
                == "0x3da24c024582931cfaf8267d8ed24d13a82a8068d5bd337d30ec45cea4e506ae"
        {
            // ENSRenewController
            //ENSRenewByOldController
            handler_event_renew_ens_by_controller(&mut meta_raw, &event).await;
        }

        if schema.slug == "ens_ethereum"
            && format!("0x{:x}", event.topic0)
                == "0xca6abbe9d7f11422cb6ca7629fbf6fe9efb1c621f71ce8f02b9f2a230097404f"
        {
            // ENSRegisterByOldController
            handler_event_register_ens_by_controller_old(&mut meta_raw, &event).await;
        }

        if schema.slug == "ens_ethereum"
            && format!("0x{:x}", event.topic0)
                == "0x9b87a00e30f1ac65d898f070f8a3488fe60517182d0a2098e1b4b93a54aa9bd6"
        {
            //ENSRenewBase
            handler_event_renew_ens_by_base(&mut meta_raw, contracts[1].instance.clone(), &event)
                .await;
        }
    }

    info!("meta raw is : {:?} ", meta_raw);

    None
}
