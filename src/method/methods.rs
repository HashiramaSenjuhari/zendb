//  use rusty_postgres::method::types::{OneToMany,OneToOne};
#[macro_export]
macro_rules! model {
    ($model:expr => {$($billionaire:expr => {$($value:expr),*}),*}) => {
        {
            use rusty_postgres::serde_json;
            use std::panic;
            use rusty_postgres::rand::Rng;
            use rusty_postgres::rand;
            //  use rusty_postgres::method::types::{OneToMany,OneToOne};
            use rusty_postgres::method::types::{OneToMany, OneToOne};
            use rusty_postgres::rand::distributions::Alphanumeric;

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
                        panic!("Provide model name c    use model name is unknown {}",$billionaire);
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
                            if !primary_key.contains(&$billionaire.to_lowercase()){
                                primary_key.push_str(&format!("{},",$billionaire.to_lowercase()));
                            }
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
                    let key = format!("UNIQUE ({}),\r\n",unique);
                    // println!("{}",key);
                    table.push_str(&key);
                }
                if foriegn_key.len() != 0 {
                    // println!("{}",foriegn_key);
                    let foriegn_key = format!("{}",foriegn_key);
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
            use rusty_postgres::serde_json;
            use std::panic;
            use rusty_postgres::rand::Rng;
            use rusty_postgres::rand;
            //  use rusty_postgres::method::types::{OneToMany,OneToOne};
            use rusty_postgres::method::types::{OneToMany, OneToOne};
            use rusty_postgres::rand::distributions::Alphanumeric;

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
                        panic!("Provide model name c    use model name is unknown {}",$billionaire);
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
macro_rules! formats {
    ($client:expr) => {{
        use rusty_postgres::Uuid;
        use rusty_postgres::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::collections::BTreeMap;
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
                    "float4" => {
                        let value: f32 = billionaire.get(name);
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

///
/// # Usage
///
/// ```
/// let find = find_one! {
///     connection => postgres,
///     model:"place",
///     select:{
///         "name"
///     },
///     conditions:{
///         and => {
///             "name" => "billionairehari"
///         },
///         or => {
///             "name" => "billionairehari"
///         }
///     },
///     order:{
///         "name" => "asc"
///     }
/// };
/// ```
///
///
#[macro_export]
macro_rules! find_one {
    // * included and or order //completed
    (connection => $connection:expr,
            model:$model:expr
            $(,select:{
                $($select_value:expr),*
            })?
        $(,conditions:{
            $(and => {
                $($and_values:expr => $and_value:expr),*
            })?
            $(,)?
            $(or =>  {
                $($or_value1:expr => $or_value2:expr),*
            })?
        })?
        $(,order : {$($target:expr => $order:expr),*})?
    ) =>
    {
        {
            use std::panic;
            use rusty_postgres::ToSql;
            use rusty_postgres::formats;
            use std::io;

            let mut selectvalue = String::new();
            let mut conditions = String::new();
            let mut and_value:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut order = String::new();
            // let mut selection = String::new();
            $(
                $(
                selectvalue.push_str(&format!("{},",$select_value));
                // selection.push_str(&format!("'{}',",$select_value));
                )*
            )?
            let select_value = selectvalue.trim_end_matches(",");
            // let selection = selection.trim_end_matches(",");
            // println!("{}",select_value);
            let mut idx = 0;
            $(
                $(
                $(
                    idx+=1;
                    conditions.push_str(&format!("{} = ${} AND ",$and_values,idx));
                    and_value.push(&$and_value);
                )*
                )?
            )?
            // $()?
            $(
                $(
                    $(
                        idx+=1;
                        conditions.push_str(&format!("{} = ${} OR ",$or_value1,idx));
                        and_value.push(&$or_value2);
                    )*
                )?
            )?
            let conditions = conditions.trim_end_matches("AND ");
            let conditions = conditions.trim_end_matches("OR ");
            $(
                $(
                if !["asc","desc","ASC","DESC"].contains(&$order) {
                    panic!("Provide correct order either \"asc\" nor \"desc\"");
                }
                else {
                    let order_ = format!("{} {},",$target,$order.to_uppercase());
                    order.push_str(&order_);
                }
                )*
            )?
            let order = order.trim_end_matches(",");
            // println!("{}",and_values);
            // println!("{:?}",and_value);
            let mut select = format!("SELECT ");
            if select_value.len() != 0 {
                let selection = format!("{}",select_value);
                select.push_str(&selection);
            }
            else {
                let selection = format!("*");
                select.push_str(&selection);
            }
            select.push_str(&format!(" FROM {}",$model));
            if conditions.len() != 0 {
                select.push_str(&format!(" WHERE {}",conditions));
            }
            if order.len() != 0 {
                select.push_str(&format!(" ORDER BY {}",order));
            }
            select.push_str(" LIMIT 1");
            // println!("{}",select);
            // FROM {} WHERE {} ORDER BY {} LIMIT 1;
            let client = $connection.query(&select,&and_value);
            match client {
                Err(error) => {
                    Err(io::Error::new(io::ErrorKind::NotFound, error))
                },
                Ok(client) => {
                    let client = formats!{
                             client
                    };
                    Ok(client)
                }
            }
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

/// ## Returns
/// ```
/// Result<Vec<Vec<std::collections::BTreeMap<String, String>>>, io::Error>
/// ```
/// # Example
/// ## Basic
/// ```
///  let find = find_many! {
///     connection => postgres,
///     model:"billionaires",
///     select:{               // optional
///         "place"
///     },
///     case:{              //optional
///     (
///         "place" => ">22",
///         "place" => "<22",
///         "place" => "=22"
///         ) => (ok:"billion_dollar",
///         ok:"billionaire",
///         ok:"billionaire"
///         ,else:"trillionaire"
///     ) => "status"
///     },
///     conditions:{           // optional
///         and => {
///             "place" => "san"
///         },
///         or => {
///             "place" => "san"
///         }
///     },      
///     between => {                //optional
///         and => {
///             "place" => {
///                 "20" => "22"
///             }
///         },
///         or => {
///             "place" => {
///                 "20" => "22"
///             }
///         }
///     },
///     like => {               //optional
///         and => {
///             "name" => "billionaire"
///         },
///         or => {
///             "billionaire" => "billionaire"
///         }
///     },
///     inside:{                //optional
///         "place" => {
///             match:  user_id",
///             select:{
///                 "name"
///             },
///             conditions:{
///                 and => {
///                     "name" => "billionaire"
///                 }
///             },
///             order:6,
///             limit:6
///         }
///     },               
///     order:{                // optional
///         "place" => "asc"
///     },
///     limit:24,              // optional
///     skip:24                // optional
///  };
///  println!("{:?}", find);
/// ```
///
///
/// ## Geography Search
///
/// ```
/// let find = find_many! {
///     connection => postgres,
///     model:"billionaires",
///     select:{            // optional
///         "place"
///     },
///     within:{
///         lattitude:"12.971599",
///         longitude:"77.594566",
///         within:6                // optional
///     },
///     also_include:{
///         "location"
///     },
///     limit:6             // optional
/// };
/// ```
#[macro_export]
macro_rules! find_many {
    (connection => $connection:expr,
        model:$model:expr
        $(,match:$model_value:expr)?
        $(,select:{$($select_value:expr),*})?
        $(
            ,case:{
                    $(($($key:expr => $value:expr),*) => ($(ok:$ok:expr),*,else:$else:expr) => $case:expr),*
            }
        )?
        $(,conditions : {
            $(and => {
                $($and_key:expr => $and_value:expr),*
                }
            )?
            $(,)?
            $(
                or => {
                    $($or_key:expr => $or_value:expr),*
                }
            )?
        }
        )?
        $(,)?
        $(
            ,between => {
                $(and => {
                    $($between:expr => {$between_value:expr => $between_value2:expr}),*
                })?
                $(,)?
                $(or => {
                    $($between_or:expr => {$between_value_or:expr => $between_value2_or:expr}),*
                })?
            }
        )?
        $(,)?
        $(
            ,like => {
                $(and => {
                    $($like:expr => $like_value:expr),*
                })?
                $(,)?
                $(or => {
                    $($like_or:expr => $like_value_or:expr),*
                })?
            }
        )?
        $(,inside:
        {
            // (
            $include:expr
                => {
                    match:$include_match:expr,
                    $(select:
                        {
                            $($select:expr),*
                        }
                    ),*
                        $(,)?
                        $(
                            ,conditions: {
                                $(
                                    and => {
                                        $(
                                            $condition_key:expr => $condition_value:expr
                                        ),*
                                    }
                                )?
                                $(,)?
                                $(
                                    or => {
                                        $(
                                            $condition_or_key:expr => $condition_or_value:expr
                                            ),*
                                    }
                                )?
                            }
                        )?
                        $(,)?
                        $(
                            ,order:
                                {
                                    $(
                                        $order_key:expr => $order_value:expr
                                    )*
                                }
                        )?
                        $(,)?
                        $(
                            ,limit:$limit:expr
                        )?
                    }
                // ),*
        })?
        $(,)?
        $(,order:{
            $(
                $order:expr => $orderby:expr
            ),*
        })?
        $(,)?
        $(,limit:$main_limit:expr)?
        $(,)?
        $(,skip:$main_skip:expr)?
    ) => {
        {
            use rusty_postgres::ToSql;
            use std::io;
            use rusty_postgres::formats;

            let mut include = String::new();
            let mut relation = String::new();
            let mut selection = String::new();
            let mut case = String::new();

            let bb = $model.chars().nth(0).unwrap();
            let table_name = format!("{} {}",$model,bb);
            $(
                let table_dot = format!("{}.{}",bb,$model_value);
            )?
            let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();

            let mut idx = 0;
            $(
                $(
                    let selections = format!("{}.{},",bb,$select_value);
                    selection.push_str(&selections);
                )*
            )?

            $(
                $(
                    // $(
                        let mut cases = String::new();
                    $(
                            idx += 1;

                            let values_ = $value.to_string();
                            let value =  if values_.starts_with(">"){
                                let value = values_.trim_start_matches(">");
                                let between_and = format!(" WHEN CAST({} AS text) > ${} THEN '{}'",$key,idx,$ok);
                                cases.push_str(&between_and);
                                value
                                // values.push(&value);
                            }
                            else if values_.starts_with("<"){
                                let value = values_.trim_start_matches("<");
                                let between_and = format!(" WHEN CAST({} AS text) < ${} THEN '{}'",$key,idx,$ok);
                                cases.push_str(&between_and);
                                value
                                // values.push(&value);
                            }
                            else if values_.starts_with("="){
                                let value = values_.trim_start_matches("=");
                                let between_and = format!(" WHEN CAST({} AS text) = ${} THEN '{}'",$key,idx,$ok);
                                cases.push_str(&between_and);
                                value
                                // values.push(&value);
                            }
                            else {
                                let between_and = format!(" WHEN CAST({} AS text) = ${} THEN '{}'",$key,idx,$ok);
                                cases.push_str(&between_and);
                                // values.push(&$value);
                                $value
                            };
                            values.push(&value);

                        )*
                        let casey = format!(" CASE {} ELSE '{}' END AS {} ",cases,$else,$case);
                        case.push_str(&casey);
                    // )*
                )*
            )?
            // println!("{}",case);
            //inside
            $(
                let mut select = String::new();
                let mut inside_condition = String::new();
                let mut select = format!("(SELECT * FROM {}",$include);

                $(
                    $(
                        $(
                            idx+=1;
                            let and = format!("CAST({} AS text) = ${} AND ",$condition_key,idx);
                            inside_condition.push_str(&and);

                            values.push(&$condition_value);
                        )*
                    )*
                )?
                // let and = format!("({})",inside_condition);
                $(
                    $(
                        $(
                            idx+=1;
                            let or = format!("CAST({} AS text) = ${} OR ",$condition_or_key,idx);
                            inside_condition.push_str(&or);

                            values.push(&$condition_or_value);
                        )*
                    )*
                )?
                let inside_condition = inside_condition.trim_end_matches("AND ");
                let inside_condition = inside_condition.trim_end_matches("OR ");

                if inside_condition.len() != 0 {
                    let inside_condition = format!(" WHERE {}",inside_condition);
                    select.push_str(&inside_condition);
                }
                let mut order = String::new();
                $(
                    $(
                        if ["asc","desc","ASC","DESC"].contains(&$order_value){
                            let orders = format!("{} {},",$order_key,$order_value.to_uppercase());
                            order.push_str(&orders);
                        }
                        else {
                            panic!("Please Provide Corrent order either ASC nor DESC")
                        }
                    )*
                )?

                if order.len() != 0 {
                    let order = format!(" ORDER BY {}",order.trim_end_matches(","));
                    select.push_str(&order);
                }

                let b = $include.chars().nth(0).unwrap();

                $(
                    $(
                        let r_select = format!("{}.{},",b,$select);
                        relation.push_str(&r_select);
                    )*
                )*


                let mut limit = String::new();
                $(
                    let limits = format!("{}",$limit);
                    limit.push_str(&limits);
                )?

                if limit.len() != 0 {
                    let limit = format!(" LIMIT {}",limit);
                    select.push_str(&limit);
                }

                select.push_str(")");

                // let or = format!("({})",inside_condition);
                // println!("{}",select);

                let b = $include.chars().nth(0).unwrap();
                let r1 = format!("{} {}",$include,b);
                let r2 = format!("{}.{}",b,$include_match);
                let inside = format!("LEFT JOIN {} {} ON {} = {}",select,b,table_dot,r2);
                println!("{}",inside);

                include.push_str(&inside);
            )?

            // println!("{}",include);
            let mut conditions = String::new();
            $(
                $(
                    $(
                        idx += 1;
                        let and = format!("CAST({}.{} AS text) = ${} AND ",bb,$and_key,idx);
                        conditions.push_str(&and);

                        values.push(&$and_value);
                    )*
                )?
            )?
            $(
                $(
                    $(
                        idx += 1;
                        let or = format!("CAST({}.{} AS text) = ${} OR ",bb,$or_key,idx);
                        conditions.push_str(&or);

                        values.push(&$or_value);
                    )*
                )?
            )?
            $(
                $(
                    $(
                        idx+=1;
                        let first = format!("${}",idx);
                        values.push(&$between_value);
                        idx+=1;
                        let second = format!("${}",idx);
                        values.push(&$between_value2);
                        let between = format!(" CAST({} AS text) BETWEEN {} AND {} AND ",$between,first,second);
                        conditions.push_str(&between);
                    )*
                )*
            )?
            $(
                $(
                    $(
                        idx+=1;
                        let first = format!("${}",idx);
                        values.push(&$between_value_or);
                        idx+=1;
                        let second = format!("${}",idx);
                        values.push(&$between_value2_or);
                        let between = format!(" CAST({} AS text) BETWEEN {} AND {} OR ",$between_or,first,second);
                        conditions.push_str(&between);
                    )*
                )*
            )?
            $(
                $(
                    $(
                        idx+=1;
                        values.push(&$like_value);
                        let like = format!("CAST({} AS text) LIKE '%' || ${} || '%' AND ",$like,idx);
                        conditions.push_str(&like);
                    )*
                )*
            )?
            $(
                $(
                    $(
                        idx+=1;
                        values.push(&$like_value_or);
                        let like = format!("CAST({} AS text) LIKE '%' || ${} || '%' OR ",$like_or,idx);
                        conditions.push_str(&like);
                    )*
                )*
            )?
            let conditions = conditions.trim_end_matches("AND ");
            let conditions = conditions.trim_end_matches("OR ");
            // $(
                // let mut limit = String::new();
                // let mut order = String::new();
                // // let include_table = format!("{}.{}",b,$model_value);
                // // let relation_table = format!("{}.{}",b,$include);

                // let r1 = format!("{} {}",$include,b);
                // let r2 = format!("{}.{}",b,$match);

                // let order = order.trim_end_matches(",");
                // if !limit.is_empty() && !order.is_empty() {
                //     let include_format = format!("LEFT JOIN {} {} ON {} = {}\r\n",select,b,table_dot,r2);
                //     include.push_str(&include_format);
                // }
                // else if !limit.is_empty() && order.is_empty() {
                //     let select = format!("(SELECT * FROM {} LIMIT {})",$include,limit);
                //     let include_format = format!("LEFT JOIN {} {} ON {} = {}\r\n",select,b,table_dot,r2);
                //     include.push_str(&include_format);
                // }
                // else if limit.is_empty() && !order.is_empty() {
                //     let select = format!("(SELECT * FROM {} ORDER BY {})",$include,order);
                //     let include_format = format!("LEFT JOIN {} {} ON {} = {}\r\n",select,b,table_dot,r2);
                //     include.push_str(&include_format);
                // }
                // else  {
                //     let include_format = format!("LEFT JOIN {} ON {} = {}\r\n",r1,table_dot,r2);
                //     include.push_str(&include_format);
                // }
                // println!("{}",include_format);

            // )*

            let mut relation = relation.trim_end_matches(",").to_string();
            let selection = selection.trim_end_matches(",");

            let mut query = format!("SELECT ");
            if selection.len() != 0 {
                query.push_str(&selection);
            }
            if case.len() != 0 {
                let case = format!("{},",case);
                query.push_str(&case);
            }
            // else {
            //     query.push_str(&format!("{}.*,",bb));
            // }
            if relation.len() != 0 {
                if selection.len() != 0 {
                    query.push_str(&format!(",{}",relation));
                }
                else {
                    query.push_str(&format!("{}",relation));
                }
            }
            // else {
            //     $(
            //         let b = $include.chars().nth(0).unwrap();
            //         let r_select = format!("{}.*,",b);
            //         relation.push_str(&r_select);
            //     )*
            //     let relation = relation.trim_end_matches(",");
            //     query.push_str(relation);
            // }
            if table_name.len() != 0 {
                let from = format!(" FROM {} ",table_name);
                query.push_str(&from);
            }
            if include.len() != 0 {
                query.push_str(&include);
            }
            if conditions.len() != 0 {
                let conditions = format!(" WHERE {}",conditions);
                query.push_str(&conditions)
            }
            $(
                $(
                    if !["asc","desc","ASC","DESC"].contains(&$orderby){
                        panic!("Please Provide Correct Order ASC DESC")
                    }
                    else {
                        let order = format!("{} {}",$order,$orderby);
                        query.push_str(&format!(" ORDER BY {} {}",$order,$orderby))
                    }
                )*
            )?
            $(
                query.push_str(&format!(" LIMIT {}",$main_limit));
            )?
            $(
                query.push_str(&format!(" OFFSET {}",$main_skip));
            )?
            // println!("{}",conditions);
            // println!("{}",relation);
            println!("{}",query);
            // println!("{:?}",values);
            let client = $connection.query(&query,&values);
            match client {
                Err(error) => {
                    Err(io::Error::new(io::ErrorKind::NotFound, error))
                },
                Ok(client) => {
                    let client = formats!{
                             client
                    };
                    Ok(client)
                }
            }
        }
    };
    (connection => $connection:expr,model:$model:expr,select:{$($select_value:expr),*},
    within:{
        lattitude:$lattitude:expr,
        longitude:$longitude:expr
        $(,within:$within:expr)?
    },
    based_on:{
         $location:expr
    }$(,limit:$limit:expr)?) => {
        {
            use rusty_postgres::ToSql;
            use std::io;

            let mut selection = String::new();
            let mut location_value = String::new();
            let mut condition:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut select = String::new();
            let mut within = String::new();
            let mut limit = String::new();


            $(
                let select_value = format!("{},",$select_value);
                selection.push_str(&select_value);

                let selects = select_value.trim_end_matches(",");
                select.push_str(&format!("'{}',",selects));
            )*

            $(
                within.push_str(&$within.to_string());
            )?
            $(
                limit.push_str(&$limit.to_string());
            )?

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

            if !within.is_empty() && !limit.is_empty() {
                let query = format!("SELECT {} FROM {} WHERE ST_DWIthin({},ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')'),{}) LIMIT {};",selection,$model,$location,condition_longitude,condition_lattitude,within,limit);
                // println!("{}",query);
                // println!("{:?}",condition);
                let client = $connection.query(&query,&condition).unwrap();
                // client
                find_many!(@format client)
            }
            else if within.is_empty() && !limit.is_empty() {
                let query = format!("SELECT {} FROM {} WHERE ST_DWIthin({},ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')'),0) LIMIT {};",selection,$model,$location,condition_longitude,condition_lattitude,limit);
                // println!("{}",query);
                // println!("{:?}",condition);
                let client = $connection.query(&query,&condition).unwrap();
                // client
                find_many!(@format client)
            }
            else if !within.is_empty() && limit.is_empty() {
                let query = format!("SELECT {} FROM {} WHERE ST_DWIthin({},ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')'),{});",selection,$model,$location,condition_longitude,condition_lattitude,within);
                // println!("{}",query);
                // println!("{:?}",condition);
                let client = $connection.query(&query,&condition).unwrap();
                // client
                find_many!(@format client)
            }
            else  {
                let query = format!("SELECT {} FROM {} WHERE ST_DWIthin({},ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')'),0);",selection,$model,$location,condition_longitude,condition_lattitude);
                // println!("{}",query);
                // println!("{:?}",condition);
                let client = $connection.query(&query,&condition);
                // client
                match client {
                    Err(error) => {
                        Err(io::Error::new(io::ErrorKind::NotFound, error))
                    },
                    Ok(client) => {
                        let client = formats!{
                                 client
                        };
                        Ok(client)
                    }
                }
            }
        }
    };
}

///
/// # Usage
///
/// ```
/// let drop = delete_many!{
///     connection => postgres,
///     model:"place"
/// };
/// ```
///
///
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
/// let delete = delete! {
///     connection => postgres,
///     model:"billionaires",
///     select:{                // Optional
///         "place"
///     },
///     conditions:{            // Optional
///         and => {            // Optional
///             "place" => 24 as i32
///         },
///         or => {             // Optional
///             "place" => 24 as i32
///         }
///     },
///     cascade:true            // Optional
/// };
/// ```
#[macro_export]
macro_rules! delete {
    // * and select
    (connection => $connection:expr,
        model:$model:expr
    $(,
        select:{
            $($value:expr),*
        }
    )?
    $(
        ,conditions:{
            $(and => {
                $($cak:expr => $cav:expr),*
            })?
            $(,)?
            $(or => {
                $($cok:expr => $cov:expr),*
            })?
        }
    )?
    $(
        ,between => {
            $(and => {
                $($between:expr => {$between_value:expr => $between_value2:expr}),*
            })?
            $(,)?
            $(or => {
                $($between_or:expr => {$between_value_or:expr => $between_value2_or:expr}),*
            })?
        }
    )?
    $(
        ,cascade:$cascade:expr
    )?
        ) => {{
        use rusty_postgres::ToSql;
        use rusty_postgres::formats;
        use std::io;

        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut selection = String::new();
        let mut condition = String::new();

        if $model.is_empty() {
            panic!("{}","Please Provide Table Name")
        }

        let mut idx = 0;

        // conditions

        $(
            $(
                $(
                    if $cak.is_empty(){
                        panic!("{}","Please Provide Condition for AND Values")
                    }
                    idx += 1;
                    let and=  format!("{} = ${} AND ",$cak,idx);
                    condition.push_str(&and);
                    value.push(&$cav);
                )*
            )?
            $(
                $(
                    if $cok.is_empty(){
                        panic!("{}","Please Provide Condition for OR Values")
                    }
                    idx += 1;
                    let and=  format!("{} = ${} OR ",$cok,idx);
                    condition.push_str(&and);
                    value.push(&$cov);
                )*
            )?
        )?
        $(
            $(
                $(
                    idx+=1;
                    let first = format!("${}",idx);
                    value.push(&$between_value);
                    idx+=1;
                    let second = format!("${}",idx);
                    value.push(&$between_value2);
                    let between = format!(" CAST({} AS text) BETWEEN {} AND {} AND ",$between,first,second);
                    condition.push_str(&between);
                )*
            )*
        )?
        $(
            $(
                $(
                    idx+=1;
                    let first = format!("${}",idx);
                    value.push(&$between_value_or);
                    idx+=1;
                    let second = format!("${}",idx);
                    value.push(&$between_value2_or);
                    let between = format!(" CAST({} AS text) BETWEEN {} AND {} OR ",$between_or,first,second);
                    condition.push_str(&between);
                )*
            )*
        )?
        let condition = condition.trim_end_matches("OR ");
        let condition = condition.trim_end_matches("AND ");

        $(
            $(
                if $value.is_empty(){
                    panic!("{}","Please Provide Select Values")
                }
                selection.push_str(&format!("{},",$value));
            )*
        )?
        let selection = selection.trim_end_matches(",");

        let mut delete = format!("DELETE FROM {}",$model);
        if condition.len() != 0 {
            let condition = format!(" WHERE {}",condition);
            delete.push_str(&condition);
        }
        if selection.len() != 0 {
            let selection = format!(" RETURNING {}",selection);
            delete.push_str(&selection);
        }
        $(
            if $cascade {
                delete.push_str(" CASECADE");
            }
        )?
        delete.push_str(";");
        println!("{}",delete);
        // println!("{:?}",value);
        let client = $connection.query(&delete,&value);
        // println!("{:?}",client);
        match client {
            Err(error) => {
                Err(io::Error::new(io::ErrorKind::NotFound, error))
            },
            Ok(client) => {
                let client = formats!{
                         client
                };
                Ok(client)
            }
        }

    }};
}

///
/// # Example
///
/// ```
/// let update = update! {
///     connection => postgres,
///     model:"place",
///     select:{
///         "id"
///     },
///     data:{
///         "name" => "billionairehari"
///     },
///     conditions:{
///         and => {
///             "name" => "billionairehari"
///         },
///         or => {
///             "" => ""
///         }
///     }
/// };
/// ```
///
/// ## Usage
/// ```
/// let update = update! {
///        connection => postgres,
///        model:"billionaires",
///        match:"id",
///        inside:{
///            "place"  => {
///                match:   user_id",
///                conditions:{
///                    and => {
///                        "name" => "billionaires",
///                         user_id" => "c4a97a50-8679-4f85-a1d8-5bba0113b596"
///                    }
///                },
///                data:{
///                    "name" => "billionairehari"
///                },
///                select:{
///                    "name"
///                }
///            }
///        }
///    };
/// ```
///
#[macro_export]
// * data conditions
macro_rules! update {
    // * data select conditions
    (connection => $connection:expr,model : $model:expr
    $(,select:{
        $($select:expr),*
    })?
      ,data:{
        $($from:expr => $data:expr),*
    }
    $(,conditions:{
        $(and => {$($conditions:expr => $value:expr),*})?
        $(,)?
        $(or => {$($conditions_or:expr => $value_or:expr),*})?
    })?
    $(
        ,between => {
            $(and => {
                $($between:expr => {$between_value:expr => $between_value2:expr}),*
            })?
            $(,)?
            $(or => {
                $($between_or:expr => {$between_value_or:expr => $between_value2_or:expr}),*
            })?
        }
    )?
    ) => {{
        use rusty_postgres::ToSql;
        use rusty_postgres::formats;
        use std::io;

        let mut condition = String::new();
        let mut set = String::new();
        let mut select = String::new();
        let mut idx = 0;
        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();

        $(
            idx+=1;
            let update = format!("{} = ${},",$from,idx);
            set.push_str(&update);

            value.push(&$data);
        )*
        $(
            $(
                $(
                    idx+=1;
                    let and = format!("CAST({} AS TEXT) = ${} AND ",$conditions,idx);
                    condition.push_str(&and);

                    value.push(&$value);
                )*
            )?
        )?
        $(
            $(
                $(
                    idx+=1;
                    let or = format!("CAST({} AS TEXT) = ${} OR ",$conditions_or,idx);
                    condition.push_str(&or);

                    value.push(&$value_or);
                )*
            )?
        )?
        $(
            $(
                $(
                    idx+=1;
                    let first = format!("${}",idx);
                    value.push(&$between_value);
                    idx+=1;
                    let second = format!("${}",idx);
                    value.push(&$between_value2);
                    let between = format!(" CAST({} AS text) BETWEEN {} AND {} AND ",$between,first,second);
                    condition.push_str(&between);
                )*
            )*
        )?
        $(
            $(
                $(
                    idx+=1;
                    let first = format!("${}",idx);
                    value.push(&$between_value_or);
                    idx+=1;
                    let second = format!("${}",idx);
                    value.push(&$between_value2_or);
                    let between = format!(" CAST({} AS text) BETWEEN {} AND {} OR ",$between_or,first,second);
                    condition.push_str(&between);
                )*
            )*
        )?
        $(
            $(
                let selection = format!("{},",$select);
                select.push_str(&selection);
            )*
        )?
        let set = set.trim_end_matches(",");
        let select = select.trim_end_matches(",");
        let condition = condition.trim_end_matches("OR ");
        let condition = condition.trim_end_matches("AND ");
        // println!("{}",set);
        // println!("{}",select);
        // println!("{}",condition);
        // println!("{:?}",value);
        let mut query = format!("UPDATE {} SET {}",$model,set);
        if condition.len() != 0 {
            let condition = format!(" WHERE {}",condition);
            query.push_str(&condition);
        }
        if select.len() != 0 {
            let select = format!(" RETURNING {}",select);
            query.push_str(&select);
        }
        query.push_str(";");
        println!("{}",query);
        let client = $connection.query(&query,&value);
        match client {
            Err(error) => {
                Err(io::Error::new(io::ErrorKind::NotFound, error))
            },
            Ok(client) => {
                let client = formats!{
                        client
                };
                Ok(client)
            }
        }
    }};
    (connection => $connection:expr,
        model:$model:expr,
        match:$model_value:expr
        $(,select:{
            $($select:expr),*
        })?
        $(,conditions:{
            $(and => {$($conditions:expr => $value:expr),*})?
            $(or => {$($conditions_or:expr => $value_or:expr),*})?
        })?
        $(
            ,between => {
                $(and => {
                    $($between:expr => {$between_value:expr => $between_value2:expr}),*
                })?
                $(,)?
                $(or => {
                    $($between_or:expr => {$between_value_or:expr => $between_value2_or:expr}),*
                })?
            }
        )?
        ,inside:
        {
            $from:expr => {
                match:$match:expr,
                data:{
                    $($data_from:expr => $data_value:expr),*
                }
                // $(,)?
                $(,conditions : {
                    $(and => {$($and_from_key:expr => $and_from_value:expr),*})?
                    $(,)?
                    $(or => {$($or_from_key:expr => $or_from_value:expr),*})?
                })?
                $(,)?
                $(
                    ,select:{
                    $(
                        $select_from:expr
                    ),*
                    }
                )?
                $(
                    ,between => {
                        $(and => {
                            $($from_between:expr => {$from_between_value:expr => $from_between_value2:expr}),*
                        })?
                        $(,)?
                        $(or => {
                            $($from_between_or:expr => {$from_between_value_or:expr => $from_between_value2_or:expr}),*
                        })?
                    }
                )?
            }
        }
        ) => {{
            use rusty_postgres::ToSql;
            use std::io;
            // use rusty_postgres::Uuid;


            let mut relation = String::new();
            let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
            let mut updates = String::new();
            use rusty_postgres::formats;

            // $(
                // $(
                let mut idx = 0;
            $(
                let from = $from;
                let mut selection = String::new();
                let mut set = String::new();
                let mut conditions = String::new();

                idx +=1;
                let update = format!("{} = ${},",$data_from,idx);
                    // println!("{}",update);
                set.push_str(&update);
                value.push(&$data_value);

            )*
            $(
                $(
                    $(
                        idx+=1;
                        // let b = $from.chars().nth(0).unwrap();
                        let condition = format!("CAST({}.{} AS text) = ${} AND ",$from,$and_from_key,idx);
                        conditions.push_str(&condition);

                        value.push(&$and_from_value);
                    )*
                )?
                $(
                    $(
                        idx+=1;
                        let condition = format!("CAST({}.{} AS text) = ${} OR ",$from,$or_from_key,idx);
                        conditions.push_str(&condition);

                        value.push(&$or_from_value);
                    )*
                )?
            )*
            $(
                $(
                    $(
                        idx+=1;
                        let and = format!("CAST({}.{} AS TEXT) = ${} AND ",$model,$conditions,idx);
                        conditions.push_str(&and);

                        value.push(&$value);
                    )*
                )?
            )?
            $(
                $(
                    $(
                        idx+=1;
                        let or = format!("CAST({}.{} AS TEXT) = ${} OR ",$model,$conditions_or,idx);
                        conditions.push_str(&or);

                        value.push(&$value_or);
                    )*
                )?
            )?
            $(
                $(
                    $(
                        idx+=1;
                        let first = format!("${}",idx);
                        value.push(&$between_value);
                        idx+=1;
                        let second = format!("${}",idx);
                        value.push(&$between_value2);
                        let between = format!(" CAST({}.{} AS text) BETWEEN {} AND {} AND ",$model,$between,first,second);
                        conditions.push_str(&between);
                    )*
                )*
            )?
            $(
                $(
                    $(
                        idx+=1;
                        let first = format!("${}",idx);
                        value.push(&$between_value_or);
                        idx+=1;
                        let second = format!("${}",idx);
                        value.push(&$between_value2_or);
                        let between = format!(" CAST({}.{} AS text) BETWEEN {} AND {} OR ",$model,$between_or,first,second);
                        conditions.push_str(&between);
                    )*
                )*
            )?
            $(
                $(
                    $(
                        idx+=1;
                        let first = format!("${}",idx);
                        value.push(&$from_between_value);
                        idx+=1;
                        let second = format!("${}",idx);
                        value.push(&$from_between_value2);
                        let between = format!(" CAST({}.{} AS text) BETWEEN {} AND {} AND ",$from,$from_between,first,second);
                        conditions.push_str(&between);
                    )*
                )*
            )?
            $(
                $(
                    $(
                        idx+=1;
                        let first = format!("${}",idx);
                        value.push(&$from_between_value_or);
                        idx+=1;
                        let second = format!("${}",idx);
                        value.push(&$from_between_value2_or);
                        let between = format!(" CAST({}.{} AS text) BETWEEN {} AND {} OR ",$from,$from_between_or,first,second);
                        conditions.push_str(&between);
                    )*
                )*
            )?
            $(
                $(
                    let select = format!("{},",$select_from);
                    selection.push_str(&select);
                )*
                let conditions = conditions.trim_end_matches("AND ");
                let conditions = conditions.trim_end_matches("OR ");
                let mut condition = format!("{}.{} = {}.{} AND ",$model,$model_value,$from,$match);
                if conditions.len() != 0 {
                    condition.push_str(&format!("({})",conditions));
                }
                let condition = condition.trim_end_matches("AND ");
                let condition = condition.trim_end_matches("OR ");
                let selection = selection.trim_end_matches(",");
                let set = set.trim_end_matches(",");
                // println!("{}",set);
                let mut update = format!("UPDATE {}",$from);
                if set.len() != 0 {
                    let set = format!(" SET {}",set);
                    update.push_str(&set);
                }
                update.push_str(&format!(" FROM {}",$model));
                if condition.len() != 0 {
                    let condition = format!(" WHERE {}",condition);
                    update.push_str(&condition);
                }
                if selection.len() != 0 {
                    let selection = format!(" RETURNING {}",selection);
                    update.push_str(&selection);
                }
                update.push_str(";");
                // println!("{}",update);
                updates.push_str(&update);
            )*
            // )*
            // let transactions = format!("{}",updates);
            // println!("{}",transactions);
                // )*
            // )?
            // println!("{}",updates);
            // println!("{:?}",value);
            let client = $connection.query(&updates,&value);
            // println!("{:?}",client);
            match client {
                Err(error) => {
                    Err(io::Error::new(io::ErrorKind::NotFound, error))
                },
                Ok(client) => {
                    let client = formats!{
                             client
                    };
                    Ok(client)
                }
            }
        }};
}

/// # Example
///
/// ```
/// let create = create! {
///     connection => postgres,
///     model:"shop",
///     data:{
///         "place" => "san",
///         "age" => 24 as i32,
///         "bool" => true
///     }
/// };
/// ```
///
///```
/// let create = create! {
///     connection => postgres,
///     model:  user_",
///     data:{
///         "story" => "billionairehari",
///         "age" => 24 as i32
///     },
///     select:{
///         "id"
///     }
/// };
/// ```
#[macro_export]
macro_rules! create {
    (connection => $connection:expr,model:$model:expr,data:{
        $($from:expr => $data:expr),*
    }
    $(
        ,select:{
            $($select_value:expr),*
        }
    )?) => {
        {
        use rusty_postgres::ToSql;
        use rusty_postgres::formats;
        use std::io;

        let mut data = String::new();
        let mut data_value = String::new();
        let mut value:Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut select_value = String::new();
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
            $(
                select_value.push_str(&format!("{},",$select_value));
            )*
        )?
        let data = data.trim_end_matches(",");
        let data_value = data_value.trim_end_matches(",");
        let select = select_value.trim_end_matches(",");
        let mut create = format!("INSERT INTO {} ({}) VALUES ({})",$model,data,data_value);
        // println!("{}",create);
        if select_value.len() != 0 {
            create.push_str(&format!(" RETURNING {};",select))
        }
        // println!("{:?}",create);

        let client = $connection.query(&create,&value);
        match client {
            Err(error) => {
                Err(io::Error::new(io::ErrorKind::NotFound, error))
            },
            Ok(client) => {
                let client = formats!{
                         client
                };
                Ok(client)
            }
        }
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
///
/// # Example
///
/// ```
/// let search = similar_search {
///     connection => postgres,
///     model:"place",
///     similarity:{
///         score:"0.6", //similarity score
///         believe:"name" //based_on
///         text:"san" //text
///     },
///     order_by:{              //optional
///         order:"asc",
///         believe:"name" //based
///     }
/// }
/// ```
///
#[cfg(feature = "similar_search")]
#[macro_export]
macro_rules! similar_search {
    (connection => $connection:expr,
        model:$model:expr,
        similarity:{
        score:$score:expr,
        believe:$believe:expr,
        text:$text:expr
    }
    $(,order_by:{
        order:$order:expr,
        believe:$o_believe:expr,
        text:$o_text:expr
    })?
) => {{
        use rusty_postgres::{NaiveDate, NaiveDateTime, NaiveTime};
        use std::panic;
        use std::collections::{BTreeMap, HashMap};
        use rusty_postgres::Uuid;
        use rusty_postgres::formats;
        use std::io;

        let mut search = format!(
            "SELECT * FROM {} WHERE similarity({},'{}') > {} ",
            $model, $believe, $text, $score
        );
        $(
            let order = format!("ORDER BY similarity({},'{}') {}", $o_believe, $o_text, $order);
            search.push_str(&order);
        )?
        // println!("{}", search);
        let client = $connection.query(&search, &[]);
        match client {
            Err(error) => {
                Err(io::Error::new(io::ErrorKind::NotFound, error))
            },
            Ok(client) => {
                let client = formats!{
                         client
                };
                Ok(client)
            }
        }
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

///
/// # Usage
///
/// ```
/// let count = count! {
///     connection => postgres,
///     model:"place",
///     count:{
///         "name"
///     },
///     conditions:{
///         and => {
///             "name" => "billionaires"
///         }
///     },
///     group_by:{
///         "name"
///     }
/// };
/// ```
///
///
#[cfg(feature = "count")]
#[macro_export]
macro_rules! count {
    (connection => $connection:expr,
    model:$model:expr,
    count:{
        $($count_value:expr),*
    },
    $(
        conditions:{
            $(and => {$($condition:expr => $value:expr),*})?
            $(or => {$($or_condition:expr => $or_value:expr),*})?
        }
    )?
    $(
       ,group_by:{
            $($group_value:expr),*
         }
    )?
    ) => {{
        use std::collections::HashMap;
        use rusty_postgres::ToSql;

         let mut value = String::new();
         let mut conditions = String::new();
         let mut values:Vec<&(dyn ToSql + Sync)> = Vec::new();
         let mut groupby = String::new();
         let mut idx = 0;
         $(
             value.push_str(&format!("{},",&$count_value));
         )*
        $(
            $(
                groupby.push_str(&format!("{},",&$group_value));
            )*
        )?
         $(
            $(
                $(
                    idx += 1;
                    conditions.push_str(&format!("CAST({} AS TEXT) = ${} AND ",$condition,idx));
                    values.push(&$value);
                )*
            )?
        )?
        $(
            $(
                $(
                idx += 1;
                conditions.push_str(&format!("CAST({} AS TEXT) = ${} OR ",$or_condition,idx));
                values.push(&$or_value);
                )*
            )?
        )?
         let value = value.trim_end_matches(",");
         let conditions = conditions.trim_end_matches("AND ");
         let conditions = conditions.trim_end_matches("OR ");
         let groupby = groupby.trim_end_matches(",");
         let mut count = format!("SELECT COUNT(DISTINCT({})) FROM {}", value,$model);
        //  println!("{}",count);
        //  println!("{:?}",values);
        if conditions.len() != 0 {
            count.push_str(&format!(" WHERE {}",conditions));
        }
        if groupby.len() != 0 {
            count.push_str(&format!(" GROUP BY {}",groupby));
        }
        count.push_str(";");
        // println!("{}",count);
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
}

///
/// # Usage
///
/// ```
/// let search = full_search! {
///     connection => postgres,
///     model:"place",
///     based_on:"name",
///     search:{
///         value:"billionaire"
///     },
///     select:{
///         ,"name"
///     },
///     take:6,
///     skip:0
/// };
/// ```
///
// #[cfg(feature = "full_search")]
#[macro_export]
macro_rules! full_search {
    (connection => $connection:expr,model:$model:expr,based_on:$search:expr,search:{
        value:$value:expr
    }$(
        ,select:{
            $($select:expr),*
        }
    )?
    $(,take:$take:expr)?
    $(,skip:$skip:expr)?
    ) => {{
        use std::io;
        use rusty_postgres::formats;
        let mut selection = String::new();
        $(
            $(
                selection.push_str(&format!("{},",$select));
            )*
        )?
        let selection = selection.trim_end_matches(",");

        let mut query = format!("SELECT ");
        // println!("{}",rank);

        if selection.len() != 0 {
            query.push_str(&format!("{}",selection));
        }else {
            query.push_str("*");
        }
        query.push_str(&format!(" FROM {} WHERE to_tsvector('english',CAST({} AS TEXT)) @@  to_tsquery(CAST('{}' AS TEXT))",$model, $search, $value));
        $(
            query.push_str(&format!(" LIMIT {}",$take));
        )?
        $(
            query.push_str(&format!(" OFFSET {}",$skip));
        )?
        let client = $connection.query(&query, &[]);
        match client {
            Err(error) => {
                Err(io::Error::new(io::ErrorKind::NotFound, error))
            },
            Ok(client) => {
                let client = formats!{
                         client
                };
                Ok(client)
            }
        }
    }};
}

///
/// # Usage
///
/// ```
/// let search = ranked_search! {
///     connection => postgres,
///     model:"place",
///     based_on:"name",
///     search:{
///         value:"billionaire"
///     },
///     select:{
///         "name"
///     }
/// };
/// ```
///
///
// #[cfg(feature = "ranked_search")]
#[macro_export]
macro_rules! ranked_search {
    (connection => $connection:expr,model:$model:expr,based_on:$search:expr,search:{
        value:$value:expr
    },$(
        select:{
            $($select:expr),*
        }
    )?) => {{
        use std::io;
        use rusty_postgres::formats;

        #[derive(Debug,Clone)]
        struct Score {
            score:f32,
            data:String
        };
        let mut select = String::new();
        let mut selection = String::new();
        $(
            $(
                select.push_str(&format!("{},",$select));
                selection.push_str(&format!("'{}',",$select));
            )*
        )?
        let select = select.trim_end_matches(",");
        let selection = selection.trim_end_matches(",");

        if selection.len() != 0 {
            let rank = format!(
                "SELECT {} , ts_rank_cd(to_tsvector('english',{}),to_tsquery('{}')) FROM {} WHERE to_tsvector('english',CAST({} AS TEXT)) @@  to_tsquery(CAST('{}' AS TEXT))",
                select,$search,$value,$model, $search, $value
            );
            let client = $connection.query(&rank, &[]);
            println!("{:?}",client);
            match client {
                Err(error) => {
                    Err(io::Error::new(io::ErrorKind::NotFound, error))
                },
                Ok(client) => {
                    let client = formats!{
                             client
                    };
                    Ok(client)
                }
            }
        }
        else {
            let rank = format!(
                "SELECT * , ts_rank_cd(to_tsvector('english',{}),to_tsquery('{}')) FROM {} WHERE to_tsvector('english',CAST({} AS TEXT)) @@  to_tsquery(CAST('{}' AS TEXT))",
                $search,$value,$model, $search, $value
            );
            let client = $connection.query(&rank, &[]);
            println!("{:?}",client);
            match client {
                Err(error) => {
                    Err(io::Error::new(io::ErrorKind::NotFound, error))
                },
                Ok(client) => {
                    let client = formats!{
                             client
                    };
                    Ok(client)
                }
            }
        }
        // println!("{}",rank);
    }};
}

///
/// # Usage
///
/// ```
/// let partition = create_partition {
///     connection => postgres,
///     model:"place",
///     name:"partition_name",
///     field:"value_to_match"
/// }
/// ```
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

///
/// # Usage
///
/// ```
/// horizontal_splitting {
///     connection => postgres,
///     model:"",
///     name:"name_of_spllit",
///     based_on:{
///     "country" => "usa"    
///     }
/// }
/// ```
#[cfg(feature = "horizontal_split")]
#[macro_export]
macro_rules! horizontal_splitting {
    (connection => $connection:expr,model:$model:expr,name:$name:expr,based_on:{$based_on:expr => $value:expr}) => {{
        use rand::Rng;
        use rusty_postgres::Alphanumeric;

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

///
/// # Usage
///
/// ```
/// let query = custome_query {
///     connection => postgres,
///     query:{
///         "SELECT * FROM place WHERE name = $1"
///     },
///     value:{
///         "billionaires"
///     }
/// }
/// ```
// #[cfg(feature = "geography")]
#[macro_export]
macro_rules! custome_query {
    (connection => $connection:expr,query:{$($query:expr),*} $(,value:{$($value:expr),*})?) => {{
        use rusty_postgres::{NaiveDate, NaiveDateTime, NaiveTime};
        use rusty_postgres::ToSql;
        use std::collections::BTreeMap;
        use std::panic;
        use rusty_postgres::uuid::Uuid;

        let mut query = String::new();
        $(
            let format = format!("{}\r\n", $query);
            query.push_str(&format);
        )*
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();
        $(
            $(
                value.push(&$value);
            )*
        )?
        let client = $connection.query(&format, &value);
        match client {
            Err(error) => {
                Err(io::Error::new(io::ErrorKind::NotFound, error))
            },
            Ok(client) => {
                let client = formats!{
                         client
                };
                Ok(client)
            }
        }
    }};
}

///
/// # Usage
///
/// ```
/// let partition = custome_execute! {
///     connection => postgres,
///     query:{
///         "INSERT INTO place VALUES ($1)"
///     },
///     value:{
///         "billionaires"
///     }
/// };
/// ```
#[macro_export]
macro_rules! custome_execute {
    (connection => $connection:expr,query:{$($query:expr),*} $(,value:{$($value:expr),*})?) => {
        {
            use rusty_postgres::ToSql;

            let mut query = String::new();
            $(
                let format = format!("{}\r\n", $query);
                query.push_str(&format);
            )*
            let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();
            $(
                $(
                    value.push(&$value);
                )*
            )?
            let client = $connection.execute(&format, &value);
            client
        }
    };
}

/// # Example
///
///
/// ```
/// let location = nearby_location! {
///     connection => postgres,
///     model:"billionaires",
///     select:{
///         "place"
///     },
///     location:{
///         lattitude:"12.971560",
///         longitude:"77.594560",
///         within:"4000"               //optional
///     },
///     select_from:{
///         "location"
///     },
///     order:{                 //optiona
///         location:"asc",     //optional
///         "place" => "asc"    //optional
///     }
/// };
/// ```
#[macro_export]
#[cfg(feature = "geography")]
macro_rules! nearby_location {
    (connection => $connection:expr,model:$model:expr
        $(
            ,select:{
                $($select:expr),*
            }
        )?
        ,location:{
            lattitude:$lattitude:expr,
            longitude:$longitude:expr
            $(,within:$within:expr)?
    },select_from:{
        $select_location:expr
    }
    $(
        ,order:{
            $(location:$location_order:expr)?
            $(
                ,$(
                    $order:expr => $orderby:expr
                ),*
            )?
        }
    )?
) => {
    {
        use rusty_postgres::ToSql;
        use std::io;
        use rusty_postgres::formats;

        if $lattitude.is_empty() {
            panic!("Provide Value for lattitude in float")
        }
        if $longitude.is_empty() {
            panic!("Provide Value for longitude in float")
        }

        let mut selection = String::new();
        let mut value: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut order = String::new();

        let mut idx = 0;
        idx += 1;
        value.push(&$longitude);
        let longitude = format!("${}",idx);
        value.push(&$lattitude);
        idx += 1;
        let lattitude = format!("${}",idx);

        $(
            $(
                let select = format!("{},",$select);
                selection.push_str(&select);
            )*
        )?
        $(
            $(
                $(
                    if !["asc","desc","ASC","DESC"].contains(&$orderby){
                        panic!("Provide Correct Order")
                    }
                    else {
                        let orders = format!("{} {},",$order,$orderby.to_uppercase());
                        order.push_str(&orders);
                    }
                )?
            )*
        )?

        let selection = selection.trim_end_matches(",");
        let order = order.trim_end_matches(",");

        let mut location = format!("WITH current_location AS (
            SELECT ST_GeogFromText('SRID=4326;POINT('||{}||' '||{}||')') AS c_location
        )
        SELECT {},ST_AsGeoJson({}) as location,ST_Distance({}, cl.c_location) as distance FROM {} , current_location cl WHERE ST_DWithin({},cl.c_location
        ",longitude,lattitude,selection,$select_location,$select_location,$model,$select_location);
        // ,{})
        $(
            location.push_str(&format!(",{})",$within));
        )?
        if !location.ends_with(")"){
            location.push_str(",0)");
        }
        $(
            $(
                let orders = format!(" ORDER BY distance {}",$location_order);
                location.push_str(&orders);
            )?
        )?

        if order.len() != 0 {
            if location.contains("ORDER BY"){
                location.push_str(&format!(",{}",order));
            }
            else{
                location.push_str(&format!(" ORDER BY {}",order));
            }
        }
        println!("{}",location);
        println!("{:?}",value);
        let client = $connection.query(&location,&value);
        match client {
            Err(error) => {
                Err(io::Error::new(io::ErrorKind::NotFound, error))
            },
            Ok(client) => {
                let client = formats!{
                         client
                };
                Ok(client)
            }
        }
    }};
}
