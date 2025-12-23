use cosmwasm_schema::write_api;
use gmp_sample::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InstantiateMsg,
        query: QueryMsg,
        migrate: MigrateMsg,
    }
}
