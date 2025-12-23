use cosmwasm_schema::write_api;
use cw20_marker::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        query: QueryMsg,
        migrate: MigrateMsg,
    }
}
