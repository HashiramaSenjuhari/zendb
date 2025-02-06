// use std::io;
// use std::time::Instant;

// use chrono::format::Numeric;
// use postgres::types::PgLsn;
// use postgres::NoTls;
use rusty_postgres::method::types::id::{BIGINT, CUID, UUID};
// use rusty_postgres::method::types::{DEFAULT, FLOAT, NOW};
use rusty_postgres::{
    container,
    method::types::{
        Date, DateTime, Time, BOOL, ID, INDEX, NOTNULL, NUMBER, PRIMARY, STRING, UNIQUE,
    },
};
use rusty_postgres::{
    create, custome_execute, custome_query, delete, delete_table, find_many, find_one, model,
    update, AsyncNoTls, TokioClient,
};
use uuid::Uuid;
// use tokio_postgres::GenericClient;
// use tokio_postgres::GenericClient;
// use uuid::Uuid;
// use rusty_postgres::{
//     container, count, create, create_brin_index, create_gin_index, create_index, create_partition,
//     delete, delete_table, find_many, find_one, full_search, horizontal_splitting, model,
//     pagination, partitions, ranked_search, show_index_list, table_structure, transaction,
// };
// use rusty_postgres::{similar_search, update};

// use std::fs::DirBuilder;
// use std::fs::File;
// use std::io::Write;

