use postgres::{types::ToSql, Client, Error};

use crate::{ Condtion, Delete, DeleteBuilder,
    TableNotPresent, TablePresent,
};

impl<'delete> DeleteBuilder<'delete, TableNotPresent> {
    pub fn new(conn: &'delete mut Client) -> Self {
        DeleteBuilder {
            connection: Some(conn),
            table: Vec::new(),
            condition: Vec::new(),
            tablestate: std::marker::PhantomData,
        }
    }
    pub fn table(mut self, table: &'delete str) -> DeleteBuilder<TablePresent> {
        self.table.push(table);
        DeleteBuilder {
            connection: self.connection,
            table: self.table,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
        }
    }
}

impl<'delete> DeleteBuilder<'delete, TablePresent> {
    pub fn condition(
        mut self,
        condition: Option<Condtion<'delete>>,
    ) -> DeleteBuilder<'delete, TablePresent> {
        self.condition.push(condition);
        DeleteBuilder {
            connection: self.connection,
            table: self.table,
            condition: self.condition,
            tablestate: std::marker::PhantomData,
        }
    }
    // pub fn or(mut self) -> DeleteBuilder<'delete, Condition<'delete>> {
    //     self.or.push("OR");
    //     DeleteBuilder {
    //         connection: self.connection,
    //         table: self.table,
    //         condition: self.condition,
    //         and: self.and,
    //         or: self.or,
    //         tablestate: std::marker::PhantomData,
    //     }
    // }
    // pub fn and(mut self) -> DeleteBuilder<'delete, Condition<'delete>> {
    //     self.and.push("AND");
    //     DeleteBuilder {
    //         connection: self.connection,
    //         table: self.table,
    //         condition: self.condition,
    //         and: self.and,
    //         or: self.or,
    //         tablestate: std::marker::PhantomData,
    //     }
    // }
}

impl<'delete> DeleteBuilder<'delete, TablePresent> {
    pub fn confirm(mut self) -> Delete<'delete> {
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
                    let conditions = format!("{} = {} AND ", conditions_.key, index);
                    condition.push_str(&conditions);
                    // println!("{}", condition);

                    let values = &conditions_.value;
                    value.push(Box::new(values));
                    // println!("{:?}", &value);
                    // println!("{}", delete);
                }
                None => {
                    let deletes = format!("DELETE FROM {};", table);
                    delete.push_str(&deletes);
                    // println!("{}", delete);
                }
            }
        }
        let condition = condition.trim_end_matches("AND ");
        let deletes = format!("DELETE FROM {} WHERE {};", table, condition);
        delete.push_str(&deletes);
        let values: Vec<&(dyn ToSql + Sync)> = value.iter().map(|b| b.as_ref()).collect();

        // println!("{:?}",values);
        // println!("{}", delete);
        let delete = self.connection.execute(&delete, &values);

        match delete {
            Ok(success) => Ok(success),
            Err(err) => Err(err),
        }
    }
}
