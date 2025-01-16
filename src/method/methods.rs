// use rusty_postgres::method::types::{OneToMany,OneToOne};
#[macro_export]
macro_rules! model {
    ($model:expr => {$($billionaire:expr => {$($value:expr),*}),*}) => {
        {
          use core::panic;
          use rand::Rng;
          // use rusty_postgres::method::types::{OneToMany,OneToOne};
          use rusty_postgres::method::types::{OneToMany, OneToOne};
          use rand::distributions::Alphanumeric;

            let mut table = String::new();
            let mut primary_key = String::new();
            let mut index = String::new();
            let mut foriegn_key = String::new();
            // $(
                if $model.len() != 0 {
                    table.push_str(&format!("\r\nCREATE TABLE IF NOT EXISTS {} (\r\n",$model));
                }
                else {
                    panic!("Provide model")
                }
                // table.push_str(&$model.to_lowercase());
                // println!("{}",$model);
                $(
                    if $billionaire.len() != 0 {
                        table.push_str(&format!("{} ",$billionaire.to_lowercase()));
                    }
                    else {
                        panic!("Provide model name cause model name is unknown {}",$billionaire);
                    }
                    //
                    $(
                        let value = stringify!($value);
                        // println!("{}",stringify!($value));
                        // for id
                        if value.starts_with("ID(") {
                            if value == ("ID(\"uuid\")") {
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                table.push_str("UUID ");
                                // table.push_str("PRIMARY KEY ");
                                table.push_str("DEFAULT uuid_generate_v4() ");
                            }
                            else if value == ("ID(\"cuid\")"){
                                table.push_str("TEXT ");
                                // table.push_str("PRIMARY KEY ");
                                table.push_str("DEFAULT encode(gen_random_bytes(12),'hex') ");
                            }
                            else if value == ("ID(\"auto\")") {
                                table.push_str("INT GENERATED ALWAYS AS IDENTITY ");
                                // table.push_str("PRIMARY KEY ");
                            }
                            else {
                                panic!("please provide correct (uuid,cuid,auto)")
                            }
                        }

                        // * ONE_TO_ONE
                        else if value.starts_with("OneToOne"){
                            let serialize = serde_json::to_string(&$value).unwrap();
                            // println!("{:?}", serialize);
                            let deserialize = serde_json::from_str::<OneToOne>(&serialize).unwrap();

                            // * UUID
                            if table.contains("UUID") {
                                let line = format!("UUID UNIQUE REFERENCES {}({}) ON DELETE CASCADE",deserialize.table,deserialize.table_field);
                                table.push_str(&line);
                            }

                            // * AUTO
                            else if table.contains("INT") {
                                let line = format!("INT UNIQUE REFERENCES {}({}) ",deserialize.table,deserialize.table_field);
                                table.push_str(&line);
                            }

                           // // ! ADD CUID

                            else {
                                panic!("Provide correct variable in id")
                            }
                        }

                        // * ONE_TO_MANY
                        else if value.starts_with("OneToMany"){
                            let serialize = serde_json::to_string(&$value).unwrap();
                            // println!("{:?}", serialize);
                            let deserialize = serde_json::from_str::<OneToMany>(&serialize).unwrap();

                            // * UUID
                            if table.contains("UUID") {
                                let line = format!("UUID ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                            }
                            // * AUTO
                            else if table.contains("INT") {
                                let line = format!("INT UNIQUE REFERENCES {}({}) ",deserialize.table,deserialize.table_field);
                                table.push_str(&line);
                            }

                        //   //! ADD CUID

                            else {
                                panic!("Provide correct variable in id in ONE_TO_MANY")
                            }

                        }

                        // * DATE
                        else if value.starts_with("Date") && !value.starts_with("DateTime") {

                            // * NOW
                            if value == stringify!(Date("now()")) {
                                // println!("{}","billionaire");
                                table.push_str(&format!("{}","DATE DEFAULT CURRENT_DATE"))
                            }
                            // * CUSTOME
                            else if value == "Date(\"custome()\")" {
                                table.push_str(&format!("{}","DATE NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for DATE")
                            }
                        }

                        // * TIME
                        else if value.starts_with("Time") {

                            // * NOW
                            if value == ("Time(\"now()\")") {
                                // println!("{}")
                               table.push_str(&format!("{}","TIME DEFAULT CURRENT_TIME"))
                            }

                            // * CUSTOME
                            else if value == ("Time(\"custome()\")") {
                                table.push_str(&format!("{}","TIME NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for TIME")
                            }
                       }

                        //  * date_time
                       else if value.starts_with("DateTime") {
                            if value == ("DateTime(\"now()\")") {
                                table.push_str(&format!("{}","TIMESTAMP DEFAULT NOW()"))
                            }
                            else if value == ("DateTime(\"custome()\")") {
                                table.push_str(&format!("{}","TIMESTAMP NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for DATETIME")
                            }
                        }
                        else if value.starts_with("STRING"){
                            table.push_str("TEXT ")
                        }
                        else if value.starts_with("NUMBER"){
                            table.push_str("INT ")
                        }
                        else if value.starts_with("BOOL"){
                            table.push_str("BOOL ")
                        }

                        // * DEFAULT
                        else if value.starts_with("DEFAULT"){
                            let value = value.split("(").nth(1).unwrap().trim_end_matches(")");
                            let default = format!("DEFAULT {}", value);
                            table.push_str(&default);
                        }

                        // * UNIQUE
                        else if value.starts_with("UNIQUE"){
                            table.push_str("UNIQUE ")
                        }

                        // * JSON
                        else if value.starts_with("JSON"){
                            table.push_str("JSONB ")
                        }

                        // * GEOGRAPHY
                        else if value.starts_with("Geography"){
                            table.push_str(&format!("GEOGRAPHY(POINT,4326) "))
                        }
                        else if value.starts_with("NOTNULL"){
                            table.push_str(&format!("NOT NULL "))
                        }
                        else if value.starts_with("PRIMARY"){
                            primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                        }
                        else if value.starts_with("INDEX"){
                            index.push_str(&format!("{},",$billionaire.to_lowercase()))
                        }
                        else {
                            panic!("Provide Related Key at {}",stringify!($value))
                        }
                        // println!("{:?}",$value);
                    )*
                    let mut table = table.trim_end_matches(" ").to_string();
                    table.push_str(",");
                    table.push_str("\n");
                )*
                // println!("{}",table);
                if primary_key.len() != 0 {
                    let mut primary_key = primary_key.trim_end_matches(",").to_string();
                    let key = format!("PRIMARY KEY ({})",primary_key);
                    // println!("{:?}",key);
                    table.push_str(&key);
                }
                if foriegn_key.len() != 0 {
                    // println!("{}",foriegn_key);
                    let foriegn_key = format!(",\r\n{}",foriegn_key);
                    table.push_str(&foriegn_key);
                }
                let table = table.trim_end_matches("\n");
                let mut table = table.trim_end_matches(",").to_string();
                // println!("{}",tables);
                table.push_str("\r\n);\r\n");
                let index = index.trim_end_matches(",");
                // println!("{}",index);
                if index.len() != 0 {
                    let random: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(6)
                    .map(char::from)
                    .collect();
                    table.push_str(&format!("CREATE INDEX index_{} ON {} ({});\r\n",random,$model,index));
                    table.push_str(&format!("CLUSTER {} USING index_{};\r\n",$model,random));
                }
            // )*
            table
        }
    };

    ($model:expr => {$($billionaire:expr => {$($value:expr),*}),*},partition:{
        type:$type:expr,
        to:$to:expr
    }) => {
        {
          use rand::Rng;
          use core::panic;
          // use rusty_postgres::method::types::{OneToOne,OneToMany};
          use rusty_postgres::method::types::{OneToMany, OneToOne};
          use rand::distributions::Alphanumeric;

            let mut table = String::new();
            let mut primary_key = String::new();
            let mut index = String::new();
            let mut foriegn_key = String::new();
            // $(
                if $model.len() != 0 {
                    table.push_str(&format!("\r\nCREATE TABLE IF NOT EXISTS {} (\r\n",$model));
                }
                else {
                    panic!("Provide model")
                }
                // table.push_str(&$model.to_lowercase());
                // println!("{}",$model);
                $(
                    if $billionaire.len() != 0 {
                        table.push_str(&format!("{} ",$billionaire.to_lowercase()));
                    }
                    else {
                        panic!("Provide model name cause model name is unknown {}",$billionaire);
                    }
                    //
                    $(
                        let value = stringify!($value);
                        // println!("{}",stringify!($value));
                        // for id
                        if value.starts_with("ID(") {
                            if value == ("ID(\"uuid\")") {
                                table.push_str("UUID ");
                                // table.push_str("PRIMARY KEY ");
                                table.push_str("DEFAULT uuid_generate_v4() ");
                            }
                            else if value == ("ID(\"cuid\")"){
                                table.push_str("TEXT ");
                                // table.push_str("PRIMARY KEY ");
                                table.push_str("DEFAULT encode(gen_random_bytes(12),'hex') ");
                            }
                            else if value == ("ID(\"auto\")") {
                                table.push_str("INT GENERATED ALWAYS AS IDENTITY ");
                                // table.push_str("PRIMARY KEY ");
                            }
                            else {
                                panic!("please provide correct (uuid,cuid,auto)")
                            }
                        }

                        // * ONE_TO_ONE
                        else if value.starts_with("OneToOne"){
                            let serialize = serde_json::to_string(&$value).unwrap();
                            // println!("{:?}", serialize);
                            let deserialize = serde_json::from_str::<OneToOne>(&serialize).unwrap();

                            // * UUID
                            if table.contains("UUID") {
                                let line = format!("UUID UNIQUE REFERENCES {}({}) ON DELETE CASCADE",deserialize.table,deserialize.table_field);
                                table.push_str(&line);
                            }

                            // * AUTO
                            else if table.contains("INT") {
                                let line = format!("INT UNIQUE REFERENCES {}({}) ",deserialize.table,deserialize.table_field);
                                table.push_str(&line);
                            }

                           // // ! ADD CUID

                            else {
                                panic!("Provide correct variable in id")
                            }
                        }

                        // * ONE_TO_MANY
                        else if value.starts_with("OneToMany"){
                            let serialize = serde_json::to_string(&$value).unwrap();
                            // println!("{:?}", serialize);
                            let deserialize = serde_json::from_str::<OneToMany>(&serialize).unwrap();

                            // * UUID
                            if table.contains("UUID") {
                                let line = format!("UUID ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                            }
                            // * AUTO
                            else if table.contains("INT") {
                                let line = format!("INT UNIQUE REFERENCES {}({}) ",deserialize.table,deserialize.table_field);
                                table.push_str(&line);
                            }

                        //   //! ADD CUID

                            else {
                                panic!("Provide correct variable in id in ONE_TO_MANY")
                            }

                        }

                        // * DATE
                        else if value.starts_with("Date") && !value.starts_with("DateTime") {

                            // * NOW
                            if value == stringify!(Date("now()")) {
                                // println!("{}","billionaire");
                                table.push_str(&format!("{}","DATE DEFAULT CURRENT_DATE"))
                            }
                            // * CUSTOME
                            else if value == "Date(\"custome()\")" {
                                table.push_str(&format!("{}","DATE NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for DATE")
                            }
                        }

                        // * TIME
                        else if value.starts_with("Time") {

                            // * NOW
                            if value == ("Time(\"now()\")") {
                                // println!("{}")
                               table.push_str(&format!("{}","TIME DEFAULT CURRENT_TIME"))
                            }

                            // * CUSTOME
                            else if value == ("Time(\"custome()\")") {
                                table.push_str(&format!("{}","TIME NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for TIME")
                            }
                       }

                        //  * date_time
                       else if value.starts_with("DateTime") {
                            if value == ("DateTime(\"now()\")") {
                                table.push_str(&format!("{}","TIMESTAMP DEFAULT NOW()"))
                            }
                            else if value == ("DateTime(\"custome()\")") {
                                table.push_str(&format!("{}","TIMESTAMP NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for DATETIME")
                            }
                        }
                        else if value.starts_with("STRING"){
                            table.push_str("TEXT ")
                        }
                        else if value.starts_with("NUMBER"){
                            table.push_str("INT ")
                        }
                        else if value.starts_with("BOOL"){
                            table.push_str("BOOL ")
                        }

                        // * DEFAULT Bool
                        else if value.starts_with("DEFAULT"){
                             if value.starts_with("DEFAULT(\"TRUE\")"){
                                table.push_str("DEFAULT TRUE ")
                            }
                            else if value.starts_with("DEFAULT(\"TRUE\")"){
                                table.push_str("DEFAULT FALSE ")
                            }
                        }

                        // * UNIQUE
                        else if value.starts_with("UNIQUE"){
                            table.push_str("UNIQUE ")
                        }

                        // * JSON
                        else if value.starts_with("JSON"){
                            table.push_str("JSONB ")
                        }

                        // * GEOGRAPHY
                        else if value.starts_with("Geography"){
                            table.push_str(&format!("GEOGRAPHY(POINT,4326) "))
                        }
                        else if value.starts_with("NOTNULL"){
                            table.push_str(&format!("NOT NULL "))
                        }
                        else if value.starts_with("PRIMARY"){
                            primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                        }
                        else if value.starts_with("INDEX"){
                            index.push_str(&format!("{},",$billionaire.to_lowercase()))
                        }
                        else {
                            panic!("Provide Related Key at {}",stringify!($value))
                        }
                        // println!("{:?}",$value);
                    )*
                    let mut table = table.trim_end_matches(" ").to_string();
                    table.push_str(",");
                    table.push_str("\n");
                )*
                if primary_key.len() != 0 {
                    let mut primary_key = primary_key.trim_end_matches(",").to_string();
                    let key = format!("PRIMARY KEY ({})",primary_key);
                    // println!("{:?}",key);
                    table.push_str(&key);
                }
                // println!("{}",table);
                // table.push_str("PRIMARY KEY (id,story)");
                // let mut primary_key = primary_key.trim_end_matches(",");
                // println!("bbbbbbbbbbbbb {}",primary_key);
                let table = table.trim_end_matches("\n");
                let index = index.trim_end_matches(",");
                let mut table = table.trim_end_matches(",").to_string();

                if $type.len() != 0 && $to.len() != 0 {
                    if $type == "range" {
                        table.push_str(&format!("\r\n) PARTITION BY RANGE ({});\r\n",$to));
                    }
                    else if $type == "list" {
                        table.push_str(&format!("\r\n) PARTITION BY LIST ({});\r\n",$to));
                    }
                    else {
                        panic!("Invalid partition type")
                    }
                }
                if index.len() != 0 {
                    let random: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(6)
                    .map(char::from)
                    .collect();
                    table.push_str(&format!("CREATE INDEX index_{} ON {} ({});\r\n",random,$model,index));
                    table.push_str(&format!("CLUSTER {} USING index_{};",$model,random));
                }
                // println!("{}",tables);
                    // table.push_str(&format!("\r\n) PARTITION BY {} ({});\r\n",$type,$to));
                    // table.push_str(&format!("\r\n);\r\n"));
            // )*
            table
        }
    };
}

#[macro_export]
macro_rules! container {
     (client => $url:expr ,models => { $($model:expr),*}) => {
        {
          use std::fs::DirBuilder;
          use std::fs::File;
          use std::io::Write;

            // let model = $model.clone();
            let mut container = String::new();
            let mut containers = String::new();
            let mut fresh = String::new();
            $(
                // container.push_str(&format!("{}",$model));
                if let Some((first,second)) = $model.split_once("\r\n") {
                    let first = &first;
                    fresh.push_str(first);
                    let second = &second;
                    container.push_str(&format!("{}",second));
                }
                // println!("{}",$model);
            )*
            if container.contains("GEOGRAPHY"){
                containers.push_str("CREATE EXTENSION IF NOT EXISTS postgis;\r\n");
                // containers.push_str(&container);
            }
            if container.contains("UUID"){
                containers.push_str("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";\r\n");
                // containers.push_str(&container);
            }
            if container.contains("encode(gen_random_bytes(12)"){
                containers.push_str("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";\r\n");
                // containers.push_str(&container);
            }
            containers.push_str(&container);
            // let mut cluster = String::new();
            if let Some(cluster) = containers.find("CLUSTER"){
                let clusters = &containers[cluster..];
                let containers = &containers[..cluster];
                // println!("{}",containers);
                // println!("{}",clusters);

                $url.batch_execute(&fresh).unwrap();
                let db = $url.batch_execute(&containers);
                $url.batch_execute(&clusters).unwrap();
                DirBuilder::new()
                .recursive(true)
                .create("database")
                .unwrap();
                let mut sql = File::create("database/db.sql").unwrap();
                sql.write_all(format!("/* Reference Schema */\r\n").as_bytes()).unwrap();
                sql.write_all(containers.as_bytes()).unwrap();
                sql.write_all(clusters.as_bytes()).unwrap();
                db
            }
            else {
                // println!("{}",containers);
                $url.batch_execute(&fresh).unwrap();
                let db = $url.batch_execute(&containers);
                DirBuilder::new()
                .recursive(true)
                .create("database")
                .unwrap();
                let mut sql = File::create("database/db.sql").unwrap();
                sql.write_all(format!("/* Reference Schema */\r\n").as_bytes()).unwrap();
                sql.write_all(containers.as_bytes()).unwrap();
                db
            }
        }
    };
}

#[macro_export]
macro_rules! find_one {
  (@all $connection:expr,$model:expr,$client:expr) => {
    {
      use core::panic;
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}';",$model);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
  (@select $connection:expr,$model:expr,$selection:expr,$client:expr) => {
    {
      use core::panic;
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}' AND column_name IN ({})",$model,$selection);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in $client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
    // * model //completed
    (connection => $connection:expr,model:$model:expr) => {
        {
          use core::panic;
            let mut command = String::new();
            if $model.len() != 0 {
                let query = format!("SELECT * FROM {} LIMIT 1;",$model);
                command.push_str(&query);
                let client = $connection.query(&command,&[]).unwrap();

                // * listing column_name,data_type
                find_many!(@all $connection,$model,client)
            }
            else {
                panic!("\x1b[44mProvide Model Name\x1b[0m")
            }
    }
    };
        // * where //completed
        (connection => $connection:expr,
            model:$model:expr,
        condition:{
            $($value_where:expr => $where_by:expr),*
        }) => {
            {
              use postgres::types::ToSql;
                // println!("{}","billionaire");
                // let mut command = String::new();
                // let mut select_value = String::new();
                let mut where_value = String::new();
                let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut selections = String::new();

                if $model.len() != 0 {
                    $(
                        // let selectvalue = format!("{},",$value_where);
                        // select_value.push_str(&selectvalue);

                        let selectvalue = format!("'{}',",$value_where);
                        selections.push_str(&selectvalue);
                    )*
                    let mut idx = 0;
                    $(
                            idx +=1;
                            let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                            where_value.push_str(&wherevalue);
                    )*
                    // println!("{}",idx);
                    $(
                        value.push(&$where_by);
                    )*
                    // let select_value = select_value.trim_end_matches(",");
                    let where_value = where_value.trim_end_matches("AND ");
                    let selections = selections.trim_end_matches(",");

                    let query = format!("SELECT * FROM {} WHERE {} LIMIT 1;",$model,where_value);
                    // println!("{}",query);
                    // println!("{:?}",value);
                    // command.push_str(&query);
                    let client = $connection.query(&query,&value).unwrap();
                    // println!("{:?}","client");

                    find_many!(@all $connection,$model,client)
            }
            else {
                panic!("\x1b[44mProvide Model Name\x1b[0m")
            }
        }
        };
    // * where //completed
    (connection => $connection:expr,
        model:$model:expr,
        select:{
        $($values:expr),*
    },
    condition:{
        $($value_where:expr => $where_by:expr),*
    }) => {
        {
          use core::panic;
            use postgres::types::ToSql;
            // println!("{}","billionaire");
            // let mut command = String::new();
            let mut select_value = String::new();
            let mut where_value = String::new();
            let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut selection = String::new();

            if $model.len() != 0 {
                $(
                    let selectvalue = format!("{},",$values);
                    select_value.push_str(&selectvalue);

                    let selectvalue = format!("'{}',",$values);
                    selection.push_str(&selectvalue);
                )*
                let mut idx = 0;
                $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                        where_value.push_str(&wherevalue);
                )*
                // println!("{}",idx);
                $(
                    value.push(&$where_by);
                )*
                let select_value = select_value.trim_end_matches(",");
                let where_value = where_value.trim_end_matches("AND ");
                let selection = selection.trim_end_matches(",");

                // println!("{}",select_value);
                let query = format!("SELECT {} FROM {} WHERE {} LIMIT 1;",select_value,$model,where_value);
                // println!("{}",query);
                // println!("{:?}",value);
                // command.push_str(&query);
                let client = $connection.query(&query,&value).unwrap();
                // println!("{:?}","client");

                find_many!(@select $connection,$model,selection,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }
    };
    // *
    (connection => $connection:expr,
        model:$model:expr,
        select:{
        $($values:expr),*
    },
    condition:{
        $($value_where:expr => $where_by:expr),*
    },
    between:{
        $value:expr => {
            $first:expr,
            $second:expr
        }
    }
    ) => {
        {
          use core::panic;
            use postgres::types::ToSql;
            // println!("{}","billionaire");
            // let mut command = String::new();
            let mut select_value = String::new();
            let mut where_value = String::new();
            let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut selection = String::new();
            let mut between = String::new();
            // between.push_str(&format!("AND {} BETWEEN  {} AND {};"));

            if $model.len() != 0 {
                $(
                    let selectvalue = format!("{},",$values);
                    select_value.push_str(&selectvalue);

                    let selectvalue = format!("'{}',",$values);
                    selection.push_str(&selectvalue);
                )*
                let mut idx = 0;
                $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                        where_value.push_str(&wherevalue);
                )*
                // println!("{}",idx);
                $(
                    value.push(&$where_by);
                )*
                let select_value = select_value.trim_end_matches(",");
                let mut where_value = where_value.trim_end_matches("AND ").to_string();
                let selection = selection.trim_end_matches(",");

                where_value.push_str(&format!("AND {} BETWEEN  '{}' AND '{}'",$value,$first,$second));
                                // println!("{}",where_value);
                // println!("{}",select_value);
                let query = format!("SELECT {} FROM {} WHERE {} LIMIT 1;",select_value,$model,where_value);
                println!("{}",query);
                // println!("{:?}",value);
                // command.push_str(&query);
                let client = $connection.query(&query,&value).unwrap();
                // println!("{:?}","client");

                find_many!(@select $connection,$model,selection,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }
    };
    // * included or //completed
    (connection => $connection:expr,
        model:$model:expr,
        select:{
         $($select_value:expr),*
    },
    conditions : {
        or => {
            $($or_value_or:expr => $or_value_orr:expr),*
        }
    }) => {
        {
          use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut or_values = String::new();
                let mut or_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut selection = String::new();

                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*

                let select_value = selectvalue.trim_end_matches(",");
                let selection = selection.trim_end_matches(",");
                // println!("{}",select_value);
                let mut idx = 0;
                $(
                    idx+=1;
                    or_values.push_str(&format!("{} = ${} OR ",$or_value_or,idx));
                    or_value.push(&$or_value_orr);
                )*

                let or_values = or_values.trim_end_matches("OR ");
                let format = format!("SELECT {} FROM {} WHERE {} LIMIT 1;",select_value,$model,or_values);
                // println!("{}",format);
                let client = $connection.query(&format,&or_value).unwrap();
                find_many!(@select $connection,$model,selection,client)
    }
    };
    // * included and or //completed
    (connection => $connection:expr,
        model:$model:expr,
        select:{
         $($select_value:expr),*
    },
    conditions : {
        or => {
            $($or_value1:expr => $or_value2:expr),*
        },
        $($and_values:expr => $and_value:expr),*
    }
) => {
        {
          use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut selection = String::new();
                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*
                let select_value = selectvalue.trim_end_matches(",");
                let selection = selection.trim_end_matches(",");
                // println!("{}",select_value);
                let mut idx = 0;
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                let and_values = and_values.trim_end_matches("OR ");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} LIMIT 1;",select_value,$model,and_values);
                // println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                // println!("{:?}",client);
                find_many!(@select $connection,$model,selection,client)
    }
    };
    //* included or order //completed
    (connection => $connection:expr,
        model:$model:expr,
        select:{
         $($select_value:expr),*
    },
    conditions:{
        or => {$($or_values:expr => $or_value:expr),*}
    },
    order : {$($target:expr => $order:expr),*}) => {
        {
          use core::panic;
          use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut order = String::new();
                let mut selection = String::new();

                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*

                let select_value = selectvalue.trim_end_matches(",");
                let selection = selection.trim_end_matches(",");

                let mut idx = 0;

                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} OR ",$or_values,idx));
                    and_value.push(&$or_value);
                )*

                let and_values = and_values.trim_end_matches("OR ");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                $(
                    if !["asc","desc"].contains(&$order) {
                        panic!("Provide correct order either \"asc\" nor \"desc\"");
                    }
                    let order_ = format!("{} {},",$target,$order);
                    order.push_str(&order_);
                )*

                let order = order.trim_end_matches(",");

                let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT 1;",select_value,$model,and_values,order);

                // println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                find_many!(@select $connection,$model,selection,client)
    }
    };
    //* included or limit skip //completed
//     (connection => $client:expr,
//         model:$model:expr,
//         select:{
//          $($select_value:expr),*
//     },
//     conditions:{
//         or => {$($or_values:expr => $or_value:expr),*}
//     },
//     limit:$limit:expr,
//     skip:$skip:expr
// ) => {
//         {
//             let mut selectvalue = String::new();
//             let mut or_values = String::new();
//             let mut or_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
//         $(
//             selectvalue.push_str(&format!("{},",$select_value));
//         )*
//         let select_value = selectvalue.trim_end_matches(",");
//         // println!("{}",select_value);
//         let mut idx = 0;
//         $(
//             idx+=1;
//             or_values.push_str(&format!("{} = ${} OR ",$or_values,idx));
//             or_value.push(&$or_value);
//         )*
//         let or_values = or_values.trim_end_matches("OR ");
//         // println!("{}",or_values);
//         // println!("{:?}",or_value);
//         let format = format!("SELECT {} FROM {} WHERE {} LIMIT {} OFFSET {};",select_value,$model,or_values,$limit,$skip);
//         println!("{}",format);
//         let response = $client.query(&format,&or_value).unwrap();
//         response
//     }
//     };
    // * included and or limit //completed
    // (connection => $client:expr,
    //         model:$model:expr,
    //         select:{
    //          $($select_value:expr),*
    //     },
    //     conditions:{
    //         or =>  {$($or_value1:expr => $or_value2:expr),*},
    //         $($and_values:expr => $and_value:expr),*
    //     },
    //     limit:$limit:expr
    // ) => {
    //         {
    //             let mut selectvalue = String::new();
    //             let mut and_values = String::new();
    //             let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
    //         $(
    //             selectvalue.push_str(&format!("{},",$select_value));
    //         )*
    //         let select_value = selectvalue.trim_end_matches(",");
    //         // println!("{}",select_value);
    //         let mut idx = 0;
    //         $(
    //             idx+=1;
    //             and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
    //             and_value.push(&$and_value);
    //         )*
    //         $(
    //             idx+=1;
    //             and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
    //             and_value.push(&$and_value);
    //         )*
    //         let and_values = and_values.trim_end_matches("OR ");
    //         // println!("{}",and_values);
    //         // println!("{:?}",and_value);
    //         let format = format!("SELECT {} FROM {} WHERE {} LIMIT {};",select_value,$model,and_values,$limit,$skip);
    //         println!("{}",format);
    //         let response = $client.query(&format,&and_value).unwrap();
    //         response
    //     }
    // };
    // * included and or order //completed
    (connection => $connection:expr,
            model:$model:expr,
            select:{
             $($select_value:expr),*
        },
        conditions:{
            or =>  {$($or_value1:expr => $or_value2:expr),*},
            $($and_values:expr => $and_value:expr),*
        },
        order : {$($target:expr => $order:expr),*}
    ) => {
            {
              use core::panic;
              use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut order = String::new();
                let mut selection = String::new();
            $(
                selectvalue.push_str(&format!("{},",$select_value));
                selection.push_str(&format!("'{}',",$select_value));
            )*
            let select_value = selectvalue.trim_end_matches(",");
            let selection = selection.trim_end_matches(",");
            // println!("{}",select_value);
            let mut idx = 0;
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                if !["asc","desc"].contains(&$order) {
                    panic!("Provide correct order either \"asc\" nor \"desc\"");
                }
                let order_ = format!("{} {},",$target,$order);
                order.push_str(&order_);
            )*
            let and_values = and_values.trim_end_matches("OR ");
            let order = order.trim_end_matches(",");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT 1;",select_value,$model,and_values,order);
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@select $connection,$model,selection,client)
        }
    };
    // * included and or limit skip //completed
//     (connection => $client:expr,
//         model:$model:expr,
//         select:{
//          $($select_value:expr),*
//     },
//     conditions:{
//         or =>  {$($or_value1:expr => $or_value2:expr),*},
//         $($and_values:expr => $and_value:expr),*
//     },
//     limit:$limit:expr,
//     skip:$skip:expr
// ) => {
//         {
//             let mut selectvalue = String::new();
//             let mut and_values = String::new();
//             let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
//         $(
//             selectvalue.push_str(&format!("{},",$select_value));
//         )*
//         let select_value = selectvalue.trim_end_matches(",");
//         // println!("{}",select_value);
//         let mut idx = 0;
//         $(
//             idx+=1;
//             and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
//             and_value.push(&$and_value);
//         )*
//         $(
//             idx+=1;
//             and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
//             and_value.push(&$and_value);
//         )*
//         let and_values = and_values.trim_end_matches("OR ");
//         // println!("{}",and_values);
//         // println!("{:?}",and_value);
//         let format = format!("SELECT {} FROM {} WHERE {} LIMIT {} OFFSET {};",select_value,$model,and_values,$limit,$skip);
//         println!("{}",format);
//         let response = $client.query(&format,&and_value).unwrap();
//         response
//     }
//     };
    // * included and order //completed
    (connection => $connection:expr,
        model:$model:expr,
        select:{
        $($select_value:expr),*
    },
    conditions : {
        $($and_values:expr => $and_value:expr),*
    },
    order : {$($target:expr => $order:expr),*}
) => {
        {
          use core::panic;
          use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut order = String::new();
                let mut selection = String::new();
                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*
                let select_value = selectvalue.trim_end_matches(",");
                let selection = selection.trim_end_matches(",");
                // println!("{}",select_value);
                let mut idx = 0;
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                $(
                    if !["asc","desc"].contains(&$order) {
                        panic!("Provide correct order either \"asc\" nor \"desc\"");
                    }
                    let order_ = format!("{} {},",$target,$order);
                    order.push_str(&order_);
                )*
                let order = order.trim_end_matches(",");
                let and_values = and_values.trim_end_matches("AND ");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT 1;",select_value,$model,and_values,order);
                // println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                find_many!(@select $connection,$model,selection,client)
        }
    };

}

#[macro_export]
macro_rules! delete_table {
    (connection => $connection:expr,model => $model:expr) => {{
        let delete = format!("DROP TABLE IF EXISTS {};", $model);
        $connection.execute(&delete, &[])
    }};
}

#[macro_export]
macro_rules! find_many {
  (@all $connection:expr,$model:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}';",$model);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in $client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
  (@select $connection:expr,$model:expr,$selection:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}' AND column_name IN ({})",$model,$selection);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in $client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
    (connection => $connection:expr,model => $model:expr) => {{
                let query = format!("SELECT * FROM {}", $model);
                let client = $connection.query(&query, &[]).unwrap();
                find_many!(@all $connection,$model,client)
    }};
        // * model //completed
        (connection => $connection:expr,model:$model:expr) => {
            {
                use core::panic;
                // let mut command = String::new();
                if $model.len() != 0 {
                    let client = format!("SELECT * FROM {} ;",$model);
                    let client = $connection.query(&client,&[]).unwrap();
                    // command.push_str(&query);
                    find_many!(@all $connection,$model,client)
                }
                else {
                    panic!("\x1b[44mProvide Model Name\x1b[0m")
                }
        }
        };
                // * where //completed
                (connection => $connection:expr,
                model:$model:expr,
                condition:{
                    $($value_where:expr => $where_by:expr),*
                }) => {
                    {
                      use core::panic;
                      use postgres::types::ToSql;

                        let mut command = String::new();
                        // let mut select_value = String::new();
                        let mut where_value = String::new();
                        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                        // let mut selection = String::new();
                    if $model.len() != 0 {
                        let mut idx = 0;
                        $(
                                idx +=1;
                                let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                                where_value.push_str(&wherevalue);
                        )*
                        // println!("{}",idx);
                        $(
                            value.push(&$where_by);
                        )*
                        let where_value = where_value.trim_end_matches("AND ");
                        // println!("{:?}",where_value);
                        // let len = params.len();

                        let query = format!("SELECT * FROM {} WHERE {};",$model,where_value);
                        // println!("{}",query);
                        // println!("{:?}",value);
                        command.push_str(&query);
                        let client = $connection.query(&command,&value).unwrap();
                        find_many!(@all $connection,$model,client)
                    }
                    else {
                        panic!("\x1b[44mProvide Model Name\x1b[0m")
                    }
                }
                };
        // * select where //completed
        (connection => $connection:expr,
            model:$model:expr,
            select:{
            $($values:expr),*
        },
        condition:{
            $($value_where:expr => $where_by:expr),*
        }) => {
            {
              use core::panic;
              use postgres::types::ToSql;

                let mut command = String::new();
                let mut select_value = String::new();
                let mut where_value = String::new();
                let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut selection = String::new();
            if $model.len() != 0 {
                $(
                    let selectvalue = format!("{},",$values);
                    select_value.push_str(&selectvalue);

                    let selectvalue = format!("'{}',",$values);
                    selection.push_str(&selectvalue);
                )*
                let mut idx = 0;
                $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                        where_value.push_str(&wherevalue);
                )*
                // println!("{}",idx);
                $(
                    value.push(&$where_by);
                )*
                let select_value = select_value.trim_end_matches(",");
                let selection = selection.trim_end_matches(",");
                let where_value = where_value.trim_end_matches("AND ");
                // println!("{:?}",where_value);
                // let len = params.len();

                let query = format!("SELECT {} FROM {} WHERE {};",select_value,$model,where_value);
                // println!("{}",query);
                // println!("{:?}",value);
                command.push_str(&query);
                let client = $connection.query(&command,&value).unwrap();
                find_many!(@select $connection,$model,selection,client)
            }
            else {
                panic!("\x1b[44mProvide Model Name\x1b[0m")
            }
        }
        };
                // * select where //completed
                (connection => $connection:expr,
                  model:$model:expr,
              conditions:{
                or => {
                  $($value_where:expr => $where_by:expr),*
                }
              }) => {
                  {
                    use core::panic;
                    use postgres::types::ToSql;

                      let mut command = String::new();
                      let mut where_value = String::new();
                      let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                  if $model.len() != 0 {
                      let mut idx = 0;
                      $(
                              idx +=1;
                              let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                              where_value.push_str(&wherevalue);
                      )*
                      // println!("{}",idx);
                      $(
                          value.push(&$where_by);
                      )*
                      let where_value = where_value.trim_end_matches("AND ");
                      // println!("{:?}",where_value);
                      // let len = params.len();

                      let query = format!("SELECT * FROM {} WHERE {};",$model,where_value);
                      // println!("{}",query);
                      // println!("{:?}",value);
                      command.push_str(&query);
                      let client = $connection.query(&command,&value).unwrap();
                      find_many!(@all $connection,$model,client)
                  }
                  else {
                      panic!("\x1b[44mProvide Model Name\x1b[0m")
                  }
              }
              };
                // * select where //completed
                (connection => $connection:expr,
                  model:$model:expr,
              conditions:{
                or => {
                  $($value_where:expr => $where_by:expr),*
                },
                $($value_where_and:expr => $where_by_and:expr)*
              }) => {
                  {
                    use core::panic;
                    use postgres::types::ToSql;

                      let mut command = String::new();
                      let mut where_value = String::new();
                      let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                  if $model.len() != 0 {
                      let mut idx = 0;
                      $(
                              idx +=1;
                              let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                              where_value.push_str(&wherevalue);
                      )*
                      $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                        where_value.push_str(&wherevalue);
                      )*
                      // println!("{}",idx);
                      $(
                          value.push(&$where_by);
                      )*
                      $(
                        value.push(&$where_by_and);
                    )*
                      let where_value = where_value.trim_end_matches("AND ");
                      // println!("{:?}",where_value);
                      // let len = params.len();

                      let query = format!("SELECT * FROM {} WHERE {};",$model,where_value);
                      // println!("{}",query);
                      // println!("{:?}",value);
                      command.push_str(&query);
                      let client = $connection.query(&command,&value).unwrap();
                      find_many!(@all $connection,$model,client)
                  }
                  else {
                      panic!("\x1b[44mProvide Model Name\x1b[0m")
                  }
              }
              };
                // * select where //completed
                (connection => $connection:expr,
                  model:$model:expr,
              conditions:{
                or => {
                  $($value_where:expr => $where_by:expr),*
                },
                $($value_where_and:expr => $where_by_and:expr)*
              },limit:$limit:expr) => {
                  {
                    use core::panic;
                    use postgres::types::ToSql;

                      let mut command = String::new();
                      let mut where_value = String::new();
                      let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                  if $model.len() != 0 {
                      let mut idx = 0;
                      $(
                              idx +=1;
                              let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                              where_value.push_str(&wherevalue);
                      )*
                      $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                        where_value.push_str(&wherevalue);
                      )*
                      // println!("{}",idx);
                      $(
                          value.push(&$where_by);
                      )*
                      $(
                        value.push(&$where_by_and);
                    )*
                      let where_value = where_value.trim_end_matches("AND ");
                      // println!("{:?}",where_value);
                      // let len = params.len();

                      let query = format!("SELECT * FROM {} WHERE {} LIMIT {};",$model,where_value,$limit);
                      // println!("{}",query);
                      // println!("{:?}",value);
                      command.push_str(&query);
                      let client = $connection.query(&command,&value).unwrap();
                      find_many!(@all $connection,$model,client)
                  }
                  else {
                      panic!("\x1b[44mProvide Model Name\x1b[0m")
                  }
              }
              };
                // * select where //completed
                (connection => $connection:expr,
                  model:$model:expr,
              conditions:{
                or => {
                  $($value_where:expr => $where_by:expr),*
                },
                $($value_where_and:expr => $where_by_and:expr)*
              },limit:$limit:expr,skip:$skip:expr) => {
                  {
                    use core::panic;
                    use postgres::types::ToSql;

                      let mut command = String::new();
                      let mut where_value = String::new();
                      let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                  if $model.len() != 0 {
                      let mut idx = 0;
                      $(
                              idx +=1;
                              let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                              where_value.push_str(&wherevalue);
                      )*
                      $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                        where_value.push_str(&wherevalue);
                      )*
                      // println!("{}",idx);
                      $(
                          value.push(&$where_by);
                      )*
                      $(
                        value.push(&$where_by_and);
                    )*
                      let where_value = where_value.trim_end_matches("AND ");
                      // println!("{:?}",where_value);
                      // let len = params.len();

                      let query = format!("SELECT * FROM {} WHERE {} LIMIT {} OFFSET {};",$model,where_value,$limit,$skip);
                      // println!("{}",query);
                      // println!("{:?}",value);
                      command.push_str(&query);
                      let client = $connection.query(&command,&value).unwrap();
                      find_many!(@all $connection,$model,client)
                  }
                  else {
                      panic!("\x1b[44mProvide Model Name\x1b[0m")
                  }
              }
              };
                              // * select where //completed
                (connection => $connection:expr,
                  model:$model:expr,
              conditions:{
                or => {
                  $($value_where:expr => $where_by:expr),*
                },
                $($value_where_and:expr => $where_by_and:expr)*
              },limit:$limit:expr,skip:$skip:expr,order:{
                $($order_by:expr => $order:expr),*
              }) => {
                  {
                    use core::panic;
                    use postgres::types::ToSql;

                      let mut command = String::new();
                      let mut where_value = String::new();
                      let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                      let mut order = String::new();
                  if $model.len() != 0 {
                      let mut idx = 0;
                      $(
                              idx +=1;
                              let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                              where_value.push_str(&wherevalue);
                      )*
                      $(
                        let order_by = format!("{} {} AND ",$order_by,$order.to_uppercase());
                        order.push_str(&order_by);
                      )*
                      $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                        where_value.push_str(&wherevalue);
                      )*
                      // println!("{}",idx);
                      $(
                          value.push(&$where_by);
                      )*
                      $(
                        value.push(&$where_by_and);
                    )*
                      let where_value = where_value.trim_end_matches("AND ");
                      let order = order.trim_end_matches("AND ");
                      // println!("{:?}",where_value);
                      // let len = params.len();

                      let query = format!("SELECT * FROM {} WHERE {} ORDER BY {} LIMIT {} OFFSET {};",$model,where_value,order,$limit,$skip);
                      // println!("{}",query);
                      // println!("{:?}",value);
                      command.push_str(&query);
                      let client = $connection.query(&command,&value).unwrap();
                      find_many!(@all $connection,$model,client)
                  }
                  else {
                      panic!("\x1b[44mProvide Model Name\x1b[0m")
                  }
              }
              };
                                            // * select where //completed
                (connection => $connection:expr,
                  model:$model:expr,
              conditions:{
                or => {
                  $($value_where:expr => $where_by:expr),*
                },
                $($value_where_and:expr => $where_by_and:expr)*
              },skip:$skip:expr,order:{
                $($order_by:expr => $order:expr),*
              }) => {
                  {
                    use core::panic;
                    use postgres::types::ToSql;

                      let mut command = String::new();
                      let mut where_value = String::new();
                      let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                      let mut order = String::new();
                  if $model.len() != 0 {
                      let mut idx = 0;
                      $(
                              idx +=1;
                              let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                              where_value.push_str(&wherevalue);
                      )*
                      $(
                        let order_by = format!("{} {} AND ",$order_by,$order.to_uppercase());
                        order.push_str(&order_by);
                      )*
                      $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                        where_value.push_str(&wherevalue);
                      )*
                      // println!("{}",idx);
                      $(
                          value.push(&$where_by);
                      )*
                      $(
                        value.push(&$where_by_and);
                    )*
                      let where_value = where_value.trim_end_matches("AND ");
                      let order = order.trim_end_matches("AND ");
                      // println!("{:?}",where_value);
                      // let len = params.len();

                      let query = format!("SELECT * FROM {} WHERE {} ORDER BY {} OFFSET {};",$model,where_value,order,$skip);
                      // println!("{}",query);
                      // println!("{:?}",value);
                      command.push_str(&query);
                      let client = $connection.query(&command,&value).unwrap();
                      find_many!(@all $connection,$model,client)
                  }
                  else {
                      panic!("\x1b[44mProvide Model Name\x1b[0m")
                  }
              }
              };
                                                          // * select where //completed
                (connection => $connection:expr,
                  model:$model:expr,
              conditions:{
                or => {
                  $($value_where:expr => $where_by:expr),*
                },
                $($value_where_and:expr => $where_by_and:expr)*
              },order:{
                $($order_by:expr => $order:expr),*
              }) => {
                  {
                    use core::panic;
                    use postgres::types::ToSql;

                      let mut command = String::new();
                      let mut where_value = String::new();
                      let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                      let mut order = String::new();
                  if $model.len() != 0 {
                      let mut idx = 0;
                      $(
                              idx +=1;
                              let wherevalue = format!("{} = ${} AND ",$value_where,idx);
                              where_value.push_str(&wherevalue);
                      )*
                      $(
                        let order_by = format!("{} {} AND ",$order_by,$order.to_uppercase());
                        order.push_str(&order_by);
                      )*
                      $(
                        idx +=1;
                        let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                        where_value.push_str(&wherevalue);
                      )*
                      // println!("{}",idx);
                      $(
                          value.push(&$where_by);
                      )*
                      $(
                        value.push(&$where_by_and);
                    )*
                      let where_value = where_value.trim_end_matches("AND ");
                      let order = order.trim_end_matches("AND ");
                      // println!("{:?}",where_value);
                      // let len = params.len();

                      let query = format!("SELECT * FROM {} WHERE {} ORDER BY {} ;",$model,where_value,order);
                      // println!("{}",query);
                      // println!("{:?}",value);
                      command.push_str(&query);
                      let client = $connection.query(&command,&value).unwrap();
                      find_many!(@all $connection,$model,client)
                  }
                  else {
                      panic!("\x1b[44mProvide Model Name\x1b[0m")
                  }
              }
              };
        // * included or //completed
        (connection => $connection:expr,
            model:$model:expr,
            select:{
             $($select_value:expr),*
        },
        conditions : {
            or => {
                $($or_value_or:expr => $or_value_orr:expr),*
            }
        }) => {
            {
              use postgres::types::ToSql;

                        let mut selectvalue = String::new();
                        let mut or_values = String::new();
                        let mut or_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                        let mut selection = String::new();
                    $(
                        selectvalue.push_str(&format!("{},",$select_value));

                        let select = format!("'{}',",$select_value);
                        selection.push_str(&select);
                    )*
                    let select_value = selectvalue.trim_end_matches(",");
                    let selection = selection.trim_end_matches(",");
                    // println!("{}",select_value);
                    let mut idx = 0;
                    $(
                        idx+=1;
                        or_values.push_str(&format!("{} = ${} OR ",$or_value_or,idx));
                        or_value.push(&$or_value_orr);
                    )*
                    let or_values = or_values.trim_end_matches("OR ");
                    // println!("{}",or_values);
                    // println!("{:?}",or_value);
                    let format = format!("SELECT {} FROM {} WHERE {};",select_value,$model,or_values);
                    println!("{}",format);
                    let client = $connection.query(&format,&or_value).unwrap();
                    find_many!(@select $connection,$model,selection,client)
        }
        };
        // * included and or //completed
        (connection => $connection:expr,
            model:$model:expr,
            select:{
             $($select_value:expr),*
        },
        conditions : {
            or => {
                $($or_value1:expr => $or_value2:expr),*
            },
            $($and_values:expr => $and_value:expr),*
        }
        ) => {
            {
              use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut selection = String::new();
            $(
                selectvalue.push_str(&format!("{},",$select_value));
                let select = format!("'{}',",$select_value);
                selection.push_str(&select);
            )*
            let select_value = selectvalue.trim_end_matches(",");
            let selection = selection.trim_end_matches(",");
            // println!("{}",select_value);
            let mut idx = 0;
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            let and_values = and_values.trim_end_matches("OR ");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {};",select_value,$model,and_values);
            println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@select $connection,$model,selection,client)
        }
        };
        //* included or order //completed
        (connection => $connection:expr,
            model:$model:expr,
            select:{
             $($select_value:expr),*
        },
        conditions:{
            or => {$($or_values:expr => $or_value:expr),*}
        },
        order : {$($target:expr => $order:expr),*}) => {
            {
              use core::panic;
              use postgres::types::ToSql;

                    let mut selectvalue = String::new();
                    let mut and_values = String::new();
                    let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                    let mut order = String::new();
                    let mut selection = String::new();
                    $(
                        selectvalue.push_str(&format!("{},",$select_value));
                        selection.push_str(&format!("'{}',",$select_value));
                    )*
                    let select_value = selectvalue.trim_end_matches(",");
                    let selection = selection.trim_end_matches(",");
                    // println!("{}",select_value);
                    let mut idx = 0;
                    $(
                        idx+=1;
                        and_values.push_str(&format!("{} = ${} OR ",$or_values,idx));
                        and_value.push(&$or_value);
                    )*
                    let and_values = and_values.trim_end_matches("OR ");
                    // println!("{}",and_values);
                    // println!("{:?}",and_value);
                    $(
                        if !["asc","desc"].contains(&$order) {
                            panic!("Provide correct order either \"asc\" nor \"desc\"");
                        }
                        let order_ = format!("{} {},",$target,$order);
                        order.push_str(&order_);
                    )*
                    let order = order.trim_end_matches(",");
                    let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {};",select_value,$model,and_values,order);
                    println!("{}",format);
                    let client = $connection.query(&format,&and_value).unwrap();
                    find_many!(@select $connection,$model,selection,client)
        }
        };
        //* included or limit skip //completed
        (connection => $connection:expr,
            model:$model:expr,
            select:{
             $($select_value:expr),*
        },
        conditions:{
            or => {$($or_values:expr => $or_value:expr),*}
        },
        limit:$limit:expr,
        skip:$skip:expr
        ) => {
            {
                let mut selectvalue = String::new();
                let mut or_values = String::new();
                let mut or_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut selection = String::new();
            $(
                selectvalue.push_str(&format!("{},",$select_value));
                selection.push_str(&format!("'{}',",$select_value));
            )*
            let select_value = selectvalue.trim_end_matches(",");
            let selection = selection.trim_end_matches(",");
            // println!("{}",select_value);
            let mut idx = 0;
            $(
                idx+=1;
                or_values.push_str(&format!("{} = ${} OR ",$or_values,idx));
                or_value.push(&$or_value);
            )*
            let or_values = or_values.trim_end_matches("OR ");
            // println!("{}",or_values);
            // println!("{:?}",or_value);
            let format = format!("SELECT {} FROM {} WHERE {} LIMIT {} OFFSET {};",select_value,$model,or_values,$limit,$skip);
            println!("{}",format);
            let client = $connection.query(&format,&or_value).unwrap();
            find_many!(@select $connection,$model,selection,client)
        }
        };
        // * included and or limit //completed
        (connection => $connection:expr,
                model:$model:expr,
                select:{
                 $($select_value:expr),*
            },
            conditions:{
                or =>  {$($or_value1:expr => $or_value2:expr),*},
                $($and_values:expr => $and_value:expr),*
            },
            limit:$limit:expr
        ) => {
                {
                  use postgres::types::ToSql;

                    let mut selectvalue = String::new();
                    let mut and_values = String::new();
                    let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                    let mut selection = String::new();
                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*
                let select_value = selectvalue.trim_end_matches(",");
                let selection = selection.trim_end_matches(",");
                // println!("{}",select_value);
                let mut idx = 0;
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                let and_values = and_values.trim_end_matches("OR ");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} LIMIT {};",select_value,$model,and_values,$limit);
                println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                find_many!(@select $connection,$model,selection,client)
            }
        };
        // * included and or skip //completed
        (connection => $connection:expr,
                model:$model:expr,
                select:{
                 $($select_value:expr),*
            },
            conditions:{
                or =>  {$($or_value1:expr => $or_value2:expr),*},
                $($and_values:expr => $and_value:expr),*
            },
            skip:$skip:expr
        ) => {
                {
                  use postgres::types::ToSql;

                    let mut selectvalue = String::new();
                    let mut and_values = String::new();
                    let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                    let mut selection = String::new();
                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*
                let select_value = selectvalue.trim_end_matches(",");
                let selection = selection.trim_end_matches(",");
                // println!("{}",select_value);
                let mut idx = 0;
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                let and_values = and_values.trim_end_matches("OR ");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} OFFSET {};",select_value,$model,and_values,$skip);
                // println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                find_many!(@select $connection,$model,selection,client)
            }
        };
        // * included and or order //completed
        (connection => $connection:expr,
                model:$model:expr,
                select:{
                 $($select_value:expr),*
            },
            conditions:{
                or =>  {$($or_value1:expr => $or_value2:expr),*},
                $($and_values:expr => $and_value:expr),*
            },
            order : {$($target:expr => $order:expr),*}
        ) => {
                {
                  use core::panic;
                  use postgres::types::ToSql;

                    let mut selectvalue = String::new();
                    let mut and_values = String::new();
                    let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                    let mut order = String::new();
                    let mut selection = String::new();
                    $(
                        selectvalue.push_str(&format!("{},",$select_value));
                        selection.push_str(&format!("'{}',",$select_value));
                    )*
                let select_value = selectvalue.trim_end_matches(",");
                let selection = selection.trim_end_matches(",");
                // println!("{}",select_value);
                let mut idx = 0;
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                $(
                    if !["asc","desc"].contains(&$order) {
                        panic!("Provide correct order either \"asc\" nor \"desc\"");
                    }
                    let order_ = format!("{} {},",$target,$order);
                    order.push_str(&order_);
                )*
                let and_values = and_values.trim_end_matches("OR ");
                let order = order.trim_end_matches(",");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {};",select_value,$model,and_values,order);
                println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                find_many!(@select $connection,$model,selection,client)
            }
        };
        // * included and or limit skip //completed
        (connection => $connection:expr,
            model:$model:expr,
            select:{
             $($select_value:expr),*
        },
        conditions:{
            or =>  {$($or_value1:expr => $or_value2:expr),*},
            $($and_values:expr => $and_value:expr),*
        },
        limit:$limit:expr,
        skip:$skip:expr
    ) => {
            {
              use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut selection = String::new();
                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*
            let select_value = selectvalue.trim_end_matches(",");
            // println!("{}",select_value);
            let selection = selection.trim_end_matches(",");
            let mut idx = 0;
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            let and_values = and_values.trim_end_matches("OR ");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} LIMIT {} OFFSET {};",select_value,$model,and_values,$limit,$skip);
            println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@select $connection,$model,selection,client)
        }
        };
        // * included and order //completed
        (connection => $connection:expr,
            model:$model:expr,
            select:{
            $($select_value:expr),*
        },
        conditions : {
            $($and_values:expr => $and_value:expr),*
        },
        order : {$($target:expr => $order:expr),*}
    ) => {
            {
              use core::panic;
              use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut order = String::new();
                let mut selection = String::new();
                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*
            let select_value = selectvalue.trim_end_matches(",");
            // println!("{}",select_value);
                            let selection = selection.trim_end_matches(",");
            let mut idx = 0;
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                if !["asc","desc"].contains(&$order) {
                    panic!("Provide correct order either \"asc\" nor \"desc\"");
                }
                let order_ = format!("{} {},",$target,$order);
                order.push_str(&order_);
            )*
            let order = order.trim_end_matches(",");
            let and_values = and_values.trim_end_matches("AND ");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {};",select_value,$model,and_values,order);
            println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@select $connection,$model,selection,client)
            }
        };
        // * included and or order limit skip
        (connection => $connection:expr,
            model:$model:expr,
            select:{
             $($select_value:expr),*
        },
        conditions => {
            or =>  {$($or_value1:expr => $or_value2:expr),*},
            $($and_values:expr => $and_value:expr),*
        },
        order : {$($target:expr => $order:expr),*},
        limit:$limit:expr,
        skip:$skip:expr
    ) => {
            {
              use core::panic;
              use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut order = String::new();
                    let mut selection = String::new();
                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*
            let select_value = selectvalue.trim_end_matches(",");
            // println!("{}",select_value);
                            let selection = selection.trim_end_matches(",");
            let mut idx = 0;
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                if !["asc","desc"].contains(&$order) {
                    panic!("Provide correct order either \"asc\" nor \"desc\"");
                }
                let order_ = format!("{} {},",$target,$order);
                order.push_str(&order_);
            )*
            let order = order.trim_end_matches(",");
            let and_values = and_values.trim_end_matches("OR ");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT {} OFFSET {};",select_value,$model,and_values,order,$limit,$skip);
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@select $connection,$model,selection,client)
        }
        };
        // * included and or order limit
        (connection => $connection:expr,
                    model:$model:expr,
                    select:{
                     $($select_value:expr),*
                },
                conditions => {
                    or =>  {$($or_value1:expr => $or_value2:expr),*},
                    $($and_values:expr => $and_value:expr),*
                },
                order : {$($target:expr => $order:expr),*},
                limit:$limit:expr
        ) => {
                    {
                      use core::panic;
                      use postgres::types::ToSql;

                        let mut selectvalue = String::new();
                        let mut and_values = String::new();
                        let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                        let mut order = String::new();
                        let mut selection = String::new();
                    $(
                        selectvalue.push_str(&format!("{},",$select_value));
                        selection.push_str(&format!("'{}',",$select_value));
                    )*
                    let select_value = selectvalue.trim_end_matches(",");
                    let selection = selection.trim_end_matches(",");
                    // println!("{}",select_value);
                    let mut idx = 0;
                    $(
                        idx+=1;
                        and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                        and_value.push(&$and_value);
                    )*
                    $(
                        idx+=1;
                        and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                        and_value.push(&$and_value);
                    )*
                    $(
                        if !["asc","desc"].contains(&$order) {
                            panic!("Provide correct order either \"asc\" nor \"desc\"");
                        }
                        let order_ = format!("{} {},",$target,$order);
                        order.push_str(&order_);
                    )*
                    let order = order.trim_end_matches(",");
                    let and_values = and_values.trim_end_matches("OR ");
                    // println!("{}",and_values);
                    // println!("{:?}",and_value);
                    let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT {};",select_value,$model,and_values,order,$limit);
                    println!("{}",format);
                    let client = $connection.query(&format,&and_value).unwrap();
                    find_many!(@select $connection,$model,selection,client)
                }
        };
        // * included and or order skip
        (connection => $connection:expr,
            model:$model:expr,
            select:{
             $($select_value:expr),*
        },
        conditions => {
            or =>  {$($or_value1:expr => $or_value2:expr),*},
            $($and_values:expr => $and_value:expr),*
        },
        order : {$($target:expr => $order:expr),*},
        skip:$skip:expr
        ) => {
            {
              use core::panic;
              use postgres::types::ToSql;

                let mut selectvalue = String::new();
                let mut and_values = String::new();
                let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut order = String::new();
                let mut selection = String::new();
                $(
                    selectvalue.push_str(&format!("{},",$select_value));
                    selection.push_str(&format!("'{}',",$select_value));
                )*
            let select_value = selectvalue.trim_end_matches(",");
            // println!("{}",select_value);
                            let selection = selection.trim_end_matches(",");
            let mut idx = 0;
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                if !["asc","desc"].contains(&$order) {
                    panic!("Provide correct order either \"asc\" nor \"desc\"");
                }
                let order_ = format!("{} {},",$target,$order);
                order.push_str(&order_);
            )*
            let order = order.trim_end_matches(",");
            let and_values = and_values.trim_end_matches("OR ");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} OFFSET {};",select_value,$model,and_values,order,$skip);
            println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@select $connection,$model,selection,client)
        }
        };
}

