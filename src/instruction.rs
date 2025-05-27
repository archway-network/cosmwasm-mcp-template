// System Instructions

/// (Optionally) replace the below with a description that matches the functionality 
/// provided by the contract which the MCP server to supports
pub static SERVER_INFO_DESCR: &str = r#"
This MCP server provides tools for aiding with queries and transactions to 
a deployed version of a contract. It does not broadcast these 
queries or txs, nor does it sign the txs. 

It allows users to perform the following actions: 
- List available contract addresses and their associated network and 
chain id
- List the available query entry points, and any parameters required for building 
them ('list_query_entry_points')
- Build a query message that can be broadcast by any RPC enabled tool 
('build_query_msg')
- List the available execute (tx) entry points, and any parameters required for 
building them ('list_tx_entry_points')
- Build an execute message (tx message) that can be signed and broadcast by any 
RPC enabled tool with wallet signing capabilities"#;

// Contract Instructions
pub static LIST_CONTRACTS_DESCR: &str = r#"
Call this tool to get a list of contract addresses where the contract has been deployed. 
This tool is helpful for discovering the mainnet and testnet contract addresses for the 
smart contract."#;

// Query Instructions

/// (Optionally) replace the below with a more detailed description that matches the 
/// query functionality provided by the contract the MCP server to supports
pub static LIST_QUERY_ENTRY_POINTS_DESCR: &str = r#"
Call this tool to get a list of possible queries that can be made (e.g. query entry 
points) to the contract, as well as their associated calling parameters. This tool 
is helpful for discovering what parameters a user must provide in order to build a 
prepared query message for a query to the smart contract.

The response provided from this tool is a JSON schema for the QueryMsg enum of the 
smart contract. It would be too verbose to provide it to your chat partner, so 
summarizing it will be crucial."#;

// (Optionally) replace the below with a detailed description of any custom query 
// responses (if any) provided by queries to the contract your MCP server supports
// pub static LIST_QUERY_RESPONSE_DESCR: &str = "";

/// (Optionally) replace the below with a more detailed description of the query 
/// messages that can be built by the MCP server
pub static BUILD_QUERY_MSG_DESCR: &str = r#"
Call this tool to build a prepared query message for a query to the contract. This tool 
won't broadcast the query or return the query result, but can be combined with any RPC 
connected query tool that accepts a well-formed Cosmos QueryRequest.

There are two calling parameters required when calling this tool: 1) the contract address 
('contract_addr'). E.g. the mainnet or testnet contract address; for deriving the deployed 
contract addresses, see tool: 'list_contract_deployments'; 2) the QueryMsg variant to be 
built into a Cosmos QueryRequest; for deriving the appropriate QueryMsg variant (and the 
calling variant's parameters), see tool: 'list_query_entry_points'."#;

/// (Optionally) replace the below with a more detailed description that matches the 
/// execute functionality provided by the contract the MCP server to supports
pub static LIST_TX_ENTRY_POINTS_DESCR: &str = r#"
Call this tool to get a list of possible transactions that can be made (e.g.  
execute entry points) to the Ambur marketplace contract, as well as their 
associated calling parameters. This tool is helpful for discovering what parameters 
a user must provide in order to build a perpared execute message for a tx to the 
Ambur NFT marketplace smart contract.

The response provided from this tool is a JSON schema for the ExecuteMsg enum of the 
Ambur maketplace contract. It would be too verbose to provide it to your chat 
partner, so summarizing it will be crucial."#;

/// (Optionally) replace the below with a more detailed description of the execute  
/// messages that can be built by the MCP server
pub static BUILD_EXECUTE_MSG_DESCR: &str = r#"
Call this tool to build a prepared execute message for a transaction to the smart 
contract. This tool won't sign the message, or broadcast it to the blockchain, but can be 
combined with any RPC connected tx tool that accepts a well-formed CosmosMsg for an 
ExecuteMsg variant for any valid execute (tx) entry point to the contract.

There are three calling parameters required when calling this tool: the contract address 
('contract_addr', e.g. either the mainnet or testnet contract address; see 
tool: 'list_contract_deployments'), the amount of native funds ('payment') to send in the 
transaction, and the ExecuteMsg variant ('execute_msg') to be built into a CosmosMsg that 
can be signed and broadcast by an RPC connected signing wallet."#;