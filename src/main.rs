use postgres::{Client, NoTls};
use zendb::{CreateBuilder, DeleteBuilder, FindBuilder, UpdateBuilder};

fn main() {
    let mut conn = Client::connect("", NoTls).unwrap();
    // println!("Hello, world!");
    let create = CreateBuilder::new(&mut conn)
        .table("zen")
        .key(&[&"email", "name"])
        .value(&[&"truebillionhari@gmail.com", &"ertyui"])
        .confirm()
        .build();
    println!("{:?}", create);
    // let billion = FindBuilder::new(&mut conn)
    //     .table("zen")
    //     .item(zen::find::Item::Specific(&["email"]))
    //     .condition(zen::Condition {
    //         condition: "email",
    //         value: "rtyui",
    //     })
    //     .condition(zen::Condition {
    //         condition: "email",
    //         value: "yuio",
    //     })
    //     .confirm()
    //     .build();
    // let c = DeleteBuilder::new(&mut conn)
    //     .table("zen")
    //     .condition(Some(zen::Condtion {
    //         key: "name",
    //         value: "tyuioi",
    //     }))
    //     .condition(Some(zen::Condtion {
    //         key: "email",
    //         value: "ytuyiuoi",
    //     }))
    //     .condition(Some(zen::Condtion {
    //         key: "email",
    //         value: "tryui",
    //     }))
    //     .confirm()
    //     .build();
    // let u = UpdateBuilder::new(&mut conn)
    //     .table("zen")
    //     .values(zen::Condition {
    //         condition: "email",
    //         value: "tryuio",
    //     })
    //     .condition(Some(zen::Condition {
    //         condition: "name",
    //         value: "ertuyi",
    //     }))
    //     .condition(Some(zen::Condition {
    //         condition: "email",
    //         value: "ertyui",
    //     }))
    //     .condition(Some(zen::Condition {
    //         condition: "email",
    //         value: "ertyui",
    //     }))
    //     .confirm()
    //     .build();
    // println!("{:?}", billion);
}
