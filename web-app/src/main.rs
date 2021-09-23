#![recursion_limit = "1024"]

mod app;
mod components;
mod pages;
mod utils;

use crate::app::Props;
use crate::utils::{IpfsService, LocalStorage, Web3Service};

/// ENS Domain name with "defluencer" as subdomain. egg. defluencer.sionois.eth
/// OR a beacon CID.
const BEACON: &str = "bafyreia54drccgxtadnrlqpnlxn2lvndslkiw76ihwvuxi23fjtqcfwx6i";

fn main() {
    let web3 = Web3Service::new();
    let storage = LocalStorage::new();
    let ipfs = IpfsService::new(&storage);

    let props = Props {
        web3,
        ipfs,
        storage,
        beacon: BEACON,
    };

    yew::start_app_with_props::<app::App>(props);
}
