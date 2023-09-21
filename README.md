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
    1. <details><summary>contract_metadata</summary></details>
    2. <details><summary>contract_abi</summary></details>

</details>
