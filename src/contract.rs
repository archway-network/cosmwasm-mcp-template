use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Network {
    Mainnet,
    Testnet,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CwContract {
    pub network: Network,
    pub chain_id: String,
    pub contract_address: String,
}

/// Replace with your deployed contract addresses.
/// This helps the query msg and tx msg builders wrap
/// your query and tx messages to the contract into
/// CosmWasm's `QueryRequest` and `CosmosMessage` types
/// that can be broadcast by a rpc enabled wallet tool
pub static CONTRACT_MAINNET: &str =
    "archway1gaf9nw7n8v5lpjz9caxjpps006kxfcrzcuc8y5qp4clslhven2ns2g0ule";
pub static CONTRACT_TESTNET: &str =
    "archway1r8kepegwhldwqanuurc769l2g0qxlsm2sm6t5rhqjzcerxsgshls267f7a";
