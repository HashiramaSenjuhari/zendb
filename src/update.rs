use postgres::{Client, Error};

use crate::{ Condition, ConditionPresent, CondtionNotPresent, TableNotPresent, TablePresent, Update, UpdateBuilder};

pub struct Where;
pub struct WhereNot;

impl<'update> UpdateBuilder<'update, TableNotPresent, CondtionNotPresent, WhereNot> {
    pub fn new(
        connection: &'update mut Client,
    ) -> UpdateBuilder<TableNotPresent, CondtionNotPresent, WhereNot> {
        Self {
            conn: connection,
            table: Vec::new(),
            values: Vec::new(),
            condition: Vec::new(),
            tablestate: std::marker::PhantomData,
            conditionstate: std::marker::PhantomData,
            wherestate: std::marker::PhantomData,
        }
    }
    pub fn table(
        mut self,
        table: &'update str,
    ) -> UpdateBuilder<TablePresent, CondtionNotPresent, WhereNot> {
        self.table.push(table);
        UpdateBuilder {
            conn: self.conn,
            table: self.table,
            values: self.values,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
            conditionstate: std::marker::PhantomData,
            wherestate: std::marker::PhantomData,
        }
    }
}

impl<'update> UpdateBuilder<'update, TablePresent, CondtionNotPresent, WhereNot> {
    pub fn values(
        mut self,
        values: Condition<'update>,
    ) -> UpdateBuilder<'update, TablePresent, ConditionPresent, WhereNot> {
        self.values.push(values);
        UpdateBuilder {
            conn: self.conn,
            table: self.table,
            values: self.values,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
            conditionstate: std::marker::PhantomData,
            wherestate: std::marker::PhantomData,
        }
    }
}

impl<'update> UpdateBuilder<'update, TablePresent, ConditionPresent, WhereNot> {
    pub fn condition(
        mut self,
        condition: Option<Condition<'update>>,
    ) -> UpdateBuilder<'update, TablePresent, ConditionPresent, Where> {
        // match condition {
        //     Some(wheree) => {}
        // }
        self.condition.push(condition);
        UpdateBuilder {
            conn: self.conn,
            table: self.table,
            values: self.values,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
            conditionstate: std::marker::PhantomData,
            wherestate: std::marker::PhantomData,
        }
    }
}

impl<'update> UpdateBuilder<'update, TablePresent, ConditionPresent, Where> {
    pub fn setttt(self) -> Update<'update> {
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

        for (index, condition) in self.values.iter().enumerate() {
            let condition = format!("{} = '{}',", condition.condition, condition.value);
            values.push_str(&condition);
        }

        for tables in self.table {
            table.push_str(&tables);
        }

        let values = values.trim_end_matches(",");

        for wheree in self.condition {
            match wheree {
                Some(wheree) => {
                    // for (index, conditions) in .iter().enumerate() {
                    let wheree = format!("{} = '{}'", wheree.condition, wheree.value);
                    condition.push_str(&wheree);
                    // }
                    let update_ =
                        format!("UPDATE {} SET {} WHERE {};", table, condition, condition);
                    update.push_str(&update_);
                }
                None => {
                    let update_ = format!("UPDATE {} SET {};", table, values);
                    update.push_str(&update_);
                }
            }
        }
        println!("{}", update);
        let update = self.conn.execute(&update, &[]);
        update
        // Ok(6666)
    }
}
