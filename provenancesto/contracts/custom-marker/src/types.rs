use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use provwasm_std::Marker as ProvMarker;

#[cw_serde]
pub struct Marker {
    pub marker_account: ProvMarker,
    pub coins: Vec<Coin>,
}
