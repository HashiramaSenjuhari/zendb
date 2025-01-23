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
            let mut index_relation = String::new();
            let mut id = String::new();
            let mut unique = String::new();
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
                            if value == ("ID(UUID)") {
                                // primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                table.push_str("UUID ");
                                id.push_str("UUID");
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                // table.push_str("PRIMARY KEY ");
                                table.push_str("DEFAULT uuid_generate_v4() ");
                            }
                            else if value == ("ID(CUID)"){
                                table.push_str("TEXT ");
                                id.push_str("TEXT ");
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                // table.push_str("PRIMARY KEY ");
                                table.push_str("DEFAULT encode(gen_random_bytes(12),'hex') ");
                            }
                            else if value == ("ID(AUTO)") {
                                id.push_str("INT ");
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                table.push_str("INT GENERATED ALWAYS AS IDENTITY ");
                                // table.push_str("PRIMARY KEY ");
                            }
                            else if value == ("ID(BIGINT)") {
                                id.push_str("BIGINT ");
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                table.push_str("BIGINT ");
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
                            if id.contains("UUID") {
                                let line = format!("UUID UNIQUE ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ,\r\n",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }
                            // * AUTO
                            else if id.contains("INT") {
                                let line = format!("INT UNIQUE ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ,\r\n",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }

                            // * CUID
                            else if id.contains("TEXT "){
                                let line = format!("TEXT UNIQUE ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ,\r\n",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }
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
                            if id.contains("UUID") {
                                let line = format!("UUID ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ,\r\n",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }
                            // * AUTO
                            else if id.contains("INT") {
                                let line = format!("INT ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ,\r\n",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }

                            // * CUID
                            else if id.contains("TEXT "){
                                let line = format!("TEXT ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ,\r\n",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }

                            else {
                                panic!("Provide correct variable in id in ONE_TO_MANY")
                            }

                        }

                        // * DATE
                        else if value.starts_with("Date") && !value.starts_with("DateTime") {

                            // * NOW
                            if value == ("Date(NOW)") {
                                // println!("{}","billionaire");
                                table.push_str(&format!("{}","DATE DEFAULT CURRENT_DATE"))
                            }
                            // * CUSTOME
                            else if value == "Date(CUSTOME)" {
                                table.push_str(&format!("{}","DATE NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for DATE")
                            }
                        }

                        // * TIME
                        else if value.starts_with("Time") {

                            // * NOW
                            if value == ("Time(NOW)") {
                                // println!("{}")
                               table.push_str(&format!("{}","TIME DEFAULT CURRENT_TIME"))
                            }

                            // * CUSTOME
                            else if value == ("Time(CUSTOME)") {
                                table.push_str(&format!("{}","TIME NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for TIME")
                            }
                       }

                        //  * date_time
                       else if value.starts_with("DateTime") {
                            if value == ("DateTime(NOW)") {
                                table.push_str(&format!("{}","TIMESTAMP DEFAULT NOW()"))
                            }
                            else if value == ("DateTime(CUSTOME)") {
                                table.push_str(&format!("{}","TIMESTAMP NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for DATETIME")
                            }
                       }
                        else if value.starts_with("STRING"){
                            table.push_str("TEXT ")
                        }
                        else if value.starts_with("FLOAT"){
                            table.push_str("NUMERIC ")
                        }
                        else if value.starts_with("NUMBER"){
                            table.push_str("INT ")
                        }
                        else if value.starts_with("BOOL"){
                            table.push_str("BOOL ")
                        }

                        // * DEFAULT
                        else if value.starts_with("DEFAULT"){
                            // let value = $value;
                            // println!("{:?}",value);
                            let value = value.split("(").nth(1).unwrap().trim_end_matches(")");
                            // println!("{}",value);
                            if value.starts_with("\""){
                                let value = format!("'{}' ",value);
                                let value=  value.replace("\"","");
                                let default = format!("DEFAULT {}", value);
                                table.push_str(&default);
                            }
                            else {
                                let value = value.replace("\"\"","");
                                // println!("integers {}",value);
                                // let value = format!("{}",value);
                                let default = format!("DEFAULT {} ", value);
                                table.push_str(&default);
                            }
                        }

                        // * JSON
                        else if value.starts_with("JSON"){
                            table.push_str("JSONB ")
                        }

                        // * GEOGRAPHY
                        else if value.starts_with("Geography("){
                            if value.starts_with("Geography(POINT(Epsg3857))"){
                                let value = format!("GEOGRAPHY(POINT,3857) ");
                                table.push_str(&value);
                            }
                            else if value.starts_with("Geography(POINT(Epsg4326))"){
                                let value = format!("GEOGRAPHY(POINT,4326) ");
                                table.push_str(&value);
                            }
                            else if value.starts_with("Geography(POLYGON(Epsg3857))"){
                                let value = format!("GEOGRAPHY(POLYGON,3857) ");
                                table.push_str(&value);
                            }
                            else if value.starts_with("Geography(POLYGON(Epsg4326))"){
                                let value = format!("GEOGRAPHY(POLYGON,4326) ");
                                table.push_str(&value);
                            }
                            else {
                                panic!("Please provide correct Geography in table {}",$model)
                            }
                        }

                        else if value.starts_with("NOTNULL"){
                            table.push_str(&format!("NOT NULL "))
                        }
                        // * UNIQUE
                        else if value.starts_with("UNIQUE"){
                            // println!("{}",$billionaire.to_lowercase());
                            unique.push_str(&format!("{},",$billionaire.to_lowercase()))
                        }
                        else if value.starts_with("PRIMARY"){
                            primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                        }
                        else if value.starts_with("INDEX"){
                            index.push_str(&format!("{},",$billionaire.to_lowercase()))
                        }
                        else {
                            panic!("Provide Related Key at {} in table {}",stringify!($value),$model)
                        }
                        // println!("{:?}",$value);
                    )*
                    let mut table = table.trim_end_matches(" ").to_string();
                    table.push_str(",");
                    table.push_str("\n");
                )*
                // println!("{}",table);
                if primary_key.len() != 0 {
                    let mut primary_key = primary_key.trim_end_matches(",");
                    let key = format!("PRIMARY KEY ({}),\r\n",primary_key);
                    // println!("{:?}",key);
                    table.push_str(&key);
                }
                if unique.len() != 0 {
                    let mut unique = unique.trim_end_matches(",");
                    let key = format!("UNIQUE ({})",unique);
                    // println!("{}",key);
                    table.push_str(&key);
                }
                if foriegn_key.len() != 0 {
                    // println!("{}",foriegn_key);
                    let foriegn_key = format!(",\r\n{}",foriegn_key);
                    table.push_str(&foriegn_key);
                }
                let table = table.trim_end_matches(",\r\n");
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
                if index_relation.len() != 0 {
                    let index_relation = index_relation.trim_end_matches(",");
                    let random: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(6)
                    .map(char::from)
                    .collect();
                    table.push_str(&format!("CREATE INDEX index_relation_{} ON {} ({});\r\n",random,$model,index_relation));
                }
                // println!("{}",foriegn_key);
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
            let mut index_relation = String::new();
            let mut id = String::new();
            let mut unique = String::new();
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
                            if value == ("ID(UUID)") {
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                table.push_str("UUID ");
                                id.push_str("UUID");

                                // table.push_str("PRIMARY KEY ");
                                table.push_str("DEFAULT uuid_generate_v4() ");
                            }
                            else if value == ("ID(CUID)"){
                                table.push_str("TEXT ");
                                id.push_str("TEXT ");
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                // table.push_str("PRIMARY KEY ");
                                table.push_str("DEFAULT encode(gen_random_bytes(12),'hex') ");
                            }
                            else if value == ("ID(AUTO)") {
                                id.push_str("INT ");
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                table.push_str("INT GENERATED ALWAYS AS IDENTITY ");
                                // table.push_str("PRIMARY KEY ");
                            }
                            else if value == ("ID(BIGINT)") {
                                id.push_str("BIGINT ");
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                                table.push_str("BIGINT ");
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
                             if id.contains("UUID") {
                                let line = format!("UUID UNIQUE ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }
                            // * AUTO
                            else if id.contains("INT") {
                                let line = format!("INT UNIQUE");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }

                            // * CUID
                            else if id.contains("TEXT "){
                                let line = format!("TEXT UNIQUE");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }

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
                            if id.contains("UUID") {
                                let line = format!("UUID ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }
                            // * AUTO
                            else if id.contains("INT") {
                                let line = format!("INT ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }

                            // * CUID
                            else if id.contains("TEXT "){
                                let line = format!("TEXT ");
                                table.push_str(&line);
                                let line = format!("FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE ",$billionaire.to_lowercase(),deserialize.table,deserialize.table_field);
                                foriegn_key.push_str(&line);
                                index_relation.push_str(&format!("{},",$billionaire.to_lowercase()))
                            }

                            else {
                                panic!("Provide correct variable in id in ONE_TO_MANY")
                            }

                        }

                        // * DATE
                        else if value.starts_with("Date") && !value.starts_with("DateTime") {

                            // * NOW
                            if value == stringify!(Date(NOW)) {
                                // println!("{}","billionaire");
                                table.push_str(&format!("{}","DATE DEFAULT CURRENT_DATE"))
                            }
                            // * CUSTOME
                            else if value == "Date(CUSTOME)" {
                                table.push_str(&format!("{}","DATE NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for DATE")
                            }
                        }

                        // * TIME
                        else if value.starts_with("Time") {

                            // * NOW
                            if value == ("Time(NOW)") {
                                // println!("{}")
                               table.push_str(&format!("{}","TIME DEFAULT CURRENT_TIME"))
                            }

                            // * CUSTOME
                            else if value == ("Time(CUSTOME)") {
                                table.push_str(&format!("{}","TIME NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for TIME")
                            }
                       }

                        //  * date_time
                       else if value.starts_with("DateTime") {
                            if value == ("DateTime(NOW)") {
                                table.push_str(&format!("{}","TIMESTAMP DEFAULT NOW()"))
                            }
                            else if value == ("DateTime(CUSTOME)") {
                                table.push_str(&format!("{}","TIMESTAMP NOT NULL"))
                            }
                            else {
                                panic!("{}","Provide correct method for DATETIME")
                            }
                        }
                        else if value.starts_with("STRING"){
                            table.push_str("TEXT ")
                        }
                        else if value.starts_with("FLOAT"){
                            table.push_str("NUMERIC ")
                        }
                        else if value.starts_with("NUMBER"){
                            table.push_str("INT ")
                        }
                        else if value.starts_with("BOOL"){
                            table.push_str("BOOL ")
                        }

                        // * DEFAULT
                        else if value.starts_with("DEFAULT"){
                            // let value = $value;
                            // println!("{:?}",value);
                            let value = value.split("(").nth(1).unwrap().trim_end_matches(")");
                            // println!("{}",value);
                            if value.starts_with("\""){
                                let value = format!("'{}' ",value);
                                let value=  value.replace("\"","");
                                let default = format!("DEFAULT {}", value);
                                table.push_str(&default);
                            }
                            else {
                                let value = value.replace("\"\"","");
                                // println!("integers {}",value);
                                // let value = format!("{}",value);
                                let default = format!("DEFAULT {} ", value);
                                table.push_str(&default);
                            }
                        }

                        else if value.starts_with("Geography("){
                            if value.starts_with("Geography(POINT(Epsg3857))"){
                                let value = format!("GEOGRAPHY(POINT,3857) ");
                                table.push_str(&value);
                            }
                            else if value.starts_with("Geography(POINT(Epsg4326))"){
                                let value = format!("GEOGRAPHY(POINT,4326) ");
                                table.push_str(&value);
                            }
                            else if value.starts_with("Geography(POLYGON(Epsg3857))"){
                                let value = format!("GEOGRAPHY(POLYGON,3857) ");
                                table.push_str(&value);
                            }
                            else if value.starts_with("Geography(POLYGON(Epsg4326))"){
                                let value = format!("GEOGRAPHY(POLYGON,4326) ");
                                table.push_str(&value);
                            }
                            else {
                                panic!("Please provide correct Geography in table {}",$model)
                            }
                        }

                        // * UNIQUE
                        else if value.starts_with("UNIQUE"){
                            // println!("{}",$billionaire.to_lowercase());
                            unique.push_str(&format!("{},",$billionaire.to_lowercase()));
                        }

                        // * JSON
                        else if value.starts_with("JSON"){
                            table.push_str("JSONB ")
                        }
                        // else if value == ("UUID"){
                        //     table.push_str("UUID ");
                        //     // table.push_str("PRIMARY KEY ");
                        //     table.push_str("DEFAULT uuid_generate_v4() ");
                        // }
                        // else if value == ("CUID"){
                        //     table.push_str("TEXT ");
                        //     // table.push_str("PRIMARY KEY ");
                        //     table.push_str("DEFAULT encode(gen_random_bytes(12),'hex') ");
                        // }
                        // else if value == ("AUTO") {
                        //     table.push_str("INT GENERATED ALWAYS AS IDENTITY ");
                        //     // table.push_str("PRIMARY KEY ");
                        // }

                        // * GEOGRAPHY
                        else if value.starts_with("Geography"){
                            table.push_str(&format!("GEOGRAPHY(POINT,4326)  "))
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
                            panic!("Provide Related Key at {} in table {}",stringify!($value),$model)
                        }
                        // println!("{:?}",$value);
                    )*
                    let mut table = table.trim_end_matches(" ").to_string();
                    table.push_str(",");
                    table.push_str("\n");
                )*
                if unique.len() != 0 {
                    let mut unique = unique.trim_end_matches(",");
                    let key_unique = format!("UNIQUE ({}),\r\n",unique);
                    // println!("{:?}",key_unique);
                    table.push_str(&key_unique);
                }
                if primary_key.len() != 0 {
                    let mut primary_key = primary_key.trim_end_matches(",");
                    let key = format!("PRIMARY KEY ({}),\r\n",primary_key);
                    // println!("{:?}",key);
                    table.push_str(&key);
                }
                // println!("{}",table);
                // table.push_str("PRIMARY KEY (id,story)");
                // let mut primary_key = primary_key.trim_end_matches(",");
                // println!("bbbbbbbbbbbbb {}",primary_key);
                let table = table.trim_end_matches(",\r\n");
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
    (client => $url:expr ,models => { $($model:expr),*}) => {{
        use std::fs::DirBuilder;
        use std::fs::File;
        use std::io::Write;

        // let model = $model.clone()
        let mut schema = String::new();
        $(
            let mut container = String::new();
            let mut containers = String::new();

            container.push_str(&format!("{}",$model));

            if container.contains("GEOGRAPHY") {
                containers.push_str("CREATE EXTENSION IF NOT EXISTS postgis;\r\n");
                // containers.push_str(&container);
            }
            if container.contains("UUID") {
                containers.push_str("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";\r\n");
                // containers.push_str(&container);
            }
            if container.contains("encode(gen_random_bytes(12)") {
                containers.push_str("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";\r\n");
                // containers.push_str(&container);
            }
            containers.push_str(&container);
            schema.push_str(&containers);
            // let mut cluster = String::new();
            // println!("{}",containers);
            if let Some(cluster) = containers.find("CLUSTER") {
                // println!("{}",containers);
                let clusters = &containers[cluster..];
                // println!("{}", clusters);
                let containers = &containers[..cluster];
                println!("{}", containers);
                // println!("{}",&fresh);

                let db = $url.batch_execute(&containers).unwrap();
                $url.batch_execute(&clusters).unwrap();
            } else {
                let db = $url.batch_execute(&containers).unwrap();
            }
            println!("......................................................");
        )*
        DirBuilder::new()
        .recursive(true)
        .create("database")
        .unwrap();
        let mut sql = File::create("database/db.sql").unwrap();
        sql.write_all(format!("/* Reference Schema */\r\n").as_bytes())
            .unwrap();
        sql.write_all(schema.as_bytes()).unwrap();
    }};
}

#[macro_export]
macro_rules! find_one {
  (@format $connection:expr,$model:expr,$client:expr) => {
    {
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use uuid::Uuid;
        use std::panic;

        let mut billionaires = Vec::new();
        for billionaire in $client.iter() {
            let mut collection = Vec::new();
            let billionaire_column = billionaire.columns();
            for billionaires in billionaire_column.iter() {
                let mut map = BTreeMap::new();

                let name = billionaires.name();
                let billion = billionaires.type_().name();

                    // println!("{:?}", billion);

                let value = match billion.clone() {
                        "text" => {
                            let value: String = billionaire.get(name);
                            value
                        }
                        "date" => {
                            let value: NaiveDate = billionaire.get(name);
                            value.to_string()
                        }
                        "timestamp" => {
                            let value: NaiveDateTime = billionaire.get(name);
                            value.to_string()
                        }
                        "int4" => {
                            let value: i32 = billionaire.get(name);
                            value.to_string()
                        }
                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                        }
                        "time" => {
                            let value: NaiveTime = billionaire.get(name);
                            value.to_string()
                        }
                        "uuid" => {
                            let value: Uuid = billionaire.get(name);
                            value.to_string()
                        }
                        "bool" => {
                            let value: bool = billionaire.get(name);
                            value.to_string()
                        }
                        _ => {
                            panic!("")
                        }
                    };
                    map.insert(name.to_string(), value);
                    collection.push(map)
                }
                // println!("{:?}",collection);
                billionaires.push(collection);
            }
        billionaires
    }
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
                find_many!(@format $connection,$model,client)
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
                            let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                            where_value.push_str(&wherevalue);
                    )*
                    let where_value = where_value.trim_end_matches("OR ").to_string();
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

                    find_many!(@format $connection,$model,client)
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

                find_many!(@format $connection,$model,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }};
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
                let where_value = where_value.trim_end_matches("AND ").to_string();
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
                // println!("{}",query);
                // println!("{:?}",value);
                // command.push_str(&query);
                let client = $connection.query(&query,&value).unwrap();
                // println!("{:?}","client");

                find_many!(@format $connection,$model,client)
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
                find_many!(@format $connection,$model,client)
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
                    and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                    and_value.push(&$and_value);
                )*

                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                let mut and_values = and_values.trim_end_matches("AND ").to_string();
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} LIMIT 1;",select_value,$model,and_values);
                // println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                // println!("{:?}",client);
                find_many!(@format $connection,$model,client)
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

                let and_values = and_values.trim_end_matches("AND ");
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
                find_many!(@format $connection,$model,client)
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
    //         let and_values = and_values.trim_end_matches("AND ");
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
    ) =>
    {
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
                and_values.push_str(&format!("{} = ${} OR ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            let and_values = and_values.trim_end_matches("AND ");
            $(
                if !["asc","desc"].contains(&$order) {
                    panic!("Provide correct order either \"asc\" nor \"desc\"");
                }
                let order_ = format!("{} {},",$target,$order);
                order.push_str(&order_);
            )*
            let order = order.trim_end_matches(",");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT 1;",select_value,$model,and_values,order);
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@format $connection,$model,client)
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
//         let and_values = and_values.trim_end_matches("AND ");
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
                find_many!(@format $connection,$model,client)
        }
    };

}

#[macro_export]
macro_rules! delete_table {
    (connection => $connection:expr,model => $model:expr) => {{
        let delete = format!("DROP TABLE IF EXISTS {} ;", $model);
        $connection.execute(&delete, &[])
    }};
    (connection => $connection:expr,model => $model:expr,cascade) => {{
        let delete = format!("DROP TABLE IF EXISTS {} CASCADE;", $model);
        $connection.execute(&delete, &[])
    }};
}

/// # Example
/// 1
/// ```
/// let find = find_many! {
///     connection => postgres,
///     model:"elonmusk",
///     condition:{
///         "story" => "haribillionaire"
///     }
/// };
/// `````
/// 2`
/// ```
/// let find = find_many! {
///     connection => postgres,
///     model:"elonmusk",
///     select:{
///         "story"
///     },
///     condition:{
///         "story" => "haribillionaire"
///     }
/// };
/// ```
/// 10
/// ```
/// let find = find_many! {
///     connection => postgres,
///     model:"elonmusk",
///     conditions:{
///         or => {
///             "story" => "billionairehari"
///         },
///         "story" => "billionairehari"
///     },
///     limit:0,
///     skip:0,
///     order:{
///         "story" => "asc"
///     }
/// };
/// ```
/// ```
/// let location = nearby_location! {
/// connection => postgres,
/// model:"shop",
/// select:{
///     "other_than_location_type"
/// },
/// location:{
///     lattitude:"12.971599",
///     longitude:"77.594566"
/// },
/// select_from:{
///     "location"
/// }
/// };
/// ```
/// /// ```
/// let location = nearby_location! {
/// connection => postgres,
/// model:"shop",
/// select:{
///     "other_than_location_type"
/// },
/// location:{
///     lattitude:"12.971599",
///     longitude:"77.594566"
/// },
/// select_from:{
///     "location"
/// }
/// };
/// ```
#[macro_export]
macro_rules! find_many {

    (@format $connection:expr,$model:expr,$client:expr) => {
        {
            use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
            use std::collections::BTreeMap;
            use uuid::Uuid;
            use std::panic;

            let mut billionaires = Vec::new();
            for billionaire in $client.iter() {
                let mut collection = Vec::new();
                let billionaire_column = billionaire.columns();
                for billionaires in billionaire_column.iter() {
                    let mut map = BTreeMap::new();

                    let name = billionaires.name();
                    let billion = billionaires.type_().name();

                    // println!("{:?}", billion);

                    let value = match billion.clone() {
                        "text" => {
                            let value: String = billionaire.get(name);
                            value
                        }
                        "date" => {
                            let value: NaiveDate = billionaire.get(name);
                            value.to_string()
                        }
                        "timestamp" => {
                            let value: NaiveDateTime = billionaire.get(name);
                            value.to_string()
                        }
                        "int4" => {
                            let value: i32 = billionaire.get(name);
                            value.to_string()
                        }
                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                        }
                        "time" => {
                            let value: NaiveTime = billionaire.get(name);
                            value.to_string()
                        }
                        "uuid" => {
                            let value: Uuid = billionaire.get(name);
                            value.to_string()
                        }
                        "bool" => {
                            let value: bool = billionaire.get(name);
                            value.to_string()
                        }
                        _ => {
                            panic!("")
                        }
                    };
                    map.insert(name.to_string(), value);
                    collection.push(map)
                }
                // println!("{:?}",collection);
                billionaires.push(collection);
            }
        billionaires
        }
      };
    (connection => $connection:expr,model => $model:expr) => {{
                let query = format!("SELECT * FROM {}", $model);
                let client = $connection.query(&query, &[]).unwrap();
                find_many!(@format $connection,$model,client)
    }};
    // * model //completed
    (connection => $connection:expr,model:$model:expr) => {
            {
                use core::panic;
                // let mut command = String::new();
                if $model.len() != 0 {
                    let client = format!("SELECT * FROM {} ;",$model);
                    let client = $connection.query(&client,&[]).unwrap();
                    // println!("{}",client);
                    // command.push_str(&query);
                    find_many!(@format $connection,$model,client)
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
    }) =>
    {
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
                let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                where_value.push_str(&wherevalue);
            )*
            let where_value = where_value.trim_end_matches("OR ").to_string();
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
           find_many!(@format $connection,$model,client)
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
                            let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                            where_value.push_str(&wherevalue);
                    )*
                    let where_value = where_value.trim_end_matches("OR ").to_string();
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
                    find_many!(@format $connection,$model,client)
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
                        let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                        where_value.push_str(&wherevalue);
                    )*
                    let where_value = where_value.trim_end_matches("OR ").to_string();
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
                    find_many!(@format $connection,$model,client)
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
                        let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                        where_value.push_str(&wherevalue);
                    )*
                    let where_value = where_value.trim_end_matches("OR ").to_string();
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
                    find_many!(@format $connection,$model,client)
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
    },limit:$limit:expr) =>
    {
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
                        let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                        where_value.push_str(&wherevalue);
                )*
                let where_value = where_value.trim_end_matches("OR ").to_string();
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
                find_many!(@format $connection,$model,client)
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
                    let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                    where_value.push_str(&wherevalue);
                )*
                let where_value = where_value.trim_end_matches("OR ").to_string();
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
                find_many!(@format $connection,$model,client)
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
    },limit:$limit:expr,
    skip:$skip:expr,
    order:{
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
                        let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                        where_value.push_str(&wherevalue);
                    )*
                    let where_value = where_value.trim_end_matches("OR ").to_string();
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
                    find_many!(@format $connection,$model,client)
                }
                else {
                    panic!("\x1b[44mProvide Model Name\x1b[0m")
                }
        }
    };
    // * select, where,limit,skip,order //completed
    (connection => $connection:expr,
    model:$model:expr,
    condition:{
        $($value_where_and:expr => $where_by_and:expr)*
    },
    limit:$limit:expr,
    skip:$skip:expr,
    order:{
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
            find_many!(@format $connection,$model,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }
};
    // * select, where,limit,order //completed
    (connection => $connection:expr,
    model:$model:expr,
    condition:{
        $($value_where_and:expr => $where_by_and:expr)*
    },
    limit:$limit:expr,
    order:{
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
                    value.push(&$where_by_and);
                )*
                let where_value = where_value.trim_end_matches("AND ");
                let order = order.trim_end_matches("AND ");
                // println!("{:?}",where_value);
                // let len = params.len();

                let query = format!("SELECT * FROM {} WHERE {} ORDER BY {} LIMIT {} ;",$model,where_value,order,$limit);
                // println!("{}",query);
                // println!("{:?}",value);
                command.push_str(&query);
                let client = $connection.query(&command,&value).unwrap();
                find_many!(@format $connection,$model,client)
            }
            else {
                panic!("\x1b[44mProvide Model Name\x1b[0m")
            }
      }
    };
    // * select, where,skip,order //completed
    (connection => $connection:expr,
        model:$model:expr,
        condition:{
            $($value_where_and:expr => $where_by_and:expr)*
        },
        skip:$skip:expr,
        order:{
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
            find_many!(@format $connection,$model,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }
    };
    // * select, where,order //completed
    (connection => $connection:expr,
        model:$model:expr,
        condition:{
            $($value_where_and:expr => $where_by_and:expr)*
        },
        order:{
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
                value.push(&$where_by_and);
            )*
            let where_value = where_value.trim_end_matches("AND ");
            let order = order.trim_end_matches("AND ");
              // println!("{:?}",where_value);
              // let len = params.len();

            let query = format!("SELECT * FROM {} WHERE {} ORDER BY {};",$model,where_value,order,$limit,$skip);
              // println!("{}",query);
              // println!("{:?}",value);
            command.push_str(&query);
            let client = $connection.query(&command,&value).unwrap();
            find_many!(@format $connection,$model,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }
};
    // * select, where,order //completed
    (connection => $connection:expr,
        model:$model:expr,
        select:{
            $($select:expr),*
        },
        condition:{
            $($value_where_and:expr => $where_by_and:expr),*
        },
        order:{
                $($order_by:expr => $order:expr),*
        }) => {
    {
        use core::panic;
        use postgres::types::ToSql;

        let mut command = String::new();
        let mut where_value = String::new();
        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut order = String::new();
        let mut select = String::new();

        if $model.len() != 0 {
            let mut idx = 0;
            $(
                let selectt = format!("{},",$select);
                select.push_str(&selectt);
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
                value.push(&$where_by_and);
            )*
            let where_value = where_value.trim_end_matches("AND ");
            let order = order.trim_end_matches("AND ");
            let select = select.trim_end_matches(",");
              // println!("{:?}",where_value);
              // let len = params.len();

            let query = format!("SELECT {} FROM {} WHERE {} ORDER BY {};",select,$model,where_value,order);
              // println!("{}",query);
              // println!("{:?}",value);
            command.push_str(&query);
            let client = $connection.query(&command,&value).unwrap();
            find_many!(@format $connection,$model,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }
};
    // * select, where,order //completed
    (connection => $connection:expr,
        model:$model:expr,
        select:{
            $($select:expr),*
        },
        condition:{
            $($value_where_and:expr => $where_by_and:expr),*
        },
        limit:$limit:expr) => {
    {
        use core::panic;
        use postgres::types::ToSql;

        let mut command = String::new();
        let mut where_value = String::new();
        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut select = String::new();

        if $model.len() != 0 {
            let mut idx = 0;
            $(
                let selectt = format!("{},",$select);
                select.push_str(&selectt);
            )*
            $(
                idx +=1;
                let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                where_value.push_str(&wherevalue);
            )*
              // println!("{}",idx);
            $(
                value.push(&$where_by_and);
            )*
            let where_value = where_value.trim_end_matches("AND ");
            let select = select.trim_end_matches(",");
              // println!("{:?}",where_value);
              // let len = params.len();

            let query = format!("SELECT {} FROM {} WHERE {} LIMIT {};",select,$model,where_value,$limit);
              // println!("{}",query);
              // println!("{:?}",value);
            command.push_str(&query);
            let client = $connection.query(&command,&value).unwrap();
            find_many!(@format $connection,$model,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }
};
    // * select, where,order //completed
    (connection => $connection:expr,
        model:$model:expr,
        select:{
            $($select:expr),*
        },
        condition:{
            $($value_where_and:expr => $where_by_and:expr),*
        }) => {
    {
        use core::panic;
        use postgres::types::ToSql;

        let mut command = String::new();
        let mut where_value = String::new();
        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut select = String::new();

        if $model.len() != 0 {
            let mut idx = 0;
            $(
                let selectt = format!("{},",$select);
                select.push_str(&selectt);
            )*
            $(
                idx +=1;
                let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                where_value.push_str(&wherevalue);
            )*
              // println!("{}",idx);
            $(
                value.push(&$where_by_and);
            )*
            let where_value = where_value.trim_end_matches("AND ");
            let select = select.trim_end_matches(",");
              // println!("{:?}",where_value);
              // let len = params.len();

            let query = format!("SELECT {} FROM {} WHERE {} ;",select,$model,where_value);
              // println!("{}",query);
              // println!("{:?}",value);
            command.push_str(&query);
            let client = $connection.query(&command,&value).unwrap();
            find_many!(@format $connection,$model,client)
        }
        else {
            panic!("\x1b[44mProvide Model Name\x1b[0m")
        }
    }
};
    // * select, where,order //completed
    (connection => $connection:expr,
        model:$model:expr,
        condition:{
            $($value_where_and:expr => $where_by_and:expr),*
        },
        limit:$limit:expr) => {
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
                let wherevalue = format!("{} = ${} AND ",$value_where_and,idx);
                where_value.push_str(&wherevalue);
            )*
              // println!("{}",idx);
            $(
                value.push(&$where_by_and);
            )*
            let where_value = where_value.trim_end_matches("AND ");
            let select = select.trim_end_matches(",");
              // println!("{:?}",where_value);
              // let len = params.len();

            let query = format!("SELECT {} FROM {} WHERE {} LIMIT {};",select,$model,where_value,$limit);
              // println!("{}",query);
              // println!("{:?}",value);
            command.push_str(&query);
            let client = $connection.query(&command,&value).unwrap();
            find_many!(@format $connection,$model,client)
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
    },
    skip:$skip:expr,
    order:{
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
                    let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                    where_value.push_str(&wherevalue);
                )*
                let where_value = where_value.trim_end_matches("OR ").to_string();
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
                find_many!(@format $connection,$model,client)
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
                        let wherevalue = format!("{} = ${} OR ",$value_where,idx);
                        where_value.push_str(&wherevalue);
                    )*
                    let where_value = where_value.trim_end_matches("OR ").to_string();
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
                    find_many!(@format $connection,$model,client)
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
                // println!("{}",format);
                let client = $connection.query(&format,&or_value).unwrap();
                find_many!(@format $connection,$model,client)
        }
    };
    (connection => $connection:expr,
    model:$model:expr,
    select:{
        $($select_value:expr),*
    }) => {
        {
            use postgres::types::ToSql;

            let mut selectvalue = String::new();
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
                    // println!("{}",or_values);
                    // println!("{:?}",or_value);
            let format = format!("SELECT {} FROM {};",select_value,$model);
                    // println!("{}",format);
            let client = $connection.query(&format,&[]).unwrap();
            find_many!(@format $connection,$model,client)
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
    ) =>
    {
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
                and_values.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                and_value.push(&$or_value2);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            let and_values = and_values.trim_end_matches("AND ");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {};",select_value,$model,and_values);
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@format $connection,$model,client)
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
        let mut or_values = String::new();
        let mut or_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
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
            or_values.push_str(&format!("{} = ${} OR ",$or_values,idx));
            or_value.push(&$or_value);
        )*
        let or_values = or_values.trim_end_matches("OR ");
                    // println!("{}",or_values);
                    // println!("{:?}",or_value);
        $(
            if !["asc","desc"].contains(&$order) {
                panic!("Provide correct order either \"asc\" nor \"desc\"");
            }
            else {
                let order_ = format!("{} {},",$target,$order);
                order.push_str(&order_);
            }
        )*
        let order = order.trim_end_matches(",");
        let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {};",select_value,$model,or_values,order);
        // println!("{}",format);
        let client = $connection.query(&format,&or_value).unwrap();
        find_many!(@format $connection,$model,client)
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
        // println!("{}",format);
        let client = $connection.query(&format,&or_value).unwrap();
        find_many!(@format $connection,$model,client)
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
                    and_values.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                    and_value.push(&$or_value2);
                )*
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                let and_values = and_values.trim_end_matches("AND ");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} LIMIT {};",select_value,$model,and_values,$limit);
                // println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                find_many!(@format $connection,$model,client)
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
                    and_values.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                    and_value.push(&$or_value2);
                )*
                $(
                    idx+=1;
                    and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                let and_values = and_values.trim_end_matches("AND ");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} OFFSET {};",select_value,$model,and_values,$skip);
                // println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                find_many!(@format $connection,$model,client)
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
                    and_values.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                    and_value.push(&$or_value2);
                )*
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
                let and_values = and_values.trim_end_matches("AND ");
                let order = order.trim_end_matches(",");
                // println!("{}",and_values);
                // println!("{:?}",and_value);
                let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {};",select_value,$model,and_values,order);
                // println!("{}",format);
                let client = $connection.query(&format,&and_value).unwrap();
                find_many!(@format $connection,$model,client)
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
    ) =>
    {

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
                and_values.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                and_value.push(&$or_value2);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            let and_values = and_values.trim_end_matches("AND ");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} LIMIT {} OFFSET {};",select_value,$model,and_values,$limit,$skip);
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@format $connection,$model,client)
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
    ) =>
    {
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
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@format $connection,$model,client)
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
    ) =>
    {
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
                and_values.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                and_value.push(&$or_value2);
            )*
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
            let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT {} OFFSET {};",select_value,$model,and_values,order,$limit,$skip);
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@format $connection,$model,client)
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
    ) =>
    {
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
                and_values.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                and_value.push(&$or_value2);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                if !["asc","desc"].contains(&$order) {
                    panic!("Provide correct order either \"asc\" nor \"desc\"");
                }
                else {
                    let order_ = format!("{} {},",$target,$order);
                    order.push_str(&order_);
                }
            )*
            let order = order.trim_end_matches(",");
            let and_values = and_values.trim_end_matches("AND ");
                    // println!("{}",and_values);
                    // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT {};",select_value,$model,and_values,order,$limit);
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@format $connection,$model,client)
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
                and_values.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                and_value.push(&$or_value2);
            )*
            $(
                idx+=1;
                and_values.push_str(&format!("{} = ${} AND ",$and_values,idx));
                and_value.push(&$and_value);
            )*
            $(
                if !["asc","desc"].contains(&$order) {
                    panic!("Provide correct order either \"asc\" nor \"desc\"");
                }
                else {
                    let order_ = format!("{} {},",$target,$order);
                    order.push_str(&order_);
                }
            )*
            let order = order.trim_end_matches(",");
            let and_values = and_values.trim_end_matches("AND ");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let format = format!("SELECT {} FROM {} WHERE {} ORDER BY {} OFFSET {};",select_value,$model,and_values,order,$skip);
            // println!("{}",format);
            let client = $connection.query(&format,&and_value).unwrap();
            find_many!(@format $connection,$model,client)
        }
    };
    (connection => $connection:expr,
    model:$model:expr,
    condition:{$($condition:expr => $value:expr),*},
    include:{$($include:expr),*},
    match:{
        $match:expr => $match_value:expr
    }) => {
        {
            let mut include = String::new();
            let mut conditions = String::new();
            let mut includes = String::new();
            $(
                include.push_str(&format!("{}.*,",$include));
                includes.push_str(&format!("{},",$include));
            )*
            $(
                conditions.push_str(&format!("{}.{} = '{}',",$model,$condition,$value));
            )*
            let include = include.trim_end_matches(",");
            let conditions = conditions.trim_end_matches(",");
            let includes = includes.trim_end_matches(",");
            let query = format!("SELECT {} , {}.* FROM {} LEFT JOIN {} on {}.{} = {}.{} WHERE {};",include,$model,$model,includes,$model,$match,includes,$match_value,conditions);
            let client = $connection.query(&query,&[]).unwrap();
            // println!("{}",query);
            find_many!(@format $connection,$model,client)

        }
    };
    (connection => $connection:expr,
    model:$model:expr,
    condition:{$($condition:expr => $value:expr),*},
    include:{$($include:expr => {$($select:expr),*}),*},
    match:{
        $match:expr => $match_value:expr
    }) => {
        {
            let mut include = String::new();
            let mut conditions = String::new();
            let mut includes = String::new();
            let mut select = String::new();
            $(
                $(
                    include.push_str(&format!("{}.{},",$include,$select));
                )*
                includes.push_str(&format!("{},",$include));
            )*
            $(
                conditions.push_str(&format!("{}.{} = '{}',",$model,$condition,$value));
            )*
            let include = include.trim_end_matches(",");
            let conditions = conditions.trim_end_matches(",");
            let includes = includes.trim_end_matches(",");
            let query = format!("SELECT {} , {}.* FROM {} LEFT JOIN {} on {}.{} = {}.{} WHERE {};",include,$model,$model,includes,$model,$match,includes,$match_value,conditions);
            let client = $connection.query(&query,&[]).unwrap();
            // println!("{}",query);
            find_many!(@format $connection,$model,client)

        }
    };
    (connection => $connection:expr,
        model:$model:expr,
        select:{$($select_value:expr),*},
        condition:{$($condition:expr => $value:expr),*},
        include:{$($include:expr => {$($select:expr),*}),*},
        match:{
        $match:expr => $match_value:expr
    }) => {
        {
            let mut include = String::new();
            let mut conditions = String::new();
            let mut includes = String::new();
            let mut select = String::new();
            $(
                $(
                    include.push_str(&format!("{}.{},",$include,$select));
                )*
                includes.push_str(&format!("{},",$include));
            )*
            $(
                    include.push_str(&format!("{}.{},",$model,$select_value));
            )*
            $(
                conditions.push_str(&format!("{}.{} = '{}',",$model,$condition,$value));
            )*
            let include = include.trim_end_matches(",");
            let conditions = conditions.trim_end_matches(",");
            let includes = includes.trim_end_matches(",");
            let query = format!("SELECT {}  FROM {} LEFT JOIN {} on {}.{} = {}.{} WHERE {};",include,$model,includes,$model,$match,includes,$match_value,conditions);
            // println!("{}",query);
            let client = $connection.query(&query,&[]).unwrap();
            find_many!(@format $connection,$model,client)

        }
    };
    (connection => $connection:expr,model:$model:expr,select:{$($select_value:expr),*},
    condition:{$($condition:expr => $value:expr),*},
    within:{
        lattitude:$lattitude:expr,
        longitude:$longitude:expr,
        within:$within:expr
    },
    also_include:{
         $location:expr
    }) => {
        {
            use postgres::types::ToSql;

            let mut conditions = String::new();
            let mut select = String::new();
            let mut location_value = String::new();
            let mut condition:Vec<&(dyn ToSql + Sync)> = Vec::new();


            $(
                let select_value = format!("{},",$select_value);
                select.push_str(&select_value);
            )*

            let mut idx = 0;
            idx+=1;
            let condition_longitude = format!("${}",idx);
            idx+=1;
            let condition_lattitude = format!("${}",idx);
            // location_value.push_str(&format!("{} {}",condition_lattitude,condition_longitude));

            condition.push(&$longitude);
            condition.push(&$lattitude);
            $(
                idx+=1;
                let condition_value = format!("{} = ${} AND ",$condition,idx);
                conditions.push_str(&condition_value);

                condition.push(&$value);
            )*


            let select = select.trim_end_matches(",");
            let conditions = conditions.trim_end_matches("AND ");
            let query = format!("SELECT {},ST_AsGeoJson({}) FROM {} WHERE ST_DWIthin({},ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')'),{}) AND {};",select,$location,$model,$location,condition_longitude,condition_lattitude,$within,conditions);
            // println!("{}",query);
            // println!("{:?}",condition);
            let client = $connection.query(&query,&condition).unwrap();
            find_many!(@format $connection,$model,client)

        }
    };
    (connection => $connection:expr,model:$model:expr,select:{$($select_value:expr),*},
    within:{
        lattitude:$lattitude:expr,
        longitude:$longitude:expr,
        within:$within:expr
    },
    also_include:{
         $location:expr
    }) => {
        {
            use postgres::types::ToSql;

            let mut select = String::new();
            let mut location_value = String::new();
            let mut condition:Vec<&(dyn ToSql + Sync)> = Vec::new();


            $(
                let select_value = format!("{},",$select_value);
                select.push_str(&select_value);
            )*

            let mut idx = 0;
            idx+=1;
            let condition_longitude = format!("${}",idx);
            idx+=1;
            let condition_lattitude = format!("${}",idx);
            // location_value.push_str(&format!("{} {}",condition_lattitude,condition_longitude));

            condition.push(&$longitude);
            condition.push(&$lattitude);


            let select = select.trim_end_matches(",");
            let query = format!("SELECT {},ST_AsGeoJson({}) FROM {} WHERE ST_DWIthin({},ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')'),{});",select,$location,$model,$location,condition_longitude,condition_lattitude,$within);
            // println!("{}",query);
            // println!("{:?}",condition);
            let client = $connection.query(&query,&condition).unwrap();
            find_many!(@format $connection,$model,client)

        }
    };
    (connection => $connection:expr,model:$model:expr,select:{$($select_value:expr),*},
    within:{
        lattitude:$lattitude:expr,
        longitude:$longitude:expr,
        within:$within:expr
    },
    also_include:{
         $location:expr
    },limit:$limit:expr) => {
        {
            use postgres::types::ToSql;

            let mut selection = String::new();
            let mut location_value = String::new();
            let mut condition:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut select = String::new();


            $(
                let select_value = format!("{},",$select_value);
                selection.push_str(&select_value);

                let selects = select_value.trim_end_matches(",");
                select.push_str(&format!("'{}',",selects));
            )*

            let select_value = format!("ST_AsGeoJson({}),",$location);
            selection.push_str(&select_value);
            select.push_str(&format!("'{}'",$location));

            // select.push_str(&format!("'ST_AsText({})'",$location));

            let mut idx = 0;
            idx+=1;
            let condition_longitude = format!("${}",idx);
            idx+=1;
            let condition_lattitude = format!("${}",idx);
            // location_value.push_str(&format!("{} {}",condition_lattitude,condition_longitude));

            condition.push(&$longitude);
            condition.push(&$lattitude);


            let selection = selection.trim_end_matches(",");
            let select = select.trim_end_matches(",");
            let query = format!("SELECT {} FROM {} WHERE ST_DWIthin({},ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')'),{}) LIMIT {};",selection,$model,$location,condition_longitude,condition_lattitude,$within,$limit);
            // println!("{}",query);
            // println!("{:?}",condition);
            let client = $connection.query(&query,&condition).unwrap();
            // client
            find_many!(@format $connection,$model,client)
        }
    };
    (connection => $connection:expr,model:$model:expr,select:{$($select_value:expr),*},
    also_include:{
         $location:expr
    }) => {
        {
            use postgres::types::ToSql;

            let mut selection = String::new();
            let mut select = String::new();


            $(
                let select_value = format!("{},",$select_value);
                selection.push_str(&select_value);

                let selects = select_value.trim_end_matches(",");
                select.push_str(&format!("'{}',",selects));
            )*

            let select_value = format!("ST_AsGeoJson({}),",$location);
            selection.push_str(&select_value);
            select.push_str(&format!("'{}'",$location));

            // select.push_str(&format!("'ST_AsText({})'",$location));


            let selection = selection.trim_end_matches(",");
            let select = select.trim_end_matches(",");
            let query = format!("SELECT {} FROM {};",selection,$model);
            // println!("{}",query);
            // println!("{:?}",condition);
            let client = $connection.query(&query,&[]).unwrap();
            // client
            find_many!(@format $connection,$model,client)
        }
    }
}