#[macro_export]
macro_rules! delete_many {
    // * all rows
    (connection => $connection:expr,model:$model:expr) => {{
        let delete = format!("DELETE FROM {};", $model);
        println!("{}", delete);
        $connection.execute(&delete, &[])
    }};
}

#[macro_export]
macro_rules! delete {
  (@select $connection:expr,$model:expr,$selection:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}' AND column_name IN ({})",$model,$selection);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in $client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
    // * and
    (connection => $connection:expr,model:$model:expr,conditions:{
        $($condition:expr => $like:expr),*
    }) => {{
        use postgres::types::ToSql;

        let mut delete = String::new();
        let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx+=1;
            let query = format!("{} = ${} AND ",$condition,idx);
            delete.push_str(&query);

            values.push(&$like);
        )*
        let delete  = delete.trim_end_matches("AND ");
        let delete = format!("DELETE FROM {} WHERE {};",$model,delete);
        // println!("{}",delete);
        $connection.execute(&delete,&values)
    }};
    // * and select
    (connection => $connection:expr,
        model:$model:expr,
        conditions:{
        $($condition:expr => $like:expr),*
    },select:{
        $($value:expr)*
    }) => {{
        use postgres::types::ToSql;

        let mut delete = String::new();
        let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        let mut value = String::new();
        let mut selection = String::new();
        $(
            idx+=1;
            let query = format!("{} = ${} AND ",$condition,idx);
            delete.push_str(&query);

            values.push(&$like);
        )*
        $(
            value.push_str(&format!("{},",$value));
            selection.push_str(&format!("'{}',",$value));
        )*
        let value = value.trim_end_matches(",");
        let selection = selection.trim_end_matches(",");
        let delete  = delete.trim_end_matches("AND ");
        let delete = format!("DELETE FROM {} WHERE {} RETURNING {};",$model,delete,value);
        // println!("{}",delete);
        let client = $connection.query(&delete,&values).unwrap();
        // println!("{:?}",client);
        delete!(@select $connection,$model,selection,client)

    }};
    // * and cascade
    (connection => $connection:expr,model:$model:expr,conditions:{
            $($condition:expr => $like:expr),*
    },cascade:$casecade:expr) => {{
      use postgres::types::ToSql;

            let mut delete = String::new();
            let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut idx = 0;
            $(
                idx+=1;
                let query = format!("{} = ${} AND ",$condition,idx);
                delete.push_str(&query);

                values.push(&$like);
            )*
            let delete  = delete.trim_end_matches("AND ");
            if $casecade != "true" {
                panic!("{}","Provide boolean value for the casecade")
            }
            let delete = format!("DELETE FROM {} WHERE {}CASCADE;",$model,delete);
            println!("{}",delete);
            $connection.execute(&delete,&values)
    }};
    // * or
    (connection => $connection:expr,
        model:$model:expr,
        conditions => {
            or => {
                $($condition:expr => $like:expr),*
            }
    }) => {{
      use postgres::types::ToSql;

            let mut delete = String::new();
            let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut idx = 0;
            $(
                idx+=1;
                let query = format!("{} = ${} OR ",$condition,idx);
                delete.push_str(&query);

                values.push(&$like);
            )*
            let delete  = delete.trim_end_matches("OR ");
            let delete = format!("DELETE FROM {} WHERE {};",$model,delete);
            println!("{}",delete);
            $connection.execute(&delete,&values)
    }};
    // * or select
    (connection => $connection:expr,
            model:$model:expr,
            conditions => {
            or => {$($condition:expr => $like:expr),*}
        },select:{
            $($value:expr)*
    }) => {{
      use postgres::types::ToSql;

            let mut delete = String::new();
            let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut idx = 0;
            let mut value = String::new();
            let mut selection = String::new();
            $(
                idx+=1;
                let query = format!("{} = ${} OR ",$condition,idx);
                delete.push_str(&query);

                values.push(&$like);
            )*
            $(
                value.push_str(&format!("{},",$value));
                selection.push_str(&format!("'{}',",$value));
            )*
            let value = value.trim_end_matches(",");
            let selection = selection.trim_end_matches(",");
            let delete  = delete.trim_end_matches("OR ");
            let delete = format!("DELETE FROM {} WHERE {} RETURNING {};",$model,delete,value);
            println!("{}",delete);
            let client = $connection.query(&delete,&values).unwrap();
            delete!(@select $connection,$model,selection,client)

    }};
    // * or cascase
    (connection => $connection:expr,model:$model:expr,conditions:{
        $($condition:expr => $like:expr),*
    },cascade:$casecade:expr) => {{

      use postgres::types::ToSql;

        let mut delete = String::new();
        let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx+=1;
            let query = format!("{} = ${} OR ",$condition,idx);
            delete.push_str(&query);

            values.push(&$like);
        )*
        let delete  = delete.trim_end_matches("OR ");
        if $casecade != "true" {
            panic!("{}","Provide boolean value for the casecade")
        }
        let delete = format!("DELETE FROM {} WHERE {}CASCADE;",$model,delete);
        println!("{}",delete);
        $connection.execute(&delete,&values)
    }};
}

