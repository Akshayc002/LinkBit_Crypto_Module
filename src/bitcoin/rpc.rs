use bitcoincore_rpc::{Auth, Client};

pub fn rpc_client() -> Client {
    let url = std::env::var("BTC_RPC_URL").unwrap();
    let user = std::env::var("BTC_RPC_USER").unwrap();
    let pass = std::env::var("BTC_RPC_PASS").unwrap();

    Client::new(&url, Auth::UserPass(user, pass)).unwrap()
}
