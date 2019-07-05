use super::{db_conn, schema::peers, CONFIG};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Peer {
    pub id: i32,
    pub address: String,
}

type Result<T> = std::result::Result<T, diesel::result::Error>;

impl Peer {
    pub fn all() -> Result<Vec<Peer>> {
        use self::peers::dsl::*;

        peers.load::<Peer>(&*db_conn())
    }

    pub fn create(addr: &str) -> Result<usize> {
        let new_peer = NewPeer { address: addr.to_owned() };

        diesel::insert_into(peers::table)
            .values(&new_peer)
            .execute(&*db_conn())
    }

    pub fn get(addr: &str) -> Result<Vec<Peer>> {
        use self::peers::dsl::*;

        peers.filter(address.eq(addr)).load::<Peer>(&*db_conn())
    }

    pub fn delete(addr: &str) -> Result<usize> {
        use self::peers::dsl::*;

        diesel::delete(peers.filter(address.eq(addr))).execute(&*db_conn())
    }

    pub fn load_initial_peers() {
        for initial_peer in CONFIG.initial_peers() {
            let new_peer = NewPeer {
                address: initial_peer.address(),
            };
            diesel::insert_into(peers::table)
                .values(&new_peer)
                .execute(&*db_conn());
        }
    }

    pub fn address(&self) -> &String {
        &self.address
    }
}

#[derive(Insertable)]
#[table_name = "peers"]
pub struct NewPeer {
    pub address: String,
}