#[macro_export]
macro_rules! delete_many {
    // * all rows
    (connection => $connection:expr,model:$model:expr) => {{
        let delete = format!("DELETE FROM {};", $model);
        // println!("{}", delete);
        $connection.execute(&delete, &[])
    }};
}

/// # Example
/// ```
/// delete!{
/// connection => conn,
/// model: "users",
/// conditions: { "id" => 123, "status" => "inactive" }
/// };
/// ```
/// ```
/// delete!{
/// connection => conn,
/// model: "users",
///  conditions: { "id" => 123 },
/// select: { "email" "username" }
/// };
/// ```
/// ```
/// delete!{
/// connection => conn,
///  model: "orders",
/// conditions: { "user_id" => 123 },
/// cascade: true
/// };
/// ```
/// ```
/// delete!{connection => conn,
///  model: "products",
/// conditions => { or => { "category" => "electronics", "price" => 500 } }
/// };
/// ```
/// ```
/// delete!{
/// connection => conn,
///  model: "products",
///  conditions => { or => { "id" => 123, "category" => "clothing" } }, select: { "name","price" }
/// };
/// ```
/// ```
/// delete!{connection => conn,
///  model: "logs",
/// conditions: { "event_type" => "error", "timestamp" => "2025-01-01" },
/// cascade: "true"
/// };
/// ```
#[macro_export]
macro_rules! delete {
    (@format $connection:expr,$model:expr,$client:expr) => {
    {
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use uuid::Uuid;
        use std::panic;

            let mut billionaires = Vec::new();
            for billionaire in $client.iter() {
                let mut collection = Vec::new();
                let billionaire_column = billionaire.columns();
                for billionaires in billionaire_column.iter() {
                    let mut map = BTreeMap::new();

                    let name = billionaires.name();
                    let billion = billionaires.type_().name();

                    // println!("{:?}", billion);

                    let value = match billion.clone() {
                        "text" => {
                            let value: String = billionaire.get(name);
                            value
                        }
                        "date" => {
                            let value: NaiveDate = billionaire.get(name);
                            value.to_string()
                        }
                        "timestamp" => {
                            let value: NaiveDateTime = billionaire.get(name);
                            value.to_string()
                        }
                        "int4" => {
                            let value: i32 = billionaire.get(name);
                            value.to_string()
                        }
                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                        }
                        "time" => {
                            let value: NaiveTime = billionaire.get(name);
                            value.to_string()
                        }
                        "uuid" => {
                            let value: Uuid = billionaire.get(name);
                            value.to_string()
                        }
                        "bool" => {
                            let value: bool = billionaire.get(name);
                            value.to_string()
                        }
                        _ => {
                            panic!("")
                        }
                    };
                    map.insert(name.to_string(), value);
                    collection.push(map)
                }
                // println!("{:?}",collection);
                billionaires.push(collection);
            }
        billionaires
    }
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
        delete!(@format $connection,$model,client)

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
            let delete = format!("DELETE FROM {} WHERE {} CASCADE;",$model,delete);
            // println!("{}",delete);
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
            // println!("{}",delete);
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
            // println!("{}",delete);
            let client = $connection.query(&delete,&values).unwrap();
            delete!(@format $connection,$model,client)

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
        // println!("{}",delete);
        $connection.execute(&delete,&values)
    }};
}

