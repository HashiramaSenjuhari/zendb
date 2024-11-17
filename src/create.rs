use std::marker::PhantomData;

use postgres::{types::ToSql, Client};

use crate::{
    Create, CreateBuilder, KeyNotPresent, KeyPresent, TableNotPresent, TablePresent,
    ValueNotPresent, ValuePresent,
};

impl<'create> CreateBuilder<'create, TableNotPresent, KeyNotPresent, ValueNotPresent> {
    pub fn new(conn: &'create mut Client) -> Self {
        Self {
            conn: Some(conn),
            table: Vec::new(),
            key: Vec::new(),
            value: Vec::new(),
            table_state: std::marker::PhantomData,
            key_state: std::marker::PhantomData,
            value_state: std::marker::PhantomData,
        }
    }
    pub fn table(
        mut self,
        table: &'create str,
    ) -> CreateBuilder<TablePresent, KeyNotPresent, ValueNotPresent> {
        self.table.push(table.to_string());
        CreateBuilder {
            conn: self.conn,
            table: self.table,
            key: self.key,
            value: self.value,
            table_state: PhantomData,
            key_state: PhantomData,
            value_state: PhantomData,
        }
    }
}

impl<'create> CreateBuilder<'create, TablePresent, KeyNotPresent, ValueNotPresent> {
    pub fn key(
        mut self,
        key: &'create [&str],
    ) -> CreateBuilder<'create, TablePresent, KeyPresent, ValueNotPresent> {
        self.key.push(key);
        CreateBuilder {
            conn: self.conn,
            table: self.table,
            key: self.key,
            value: self.value,
            table_state: std::marker::PhantomData,
            key_state: std::marker::PhantomData,
            value_state: std::marker::PhantomData,
        }
    }
}

impl<'create> CreateBuilder<'create, TablePresent, KeyPresent, ValueNotPresent> {
    pub fn value(
        mut self,
        value: &'create [&'create (dyn ToSql + Sync)],
    ) -> CreateBuilder<TablePresent, KeyPresent, ValuePresent> {
        self.value.push(value);
        CreateBuilder {
            conn: self.conn,
            table: self.table,
            key: self.key,
            value: self.value,
            table_state: std::marker::PhantomData,
            key_state: std::marker::PhantomData,
            value_state: std::marker::PhantomData,
        }
    }
}

impl<'create> CreateBuilder<'create, TablePresent, KeyPresent, ValuePresent> {
    pub fn confirm(self) -> Create<'create> {
        Create {
            conn: self.conn.unwrap(),
            key: self.key,
            table: self.table,
            value: self.value,
        }
    }
}

impl<'create> Create<'create> {
    pub fn build(mut self) -> Result<u64, postgres::Error> {
        let mut connection = &mut self.conn;
        let keys = self.key;
        let value = self.value;
        let tables = &self.table;

        let mut keys_ = String::new();
        let mut id = String::new();
        let mut table_ = String::new();

        // for value in value {
        //     values.push_str(value.to_vec().iter());
        // }

        for key in keys.iter() {
            for (index, key) in key.iter().enumerate() {
                let keys = format!("{},", key);
                keys_.push_str(&keys);
                // println!("{:?}", key);
                // println!("{:?}", value[index]);
                let key_id = format!("${},", index + 1);
                id.push_str(&key_id);
            }
        }
        for table in tables {
            table_.push_str(&table);
        }
        let keys = keys_.trim_end_matches(",");
        let id = id.trim_end_matches(",");

        let create = format!("INSERT INTO {} ({}) VALUES ({})", table_, keys, id);

        // creating
        let mut values: &[&(dyn ToSql + Sync)] = &Vec::new();
        for value in value {
            values = value
        }

        let create = connection.execute(&create, &values);
        match create {
            Ok(success) => Ok(success),
            Err(err) => Err(err),
        }
    }
}