#[macro_export]
// * data conditions
macro_rules! update {
  (@select $connection:expr,$model:expr,$selection:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}' AND column_name IN ({})",$model,$selection);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in $client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
    (connection => $connection:expr,model : $model:expr,data:{
        $($from:expr => $data:expr),*
    },conditions:{
        $($conditions:expr => $value:expr),*
    }) => {{
      use postgres::types::ToSql;

        let mut data = String::new();
        let mut conditions = String::new();
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx += 1;
            let set = format!("{} = ${},",$from,idx);
            data.push_str(&set);

            value.push(&$data);
        )*
        $(
            idx += 1;
            let condition = format!("{} = ${} AND ",$conditions,idx);
            conditions.push_str(&condition);

            value.push(&$value);
        )*
        let conditions = conditions.trim_end_matches("AND ");
        let data = data.trim_end_matches(",");
        let update = format!("UPDATE {} SET {} WHERE {};\r\n",$model,data,conditions);
        // println!("{}",update);
        // println!("{:?}",value);
        $connection.execute(&update,&value)
    }};
    // * data
    (connection => $connection:expr,model : $model:expr,data:{
        $($from:expr => $data:expr),*
    }) => {{
      use postgres::types::ToSql;

        let mut data = String::new();
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx += 1;
            let set = format!("{} = ${},",$from,idx);
            data.push_str(&set);

            value.push(&$data);
        )*
        let data = data.trim_end_matches(",");
        let update = format!("UPDATE {} SET {};",$model,data);
        // println!("{}",update);
        // println!("{:?}",value);
        $connection.execute(&update,&value)
    }};
    // * data select conditions
    (connection => $connection:expr,model : $model:expr,
      select:{
        $($select:expr),*
    },
      data:{
        $($from:expr => $data:expr),*
    },conditions:{
        $($conditions:expr => $value:expr),*
    }) => {{
      use postgres::types::ToSql;

        let mut data = String::new();
        let mut conditions = String::new();
        let mut values: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut select = String::new();
        let mut selection = String::new();
        let mut idx = 0;
        $(
            idx += 1;
            let set = format!("{} = ${},",$from,idx);
            data.push_str(&set);

            values.push(&$data);
        )*
        $(
            idx += 1;
            let condition = format!("{} = ${} AND ",$conditions,idx);
            conditions.push_str(&condition);

            values.push(&$value);
        )*
        $(
            let value = format!("{},",$select);
            select.push_str(&value);

            let value = format!("'{}',",$select);
            selection.push_str(&value);
        )*
        let conditions = conditions.trim_end_matches("AND ");
        let data = data.trim_end_matches(",");
        let select = select.trim_end_matches(",");
        let selection = selection.trim_end_matches(",");
        let update = format!("UPDATE {} SET {} WHERE {} RETURNING {};",$model,data,conditions,select);
        // println!("{}",update);
        // println!("{:?}",values);
        let client = $connection.query(&update,&values).unwrap();
        update!(@select $connection,$model,selection,client)
    }};
}