#[macro_export]
// * data conditions
macro_rules! update {
    (@format $connection:expr,$model:expr,$client:expr) => {
    {
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use uuid::Uuid;
        use std::panic;

            let mut billionaires = Vec::new();
            for billionaire in $client.iter() {
                let mut collection = Vec::new();
                let billionaire_column = billionaire.columns();
                for billionaires in billionaire_column.iter() {
                    let mut map = BTreeMap::new();

                    let name = billionaires.name();
                    let billion = billionaires.type_().name();

                    // println!("{:?}", billion);

                    let value = match billion.clone() {
                        "text" => {
                            let value: String = billionaire.get(name);
                            value
                        }
                        "date" => {
                            let value: NaiveDate = billionaire.get(name);
                            value.to_string()
                        }
                        "timestamp" => {
                            let value: NaiveDateTime = billionaire.get(name);
                            value.to_string()
                        }
                        "int4" => {
                            let value: i32 = billionaire.get(name);
                            value.to_string()
                        }
                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                        }
                        "time" => {
                            let value: NaiveTime = billionaire.get(name);
                            value.to_string()
                        }
                        "uuid" => {
                            let value: Uuid = billionaire.get(name);
                            value.to_string()
                        }
                        "bool" => {
                            let value: bool = billionaire.get(name);
                            value.to_string()
                        }
                        _ => {
                            panic!("")
                        }
                    };
                    map.insert(name.to_string(), value);
                    collection.push(map)
                }
                // println!("{:?}",collection);
                billionaires.push(collection);
            }
        billionaires
    }
      };
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
        update!(@format $connection,$model,client)
    }};
}

