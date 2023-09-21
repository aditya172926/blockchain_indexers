# indexer-database

<details>
<summary><h2>main.rs</h2></summary>

### fn main()

returns -> Result which can be
    - empty result, programming successful exit of the program
    - error

**Description** - THe main function is the starting point of the indexer. It takes no parameters and is responsible for initializing the *indexer data environment* and calling other functions.

**Initializing Data Environment** - Before calling the functions which actually indexes the transactions we need to set some data which are required for these functions to work properly.

List of required data
<details>
<summary>1. contract_metadata</summary>
The contract_metadata is a struct of type `ContractMetaData` which contains the basic data fetched from our Mongodb collection `contracts`.

The `ContractMetaData` type:
```
pub struct ContractMetaData {
    pub contract_address: String,
    pub read_abi_from: String,
    pub chain_id: String,
    pub function_of_interest: String,
    pub contract_name: String,
    pub contract_description: String,
    pub contract_slug: String,
    pub method_of_interest:std::collections::HashSet<String>,
    pub methods:Document,
}
```
</details>

<details><summary>2. contract_abi</summary></details>

</details>