// union Billionaire {
//     billionaire: String,
// }
#[tokio::main]
async fn main() {
    // let billion: Point = 24;
    // #[derive(Clone)]
    // let mut postgres = postgres::Client::connect("host=127.0.0.1 port=5432 dbname=t user=postgres password=hariclash123 connect_timeout=10 sslmode=prefer", NoTls).unwrap();
    let  postgres = TokioClient::connect("host=127.0.0.1 port=5432 dbname=t user=postgres password=hariclash123 connect_timeout=10 sslmode=prefer", AsyncNoTls).await.unwrap();
    // let b = postgres.batch_execute("").await;
    // println!("{:?}", b);

    // println!("Hello, world!");
    // let create = CreateBuilder::new(&mut conn)
    //     .table("zen")
    //     .key(&[&"email", "name"])
    //     .value(&[&"truebillionhari@gmail.com", &"ertyui"])
    //     .confirm()
    //     .build();
    // println!("{:?}", create);
    // let model = model! {
    //     "elon" => {
    //         "id" => {
    //             ID("uuid")
    //         },
    //         "name" => {
    //             Date("now()"),
    //             Time("now()"),
    //             DateTime("now()")
    //         }
    //     }
    // };
    // let container = container! {
    //     client =>  postgres,
    //     models => {
    //         model
    //     }
    // };
    // delete_table! {
    //     connection => postgres,
    //     model => "billionaires",
    //     cascade
    // }
    // .unwrap();
    // delete_table! {
    //     connection => postgres,
    //     model => "billionaires",
    //     cascade
    // }
    // .unwrap();
    // delete_table! {
    //     connection => postgres,
    //     model => "place",
    //     cascade
    // }
    // .unwrap();
    // delete_table! {
    //     connection => postgres,
    //     model => "rating",
    //     cascade
    // };
    // let model = model! {
    //     "elon" => {
    //         "id" => {
    //             ID("uuid")
    //         },
    //         "name" => {
    //             Date("now()")
    //             // Time("now()"),
    //             // DateTime("now()")``
    //         },
    //         "time" => {
    //             Time("now()")
    //         },
    //         "fingerprint" => {
    //             DateTime("now()")
    //         },
    //         "names" => {
    //             Stringg
    //         },
    //         "age" => {
    //             Number
    //         },
    //         "billionaire" => {
    //             Boolean
    //         }
    //         // "area" => {
    //         //     Geography
    //         // }
    //     },
    //     index:{
    //         "id",
    //         "billionaire"
    //     }
    // };
    // let emodel = model! {
    //     connection => postgres,
    //     "user_" => {
    //         "id" => {
    //             ID(CUID),NOTNULL,UNIQUE,INDEX
    //         },
    //         "name" => {
    //             Date(NOW)
    //         },
    //         "time" => {
    //             Time(NOW)
    //         },
    //         "fingerprint" => {
    //             DateTime(NOW)
    //         },
    //         "age" => {
    //             NUMBER
    //         },
    //         "story" => {
    //             STRING,NOTNULL,PRIMARY
    //         }
    //     }
    // };
    // let ebillionairemodel = model! {
    //     "shop" => {
    //         "id" => {
    //             ID(BIGINT),NOTNULL,UNIQUE,INDEX
    //         },
    //         "name" => {
    //             Date(NOW)
    //         },
    //         "time" => {
    //             Time(NOW)
    //         },
    //         "fingerprint" => {
    //             DateTime(NOW)
    //         },
    //         "age" => {
    //             NUMBER,DEFAULT(24)
    //         },
    //         "place" => {
    //             STRING,NOTNULL,UNIQUE,INDEX,PRIMARY
    //         },
    //         "accuracy" => {
    //             FLOAT,DEFAULT(0.0)
    //         },
    //         "location" => {
    //            Geography(POINT(Epsg4326)),NOTNULL
    //         },
    //         "bool" => {
    //             BOOL
    //         }
    //     },
    //     partition:{
    //         type:"list",
    //         to:"place"
    //     }
    // };
    // let ebillion = model! {
    //     "shop" => {
    //         "id" => {
    //             ID(UUID),NOTNULL,UNIQUE
    //         },
    //         "name" => {
    //             Date(NOW)
    //         },
    //         "time" => {
    //             Time(NOW)
    //         },
    //         "fingerprint" => {
    //             DateTime(NOW)
    //         },
    //         "age" => {
    //             NUMBER,DEFAULT(24)
    //         },
    //         "place" => {
    //             STRING,NOTNULL,UNIQUE,INDEX,PRIMARY
    //         },
    //         "location" => {
    //             Geography(POINT(Epsg4326))
    //         },
    //         "accuracy" => {
    //             FLOAT,DEFAULT(0.0)
    //         },
    //         "bool" => {
    //             BOOL
    //         }
    //     },
    //     partition:{
    //         type:"list",
    //         to:"place"
    //     }
    // };
    // let partition = create_partition {
    //     connection => postgres,
    //     model:"",
    // };
    // println!("{}", ebillionairemodel);
    // println!("{}", emodel);
    // let default = DEFAULT("billionairehari");
    // default.0

    // println!("{}", ebillionairemodel);
    // let value = default.split("(").nth(1).unwrap().trim_end_matches(")");
    // println!("{}", emodel);

    // let cluster = emodel.find("CLUSTER").unwrap();
    // println!("{}", emodel);
    // let billionaires = model! {
    //     "billionaires" => {
    //         "id" => {
    //             ID(UUID),
    //             PRIMARY,
    //             NOTNULL,INDEX
    //         },
    //         "place" => {
    //             NUMBER
    //         },
    //         "location" => {
    //             Geography(POINT(Epsg4326))
    //         }
    //     }
    // };
    // let place = model! {
    //     "place" => {
    //         "id" => {
    //             ID(UUID),
    //             PRIMARY,
    //             NOTNULL,INDEX
    //         },
    //         "name" => {
    //             STRING
    //         },
    //         "user_id" => {
    //             OneToOne {
    //                 table:"billionaires",
    //                 table_field:"id"
    //             }
    //         }
    //     }
    // };
    // let rating = model! {
    //     "rating" => {
    //         "id" => {
    //             ID(UUID),
    //             PRIMARY,
    //             NOTNULL,INDEX
    //         },
    //         "rating" => {
    //             STRING
    //         },
    //         "user_id" => {
    //             OneToMany {
    //                 table:"billionaires",
    //                 table_field:"id"
    //             }
    //         }
    //     }
    // };
    // let club = model! {
    //     "billionairesclub" => {
    //         "id" => {
    //             ID("uuid"),
    //             PRIMARY,
    //             NOTNULL

    //         },
    //         "age" => {
    //             NUMBER
    //         },
    //         "billionaire_key" => {
    //             OneToOne { table: "billionaires", table_field: "id" }
    //         }
    //     }
    // };
    // println!("{}", emodel);
    // println!("{}", emodel);
    // let random: String = rand::thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(6)
    //     .map(char::from)
    //     .collect();
    // println!("{}", random);

    // let container = container! {
    //     client =>  postgres,
    //     models => {
    //         place
    //     }
    // };
    // println!("{:?}", container);

    // for i in 0..6 {
    // let create = create! {
    //     connection => postgres,
    //     model:"billionaires",
    //     data:{
    //         "place" => "san"
    //     },
    //     select:{
    //         "id","place"
    //     }
    // };
    // println!("{:?}", create);
    // }
    // let location = point_epsg_4326(12.971599, 77.594566);

    // println!("{}", location);
    // let partition = create_partition! {
    //     connection => postgres,
    //     model:"shop",
    //     name:"area",
    //     field:"san"
    // };
    // println!("{:?}", partition);

    // for i in 100..104 {
    // let create = create! {
    //     connection => postgres,
    //     model:"place",
    //     data:{
    //         "name" => "billionaireharigreat"
    //     },
    //     select:{
    //         "id"
    //     }
    // };
    // print!("{:?}", create);
    // let find = find_many! {
    //     connection => postgres,
    //     model:"chat",
    //     select:{
    //         "id"
    //     },
    //     inside:{
    //         "chats" => {
    //             match:"billionaireId",
    //             select:{
    //                 "id","input","output"
    //             }
    //         }
    //     }
    // };
    // let search = full_search! {
    //     connection => postgres,
    //     model:"place",
    //     based_on:"name",
    //     search:{
    //         value:"billionaire"
    //     },
    //     select:{
    //         "name"
    //     },
    //     take:6,
    //     skip:0
    // };
    // println!("{:?}", search);
    // let count = count! {
    //     connection => postgres,
    //     model:"place",
    //     count:{
    //         "name"
    //     },
    //     conditions:{
    //         and => {
    //             "name" => "billionaires"
    //         }
    //     }
    // };
    // let partition = custome_execute! {
    //     connection => postgres,
    //     query:{
    //         "INSERT INTO place VALUES ($1)"
    //     },
    //     value:{
    //         "billionaires"
    //     }
    // };
    // println!("{:?}", partition);
    // let location = nearby_location! {
    //     connection => postgres,
    //     model:"billionaires",
    //     select:{
    //         "place"
    //     },
    //     location:{
    //         lattitude:"12.971560",
    //         longitude:"77.594560",
    //         within:"4000"
    //     },
    //     select_from:{
    //         "location"
    //     },
    //     order:{
    //         location:"asc",
    //         "place" => "asc"
    //     }
    // };
    // println!("{:?}", location);
    // }
    // let search = similar_search! {
    //     connection => postgres,
    //     model:"place",
    //     similarity:{
    //         score:"0.6",
    //         believe:"name",
    //         text:"billionaire"
    //     }
    // };
    // println!("{:?}", search);
    // let uuid = Uuid::parse_str("65e88c87-482f-4de0-87dc-0330c3d97ca0").unwrap();
    let find = find_many! {
        connection => postgres,
        model:"chat",
        match:"id",
        select:{
            "id"
        },
        conditions:{
            and => {
                "id" => "65e88c87-482f-4de0-87dc-0330c3d97ca0"
            }
        },
        inside:{
            "chats" => {
                match:"billionaireId",
                select:{
                    "id","input","output"
                }
            }
        }
    };
    println!("{:?}", find);
    // let id = Uuid::parse_str("5ca19287-2d88-4fde-a590-dc152fac3a84").unwrap();
    // let create = create! {
    //     connection => postgres,
    //     model:"place",
    //     data:{
    //         "name" => "billionaires",
    //         "user_id" => id
    //     },
    //     select:{
    //         "name"
    //     }
    // };
    // let create = create! {
    //     connection => postgres,
    //     model:"rating",
    //     data:{
    //         "rating" => "6",
    //         "user_id" => id
    //     }
    // };
    // println!("{:?}", create);
    // }
    // let time = Instant::now();
    // let find = find_many! {
    //     connection => postgres,
    //     model:"billionaires",
    //     select:{
    //         "place"
    //     },
    //     conditions:{
    //         and => {
    //             "place" => "22"
    //         },
    //         or => {
    //             "place" => "22"
    //         }
    //     },
    //     between => {
    //         and => {
    //             "place" => {
    //                 "20" => "22"
    //             }
    //         },
    //         or => {
    //             "place" => {
    //                 "20" => "22"
    //             }
    //         }
    //     },
    //     like => {
    //         and => {
    //             "place" => "billionairebbb"
    //         },
    //         or => {
    //             "place" => "billionairebb"
    //         }
    //     },
    //     order:{
    //         "place" => "asc"
    //     }
    // };
    // // println!("{:?}", find);
    // let find = find_many! {
    //     connection => postgres,
    //     model:"place",
    //     match:"user_id",
    //     case:{
    //         ("name" => "=billionaire") => (ok:"billionaires",else:"trillionaires") => "billionaire"
    //     },
    //     inside:{
    //         "billionaires" => {
    //             match:"id",
    //             select:{
    //                 "id"
    //             }
    //         }
    //     },
    //     limit:6,
    //     skip:6
    // }
    // .unwrap();
    // let create = create! {
    //     connection => postgres,
    //     model:"place",
    //     data:{
    //         "name" => "billionairehari",
    //         "user_id" => id
    //     },
    //     select:{
    //         "name"
    //     }
    // };
    // geo_types::Geometry::Point(geo_types::Point(""));
    // let point: Point<f64> = Point(geo_types::Coordinate { x: 6.0, y: 6.0 });
    // println!("{:?}", create);
    // for place in find.iter() {
    //     let id: i32 = place.get(0);
    //     let name: String = place.get(1);
    //     println!("{} {}", id, name);
    // }
    // let mut billion = "".split_whitespace();
    // let bb = billion.nth(0);
    // bb.to_owned();
    let delete = delete! {
        connection => postgres,
        model:"billionaires",
        select:{
            "place"
        },
        conditions:{
            and => {
                "place" => 24 as i32
            },
            or => {
                "place" => 24 as i32
            }
        },
        between => {
            and => {
                "place" => {
                    "20" => "22"
                }
            }
        }
    };
    // println!("{:?}", delete);
    let update = update! {
        connection => postgres,
        model:"billionaires",
        match:"id",
        // data:{
        //     "place"
        // },
        conditions:{
            and => {
                "id" => "billionairehari"
            }
            or => {
                "place" => "billionairehari"
            }
        },
        between => {
            and => {
                "place" => {
                    "20" => "22"
                }
            }
            or => {
                "place" => {
                    "20" => "24"
                }
            }
        },
        inside:{
            "place" => {
               match:"user_id",
               data:{
                    "name"  => "billionaire"
               },
               conditions:{
                    and => {
                        "name" => "billionaire"
                    }
               },
               select:{
                    "name"
               },
               between => {
                    and => {
                        "name" => {
                            "20" => "22"
                        }
                    }
                    or => {
                        "name" => {
                            "20" => "24"
                        }
                    }
                }
            }
        }
    };
    // println!("{:?}", update);
    // let update = update! {
    //     connection => postgres,
    //     model:"place",
    //     select:{
    //         "id"
    //     },
    //     data:{
    //         "name" => "billionairehari"
    //     },
    //     conditions:{
    //         and => {
    //             "name" => "billionairehari"
    //         },
    //         or => {
    //             "" => ""
    //         }
    //     }
    // };
    // println!("{:?}", update);
    // let find = find_many! {
    //     connection => postgres,
    //     model:"billionaires",
    //     select:{
    //         "place"
    //     },
    //     within:{
    //         lattitude:"12.971599",
    //         longitude:"77.594566"
    //     },
    //     refer:{
    //         "location"
    //     },
    //     limit:6
    // };
    // println!("{:?}", find);
    // let b = "billionaire".chars().nth(0).unwrap();
    // println!("{}", b);
    // let query = custome_query! {
    //     connection => postgres,
    //     query:{
    //         "SELECT * FROM pg_stat_wal"
    //     }
    // };
    // println!("{:?}", query);
    // let count = count! {
    //     connection => postgres,
    //     model:"shop"
    // };
    // println!("{:?}", count);
    // let elapsed = time.elapsed();
    // // println!("{:?}", find);
    // println!("{:?}", elapsed.as_micros());
    // let map = BTreeMap::new();
    // let mut billionaires = Vec::new();
    // for billionaire in find.iter() {
    //     let mut map = HashMap::new();
    //     let billionaire_column = billionaire.columns();
    //     for billionaires in billionaire_column.iter() {
    //         let name = billionaires.name();
    //         let billion = billionaires.type_().name();

    //         // println!("{:?}", billion);

    //         let value = match billion.clone() {
    //             "text" => {
    //                 let value: String = billionaire.get(name);
    //                 value
    //             }
    //             "date" => {
    //                 let value: NaiveDate = billionaire.get(name);
    //                 value.to_string()
    //             }
    //             "timestamp" => {
    //                 let value: NaiveDateTime = billionaire.get(name);
    //                 value.to_string()
    //             }
    //             "int4" => {
    //                 let value: i32 = billionaire.get(name);
    //                 value.to_string()
    //             }
    //             "time" => {
    //                 let value: NaiveTime = billionaire.get(name);
    //                 value.to_string()
    //             }
    //             "uuid" => {
    //                 let value: Uuid = billionaire.get(name);
    //                 value.to_string()
    //             }
    //             "bool" => {
    //                 let value: bool = billionaire.get(name);
    //                 value.to_string()
    //             }
    //             _ => {
    //                 panic!("")
    //             }
    //         };
    //         map.insert(name, value);
    //     }
    //     billionaires.push(map);
    // }
    // println!("{:?}", billionaires);
    // let custom = custome_query! {
    //     connection => postgres,
    //     query:{"SELECT name,ST_AsText(location) FROM shop WHERE ST_DWithin(location, ST_GeogFromText('SRID=4326;POINT(77.594566 12.971599)'), 0);"}
    // };
    // println!("{:?}", custom);
    // let location = nearby_location! {
    //     connection => postgres,
    //     model:"shop",
    //     select:{
    //         "other_than_location_type"
    //     },
    //     location:{
    //         lattitude:"12.971599",
    //         longitude:"77.594566"
    //     },
    //     select_from:{
    //         "location"
    //     }
    // };
    // println!("{:?}", location);
    // let find = find_many! {
    //     connection => postgres,
    //     model:"shop",
    //     select:{
    //         "place"
    //     },
    //     condition:{
    //         "place" => "san"
    //     }
    // };
    // println!("{:?}", find);
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     select:{
    //         "story"
    //     },
    //     condition:{
    //         "story" => "haribillionaire"
    //     }
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     select:{
    //         "story"
    //     },
    //     conditions:{
    //         or => {
    //             "story" => "haribillionaire"
    //         },
    //         "story" => "haribillionaire"
    //     }
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     select:{
    //         "story"
    //     },
    //     conditions:{
    //         or => {
    //             "story" => "haribillionaire"
    //         }
    //     }
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     conditions:{
    //         or => {
    //             "story" => "haribillionaire"
    //         }
    //     }
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     conditions:{
    //         or => {
    //             "story" => "haribillionaire"
    //         },
    //         "story" => "haribillionaire"
    //     }
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     conditions:{
    //         or => {
    //             "story" => "haribillionaire"
    //         },
    //         "story" => "haribillionaire"
    //     },
    //     limit:4
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     conditions:{
    //         or => {
    //             "story" => "haribillionaire"
    //         },
    //         "story" => "haribillionaire"
    //     },
    //     limit:4
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     conditions:{
    //         or => {
    //             "story" => "haribillionaire"
    //         },
    //         "story" => "haribillionaire"
    //     },
    //     limit:4,
    //     skip:4
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     conditions:{
    //         or => {
    //             "story" => "billionairehari"
    //         },
    //         "story" => "billionairehari"
    //     },
    //     skip:0,
    //     order:{
    //         "story" => "asc"
    //     }
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     conditions:{
    //         or => {
    //             "story" => "billionairehari"
    //         },
    //         "story" => "billionairehari"
    //     },
    //     order:{
    //         "story" => "asc"
    //     }
    // };
    // // println!("{:?}", find);
    // let rank = ranked_search! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     based_on:"story",
    //     search:{
    //        value:"billionairehari"
    //     },
    //     select:{
    //         "story"
    //     }
    // };
    // let update = update! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     data:{
    //         "story" => "billionaires"
    //     },
    //     conditions:{
    //         "story" => "billionairehari"
    //     }
    // };
    // let update = update! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     select:{
    //         "story"
    //     },
    //     data:{
    //         "story" => "billionairehari"
    //     },
    //     conditions:{
    //         "story" => "billionaires"
    //     }
    // };
    // let update = update! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     select:{
    //         "story"
    //     },
    //     data:{
    //         "story" => "billionairehari"
    //     },
    //     conditions:{
    //         "story" => "billionaires"
    //     }
    // };
    // let delete = delete! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     conditions:{
    //         "story" => "billionairehari"
    //     },
    //     select:{
    //         "story"
    //     }
    // };
    // // println!("{:?}", delete);
    // create_partition! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     name:"story_musk",
    //     field:"inspring"
    // }
    // .unwrap();
    // horizontal_splitting! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     name:"musk_intelligent_2",
    //     based_on:{
    //         "story" => "intelligent"
    //     }
    // };
    // // println!("{:?}", create);
    // // let partition = postgres
    // //     .query(
    // //         "SELECT tableoid::regclass::text AS partition,* FROM elonmusk",
    // //         &[],
    // //     )
    // //     .unwrap();
    // // println!("{:?}", partition);
    // // for partition in partition.iter() {
    // //     let first: String = partition.get("partition");
    // //     let name: Uuid = partition.get("id");
    // //     let date: NaiveDate = partition.get("name");
    // //     let time: NaiveTime = partition.get("time");
    // //     let datetime: NaiveDateTime = partition.get("fingerprint");
    // //     let age: i32 = partition.get("age");
    // //     println!("{}", first);
    // //     println!("{}", name);
    // //     println!("{}", date);
    // //     println!("{}", time);
    // //     println!("{}", datetime);
    // //     println!("{}", age);
    // //     // println!("{}", date);
    // // }
    // let find = find_one! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     select:{
    //         "id"
    //     },
    //     condition:{
    //         "story" => "billionairehari"
    //     }
    // };
    // let billionaire = find[0][0].get("id").unwrap();
    // let billionaire = Uuid::parse_str(&billionaire).unwrap();
    // println!("{}", billionaire);
    // let create = create! {
    //     connection => postgres,
    //     model:"billionaires",
    //     data:{
    //         "age" => 24 as i32,
    //         "billionaire_key" => ""
    //     },
    //     select:{
    //         "id","age","billionaire_key"
    //     }
    // };
    // let billionaire = find_many! {
    //     connection => postgres,
    //     model:"elonmusk"
    // };
    // let billionaire = find_many! {
    //     connection => postgres,
    //     model:"story_musk"
    // };
    // println!("{:?}", billionaire);
    // let billionaire = Uuid::parse_str("7623f9f1-2ff5-4ee6-a7f9-64cea87784a5").unwrap();
    // let create = create! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     data:{
    //         "age" => 24 as i32,
    //     },
    //     select:{
    //         "id","age","billionaire_key"
    //     }
    // };
    // println!("{:?}", create);
    // let billionaire = uuid!(&format!("urn:uuid:{}",billionaire));
    // let uuid = Uuid::parse_str(billionaire).unwrap().as_hyphenated();
    // let uuid = Uuid::from(id);
    // let find = find_many! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     condition:{
    //         "billionaire_key" =>   billionaire
    //     }
    // };
    // println!("{:?}", find);
    // let update = update! {
    //     connection => postgres,
    //     model:"elon",
    //     data:{
    //         "names"=>"billionaire"
    //     },
    //     conditions:{
    //         "names" => "billionairehari"
    //     }

    // }
    // .unwrap();
    // let find = find_one! {
    //     connection => postgres,
    //     model:"elon",
    //     condition:{
    //         "names" => "billionaire"
    //     }
    // };
    // println!("{:?}", find);

    // let find = find_many! {
    //     connection => postgres,
    //     model:"elon"
    // };
    // println!("{:?}", find);
    // let delete = delete! {
    //     connection => postgres,
    //     model:"elon",
    //     conditions:{
    //         "names" => "billionaire"
    //     }
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elon"
    // };
    // println!("{:?}", find);
    // let s = format!(
    //     "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";
    //     DROP TABLE IF NOT EXISTS elon;
    //     CREATE TABLE IF NOT EXISTS elon"
    // );
    // if let Some((first, second)) = s.split_once(";") {
    //     println!("{:?}", first);
    //     println!("{:?}", second);
    // }
    // .unwrap();
    // println!("{:?}", container);
    // let index = create_index! {
    //     connection => postgres,
    //     model:"elon",
    //     name:"index_elon_name",
    //     index:{
    //         "name"
    //     }
    // }
    // .unwrap();
    // let index = show_index_list! {
    //     connection => postgres,
    //     model:"billionaires"
    // };
    // println!("{:?}", index);
    // let index = create_brin_index! {
    //     connection => postgres,
    //     model:"elon",
    //     name:"brin_elon_time",
    //     index:{
    //         "time"
    //     }
    // };
    // // println!("{:?}", index);
    // let index = create_gin_index! {
    //     connection => postgres,
    //     model:"elon",
    //     full_text:{
    //         name:"brin_elon_names",
    //         index:{
    //             "names"
    //         }
    //     }
    // };
    // create_gin_index! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     full_text:{
    //         name:"billionaires",
    //         index:{
    //           "story"
    //         }
    //     }
    // }
    // .unwrap();
    // let time = Instant::now();
    // let date = chrono::Utc::now().date_naive();
    // println!("{}", date);
    // let s = find_many! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     select:{
    //         "story"
    //     },
    //     condition:{
    //         "name" => date
    //     }
    // };
    // println!("{:?}", s);

    // let s = partitions! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     value:"elon_rocket",
    //     partition:"USA"
    // }
    // .unwrap();
    // let s = find_many! {
    //     connection => postgres,
    //     model:"elonmusk"
    // };
    // println!("{:?}", s);
    // println!("{:?}", time.elapsed().as_millis());
    // println!("{:?}", index);
    // let index = create_gin_index! {
    //     connection => postgres,
    //     model:"elon",
    //     pattern_match:{
    //         name:"brin_elon_namess",
    //         index:{
    //             "names"
    //         }
    //     }
    // };
    // let search = similar_search! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     similarity:{
    //         score:"0.4",
    //         believe:"story",
    //         // believe:"story",
    //         text:"billionaireha"
    //     },
    //     order_by:{
    //         order:"asc",
    //         believe:"story",
    //         text:"billionaire"
    //     }
    // };
    // println!("{:?}", search);
    // let count = count! {
    //     connection => postgres,
    //     model:"elon",
    //     count:{
    //         "age"
    //     },
    //     conditions:{
    //         "names" => "billionairehari"
    //     }

    // };
    // println!("{:?}", count);

    // #[cfg(feature = "search")]
    // let p = pagination! {
    //     connection => postgres,
    //     model:"elonmusk",
    //     take:10,
    //     skip:0,
    //     search:{
    //         "age" => "24"
    //     },
    //     select:{
    //         "story","age"
    //     },
    //     conditions:{
    //         "story" => "billionairehari"
    //     }
    // };
    // println!("{:?}", p);
    // println!("{:?}", auto_type(postgres, "elon", p));
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elon"
    // };
    // println!("{:?}", find);
    // let index = postgres
    // .query("SELECT * from pg_indexes where tablename = 'elon'", &[])
    //     .unwrap();
    // let index = show_index_list! {
    //     connection => postgres,
    //     model:"elon"
    // }
    // .unwrap();
    // for index in index.iter() {
    //     let name: String = index.get(0);
    //     let name_: String = index.get(1);
    //     let name__: String = index.get(2);
    //     println!("{}", name);
    //     println!("{}", name_);
    //     println!("{}", name__);
    //     // println!("{:?}", index);
    // }
    // println!("{:?}", index);
    // let time = Instant::now();
    // let billionaie = find_many! {
    //     connection => postgres,
    //     model:"elon"
    // };
    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
    // transaction! {
    //     connection => postgres,
    //     begin
    // };
    // let time = Instant::now();
    // for _ in 0..1000 {
    //     // thread::spawn(move || {
    //     // transaction! {
    //     //     connection => postgres,
    //     //     begin
    //     // }
    // let create = create! {
    //     connection => postgres,
    //     model:"elon",
    //     data:{
    //         "names" => "hari",
    //         "age" => 24 as i32,
    //         "billionaire" => true
    //         // "area" => "2.2945 48.8584"
    //     },
    //     select:{
    //         "id","name","time","fingerprint","names","age","billionaire"
    //     }
    // };
    // transaction! {
    //     connection => postgres,
    //     commit
    // };
    // });
    // }
    // println!("time taken to finish {}", time.elapsed().as_millis());
    // println!("{:?}", create);
    // transaction! {
    //     connection => postgres,
    //     commit
    // }
    // println!("{:?}", create);
    // let update = update! {
    //     connection => postgres,
    //     model:"elon",
    //     data:{
    //         "names" => "haribillionaire",
    //         "age" => 24 as i32,
    //         "billionaire" => true
    //     },
    //     conditions:{
    //         "names" => "haribillionaire"
    //     },
    //     select:{
    //         "id","name","time"
    //     }
    // };
    // let find = find_one! {
    //     connection => postgres,
    //     model:"elon",
    //     select:{
    //         "id",
    //         "names"
    //     },
    //     conditions:{
    //         or => {
    //         "names" => "haribillionaire"
    //         },
    //         "age" => 24 as i32
    //     },
    //     order:{
    //         "age" => "asc"
    //     }
    // };
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elon",
    //     select:{
    //         "names"
    //     },
    //     conditions => {
    //         or => {
    //         "names" => "haribillionaire"
    //         },
    //         "age" => 24 as i32
    //     },
    //     order:{
    //         "names" => "asc"
    //     },
    //     // limit:1
    //     skip:16
    // };
    // let delete = search! {
    //     connection => postgres,
    //     model:"elon",
    //     similarity:{
    //         score:"0.4",
    //         believe:"names",
    //         text:"har"
    //     },
    //     order_by:{
    //         order:"",
    //         believe:"names",
    //         text:"har"
    //     }

    // };
    // let structure = table_structure! {
    //     connection => postgres,
    //     model:"elon"
    // };
    // println!("{:?}", structure);
    // println!("{:#?}", delete);
    // // let c = BTreeMap::new();
    // // c.into_values()
    // for creates in create.iter() {
    //     for create in creates.iter() {
    //         println!("{:?}", create.values());
    //     }
    // }
    // println!("{}", create);
    // let update = update! {
    //     connection => postgres,
    //     model:"elon",
    //     data:{
    //         "name" => "billionairehari"
    //     },
    //     conditions:{
    //         "name" => "hari"
    //     },
    //     select:{
    //         "name","id"
    //     }
    // }
    // .unwrap();
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elon"
    // }
    // .unwrap();
    // // .unwrap();
    // // filter between date
    // for r in find.iter() {
    //     println!("{}", r.len());
    //     let name: Uuid = r.get(0);
    //     let id: NaiveDate = r.get(1);
    //     let names: NaiveTime = r.get(2);
    //     let time: NaiveDateTime = r.get(3);
    //     let fingerprint: String = r.get(4);
    //     let age: i32 = r.get(5);
    //     let billionaire: bool = r.get(6);
    //     println!("{}", name);
    //     println!("{}", id);
    //     println!("{}", names);
    //     println!("{}", time);
    //     println!("{}", fingerprint);
    //     println!("{}", age);
    //     println!("{}", billionaire);
    // }
    // let query = postgres.query("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = 'elon';", &[]).unwrap();
    // let mut map = HashMap::new();
    // for r in query.iter() {
    //     let first: String = r.get(0);
    //     let second: String = r.get(1);
    //     // uuid
    //     // date
    //     // time without time zone
    //     // timestamp without time zone
    //     // text
    //     // integer
    //     // boolean
    //     // println!("{}", first);
    //     // println!("{}", second);
    //     map.insert(first, second);
    // }
    // // // println!("{:?}", map);
    // let mut bucks = vec![];
    // let mut total_bucks = Vec::new();
    // for (idx, column) in find.iter().enumerate() {
    //     println!("{}", idx);
    //     let buck = BTreeMap::new();
    //     // println!("{}", column.len());
    //     for i in map.iter() {
    //         let value = i.0.to_string();
    //         let value_ = value.as_str();
    //         let kind = i.1;
    //         // println!("{} = {}", value_, kind);
    //         let mut buck = buck.clone();
    //         let value__ = match kind.as_ref() {
    //             "time without time zone" => {
    //                 let name: NaiveTime = column.get(value_);
    //                 // println!("{}", name.to_string());
    //                 name.to_string()
    //             }
    //             "uuid" => {
    //                 let uuid: Uuid = column.get(value_);
    //                 // println!("{}", uuid.to_string());
    //                 uuid.to_string()
    //             }
    //             "boolean" => {
    //                 let value: bool = column.get(value_);
    //                 value.to_string()
    //             }
    //             "integer" => {
    //                 let intger: i32 = column.get(value_);
    //                 intger.to_string()
    //             }
    //             "timestamp without time zone" => {
    //                 let datetime: NaiveDateTime = column.get(value_);
    //                 datetime.to_string()
    //             }
    //             "date" => {
    //                 let date: NaiveDate = column.get(value_);
    //                 date.to_string()
    //             }
    //             "text" => {
    //                 let string = column.get(value_);
    //                 string
    //             }
    //             _ => panic!("{}", kind),
    //         };
    //         // println!("{} = {}", value.clone(), value__.clone());
    //         buck.insert(value.clone(), value__.clone());
    //         // println!("{:?}", tick);
    //         bucks.push(buck.clone());
    //         // println!("{}", "------------------------------------------");
    //         // println!("{:?}", buck);
    //         // println!("{:?}", bucks);
    //         // println!("{}", "------------------------------------------");
    //         // println!("{}", value__)
    //     }
    //     // println!("{:?}", bucks);
    //     total_bucks.push(bucks.clone());
    //     bucks.clear(); // earase the prev bucks
    //                    // println!("{}", "------------------------------------------");
    // }
    // println!("{:#?}", total_bucks);

    // if kind.starts_with("NaiveTime") {
    //     println!("{}", "hi");
    //     // }
    //     let name: NaiveTime = column.get(&i.0.to_string().as_str());
    //     println!("{}", name.to_string());
    // }
    // else if k
    // }
    // let id: NaiveDate = r.get(1);
    // let names: NaiveTime = r.get(2);
    // let time: NaiveDateTime = r.get(3);
    // let fingerprint: String = r.get(4);
    // let age: i32 = r.get(5);
    // let billionaire: bool = r.get(6);
    // println!("{}", name);
    // println!("{}", id);
    // println!("{}", names);
    // println!("{}", time);
    // println!("{}", fingerprint);
    // println!("{}", age);
    // println!("{}", billionaire);
    // }
    // let delete = delete! {
    //     connection => postgres,
    //     model:"elon",
    //     conditions:{
    //         "name" => "billionairehari"
    //     }
    // }
    // .unwrap();
    // println!("{}", delete);
    // let find = find_many! {
    //     connection => postgres,
    //     model:"elon"
    // }
    // .unwrap();
    // for r in find.iter() {
    //     let name: String = r.get(1);
    //     let id: Uuid = r.get(0);
    //     println!("{}", name);
    //     println!("{}", id);
    // }
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
    // }

    // fn render() {
    //     let user = model! {
    //         "userrr" => {
    //             "id" => {ID("auto")},
    //             "name" => {Stringg,UNIQUE},
    //             "billion" => {
    //                 Number,UNIQUE
    //             },
    //             "billionaire_id" => {
    //                 OneToOne {
    //                     table:"userr",
    //                     table_field:"id",
    //                 }
    //             },
    //             "create_at" => {
    //                 DateTime
    //             }
    //         }
    //     };
    //     let billionaire = model! {
    //         "billionairehariprasath" => {
    //             "id" => {ID("uuid")},
    //             "create_at" => {
    //                 DateTime
    //             }
    //         }
    //     };
    //     let billionaire1 = model! {
    //         "billionairesclub" => {
    //             "id" => {ID("cuid")},
    //             "create_at" => {
    //                 DateTime
    //             },
    //             "name" => {
    //                 Stringg,
    //                 UNIQUE
    //             }
    //         }
    //     };
    //     println!("{}", billionaire1);
    //     let mut postgres = postgres::Client::connect("host=127.0.0.1 port=5432 dbname=t user=postgres password=hariclash123 connect_timeout=10 sslmode=prefer", NoTls).unwrap();
    // let models = container! {
    //    client => postgres,models => billionaire1
    // };
    // println!("{:?}", models);
    // let billionaire: Result<u64, postgres::Error> = postgres.execute(
    //     "INSERT INTO billionairesclub VALUES (DEFAULT,DEFAULT,$1);",
    //     &[&"billionaire"],
    // );
    // println!("{:?}", billionaire.unwrap());
    // let billionaire = postgres
    //     .query("SELECT * from billionairesclub;", &[])
    //     .unwrap();
    // for billionaire in billionaire.iter() {
    //     let one: String = billionaire.get(0);
    //     let date: SystemTime = billionaire.get(1);
    //     println!("{:?}", one);
    //     println!("{:?}", date);
    //     // println!("{:?}", billionaire.get(0));
    // }
    // 3147dfd4ccc6825f063ab937
    // 133800229825311900

    // postgres.query(query, params)
    // let query = find_one! {
    //     connection => postgres,
    //     model : "billionairesclub"
    // };
    // println!("{:?}", query);
    // let condition = Condition { k: "", v: "" };
    // let and = Where { and: condition };
    // println!("{:?}", and);
    // let query = find_one! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     select:{
    //         "id",
    //         "create_at"
    //     },
    //     wwhere:{
    //         and => {
    //             "id" => "408fb73d8c7d5dc0ffb3a88c",
    //             "name" => "billionaire"
    //         },
    //         or => {
    //             "id" => "408fb73d8c7d5dc0ffb3a88c"
    //         }
    //     },
    //     limit:6,
    //     skip:0
    // };
    // let query = find_many! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     select:{
    //         "id"
    //     },
    //     conditions:{
    //         or => {
    //             "name" => "billionaire"
    //         },
    //         "name" => "billionaire"
    //     },
    //     limit:6
    // };
    // let query = find_many! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     select:{
    //         "id"
    //     },
    //     condition : {
    //         // or => {
    //         //     "name" => "billionaire"
    //         // },
    //         "name" => "billionaire"
    //     }
    //     // order:{"name" => "asc"}
    //     // limit:0,
    //     // skip:0
    // }
    // .unwrap();
    // println!("{:?}",query);
    // for row in query.iter() {
    //     let id: String = row.get(0);
    //     println!("{}", id);
    // }
    // let delete = delete! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     conditions : {
    //         // or => {
    //             "name" =>"billionaire"
    //         // }
    //     }
    // };
    // let delete = delete! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     conditions : {
    //         // or => {
    //             "name" =>"billionaire"
    //         // }
    //     }
    // };
    // println!("{:?}", delete);
    // let update = update! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     data:{
    //         "name"=>"billionaires",
    //         "id"=>"yuiotfghoi9u8yghjiouyghjioiuyughjiouy"
    //     },
    //     conditions :{
    //         "name"=>"billionaire",
    //         "id"=>"yuiotfghoi9u8yghjiouyghjioiuyughjiouy"
    //     },
    //     select:{
    //         "name"
    //     }
    // };
    // let create = create! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     data:{
    //         "name" => "billionairehari"
    //     },
    //     select:{
    //         "name",
    //         "id",
    //         "create_at"
    //     }
    // }
    // .unwrap();

    // let create = create_many! {
    //     connection => postgres,
    //     model:"billionairesclub",
    //     data:{
    //         {"name" => "billionairemark","name" => "billionairehari"},
    //     }
    // select:{
    //     "name",
    //     "id",
    //     "create_at"
    // }
    // };

    // for row in create.iter() {
    //     let id: String = row.get(0);
    //     let name: String = row.get(1);
    //     println!("{}", id);
    //     println!("{}", name);
    // }
    // println!("{:?}", create);
    // let query = find_many! {
    //     connection => postgres,
    //     model => "billionairesclub"
    // }
    // .unwrap();
    // let query: String = query.get(0).unwrap().get(0);
    // // let id:String = query.get(0).unwrap();
    // // postgres.query_one(query, params)
    // println!("{:?}", query);
    // for (, billionaire) in query.iter().enumerate() {
    //     let one: String = billionaire.get(0);
    //     let date: SystemTime = billionaire.get(1);
    //     let name: String = billionaire.get(2);
    //     println!("{:?}", billionaire.get());
    //     // println!("{:?}", date);
    //     // println!("{:?}", name);
    //     // println!("{:?}", billionaire.get(0));
    // }
    // println!("{}", query);
    // let delete = delete_table! {
    //     connection => postgres,
    //     model => "billionairesclub"
    // };
    // println!("{:?}", delete);

    // let mut schema = String::new();
    // schema.push_str(&billionaire);
    // schema.push_str(&image);
    // schema.push_str("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"");
    // println!("{}", schema);
    // let l = OneToOne {
    //     table: "billionaire",
    //     table_field: "id",
    //     matches: "billionaireID",
    // };
    // let ll = serde_json::to_string(&l).unwrap();
    // println!("{:?}", ll);
    // let billionaire = serde_json::from_str::<OneToOne>(&ll);
    // println!("{:?}", billionaire);
    // stringify!();
    // "String".to_lowercase()
    // println!("{:?}", html);
    // }
}