/// # Example
///
/// ```
/// let create = create! {
/// connection => postgres,
/// model:"shop",
/// data:{
///     "place" => "san",
///     "age" => 24 as i32,
///     "bool" => true
/// }
/// };
/// ```
///
///```
/// let create = create! {
/// connection => postgres,
/// model:"user_",
/// data:{
///     "story" => "billionairehari",
///     "age" => 24 as i32
/// },
/// select:{
///     "id"
/// }
/// };
/// ```
#[macro_export]
macro_rules! create {
    (@format $connection:expr,$model:expr,$client:expr) => {
    {
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use uuid::Uuid;
        use std::panic;

            let mut billionaires = Vec::new();
            for billionaire in $client.iter() {
                let mut collection = Vec::new();
                let billionaire_column = billionaire.columns();
                for billionaires in billionaire_column.iter() {
                    let mut map = BTreeMap::new();

                    let name = billionaires.name();
                    let billion = billionaires.type_().name();

                    // println!("{:?}", billion);

                    let value = match billion.clone() {
                        "text" => {
                            let value: String = billionaire.get(name);
                            value
                        }
                        "date" => {
                            let value: NaiveDate = billionaire.get(name);
                            value.to_string()
                        }
                        "timestamp" => {
                            let value: NaiveDateTime = billionaire.get(name);
                            value.to_string()
                        }
                        "int4" => {
                            let value: i32 = billionaire.get(name);
                            value.to_string()
                        }
                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                        }
                        "time" => {
                            let value: NaiveTime = billionaire.get(name);
                            value.to_string()
                        }
                        "uuid" => {
                            let value: Uuid = billionaire.get(name);
                            value.to_string()
                        }
                        "bool" => {
                            let value: bool = billionaire.get(name);
                            value.to_string()
                        }
                        _ => {
                            panic!("")
                        }
                    };
                    map.insert(name.to_string(), value);
                    collection.push(map)
                }
                // println!("{:?}",collection);
                billionaires.push(collection);
            }
        billionaires
    }
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

            if $data.to_string().starts_with("SRID") {
                let datavalue = format!("ST_GeogFromText(${}),",idx);
                data_value.push_str(&datavalue);
            }
            else {
                let datavalue = format!("${},",idx);
                data_value.push_str(&datavalue);
            }

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

            if $data.to_string().starts_with("SRID") {
                let datavalue = format!("ST_GeogFromText(${}),",idx);
                data_value.push_str(&datavalue);
            }
            else {
                let datavalue = format!("${},",idx);
                data_value.push_str(&datavalue);
            }

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
    create!(@format $connection,$model,client)
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
        // println!("{}",value);
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

#[cfg(feature = "geography")]
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
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use core::panic;
        use std::collections::{BTreeMap, HashMap};
        use uuid::Uuid;

        let search = format!(
            "SELECT * FROM {} WHERE similarity({},'{}') > {} ORDER BY similarity({},'{}') {} ",
            $model, $believe, $text, $score, $o_believe, $o_text, $order
        );
        // println!("{}", search);
        let create = $connection.query(&search, &[]).unwrap();
        let mut billionaires = Vec::new();
        for billionaire in create.iter() {
            let mut collection = Vec::new();
            let billionaire_column = billionaire.columns();
            for billionaires in billionaire_column.iter() {
                let mut map = BTreeMap::new();

                let name = billionaires.name();
                let billion = billionaires.type_().name();

                // println!("{:?}", billion);

                let value = match billion.clone() {
                    "text" => {
                        let value: String = billionaire.get(name);
                        value
                    }
                    "date" => {
                        let value: NaiveDate = billionaire.get(name);
                        value.to_string()
                    }
                    "timestamp" => {
                        let value: NaiveDateTime = billionaire.get(name);
                        value.to_string()
                    }
                    "int4" => {
                        let value: i32 = billionaire.get(name);
                        value.to_string()
                    }
                    "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                    }
                    "time" => {
                        let value: NaiveTime = billionaire.get(name);
                        value.to_string()
                    }
                    "uuid" => {
                        let value: Uuid = billionaire.get(name);
                        value.to_string()
                    }
                    "bool" => {
                        let value: bool = billionaire.get(name);
                        value.to_string()
                    }
                    _ => {
                        panic!("")
                    }
                };
                map.insert(name.to_string(), value);
                collection.push(map)
            }
            // println!("{:?}",collection);
            billionaires.push(collection);
        }
        billionaires
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
    (@format $connection:expr,$model:expr,$client:expr) => {
    {
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use uuid::Uuid;
        use std::panic;

            let mut billionaires = Vec::new();
            for billionaire in $client.iter() {
                let mut collection = Vec::new();
                let billionaire_column = billionaire.columns();
                for billionaires in billionaire_column.iter() {
                    let mut map = BTreeMap::new();

                    let name = billionaires.name();
                    let billion = billionaires.type_().name();

                    // println!("{:?}", billion);

                    let value = match billion.clone() {
                        "text" => {
                            let value: String = billionaire.get(name);
                            value
                        }
                        "date" => {
                            let value: NaiveDate = billionaire.get(name);
                            value.to_string()
                        }
                        "timestamp" => {
                            let value: NaiveDateTime = billionaire.get(name);
                            value.to_string()
                        }
                        "int4" => {
                            let value: i32 = billionaire.get(name);
                            value.to_string()
                        }
                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                        }
                        "time" => {
                            let value: NaiveTime = billionaire.get(name);
                            value.to_string()
                        }
                        "uuid" => {
                            let value: Uuid = billionaire.get(name);
                            value.to_string()
                        }
                        "bool" => {
                            let value: bool = billionaire.get(name);
                            value.to_string()
                        }
                        _ => {
                            panic!("")
                        }
                    };
                    map.insert(name.to_string(), value);
                    collection.push(map)
                }
                // println!("{:?}",collection);
                billionaires.push(collection);
            }
        billionaires
    }
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
        pagination!(@format $connection,$model,client)
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
        pagination!(@format $connection,$model,client)
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
        pagination!(@format $connection,$model,client)
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
        pagination!(@format $connection,$model,client)
    }
    }
}

