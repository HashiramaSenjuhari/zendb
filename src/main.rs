use postgres::{Client, NoTls};
use zen::{CreateBuilder, DeleteBuilder, FindBuilder, UpdateBuilder};

fn main() {
    let mut conn = Client::connect("host=127.0.0.1 port=5432 dbname=t user=postgres password=hariclash123 connect_timeout=10 sslmode=prefer", NoTls).unwrap();
    // println!("Hello, world!");
    let create = CreateBuilder::new(&mut conn)
        .table("zenn")
        .key(&[&"email"])
        .value(&[&"truebillionhari@gmail.com"])
        .setttt()
        .build();
    println!("{:?}", create);
    let billion = FindBuilder::new(&mut conn)
        .table("zen")
        .item(zen::find::Item::Specific(&[""]))
        .condition(zen::Condition {
            condition: "email",
            value: "rtyui",
        })
        .settt()
        .build();
    let c = DeleteBuilder::new(&mut conn)
        .table("zen")
        .condition(Some(zen::Condtion { key: "", value: "" }))
        .build()
        .build();
    let u = UpdateBuilder::new(&mut conn)
        .table("zen")
        .values(zen::Condition {
            condition: "email",
            value: "tryuio",
        })
        .condition(None)
        .setttt()
        .build();
    println!("{:?}", billion);
}