#[macro_export]
macro_rules! create {
  (@all $connection:expr,$model:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}';",$model);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
  (@select $connection:expr,$model:expr,$selection:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}' AND column_name IN ({})",$model,$selection);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in $client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
    (connection => $connection:expr,model:$model:expr,data:{
        $($from:expr => $data:expr),*
    }) => {
        {

          use postgres::types::ToSql;

        let mut data = String::new();
        let mut data_value = String::new();
        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx += 1;
            let create = format!("{},",$from);
            data.push_str(&create);

            let datavalue = format!("${},",idx);
            data_value.push_str(&datavalue);

            value.push(&$data);
        )*
        let data = data.trim_end_matches(",");
        let data_value = data_value.trim_end_matches(",");
        let create = format!("INSERT INTO {} ({}) VALUES ({});",$model,data,data_value);
        // println!("{}",create);
        // println!("{:?}",value);
        $connection.execute(&create,&value)
    }
    };
    (connection => $connection:expr,model:$model:expr,data:{
        $($from:expr => $data:expr),*
    },
    select:{
        $($select_value:expr),*
    }) => {
        {
          use postgres::types::ToSql;

            let mut data = String::new();
        let mut data_value = String::new();
        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut select_value = String::new();
        let mut selection = String::new();
        let mut idx = 0;
        $(
            idx += 1;
            let create = format!("{},",$from);
            data.push_str(&create);

            let datavalue = format!("${},",idx);
            data_value.push_str(&datavalue);

            value.push(&$data);
        )*
        $(
            select_value.push_str(&format!("{},",$select_value));
            selection.push_str(&format!("'{}',",$select_value));
        )*
        let data = data.trim_end_matches(",");
        let data_value = data_value.trim_end_matches(",");
        let select = select_value.trim_end_matches(",");
        let selection = selection.trim_end_matches(",");
        let create = format!("INSERT INTO {} ({}) VALUES ({}) RETURNING {}",$model,data,data_value,select);
        // println!("{}",create);
        // println!("{:?}",value);

    let client = $connection.query(&create,&value).unwrap();
    create!(@select $connection,$model,selection,client)
    }
    }
}