#[cfg(feature = "full_search")]
#[macro_export]
macro_rules! full_search {
  // (@format $connection:expr,$model:expr,$client:expr) => {};
  // (@select $connection,$model:expr,$selection:expr,$client:expr) => {};
  (@format $connection:expr,$model:expr,$client:expr) => {
    {
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use uuid::Uuid;
        use std::panic;

        let mut billionaires = Vec::new();
        for billionaire in $client.iter() {
            let mut collection = Vec::new();
            let billionaire_column = billionaire.columns();
            for billionaires in billionaire_column.iter() {
                let mut map = BTreeMap::new();

                let name = billionaires.name();
                let billion = billionaires.type_().name();

                // println!("{:?}", billion);

                let value = match billion.clone() {
                    "text" => {
                        let value: String = billionaire.get(name);
                        value
                    }
                    "date" => {
                        let value: NaiveDate = billionaire.get(name);
                        value.to_string()
                    }
                    "timestamp" => {
                        let value: NaiveDateTime = billionaire.get(name);
                        value.to_string()
                    }
                    "int4" => {
                        let value: i32 = billionaire.get(name);
                        value.to_string()
                    }
                                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                    }
                    "time" => {
                        let value: NaiveTime = billionaire.get(name);
                        value.to_string()
                    }
                    "uuid" => {
                        let value: Uuid = billionaire.get(name);
                        value.to_string()
                    }
                    "bool" => {
                        let value: bool = billionaire.get(name);
                        value.to_string()
                    }
                    _ => {
                        panic!("")
                    }
                };
                map.insert(name.to_string(), value);
                collection.push(map)
            }
            // println!("{:?}",collection);
            billionaires.push(collection);
        }
    billionaires
    }
  };
    (connection => $connection:expr,model:$model:expr,based_on:$search:expr,search:{
        value:$value:expr
    }) => {{
        let rank = format!(
            "SELECT * FROM {} WHERE to_tsvector('english',CAST({} AS TEXT)) @@  to_tsquery(CAST('{}' AS TEXT))",
            $model, $search, $value
        );
        // println!("{}",rank);
        let client = $connection.query(&rank, &[]).unwrap();
        full_search!(@format $connection,$model,client)
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
        full_search!(@format $connection,$model,client)
    }};
}

