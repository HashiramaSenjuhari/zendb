use postgres::{Client, Error};

use crate::{
    Condition, Find, FindBuilder, KeyNotPresent, KeyPresent, TableNotPresent, TablePresent,
    ValueNotPresent, ValuePresent,
};

pub enum Item<'item> {
    All,
    Specific(&'item [&'item str]),
}

impl<'find> FindBuilder<'find, TableNotPresent, KeyNotPresent, ValueNotPresent> {
    pub fn new(connection: &'find mut Client) -> Self {
        Self {
            connection: connection,
            table: Vec::new(),
            item: Vec::new(),
            condition: Vec::new(),
            tablestate: std::marker::PhantomData,
            keystate: std::marker::PhantomData,
            valuestate: std::marker::PhantomData,
        }
    }
    pub fn table(
        mut self,
        table: &'find str,
    ) -> FindBuilder<TablePresent, KeyNotPresent, ValueNotPresent> {
        self.table.push(table);
        FindBuilder {
            connection: self.connection,
            table: self.table,
            item: self.item,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
            keystate: std::marker::PhantomData,
            valuestate: std::marker::PhantomData,
        }
    }
}

impl<'key> FindBuilder<'key, TablePresent, KeyNotPresent, ValueNotPresent> {
    pub fn item(
        mut self,
        item: Item<'key>,
    ) -> FindBuilder<TablePresent, KeyPresent, ValueNotPresent> {
        self.item.push(item);
        FindBuilder {
            connection: self.connection,
            table: self.table,
            item: self.item,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
            keystate: std::marker::PhantomData,
            valuestate: std::marker::PhantomData,
        }
    }
}

impl<'value> FindBuilder<'value, TablePresent, KeyPresent, ValueNotPresent> {
    pub fn condition(
        mut self,
        condition: Condition<'value>,
    ) -> FindBuilder<'value, TablePresent, KeyPresent, ValuePresent> {
        self.condition.push(condition);
        FindBuilder {
            connection: self.connection,
            table: self.table,
            item: self.item,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
            keystate: std::marker::PhantomData,
            valuestate: std::marker::PhantomData,
        }
    }
}

impl<'block> FindBuilder<'block, TablePresent, KeyPresent, ValuePresent> {
    pub fn settt(self) -> Find<'block> {
        Find {
            coonection: self.connection,
            table: self.table,
            item: self.item,
            condition: self.condition,
        }
    }
}

impl<'find> Find<'find> {
    pub fn build(self) -> Result<Vec<postgres::Row>, Error> {
        let mut item = String::new();
        let mut keyy = String::new();
        let mut table = String::new();

        for tablee in self.table.iter() {
            table.push_str(&tablee);
        }
        for items in self.item.iter() {
            match items {
                Item::All => {
                    item.push_str("*");
                }
                Item::Specific(items) => {
                    for items in items.iter() {
                        let items_ = format!("{},", items);
                        item.push_str(&items_);
                    }
                }
            }
        }
        for (index, conditions) in self.condition.iter().enumerate() {
            let ids = format!("${},", index + 1);
            let key = format!("{}", conditions.condition);

            let key = format!("{} = {}", key, ids);

            keyy.push_str(&key);
        }
        let keyy = keyy.trim_end_matches(",");
        let item = item.trim_end_matches(",");

        let values: Vec<&str> = self.condition.iter().map(|b| b.value.as_ref()).collect();
        let value = values.join(",");

        // println!("{:?}", values);

        let find = format!("SELECT {} FROM {} WHERE {};", item, table, keyy);

        // println!("{}", find);

        let find = self.coonection.query(&find, &[&value]);
        find
    }
}