#[macro_export]
macro_rules! transaction {
    (connection => $connection:expr,begin) => {
        let begin = $connection.execute("BEGIN;", &[]).unwrap();
        // println!("{}", begin);
    };
    (connection => $connection:expr,commit) => {
        let commit = $connection.execute("COMMIT;", &[]).unwrap();
        // println!("{}", commit);
    };
}

#[macro_export]
macro_rules! show_index_list {
    (connection => $connection:expr,model:$model:expr) => {{
        use std::collections::HashMap;

        let mut models = HashMap::new();
        let mut show = Vec::new();
        // let mut show = show.clone();
        let index = format!("SELECT * from pg_indexes where tablename = '{}'", $model);
        let result = $connection.query(&index, &[]).unwrap();
        for result in result.iter() {
            let mut models = models.clone();
            // let first: String = result.get(0);
            let second: String = result.get(1);
            let third: String = result.get(2);
            // println!("{} = {}", second, third);
            models.insert(second, third);
            show.push(models);
        }
        show
    }};
}

#[macro_export]
macro_rules! create_index {
    (connection => $connection:expr,model:$model:expr,name:$name:expr,index:{$($value:expr),*}) => {
        {
            let mut index = String::new();
        $(
            index.push_str(&format!("{},",$value));
        )*
        let index = index.trim_end_matches(",");
        let value = format!("CREATE INDEX index_{} ON {} ({})",$name,$model,index);
        println!("{}",value);
        $connection.execute(&value,&[])
    }
    };
}

