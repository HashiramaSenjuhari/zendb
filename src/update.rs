use postgres::{types::ToSql, Client, Error};

use crate::{Condition, CondtionNotPresent, TableNotPresent, TablePresent, Update, UpdateBuilder};

impl<'update> UpdateBuilder<'update, TableNotPresent> {
    pub fn new(connection: &'update mut Client) -> UpdateBuilder<TableNotPresent> {
        Self {
            conn: connection,
            table: Vec::new(),
            values: Vec::new(),
            condition: Vec::new(),
            tablestate: std::marker::PhantomData,
        }
    }
    pub fn table(mut self, table: &'update str) -> UpdateBuilder<TablePresent> {
        self.table.push(table);
        UpdateBuilder {
            conn: self.conn,
            table: self.table,
            values: self.values,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
        }
    }
}

impl<'update> UpdateBuilder<'update, TablePresent> {
    pub fn values(mut self, values: Condition<'update>) -> UpdateBuilder<'update, TablePresent> {
        self.values.push(values);
        UpdateBuilder {
            conn: self.conn,
            table: self.table,
            values: self.values,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
        }
    }
}

impl<'update> UpdateBuilder<'update, TablePresent> {
    pub fn condition(
        mut self,
        condition: Option<Condition<'update>>,
    ) -> UpdateBuilder<'update, TablePresent> {
        self.condition.push(condition);
        UpdateBuilder {
            conn: self.conn,
            table: self.table,
            values: self.values,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
        }
    }
}

impl<'update> UpdateBuilder<'update, TablePresent> {
    pub fn confirm(self) -> Update<'update> {
        Update {
            conn: self.conn,
            table: self.table,
            values: self.values,
            condition: self.condition,
        }
    }
}

impl<'update> Update<'update> {
    pub fn build(self) -> Result<u64, Error> {
        let mut values = String::new();
        let mut condition = String::new();
        let mut table = String::new();
        let mut update = String::new();
        let mut condtion_value: Vec<&(dyn ToSql + Sync)> = Vec::new();

        for (index, condition) in self.values.iter().enumerate() {
            let condition = format!("{} = '{}',", condition.condition, condition.value);
            values.push_str(&condition);
        }

        for tables in self.table {
            table.push_str(&tables);
        }

        let values = values.trim_end_matches(",");

        for (index, wheree) in self.condition.iter().enumerate() {
            match wheree {
                Some(wheree) => {
                    // println!("{}", wheree.condition);
                    // for (index, conditions) in .iter().enumerate() {
                    let wher = format!("{} = ${} AND ", wheree.condition, index + 1);
                    condition.push_str(&wher);

                    condtion_value.push(&wheree.value);
                    // }
                }
                None => {
                    let update_ = format!("UPDATE {} SET {};", table, values);
                    update.push_str(&update_);
                }
            }
        }
        let condition = condition.trim_end_matches("AND ");
        let update_ = format!("UPDATE {} SET {} WHERE {};", table, values, condition);
        update.push_str(&update_);
        // let update = update.trim_end_matches("AND ");
        // println!("{:?}", condtion_value);

        // println!("{}", update);
        let update = self.conn.execute(&update, &condtion_value[..]);
        update
        // Ok(6666)
    }
}
