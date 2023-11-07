use std::{collections::HashMap, process::exit};

use crate::{
    handlers::{
        ens_ethereum::handler::{
            handler_event_register_ens_by_base, handler_event_register_ens_by_controller,
            handler_event_register_ens_by_controller_old, handler_event_renew_ens_by_base,
            handler_event_renew_ens_by_controller, handler_event_transfer_ens_by_base,
        },
        poap_ethereum::handler::handler_transfer_poap,
    },
    structs::{contracts::ContractIndexed, extract::Schema, transactions::TransactionEvent},
};

pub async fn handler_data_from_event(
    schema: &Schema,
    contract: &mut ContractIndexed,
    event: TransactionEvent,
) -> Option<HashMap<String, String>> {
    if schema.slug == "poap_ethereum"
        && format!("0x{:x}", event.topic0)
            == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
    {
        return Some(handler_transfer_poap(contract.instance.clone(), event.clone()).await);
    } else if schema.slug == "ens_ethereum"
        && format!("0x{:x}", event.topic0)
            == "0x69e37f151eb98a09618ddaa80c8cfaf1ce5996867c489f45b555b412271ebf27"
    {
        // ENSRegisterController
        return Some(handler_event_register_ens_by_controller(&event).await);
    } else if schema.slug == "ens_ethereum"
        && format!("0x{:x}", event.topic0)
            == "0xb3d987963d01b2f68493b4bdb130988f157ea43070d4ad840fee0466ed9370d9"
    {
        //ENSRegisterBase
        return Some(handler_event_register_ens_by_base(&event).await);
    } else if schema.slug == "ens_ethereum"
        && format!("0x{:x}", event.topic0)
            == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
    {
        //ENSTransferBase
        return Some(
            handler_event_transfer_ens_by_base(contract.instance.clone(), event.clone()).await,
        );
    } else if schema.slug == "ens_ethereum"
        && format!("0x{:x}", event.topic0)
            == "0x3da24c024582931cfaf8267d8ed24d13a82a8068d5bd337d30ec45cea4e506ae"
    {
        // ENSRenewController
        //ENSRenewByOldController
        return Some(handler_event_renew_ens_by_controller(&event).await);
    } else if schema.slug == "ens_ethereum"
        && format!("0x{:x}", event.topic0)
            == "0xca6abbe9d7f11422cb6ca7629fbf6fe9efb1c621f71ce8f02b9f2a230097404f"
    {
        // ENSRegisterByOldController
        return Some(handler_event_register_ens_by_controller_old(&event).await);
    } else if schema.slug == "ens_ethereum"
        && format!("0x{:x}", event.topic0)
            == "0x9b87a00e30f1ac65d898f070f8a3488fe60517182d0a2098e1b4b93a54aa9bd6"
    {
        //ENSRenewBase
        return Some(handler_event_renew_ens_by_base(contract.instance.clone(), &event).await);
    } else {
        None
    }
}