#[cfg(feature = "ranked_search")]
#[macro_export]
macro_rules! ranked_search {
    (@format $connection:expr,$model:expr,$client:expr) => {
    {
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use uuid::Uuid;
        use std::panic;

            let mut billionaires = Vec::new();
            for billionaire in $client.iter() {
                let mut collection = Vec::new();
                let billionaire_column = billionaire.columns();
                for billionaires in billionaire_column.iter() {
                    let mut map = BTreeMap::new();

                    let name = billionaires.name();
                    let billion = billionaires.type_().name();

                    // println!("{:?}", billion);

                    let value = match billion.clone() {
                        "text" => {
                            let value: String = billionaire.get(name);
                            value
                        }
                        "date" => {
                            let value: NaiveDate = billionaire.get(name);
                            value.to_string()
                        }
                        "timestamp" => {
                            let value: NaiveDateTime = billionaire.get(name);
                            value.to_string()
                        }
                        "int4" => {
                            let value: i32 = billionaire.get(name);
                            value.to_string()
                        }
                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                        }
                        "time" => {
                            let value: NaiveTime = billionaire.get(name);
                            value.to_string()
                        }
                        "uuid" => {
                            let value: Uuid = billionaire.get(name);
                            value.to_string()
                        }
                        "bool" => {
                            let value: bool = billionaire.get(name);
                            value.to_string()
                        }
                        _ => {
                            panic!("")
                        }
                    };
                    map.insert(name.to_string(), value);
                    collection.push(map)
                }
                // println!("{:?}",collection);
                billionaires.push(collection);
            }
        billionaires
    }
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
        ranked_search!(@format $connection,$model,selection,client)
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
        // println!("{}", partition);
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

#[cfg(feature = "geography")]
#[macro_export]
macro_rules! custome_query {
    (connection => $connection:expr,query:{$query:expr}) => {{
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use std::panic;
        use uuid::Uuid;

        let format = format!("{}", $query);
        let client = $connection.query(&format, &[]).unwrap();
        let mut billionaires = Vec::new();
        for billionaire in client.iter() {
            let mut collection = Vec::new();
            let billionaire_column = billionaire.columns();
            for billionaires in billionaire_column.iter() {
                let mut map = BTreeMap::new();

                let name = billionaires.name();
                let billion = billionaires.type_().name();

                // println!("{:?}", billion);

                let value = match billion.clone() {
                    "text" => {
                        let value: String = billionaire.get(name);
                        value
                    }
                    "date" => {
                        let value: NaiveDate = billionaire.get(name);
                        value.to_string()
                    }
                    "timestamp" => {
                        let value: NaiveDateTime = billionaire.get(name);
                        value.to_string()
                    }
                    "int4" => {
                        let value: i32 = billionaire.get(name);
                        value.to_string()
                    }
                    "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                    }
                    // "numeric" => {
                    //     let value: f64 = billionaire.get(name);
                    //     value.to_string()
                    // }
                    "time" => {
                        let value: NaiveTime = billionaire.get(name);
                        value.to_string()
                    }
                    "uuid" => {
                        let value: Uuid = billionaire.get(name);
                        value.to_string()
                    }
                    "bool" => {
                        let value: bool = billionaire.get(name);
                        value.to_string()
                    }
                    _ => {
                        panic!("Error:provided unknown value called {}", billion)
                    }
                };
                map.insert(name.to_string(), value);
                collection.push(map)
            }
            // println!("{:?}",collection);
            billionaires.push(collection);
        }
        billionaires
    }};
    (connection => $connection:expr,query:{$query:expr,value:{$($value:expr),*}}) => {{
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use postgres::types::ToSql;
        use std::collections::BTreeMap;
        use std::panic;
        use uuid::Uuid;

        let format = format!("{}", $query);
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let client = $connection.query(&format, &value).unwrap();
        let mut billionaires = Vec::new();
        for billionaire in client.iter() {
            let mut collection = Vec::new();
            let billionaire_column = billionaire.columns();
            for billionaires in billionaire_column.iter() {
                let mut map = BTreeMap::new();

                let name = billionaires.name();
                let billion = billionaires.type_().name();

                // println!("{:?}", billion);

                let value = match billion.clone() {
                    "text" => {
                        let value: String = billionaire.get(name);
                        value
                    }
                    "date" => {
                        let value: NaiveDate = billionaire.get(name);
                        value.to_string()
                    }
                    "timestamp" => {
                        let value: NaiveDateTime = billionaire.get(name);
                        value.to_string()
                    }
                    "int4" => {
                        let value: i32 = billionaire.get(name);
                        value.to_string()
                    }
                    "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                    }
                    "time" => {
                        let value: NaiveTime = billionaire.get(name);
                        value.to_string()
                    }
                    "uuid" => {
                        let value: Uuid = billionaire.get(name);
                        value.to_string()
                    }
                    "bool" => {
                        let value: bool = billionaire.get(name);
                        value.to_string()
                    }
                    _ => {
                        panic!("")
                    }
                };
                map.insert(name.to_string(), value);
                collection.push(map)
            }
            // println!("{:?}",collection);
            billionaires.push(collection);
        }
        billionaires
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
        let client = $connection.execute(&format, &value);
        client
    };
}

/// # Example
///
///
/// ```
/// let location = nearby_location! {
/// connection => postgres,
/// model:"shop",
/// select:{
///     "other_than_location_type"
/// },
/// location:{
///     lattitude:"12.971599",
///     longitude:"77.594566"
/// },
/// select_from:{
///     "location"
/// }
/// };
/// ```
///
///
/// ```
/// let location = nearby_location!(
/// connection => conn,
/// model: "places",
/// select: {"id", "name"},
/// location: {
///     lattitude: 37.7749,
///     longitude: -122.4194
/// },
/// select_from: {"geom"},
/// order: {"ASC"}
/// );
/// ```
/// ```
/// let location = nearby_location!(
/// connection => conn,
/// model: "restaurants",
/// select: {"id", "name", "rating"},
/// location: {
///     lattitude: 40.7128,
///     longitude: -74.0060,
///     within: 5000
/// },
/// select_from: {"location"},
/// order: {"ASC"}
/// );
/// ```
/// ```
/// let location = nearby_location!(
/// connection => conn,
/// model: "hotels",
/// select: {"id", "name"},
/// location: {
///     lattitude: 48.8566,
///     longitude: 2.3522,
///     within: 1000
/// },
/// select_from: {"coordinates"}
/// );
/// ```
/// ```
/// let location = nearby_location!(
/// connection => conn,
/// model: "landmarks",
/// select: {"id", "name", "address"},
/// location: {
///     lattitude: 51.5074,
///     longitude: -0.1278
/// },
/// select_from: {"geo_column"}
/// );
/// ```
#[macro_export]
macro_rules! nearby_location {
    (@format $connection:expr,$model:expr,$client:expr) => {{
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
        use uuid::Uuid;
        use std::panic;

        let mut billionaires = Vec::new();
        for billionaire in $client.iter() {
            let mut collection = Vec::new();
            let billionaire_column = billionaire.columns();
            for billionaires in billionaire_column.iter() {
                let mut map = BTreeMap::new();

                let name = billionaires.name();
                let billion = billionaires.type_().name();

                // println!("{:?}", billion);

                let value = match billion.clone() {
                    "text" => {
                        let value: String = billionaire.get(name);
                        value
                    }
                    "date" => {
                        let value: NaiveDate = billionaire.get(name);
                        value.to_string()
                    }
                    "timestamp" => {
                        let value: NaiveDateTime = billionaire.get(name);
                        value.to_string()
                    }
                    "int4" => {
                        let value: i32 = billionaire.get(name);
                        value.to_string()
                    }
                                        "int8" => {
                        let value: i64 = billionaire.get(name);
                        value.to_string()
                    }
                    "time" => {
                        let value: NaiveTime = billionaire.get(name);
                        value.to_string()
                    }
                    "uuid" => {
                        let value: Uuid = billionaire.get(name);
                        value.to_string()
                    }
                    "bool" => {
                        let value: bool = billionaire.get(name);
                        value.to_string()
                    }
                    "float8" => {
                        let value: f64 = billionaire.get(name);
                        value.to_string()
                    }
                    _ => {
                        panic!("{}", billion)
                    }
                };
                map.insert(name.to_string(), value);
                collection.push(map)
            }
            // println!("{:?}",collection);
            billionaires.push(collection);
        }
        billionaires
    }};
    (connection => $connection:expr,model:$model:expr,select:{$($select:expr),*},location:{
        lattitude:$lattitude:expr,
        longitude:$longitude:expr
    },select_from:{$select_location:expr},
    order:{$order:expr}
) => {
    {
        use postgres::types::ToSql;

        if $lattitude.is_empty() {
            panic!("Provide Value for lattitude in float")
        }
        if $longitude.is_empty() {
            panic!("Provide Value for longitude in float")
        }

        let mut selection = String::new();
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();

        let mut idx = 0;
        idx += 1;
        value.push(&$longitude);
        let longitude = format!("${}",idx);
        value.push(&$lattitude);
        idx += 1;
        let lattitude = format!("${}",idx);

        $(
            let select = format!("{},",$select);
            selection.push_str(&select);
        )*

        let selection = selection.trim_end_matches(",");

        let location = format!("WITH current_location AS (
            SELECT ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')') AS c_location
        )
        SELECT {},ST_AsGeoJson({}) as location,ST_Distance({}, cl.c_location) as distance FROM {} , current_location cl ORDER BY distance {};
        ",longitude,lattitude,selection,$select_location,$select_location,$model,$order.to_uppercase());
        // println!("{}",location);
        // println!("{:?}",value);
        let client = $connection.query(&location,&value).unwrap();
        nearby_location!(@format $connection,$model,client)
    }};
    (connection => $connection:expr,model:$model:expr,select:{$($select:expr),*},location:{
        lattitude:$lattitude:expr,
        longitude:$longitude:expr,
        within:$within:expr
    },select_from:{$select_location:expr},
    order:{$order:expr}
) => {
    {
        use postgres::types::ToSql;

        if $lattitude.is_empty() {
            panic!("Provide Value for lattitude in float")
        }
        if $longitude.is_empty() {
            panic!("Provide Value for longitude in float")
        }

        let mut selection = String::new();
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();

        let mut idx = 0;
        idx += 1;
        value.push(&$longitude);
        let longitude = format!("${}",idx);
        value.push(&$lattitude);
        idx += 1;
        let lattitude = format!("${}",idx);

        $(
            let select = format!("{},",$select);
            selection.push_str(&select);
        )*

        let selection = selection.trim_end_matches(",");

        let location = format!("WITH current_location AS (
            SELECT ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')') AS c_location
        )
        SELECT {},ST_AsGeoJson({}) as location,ST_Distance({}, cl.c_location) as distance FROM {} , current_location cl WHERE ST_DWithin({},cl.c_location,{}) ORDER BY distance {};
        ",longitude,lattitude,selection,$select_location,$select_location,$model,$select_location,$within,$order.to_uppercase());
        // println!("{}",location);
        // println!("{:?}",value);
        let client = $connection.query(&location,&value).unwrap();
        nearby_location!(@format $connection,$model,client)
    }};
    (connection => $connection:expr,model:$model:expr,select:{$($select:expr),*},location:{
        lattitude:$lattitude:expr,
        longitude:$longitude:expr,
        within:$within:expr
    },select_from:{$select_location:expr}
) => {
    {
        use postgres::types::ToSql;

        if $lattitude.is_empty() {
            panic!("Provide Value for lattitude in float")
        }
        if $longitude.is_empty() {
            panic!("Provide Value for longitude in float")
        }

        let mut selection = String::new();
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();

        let mut idx = 0;
        idx += 1;
        value.push(&$longitude);
        let longitude = format!("${}",idx);
        value.push(&$lattitude);
        idx += 1;
        let lattitude = format!("${}",idx);

        $(
            let select = format!("{},",$select);
            selection.push_str(&select);
        )*

        let selection = selection.trim_end_matches(",");

        let location = format!("WITH current_location AS (
            SELECT ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')') AS c_location
        )
        SELECT {},ST_AsGeoJson({}) as location,ST_Distance({}, cl.c_location) as distance FROM {} , current_location cl WHERE ST_DWithin({},cl.c_location,{});
        ",longitude,lattitude,selection,$select_location,$select_location,$model,$select_location,$within);
        // println!("{}",location);
        // println!("{:?}",value);
        let client = $connection.query(&location,&value).unwrap();
        nearby_location!(@format $connection,$model,client)
    }};
    (connection => $connection:expr,model:$model:expr,select:{$($select:expr),*},location:{
        lattitude:$lattitude:expr,
        longitude:$longitude:expr
    },select_from:{$select_location:expr}
) => {
    {
        if $lattitude.is_empty() {
            panic!("Provide Value for lattitude in float")
        }
        if $longitude.is_empty() {
            panic!("Provide Value for longitude in float")
        }

        use postgres::types::ToSql;

        let mut selection = String::new();
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();

        let mut idx = 0;
        idx += 1;
        value.push(&$longitude);
        let longitude = format!("${}",idx);
        value.push(&$lattitude);
        idx += 1;
        let lattitude = format!("${}",idx);

        $(
            let select = format!("{},",$select);
            selection.push_str(&select);
        )*

        let selection = selection.trim_end_matches(",");

        let location = format!("WITH current_location AS (
            SELECT ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')') AS c_location
        )
        SELECT {},ST_Distance({}, cl.c_location) as distance FROM {} , current_location cl;
        ",longitude,lattitude,selection,$select_location,$model);
        // println!("{}",location);
        // println!("{:?}",value);
        let client = $connection.query(&location,&value).unwrap();
        nearby_location!(@format $connection,$model,client)
    }
};
}