#[cfg(feature = "brin_index")]
/// Used for time
#[macro_export]
macro_rules! create_brin_index {
    (connection => $connection:expr,model:$model:expr,name:$name:expr,index:{$($value:expr),*}) => {
        {
            let mut index = String::new();
        $(
            index.push_str(&format!("{},",$value));
        )*
        let index = index.trim_end_matches(",");
        let value = format!("CREATE INDEX index_{} ON {} USING brin({})",$name,$model,index);
        // println!("{}",value);
        $connection.execute(&value,&[])
    }
    };
}

#[cfg(feature = "gin_index")]
/// Used for json search
#[macro_export]
macro_rules! create_gin_index {
    (connection => $connection:expr,
        model:$model:expr,
        full_text:{
            name:$name:expr,index:{$($value:expr),*}
        }) => {
        {
            let mut index = String::new();
        $(
            index.push_str(&format!("{},",$value));
        )*
        let index = index.trim_end_matches(",");
        let value = format!("CREATE INDEX index_{} ON {} USING gin(to_tsvector('english',{}))",$name,$model,index);
        // println!("{}",value);
        $connection.execute(&value,&[])
    }
    };
    (connection => $connection:expr,
        model:$model:expr,
        pattern_match:{
            name:$name:expr,index:{$($value:expr),*}
        }) => {
        {
            let mut index = String::new();
        $(
            index.push_str(&format!("{},",$value));
        )*
        let index = index.trim_end_matches(",");
        let extension = format!("CREATE EXTENSION IF NOT EXISTS pg_trgm;");
        let value = format!("CREATE INDEX index_{} ON {} USING gin({} gin_trgm_ops);",$name,$model,index);
        // println!("{}",value);
        $connection.execute(&extension,&[]);
        $connection.execute(&value,&[])
    }
    };
}

