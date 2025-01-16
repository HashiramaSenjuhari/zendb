use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ID<'billionaire>(pub &'billionaire str);

#[derive(Debug, Deserialize, Serialize)]
pub struct OneToOne<'billionaire> {
    pub table: &'billionaire str,
    pub table_field: &'billionaire str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OneToMany<'billionaire> {
    pub table: &'billionaire str,
    pub table_field: &'billionaire str,
}

#[derive(Debug, Serialize)]
pub struct UNIQUE;

#[derive(Debug, Serialize)]
pub struct STRING;

#[derive(Debug, Serialize)]
pub struct BOOL;

#[derive(Debug, Serialize)]
pub struct NUMBER;

#[derive(Debug, Serialize)]
pub struct JSON;

#[derive(Debug, Serialize)]
pub struct DEFAULT<'billionaire>(pub &'billionaire str);

// #[derive(Debug)]
// pub struct Condition<K, V>
// where
//     K: Into<String>,
//     V: Into<String>,
// {
//     k: K,
//     v: V,
// }
// #[derive(Debug)]
// pub struct Where<K, V>
// where
//     K: Into<String>,
//     V: Into<String>,
// {
//     and: Condition<K, V>,
// }
// pub struct billionaire {
//     billionaire: String,
//     billionandollar: Vec<billiondollar>,
// }
// pub struct billiondollar {
//     billiondollar: String,
// }
#[derive(Debug, Serialize)]
pub struct Date<'billionaire>(pub &'billionaire str);

#[derive(Debug, Serialize)]
pub struct Time<'billionaire>(pub &'billionaire str);

#[derive(Debug, Serialize)]
pub struct DateTime<'billionaire>(pub &'billionaire str);

#[derive(Debug, Serialize)]
pub struct Geography;

#[derive(Debug, Serialize)]
pub struct NOTNULL;

#[derive(Debug, Serialize)]
pub struct PRIMARY;

#[derive(Debug, Serialize)]
pub struct INDEX;

