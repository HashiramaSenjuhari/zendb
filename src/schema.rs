use postgres::Client;

struct ZenSchema {
    id: u64,
    name: String,
    email: String,
    image: String,
    created_at: u64,
}
struct ZenMaster<'master> {
    connection: &'master mut Client,
}

// impl<'master> ZenMaster<'master> {
//   pub fn blueprint()
// }
