use postgres::{types::ToSql, Client, Error};

use crate::{ConditionPresent, Condtion, CondtionNotPresent, Delete, DeleteBuilder, TableNotPresent, TablePresent};


impl<'delete> DeleteBuilder<'delete, TableNotPresent, CondtionNotPresent> {
    pub fn new(conn: &'delete mut Client) -> Self {
        DeleteBuilder {
            connection: Some(conn),
            table: Vec::new(),
            condition: Vec::new(),
            tablestate: std::marker::PhantomData,
            condtionstate: std::marker::PhantomData,
        }
    }
    pub fn table(mut self, table: &'delete str) -> DeleteBuilder<TablePresent, CondtionNotPresent> {
        self.table.push(table);
        DeleteBuilder {
            connection: self.connection,
            table: self.table,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
            condtionstate: std::marker::PhantomData,
        }
    }
}

impl<'delete> DeleteBuilder<'delete, TablePresent, CondtionNotPresent> {
    pub fn condition(
        mut self,
        condition: Option<Condtion<'delete>>,
    ) -> DeleteBuilder<'delete, TablePresent, ConditionPresent> {
        self.condition.push(condition);
        DeleteBuilder {
            connection: self.connection,
            table: self.table,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
            condtionstate: std::marker::PhantomData,
        }
    }
}

impl<'delete> DeleteBuilder<'delete, TablePresent, ConditionPresent> {
    pub fn build(mut self) -> Delete<'delete> {
        Delete {
            connection: self.connection.unwrap(),
            table: self.table,
            condition: self.condition,
        }
    }
}

impl<'delete> Delete<'delete> {
    pub fn build(self) -> Result<u64, Error> {
        let tables = &self.table;
        let conditions = self.condition;

        let id = String::new();

        let mut table = String::new();
        let mut condition = String::new();
        let mut delete = String::new();
        let mut value: Vec<Box<(dyn ToSql + Sync)>> = Vec::new();

        for tables in tables {
            table.push_str(&tables);
        }

        for (index, conditions) in conditions.iter().enumerate() {
            match conditions {
                Some(conditions_) => {
                    let index = format!("${}", index + 1);
                    let conditions = format!("{} = {}", conditions_.key, index);
                    condition.push_str(&conditions);

                    let deletes = format!("DELETE FROM {} WHERE {};", table, condition);
                    delete.push_str(&deletes);

                    let values = &conditions_.value;
                    value.push(Box::new(values));
                    println!("{:?}", &value);
                    // println!("{}", delete);
                }
                None => {
                    let deletes = format!("DELETE FROM {};", table);
                    delete.push_str(&deletes);
                    // println!("{}", delete);
                }
            }
        }
        let values: Vec<&(dyn ToSql + Sync)> = value.iter().map(|b| b.as_ref()).collect();

        let delete = self.connection.execute(&delete, &values);

        match delete {
            Ok(success) => Ok(success),
            Err(err) => Err(err),
        }
    }
}
