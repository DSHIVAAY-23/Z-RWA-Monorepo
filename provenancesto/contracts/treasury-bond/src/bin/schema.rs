use cosmwasm_schema::write_api;
use treasury_contract::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        query: QueryMsg,
        migrate: MigrateMsg,
    }
}
