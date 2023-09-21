# indexer-database

<details>
<summary><h2>main.rs</h2></summary>

### fn main()

returns -> Result which can be
    - empty result, programming successful exit of the program
    - error

**Description** - THe main function is the starting point of the indexer. It takes no parameters and is responsible for initializing the *indexer data environment* and calling other functions.

**Initializing Data Environment** - Before calling the functions which actually indexes the transactions we need to set some data which are required for these functions to work properly.

**List of required data -** 

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

Key points

- contract_address: Address of the contract that we will index
- read_abi_from: Address of the contract from where we get the Application Binary Interface (ABI) of the project smart contract. This is usually required if the above `contract_address` is a proxy contract, then we cannot use that ABI, instead we get it from `read_abi_from`, which will be given by the user.
- chain_id: Id of the chain where the contract is deployed
- function_of_interest: A list of functions which will be indexed if a transaction happens on anyone of them

</details>

<details>
<summary>2. contract_result</summary>

This is tuple of `contract_metadata`, `contract_fetched_abi`, and `contract_abi`.
The function that returns the data for this comes from utils.rs -> `get_contract_data(contract_slug)`????. 
</details>

<details>
<summary>3. contract_abi</summary>

This contains the ABI of the contract that is on the second index position of `contract_result`.
At first the smart contract ABI is a `String` type, stored in `contract_fetched_abi` variable.
We convert the `String` type to

</details>

</details>

<details>
<summary>utils.rs</summary>



</details>