/// search
#[cfg(feature = "similar_search")]
#[macro_export]
macro_rules! similar_search {
    (connection => $connection:expr,
        model:$model:expr,
        similarity:{
        score:$score:expr,
        believe:$believe:expr,
        text:$text:expr
    },
    order_by:{
        order:$order:expr,
        believe:$o_believe:expr,
        text:$o_text:expr
    }
) => {{
        use std::collections::{BTreeMap, HashMap};
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use uuid::Uuid;
        use core::panic;

        let search = format!(
            "SELECT * FROM {} WHERE similarity({},'{}') > {} ORDER BY similarity({},'{}') {} ",
            $model, $believe, $text, $score, $o_believe, $o_text, $order
        );
        // println!("{}", search);
        let create = $connection.query(&search, &[]).unwrap();
        let select  = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}';",$model);
        // println!("{:?}",create);

        let query = $connection.query(&select, &[]).unwrap();
        // println!("{:?}",query);
        let mut map = HashMap::new();
        for r in query.iter() {
            let first: String = r.get(0);
            let second: String = r.get(1);
            map.insert(first, second);
        }
        // println!("{:?}", map);
        let mut bucks = vec![];
        let mut total_bucks = Vec::new();
        for (idx, column) in create.iter().enumerate() {
            let buck = BTreeMap::new();
            for i in map.iter() {
                let value = i.0.to_string();
                let value_ = value.as_str();
                let kind = i.1;
                let mut buck = buck.clone();
                let value__ = match kind.as_ref() {
                    "time without time zone" => {
                        let name: NaiveTime = column.get(value_);
                        name.to_string()
                    }
                    "uuid" => {
                        let uuid: Uuid = column.get(value_);
                        uuid.to_string()
                    }
                    "boolean" => {
                        let value: bool = column.get(value_);
                        value.to_string()
                    }
                    "integer" => {
                        let intger: i32 = column.get(value_);
                        intger.to_string()
                    }
                    "timestamp without time zone" => {
                        let datetime: NaiveDateTime = column.get(value_);
                        datetime.to_string()
                    }
                    "date" => {
                        let date: NaiveDate = column.get(value_);
                        date.to_string()
                    }
                    "text" => {
                        let string = column.get(value_);
                        string
                    }
                    _ => panic!("{}", kind),
                };
                buck.insert(value.clone(), value__.clone());
                bucks.push(buck.clone());
            }
            // println!("{:?}", bucks);
            total_bucks.push(bucks.clone());
            bucks.clear(); // earase the prev bucks
        }
        total_bucks
    }};
}

// macro_rules! geo_loc_search {
//     (connection => $connection:expr,mode:$mode:expr,radius:$radius:expr,) => {};
// }

#[macro_export]
macro_rules! table_structure {
    (connection =>  $connection:expr,model:$model:expr) => {{
      use std::collections::HashMap;

        let table = format!(
            "SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}';",
            $model
        );
        // println!("{}",table);
        let client = $connection.query(&table, &[]).unwrap();
        // println!("{:?}",client);
        let mut hashmap = HashMap::new();
        for client in client.iter() {
            let first: String = client.get(0);
            let second: String = client.get(1);
            // println!("{} = {}",first,second);
            hashmap.insert(first, second);
        }
        hashmap
    }};
}

#[cfg(feature = "count")]
#[macro_export]
macro_rules! count {
    (connection => $connection:expr,model:$model:expr) => {{
      use std::collections::HashMap;

        let count = format!("SELECT COUNT(*) FROM {};", $model);
        let result = $connection.query(&count, &[]).unwrap();
        let mut z = HashMap::new();
        for count in result.iter() {
            let count: i64 = count.get(0);
            z.insert("count", count);
        }
        z
    }};
    (connection => $connection:expr,model:$model:expr,conditions:{
        $($conditions:expr => $value:expr)*
    }) => {{
      use std::collections::HashMap;
      use postgres::types::ToSql;

        let mut condition = String::new();
        let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx += 1;
            condition.push_str(&format!("{} = ${}",$conditions,idx));
            values.push(&$value);
        )*
        let count = format!("SELECT COUNT(*) FROM {} WHERE {};", $model,condition);
        let result = $connection.query(&count, &values).unwrap();
        let mut z = HashMap::new();
        for count in result.iter() {
            let count: i64 = count.get(0);
            z.insert("count", count);
        }
        // z.insert("c","c");
        z
    }};
    (connection => $connection:expr,model:$model:expr,count:{
       $($value:expr),*
    }) => {{
      use std::collections::HashMap;
        let mut value = String::new();
        // // let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx += 1;
            value.push_str(&format!("{},",&$value));
        )*
        let value = value.trim_end_matches(",");
        let count = format!("SELECT COUNT(DISTINCT({})) FROM {};", value,$model);
        // println!("{}",count);
        let result = $connection.query(&count, &[]).unwrap();
        let mut z = HashMap::new();
        for count in result.iter() {
            let count: i64 = count.get(0);
            z.insert("count", count);
        }
        // z.insert("c","c");
        z
    }};
    (connection => $connection:expr,model:$model:expr,count:{
        $($count_value:expr),*
     },conditions:{
        $($condition:expr => $value:expr),*
     }) => {{
      use std::collections::HashMap;
      use postgres::types::ToSql;

         let mut value = String::new();
         let mut conditions = String::new();
         let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
         let mut idx = 0;
         $(
             value.push_str(&format!("{},",&$count_value));
         )*
         $(
            idx += 1;
            conditions.push_str(&format!("{}=${}",$condition,idx));
            values.push(&$value);
         )*
         let value = value.trim_end_matches(",");
         let count = format!("SELECT COUNT(DISTINCT({})) FROM {} WHERE {};", value,$model,conditions);
        //  println!("{}",count);
        //  println!("{:?}",values);
         let result = $connection.query(&count, &values).unwrap();
        //  println!("{:?}",result);
         let mut z = HashMap::new();
         for count in result.iter() {
             let count: i64 = count.get(0);
             z.insert("count", count);
         }
        //  z.insert("c","c");
         z
     }};

    // (connection => $connection:expr,model:$model:expr,group:{
    //     $($value:expr),*
    //  }) => {{
    //      let mut value = String::new();
    //      // // let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
    //      let mut idx = 0;
    //      $(
    //          idx += 1;
    //          value.push_str(&format!("{},",&$value));
    //      )*
    //      let value = value.trim_end_matches(",");
    //      let count = format!("SELECT {} ,COUNT(*) FROM {} GROUP BY {};", value,$model,value);
    //     //  println!("{}",count);
    //      let result = $connection.query(&count, &[]).unwrap();
    //      let mut z = HashMap::new();
    //      for count in result.iter() {
    //          let value: String = count.get(0);
    //          let counts: i32 = count.get(1);
    //          let count: i64 = count.get(2);
    //         println!("{}",count);
    //          z.insert(value, counts);
    //      }
    //      // z.insert("c","c");
    //      z
    //  }};
}

#[cfg(feature = "pagination")]
#[macro_export]
macro_rules! pagination {
  (@all $connection:expr,$model:expr,$client:expr) => {
    {
      use postgres::types::ToSql;
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}';",$model);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in $client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
  (@select $connection:expr,$model:expr,$selection:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}' AND column_name IN ({})",$model,$selection);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in $client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks}
  };
    (connection => $connection:expr,model:$model:expr,take:$take:expr,skip:$skip:expr,select:{$($select:expr),*},conditions:{$($condition:expr => $value:expr)*}) => {
        {
          use postgres::types::ToSql;

        let mut select = String::new();
        let mut condition = String::new();
        let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut selection = String::new();
        let mut idx = 0;
        $(
            select.push_str(&format!("{},",$select));
            selection.push_str(&format!("'{}',",$select));
        )*
        $(
            idx+=1;
            condition.push_str(&format!("{} = ${}",$condition,idx));
            values.push(&$value);
        )*
        let select = select.trim_end_matches(",");
        let selection = selection.trim_end_matches(",");
        let condition = condition.trim_end_matches(",");
        let pagination = format!("SELECT {} FROM {} WHERE {} LIMIT {} OFFSET {};",select,$model,condition,$take,$skip);
        // println!("{}",pagination);
        // println!("{:?}",values);
        let client = $connection.query(&pagination,&values).unwrap();
        // println!("{:?}",client);
        pagination!(@select $connection,$model,selection,client)
    }
    };
    (connection => $connection:expr,model:$model:expr,take:$take:expr,skip:$skip:expr,conditions:{$($condition:expr => $value:expr)*}) => {
        {
          use postgres::types::ToSql;

        let mut condition = String::new();
        let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx+=1;
            condition.push_str(&format!("{} = ${} AND ",$condition,idx));
            values.push(&$value);
        )*

        let condition = condition.trim_end_matches("AND ");
        let pagination = format!("SELECT * FROM {} WHERE {} LIMIT {} OFFSET {};",$model,condition,$take,$skip);
        // println!("{}",pagination);
        // println!("{:?}",values);
        let client = $connection.query(&pagination,&values).unwrap();
        pagination!(@all $connection,$model,client)
    }
    };
    (connection => $connection:expr,model:$model:expr,take:$take:expr,skip:$skip:expr,
        search:{$($search_value:expr => $search:expr),*},conditions:{$($condition:expr => $value:expr)*}) => {
        {
          use postgres::types::ToSql;

            let mut search = String::new();
        let mut condition = String::new();
        let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut idx = 0;
        $(
            idx+=1;
            condition.push_str(&format!("{} = ${} AND ",$condition,idx));
        )*
        $(
            values.push(&$value);
        )*
        $(
            values.push(&$search);
        )*
        $(
            idx +=1;
            search.push_str(&format!("CAST({} AS TEXT) ILIKE '%' || ${} || '%' OR ",$search_value,idx));
        )*
        let mut search = search.trim_end_matches("OR ");
        condition.push_str(&format!("{}",search));
        let pagination = format!("SELECT * FROM {} WHERE {} LIMIT {} OFFSET {};",$model,condition,$take,$skip);
                // println!("{}",pagination);
                // println!("{:?}",values);
        let client = $connection.query(&pagination,&values).unwrap();
        pagination!(@all $connection,$model,client)
    }
    };
    (connection => $connection:expr,model:$model:expr,take:$take:expr,skip:$skip:expr,
        search:{$($search_value:expr => $search:expr),*},select:{$($select:expr),*},conditions:{$($condition:expr => $value:expr)*}) => {
        {
          use postgres::types::ToSql;

            let mut search = String::new();
        let mut condition = String::new();
        let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut select = String::new();
        let mut selection = String::new();
        let mut idx = 0;
        $(
            idx+=1;
            condition.push_str(&format!("{} = ${} AND ",$condition,idx));
        )*
        $(
            values.push(&$value);
        )*
        $(
            values.push(&$search);
        )*
        $(
            select.push_str(&$select);
        )*
        $(
            selection.push_str(&format!("'{}',",&$select));
        )*
        $(
            idx +=1;
            search.push_str(&format!("CAST({} AS TEXT) ILIKE '%' || ${} || '%' OR ",$search_value,idx));
        )*
        let mut search = search.trim_end_matches("OR ");
        let select = select.trim_end_matches(",");
        let selection = selection.trim_end_matches(",");
        condition.push_str(&format!("{}",search));
        let pagination = format!("SELECT * FROM {} WHERE {} LIMIT {} OFFSET {};",$model,condition,$take,$skip);
                // println!("{}",pagination);
                // println!("{:?}",values);
        let client = $connection.query(&pagination,&values).unwrap();
        pagination!(@select $connection,$model,selection,client)
    }
    }
}

