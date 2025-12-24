use cosmwasm_schema::write_api;
use interop_core::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        query: QueryMsg,
        migrate: MigrateMsg,
    }
}
