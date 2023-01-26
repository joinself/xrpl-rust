use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// This method retrieves all of sell offers for the specified NFToken.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NftSellOffers<'a> {
    /// The unique identifier of a NFToken object.
    pub nft_id: &'a str,
    /// The request method.
    #[serde(default = "RequestMethod::nft_sell_offers")]
    pub command: RequestMethod,
}

impl Default for NftSellOffers<'static> {
    fn default() -> Self {
        NftSellOffers {
            nft_id: "",
            command: RequestMethod::NftSellOffers,
        }
    }
}

impl Model for NftSellOffers<'static> {}
