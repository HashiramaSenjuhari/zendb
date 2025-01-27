![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Crates.io](https://img.shields.io/crates/v/rusty-postgres)

# A Advanced Lightwight ORM and Query Builder for postgres

## Usage

## Client

```rust
  let mut postgres = Client::connect("host=127.0.0.1 port=5432 dbname=name user=postgres password=xxxxxxxxx connect_timeout=10 sslmode=prefer", NoTls);
```

## Create Schema without any hassale

```rust
  let shop = model! {
        "shop" => {
            "id" => {
                ID(UUID),NOTNULL,UNIQUE
            },
            "name" => {
                Date(NOW)
            },
            "time" => {
                Time(NOW)
            },
            "fingerprint" => {
                DateTime(NOW)
            },
            "age" => {
                NUMBER,DEFAULT(24)
            },
            "point" => {
                FLOAT,DEFAULT(24.1)
            },
            "place" => {
                STRING,NOTNULL,UNIQUE,INDEX,PRIMARY
            },
            "location" => {
                Geography(POINT(Epsg4326))
            }
            "accuracy" => {
                FLOAT,DEFAULT(0.0)
            },
            "bool" => {
                BOOL
            }
        },
        partition:{             //optional
            type:"list",
            to:"place"
        }
    };
```

# Container

## Build Schema

```rust
  let container = container! {
      client =>  postgres,
      models => {
        shop
      }
  };
```

## New Similar Search

```rust
let search = similar_search {
    connection => postgres,
    model:"place",
    similarity:{
        score:"0.6", //similarity score
        believe:"name" //based_on
        text:"san" //text
    },
    order_by:{              //optional
        order:"asc",
        believe:"name" //based
    }
}
```

## Full text Vector Search

```rust
let search = full_search! {
    connection => postgres,
    model:"place",
    based_on:"name",
    search:{
        value:"billionaire"
    },
    select:{
        ,"name"
    },
    take:6,
    skip:0
};
```

## Ranked Vector Search

```rust
let search = ranked_search! {
    connection => postgres,
    model:"place",
    based_on:"name",
    search:{
        value:"billionaire"
    },
    select:{
        "name"
    }
};
```

## Horizontal Split

```rust
horizontal_splitting {
    connection => postgres,
    model:"",
    name:"name_of_spllit",
    based_on:{
    "country" => "usa"
    }
}
```

## Partition

```rust
let partition = create_partition {
    connection => postgres,
    model:"place",
    name:"partition_name",
    field:"value_to_match"
}
```

## Count

```rust
let count = count! {
    connection => postgres,
    model:"place",
    count:{
        "name"
    },
    conditions:{                //optional
        and => {
            "name" => "billionaires"
        }
    },
    group_by:{          //optional
        "name"
    }
};
```

## Create

```rust
    let create = create! {
        connection => postgres,
        model:"user_",
        data:{
            "story" => "billionairehari",
            "age" => 24 as i32
        },
        select:{
            "id"
        }
    };
```

## FindOne

```rust
    let find = find_one! {
        connection => postgres,
        model:"place",
        select:{
            "name"
        },
        conditions:{
            and => {
                "name" => "billionairehari"
            },
            or => {
                "name" => "billionairehari"
            }
        },
        order:{
            "name" => "asc"
        }
    };
```

## FindMany

```rust
  let find = find_many! {
     connection => postgres,
     model:"billionaires",
     select:{               // optional
         "place"
     },
     case:{
     (
         "place" => ">22",
         "place" => "<22",
         "place" => "=22"
         ) => (ok:"billion_dollar",
         ok:"billionaire",
         ok:"billionaire"
         ,else:"trillionaire"
     ) => "status"
     },
     conditions:{           // optional
         and => {
             "place" => "san"
         },
         or => {
             "place" => "san"
         }
     },
     inside:{
         "place" => {
             match:"user_id",
             select:{
                 "name"
             },
             conditions:{
                 and => {
                     "name" => "billionaire"
                 }
             },
             order:6,
             limit:6
         }
     },
     order:{                // optional
         "place" => "asc"
     },
     limit:24,              // optional
     skip:24                // optional
  };
```

## Update

```rust
let update = update! {
    connection => postgres,
    model:"place",
    select:{
        "id"
    },
    data:{
        "name" => "billionairehari"
    },
    conditions:{
        and => {
            "name" => "billionairehari"
        },
        or => {
            "" => ""
        }
    }
};
```

## New Relation Update

```rust
let update = update! {
       connection => postgres,
       model:"billionaires",
       match:"id",
       inside:{
           "place"  => {
               match:"user_id",
               conditions:{
                   and => {
                       "name" => "billionaires",
                       "user_id" => "c4a97a50-8679-4f85-a1d8-5bba0113b596"
                   }
               },
               data:{
                   "name" => "billionairehari"
               },
               select:{
                   "name"
               }
           }
       }
   };
```

## Delete

```rust
 let delete = delete! {
     connection => postgres,
     model:"billionaires",
     select:{                // Optional
         "place"
     },
     conditions:{            // Optional
         and => {            // Optional
             "place" => 24 as i32
         },
         or => {             // Optional
             "place" => 24 as i32
         }
     },
     cascade:true            // Optional
 };
```

## DROP

```rust
  delete_table! {
      connection => postgres,
      model => "photo",
      cascade
  }
  .unwrap();
```

# INCLUDED INDEX => Brin,GIN,B-TREE INDEX
