use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use provwasm_std::Marker;

#[cw_serde]
pub struct MarkerInfo {
    pub marker_account: Marker,
    pub coins: Vec<Coin>,
}
