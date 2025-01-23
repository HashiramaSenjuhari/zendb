![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Crates.io](https://img.shields.io/crates/v/rusty-postgres)

# A Advanced Lightwight ORM and Query Builder for postgres

## Usage

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
        partition:{
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
      model:"elonmusk",
      select:{
          "id"
      },
      condition:{
          "story" => "billionairehari"
      }
  };
```

## FindMany

```rust
  let find = find_many! {
      connection => postgres,
      model:"elonmusk",
      conditions:{
          or => {
              "story" => "haribillionaire"
          },
          "story" => "haribillionaire"
      },
      limit:4
  };
```

## Update

```rust
  let update = update! {
      connection => postgres,
      model:"elonmusk",
      select:{
          "story"
      },
      data:{
          "story" => "billionairehari"
      },
      conditions:{
          "story" => "billionaires"
      }
    };
```

## Delete

```rust
  let delete = delete! {
      connection => postgres,
      model:"elonmusk",
      conditions:{
          "story" => "billionairehari"
      },
      select:{
          "story"
      }
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