#[cfg(feature = "full_search")]
#[macro_export]
macro_rules! full_search {
  // (@all $connection:expr,$model:expr,$client:expr) => {};
  // (@select $connection,$model:expr,$selection:expr,$client:expr) => {};
  (@all $connection:expr,$model:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}';",$model);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks
  }
  };
  (@select $connection:expr,$model:expr,$selection:expr,$client:expr) => {
    {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

      let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}' AND column_name IN ({})",$model,selection);
    // println!("{}",selected);
    let query = $connection.query(&selected, &[]).unwrap();
    // println!("{:?}",query);
    let mut map = HashMap::new();
    for r in query.iter() {
        let first: String = r.get(0);
        let second: String = r.get(1);
        map.insert(first, second);
    }
    // println!("{:?}", map);
    let mut bucks = vec![];
    let mut total_bucks = Vec::new();
    for (idx, column) in client.iter().enumerate() {
        let buck = BTreeMap::new();
        for i in map.iter() {
            let value = i.0.to_string();
            let value_ = value.as_str();
            let kind = i.1;
            let mut buck = buck.clone();
            let value__ = match kind.as_ref() {
                "time without time zone" => {
                    let name: NaiveTime = column.get(value_);
                    name.to_string()
                }
                "uuid" => {
                    let uuid: Uuid = column.get(value_);
                    uuid.to_string()
                }
                "boolean" => {
                    let value: bool = column.get(value_);
                    value.to_string()
                }
                "integer" => {
                    let intger: i32 = column.get(value_);
                    intger.to_string()
                }
                "timestamp without time zone" => {
                    let datetime: NaiveDateTime = column.get(value_);
                    datetime.to_string()
                }
                "date" => {
                    let date: NaiveDate = column.get(value_);
                    date.to_string()
                }
                "text" => {
                    let string = column.get(value_);
                    string
                }
                _ => panic!("{}", kind),
            };
            buck.insert(value.clone(), value__.clone());
            bucks.push(buck.clone());
        }
        total_bucks.push(bucks.clone());
        bucks.clear(); // earase the prev bucks
    }
    total_bucks
  }
  };
    (connection => $connection:expr,model:$model:expr,based_on:$search:expr,search:{
        value:$value:expr
    }) => {{
        let rank = format!(
            "SELECT * FROM {} WHERE to_tsvector('english',CAST({} AS TEXT)) @@  to_tsquery(CAST('{}' AS TEXT))",
            $model, $search, $value
        );
        println!("{}",rank);
        let client = $connection.query(&rank, &[]).unwrap();
        full_search!(@all $connection,$model,client)
    }};
    (connection => $connection:expr,model:$model:expr,based_on:$search:expr,search:{
        value:$value:expr
    },select:{
        $($select:expr),*
    }) => {{
        let mut select = String::new();
        let mut selection = String::new();
        $(
            select.push_str(&format!("{},",$select));
            selection.push_str(&format!("'{}',",$select));
        )*
        let select = select.trim_end_matches(",");
        let selection = selection.trim_end_matches(",");
        let rank = format!(
            "SELECT {} FROM {} WHERE to_tsvector('english',CAST({} AS TEXT)) @@  to_tsquery(CAST('{}' AS TEXT))",
            select,$model, $search, $value
        );
        // println!("{}",rank);
        let client = $connection.query(&rank, &[]).unwrap();
        full_search!(@select $connection,$model,selection,client)
    }};
}

#[cfg(feature = "ranked_search")]
#[macro_export]
macro_rules! ranked_search {
    (@format $model:expr,$connection:expr,$client:expr) => {
      {
      use std::collections::{BTreeMap, HashMap};
      use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
      use uuid::Uuid;
      use core::panic;

        let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}';",$model);
      // println!("{}",selected);
      let query = $connection.query(&selected, &[]).unwrap();
      // println!("{:?}",query);
      let mut map = HashMap::new();
      for r in query.iter() {
          let first: String = r.get(0);
          let second: String = r.get(1);
          map.insert(first, second);
      }
      // println!("{:?}", map);
      let mut bucks = vec![];
      let mut total_bucks = Vec::new();
      for (idx, column) in $client.iter().enumerate() {
          let total = column.len();
          let score:f32 = column.get(total-1);
          // bucks.push(score);
          for i in map.iter() {
              let value = i.0.to_string();
              let value_ = value.as_str();
              let kind = i.1;
              let value__ = match kind.as_ref() {
                  "time without time zone" => {
                      let name: NaiveTime = column.get(value_);
                      name.to_string()
                  }
                  "uuid" => {
                      let uuid: Uuid = column.get(value_);
                      uuid.to_string()
                  }
                  "boolean" => {
                      let value: bool = column.get(value_);
                      value.to_string()
                  }
                  "integer" => {
                      let intger: i32 = column.get(value_);
                      intger.to_string()
                  }
                  "timestamp without time zone" => {
                      let datetime: NaiveDateTime = column.get(value_);
                      datetime.to_string()
                  }
                  "date" => {
                      let date: NaiveDate = column.get(value_);
                      date.to_string()
                  }
                  "text" => {
                      let string = column.get(value_);
                      string
                  }
                  _ => panic!("{}", kind),
              };
              let rank = Score {
                  score:score,
                  data:value__
              };
              bucks.push(rank);
          }
          total_bucks.push(bucks.clone());
          bucks.clear(); // earase the prev bucks
      }
      total_bucks}
    };
    (@select $connection:expr,$model:expr,$selection:expr,$client:expr) => {
      {
        use std::collections::{BTreeMap, HashMap};
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use uuid::Uuid;
        use core::panic;

        let selected = format!("SELECT column_name,data_type FROM information_schema.columns WHERE table_name = '{}' AND column_name IN ({})",$model,$selection);
      // println!("{}",selected);
      let query = $connection.query(&selected, &[]).unwrap();
      // println!("{:?}",query);
      let mut map = HashMap::new();
      for r in query.iter() {
          let first: String = r.get(0);
          let second: String = r.get(1);
          map.insert(first, second);
      }
      // println!("{:?}", map);
      let mut bucks = vec![];
      let mut total_bucks = Vec::new();
      for (idx, column) in $client.iter().enumerate() {
          let score:f32 = column.get(1);
          // bucks.push(score);
          for i in map.iter() {
              let value = i.0.to_string();
              let value_ = value.as_str();
              let kind = i.1;
              let value__ = match kind.as_ref() {
                  "time without time zone" => {
                      let name: NaiveTime = column.get(value_);
                      name.to_string()
                  }
                  "uuid" => {
                      let uuid: Uuid = column.get(value_);
                      uuid.to_string()
                  }
                  "boolean" => {
                      let value: bool = column.get(value_);
                      value.to_string()
                  }
                  "integer" => {
                      let intger: i32 = column.get(value_);
                      intger.to_string()
                  }
                  "timestamp without time zone" => {
                      let datetime: NaiveDateTime = column.get(value_);
                      datetime.to_string()
                  }
                  "date" => {
                      let date: NaiveDate = column.get(value_);
                      date.to_string()
                  }
                  "text" => {
                      let string = column.get(value_);
                      string
                  }
                  _ => panic!("{}", kind),
              };
              let rank = Score {
                  score:score,
                  data:value__
              };
              bucks.push(rank);
          }
          total_bucks.push(bucks.clone());
          bucks.clear(); // earase the prev bucks
      }
      total_bucks}
    };
    (connection => $connection:expr,model:$model:expr,based_on:$search:expr,search:{
        value:$value:expr
    }) => {{
        #[derive(Debug,Clone)]
        pub struct Score {
            score:f32,
            data:String
        };
        let rank = format!(
            "SELECT * , ts_rank_cd(to_tsvector('english',{}),to_tsquery('{}')) FROM {} WHERE to_tsvector('english',CAST({} AS TEXT)) @@  to_tsquery(CAST('{}' AS TEXT))",
            $search,$value,$model, $search, $value
        );
        let client = $connection.query(&rank, &[]).unwrap();
        ranked_search!(@format client,$model,$connection)
    }};
    (connection => $connection:expr,model:$model:expr,based_on:$search:expr,search:{
        value:$value:expr
    },select:{
        $($select:expr),*
    }) => {{
        #[derive(Debug,Clone)]
        pub struct Score {
            score:f32,
            data:String
        };
        let mut select = String::new();
        let mut selection = String::new();
        $(
            select.push_str(&format!("{},",$select));
            selection.push_str(&format!("'{}',",$select));
        )*
        let select = select.trim_end_matches(",");
        let selection = selection.trim_end_matches(",");
        let rank = format!(
            "SELECT {} , ts_rank_cd(to_tsvector('english',{}),to_tsquery('{}')) FROM {} WHERE to_tsvector('english',CAST({} AS TEXT)) @@  to_tsquery(CAST('{}' AS TEXT))",
            select,$search,$value,$model, $search, $value
        );
        // println!("{}",rank);
        let client = $connection.query(&rank, &[]).unwrap();
        ranked_search!(@select $connection,$model,selection,client)
    }};
}

#[cfg(feature = "partition")]
#[macro_export]
macro_rules! create_partition {
    (connection => $connection:expr,model:$model:expr,name:$name:expr,field:$field:expr) => {{
        let partition = format!(
            "CREATE TABLE {} PARTITION OF {} FOR VALUES IN (\'{}\')",
            $name, $model, $field
        );
        println!("{}", partition);
        $connection.execute(&partition, &[])
    }};
}

// #[macro_export]
// macro_rules! partitions {
//     (connection => $connection:expr,model:$model:expr,value:$value:expr,partition:$partition:expr) => {{
//         let partition = format!(
//             "CREATE TABLE {} PARTITION OF {} FOR VALUES IN ('{}');",
//             $value, $model, $partition
//         );
//         println!("{}", partition);
//         $connection.execute(&partition, &[])
//     }};
// }

/// (connection => $connection:expr,
/// model:$model:expr,
/// name:$name:expr,
/// based_on:{$based_on:expr => $value:expr}) => {}
#[cfg(feature = "horizontal_split")]
#[macro_export]
macro_rules! horizontal_splitting {
    (connection => $connection:expr,model:$model:expr,name:$name:expr,based_on:{$based_on:expr => $value:expr}) => {{
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        let horizontal = format!(
            "CREATE TABLE {} AS SELECT * FROM {} WHERE {} = '{}';",
            $name, $model, $based_on, $value
        );
        // println!("{}", horizontal);
        let function = format!(
            "CREATE OR REPLACE FUNCTION copy_to_db()\r\n\
        RETURNS TRIGGER AS $$\r\n\
        BEGIN\r\n\
           IF NEW.{} = '{}' THEN\r\n\
                INSERT INTO {} VALUES (NEW.*);\r\n\
            END IF;\r\n\
            RETURN NEW;\r\n\
        END;\r\n\
        $$ LANGUAGE plpgsql;
         ",
            $based_on, $value, $model
        );
        let story: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();
        let trigger = format!(
            "CREATE TRIGGER insert_to_story_{}\r\n\
        AFTER INSERT ON {}\r\n\
        FOR EACH ROW\r\n\
        EXECUTE FUNCTION copy_to_db();",
            story, $model
        );
        // println!("{}", trigger);
        $connection.execute(&horizontal, &[]).unwrap();
        $connection.execute(&trigger, &[]).unwrap();
        $connection.execute(&function, &[]).unwrap();
    }};
}

#[macro_export]
macro_rules! custome_query {
    (connection => $connection:expr,query:{$query:expr}) => {{
        let format = format!("{}", query);
        $connection.query(&format, &[])
    }};
    (connection => $connection:expr,query:{$query:expr,value:{$($value:expr),*}}) => {{
        use postgres::types::ToSql;

        let format = format!("{}", query);
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();
        $connection.query(&format, &value)
    }};
}

#[macro_export]
macro_rules! custome_execute {
    (connection => $connection:expr,query:{$query:expr}) => {
        let format = format!("{}", query);
        $connection.execute(&format, &[])
    };
    (connection => $connection:expr,query:{$query:expr,value:{$($value:expr),*}}) => {
        use postgres::types::ToSql;

        let format = format!("{}", query);
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();
        $connection.execute(&format, &value)
    };
}
