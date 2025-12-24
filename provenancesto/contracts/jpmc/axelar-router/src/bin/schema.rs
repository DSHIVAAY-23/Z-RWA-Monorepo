use axelar_router::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InstantiateMsg,
        query: QueryMsg,
        migrate: MigrateMsg,
    }
}
