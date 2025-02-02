// use std::io::Read;

// use postgres::types::FromSql;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ID<Billionaire>(pub Billionaire);

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
pub struct FLOAT;

#[derive(Debug, Serialize)]
pub struct JSON;

#[derive(Debug, Serialize)]
pub struct DEFAULT<Billionaire>(pub Billionaire);

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
pub struct Date<Billionaire>(pub Billionaire);

#[derive(Debug, Serialize)]
pub struct Time<Billionaire>(pub Billionaire);

#[derive(Debug, Serialize)]
pub struct DateTime<Billionaire>(pub Billionaire);

#[cfg(feature = "geography")]
#[derive(Debug, Serialize)]
pub struct Geography<Billionaire>(pub Billionaire);

#[derive(Debug, Serialize)]
pub struct NOTNULL;

#[derive(Debug, Serialize)]
pub struct PRIMARY;

#[derive(Debug, Serialize)]
pub struct INDEX;

// * POINTS
#[cfg(feature = "geography")]
#[derive(Debug, Serialize)]
pub struct POINT<Billionaire>(pub Billionaire);

#[cfg(feature = "geography")]
#[derive(Debug, Serialize)]
pub struct POLYGON<Billionaire>(pub Billionaire);

#[cfg(feature = "geography")]
#[derive(Debug, Serialize)]
pub struct Epsg4326;

#[cfg(feature = "geography")]
#[derive(Debug, Serialize)]
pub struct Epsg3857;

#[cfg(feature = "geography")]
pub fn point_epsg_4326(lat: f64, lon: f64) -> String {
    let value = format!("SRID=4326;POINT({} {})", lon, lat);
    value
}

#[cfg(feature = "geography")]
pub fn point_epsg_3857(lat: &str, lon: &str) -> String {
    let value = format!("SRID=4326;POINT({} {})", lon, lat);
    value
}

pub mod id {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct UUID;

    #[derive(Debug, Serialize)]
    pub struct CUID;

    #[derive(Debug, Serialize)]
    pub struct AUTO;

    #[derive(Debug, Serialize)]
    pub struct BIGINT;
}

#[derive(Debug, Serialize)]
pub struct NOW;

#[derive(Debug, Serialize)]
pub struct CUSTOME;

// pub struct Points {
//     pub point: String,
// }

// impl<'a> FromSql<'a> for Points {
//     fn from_sql(
//         ty: &postgres::types::Type,
//         mut raw: &'a [u8],
//     ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
//         let mut value = String::new();
//         raw.read_to_string(&mut value);
//         println!("type {}", ty);
//         println!("data {:?}", raw);
//         println!("{}", value);
//         Ok(Points { point: value })
//     }
//     fn accepts(ty: &postgres::types::Type) -> bool {
//         true
//     }
// }
