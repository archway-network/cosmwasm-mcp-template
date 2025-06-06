# CosmWasm MCP Server Template
MCP server in Rust, for wrapping query and execute entry point messages to be broadcast by a signer. This project template should work with any CosmWasm contract.

### Building this project

To build this project requires the `nightly` build of Rust, this will allow using edition 2024 of rustc.

```sh
# Switch rustc to `nightly` channel
rustup default nightly
```

```sh
# Build for development
cargo build
```

```sh
# Build for deployment
cargo build --release
```

### How to Use

This project is an MCP server template that can be used with any CosmWasm contract, but to use this template with your own contracts, you'll need to make a few small changes.

To use this template with your own contract, implement the following _mandatory_ changes:

#### Step 1 - Update Cargo.toml
* Change the contract dependency in `Cargo.toml`

Remove the following line from `Cargo.toml` and replace it with the dependency for your contract:

```toml
cw20-wrap = { git = "https://github.com/archway-network/cw20-wrap.git", version = "1.0.0", features = ["library"] }
```

#### Step 2 - Ensure your contract can be built as a library
* You _should_ ensure the dependency that you just added to `Cargo.toml` does not export `cosmwasm_std::entry_point`, `query` and `execute`.

For example, your contract should _not_ import `entry_point`, `query` and `execute` (and so on for `instantiate`, `reply`, `migrate`, etc. as is relevant to your project) like this:
```rs
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};
// ...
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    // ...
}
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // ...
}
```

Instead, you should feature gate your contract entry points like this:
```rs
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    // ...
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // ...
}
```

#### Step 3 - Update server.rs
* Change the contract dependency in `src/server.rs`

At the top of the `src/server.rs` file ([see code](https://github.com/archway-network/cosmwasm-mcp-template/blob/main/src/server.rs#L3)), remove the default [cw20-wrap](https://github.com/archway-network/cw20-wrap/tree/main) dependency, and replace it with the corresponding dependency to your contract. Your contract must publicly export `msg::QueryMsg` and `msg::Execute` for the MCP server to be able to create JSON schemas that AI agents can understand.

```
/// Replace the below import with the contract you want the MCP server to support
use cw20_wrap::msg::{ExecuteMsg, QueryMsg};
```

#### Step 4 - Update deployed contract addresses in contract.rs

At the bottom of `src/contract.rs` ([see code](https://github.com/archway-network/cosmwasm-mcp-template/blob/main/src/contract.rs#L22-L25)), update the deployed contract addresses for the mainnet and testnet deployments of your contract. Or, if you want to remove one of the entries, so there's only a testnet or only a mainnet entry, you'll need to remove the entry from the contracts array in the `CwMcp::new()` function ([see code](https://github.com/archway-network/cosmwasm-mcp-template/blob/main/src/server.rs#L35-L46)).

```rs
/// Replace with your deployed contract addresses.
/// This helps the query msg and tx msg builders wrap
/// your query and tx messages to the contract into
/// CosmWasm's `QueryRequest` and `CosmosMessage` types
/// that can be broadcast by a rpc enabled wallet tool
pub static CONTRACT_MAINNET: &str =
    "archway1gaf9nw7n8v5lpjz9caxjpps006kxfcrzcuc8y5qp4clslhven2ns2g0ule";
pub static CONTRACT_TESTNET: &str =
    "archway1r8kepegwhldwqanuurc769l2g0qxlsm2sm6t5rhqjzcerxsgshls267f7a";
```

#### Step 5 (Optional) - Enable MCP tools for any custom types
* If your contract uses any custom types or responses that you think would be beneficial for the AI agent should have access to, there's an example (commented out) in [server.rs](https://github.com/archway-network/cosmwasm-mcp-template/blob/main/src/server.rs#L65-L77) of how to achieve that (see below snippet from `src/server.rs`).

```rs
/// (Optionally) if your contract provides any custom query response types
/// configure this tool so the MCP agent can access them. Allowing the MCP
/// agent to access the custom query responses enables it to provide smarter
/// advice, and summaries, about exacly what data can be fetched when making
/// a query to the contract.
/// @see: src/query.rs
#[tool(description = LIST_QUERY_RESPONSE_DESCR)]
async fn list_query_responses(&self) -> Result<CallToolResult, Error> {
    let schema = schema_for!(AllQueryResponse);
    let serialized: String = serde_json::to_string(&schema).unwrap_or("".to_string());
    Ok(CallToolResult::success(vec![Content::text(serialized)]))
}
```

#### Step 6 (Optional) - Customize LLM instructions
* All server instructions for the system prompt context, and the tool descriptions, are located in `src/instruction.rs`. 
* The contents of `src/instruction.rs` are basic, working examples. When working with complex contracts, and/or multi-contract systems, you'll likely want to improve the tool and server descriptions to provide more detailed context to the LLM.
* For examples of how to improve LLM instructions, and make them customized for your contract, have a look at [instruction.rs](https://github.com/phi-labs-ltd/ambur-mcp/blob/server/stream-http/src/instruction.rs) from the [Ambur MCP server](https://github.com/phi-labs-ltd/ambur-mcp).

#### Step 7 (Optional) - Set MCP server transport mode
* This template supports 3 transport modes: stdio, sse and http-streamable
* This template defaults to stdio transport mode
* About the transport modes:
    - **stdio** - The server will respond using system standard input / output
    - **sse** - Server side events server ([MDN doc](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events))
    - **http-streamable** - A newer standard for remote MCP servers that provides JSON API server functionality ([Claudemcp doc](https://www.claudemcp.com/docs/streamable-http))


### Optimizing AI Accuracy

**Adding doc comments in your is important for schema generation**

Even after expanding your server instructions, tool descriptions and tool parameter descriptions, you may find the AI continues to provide inaccurate or misleading data, or few details about the contract entry points. Normally, this happens due to lack of doc comments (e.g. triple slash comments '///') in your contract source code for `msg::QueryMsg` and `msg::ExecuteMsg`

This happens because [schemars](https://crates.io/crates/schemars) embeds doc comments directly into the schema as a description metadata field.

Here's an example of a well commented `msg::QueryMsg` that will help guide the LLM Agent:

```rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get all swaps (enumerable)
    /// Return type: ListResponse
    List {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Get all Collection Offers (enumerable)
    /// Return type: ListResponse
    ListCollectionOffers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    // ...
}
```

The above 2 variants of `QueryMsg` will generate the following embedded descriptions into the generated schemas, which can be very helpful for LLMs that need to interpret your query entry points:
```js
[
    {
      "description": "Get all swaps (enumerable) Return type: ListResponse",
      "type": "object",
      "required": [
        "list"
      ],
      // ... 
    },
    {
      "description": "Get all Collection Offers (enumerable) Return type: ListResponse",
      "type": "object",
      "required": [
        "list_collection_offers"
      ],
      // ...
    },
]
```

### Multi-Contract Systems
* Sometimes it makes sense to build an MCP server that supports multiple contracts. The strategy for achieving this is straight forward:

1. Name space the contracts (e.g. to avoid duplicate symbol imports)
2. Implement pattern matching and tool parameters for switching between the different contracts
3. For a full multi-contract example, see the [Ambur MCP server](https://github.com/phi-labs-ltd/ambur-mcp)

### Tools provided by this MCP server template

By default, this MCP server provides the following 6 tools and functionality.

1. `list_contract_deployments` - Lists Ambur core contract addresses (mainnet and testnet)
2. `list_nft_collections` - Lists Ambur NFTs (mainnet and testnet contract addresses, collection name, and collection description)
3. `list_query_entry_points` - Lists the queries that can be made to the core Ambur marketplace contract
4. `build_query_msg` - Build a query to the core Ambur marketplace contract, that can be broadcast by an RPC connected wallet
5. `list_tx_entry_points` - Lists the transactions that can be made to the core Ambur marketplace contract
6. `build_execute_msg` - Build a transaction to the core Ambur marketplace contract, that can be signed and broadcast by an RPC connected wallet

### Connecting MCP to Claude Desktop

The below instructions assume the MCP server is being built in `stdio` mode (which is the easiest mode to configure for Claude desktop). 

Build a release binary and point the mcp server's `command` to its path. No run arguments (`args`) are required:
```js
// claude_desktop_config.json
{
  "mcpServers": {
    "ambur": {
      "command": "/your-computer-path/cosmwasm-mcp-template/target/release/cosmwasm-mcp-template",
      "args": []
    }
  }
}
```

For Virtual Machine setups and WSL users, execute the VM as the `command` and use run arguments (`args`) to point the VM where to run the binary:
```js
// claude_desktop_config.json
{
  "mcpServers": {
    "ambur": {
      "command": "wsl.exe",
      "args": [
        "bash",
        "-ic",
        "/your-vm-path/cosmwasm-mcp-template/target/release/cosmwasm-mcp-template",
      ]
    }
  }
}
```

### Connecting MCP to LangGraph

[@langchain/mcp-adapters](https://www.npmjs.com/package/@langchain/mcp-adapters) must be installed in the graph project. This package will convert the MCP endpoints into Graph tools.

#### Using @langchain/mcp-adapters

```ts
// graph.ts
import { MultiServerMCPClient } from "@langchain/mcp-adapters";
// ...
// Create client and connect to server
const client = new MultiServerMCPClient({
  throwOnLoadError: true,
  prefixToolNameWithServerName: true,
  additionalToolNamePrefix: "mcp",
  mcpServers: {
    cosmwasm_contract: {
      transport: "sse",
      url: "http://localhost:8000", // Or, URL + IP of a remote host
      useNodeEventSource: true,
      reconnect: {
        enabled: true,
        maxAttempts: 5,
        delayMs: 2000,
      },
      // Or, uncomment to use transport mode `http-streamable`:
      // url: "http://localhost:8000",
      // headers: {},
      // automaticSSEFallback: false
    },
  },
});

const tools = await client.getTools();
// ...
```
