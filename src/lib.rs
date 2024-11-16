use find::Item;
use postgres::Client;

pub mod create;
pub mod delete;
pub mod find;
pub mod schema;
pub mod schemas;
pub mod update;

pub struct Condtion<'condition> {
    pub key: &'condition str,
    pub value: &'condition str,
}

pub struct ConditionPresent {}
pub struct CondtionNotPresent {}
pub struct Condition<'condition> {
    pub condition: &'condition str,
    pub value: &'condition str,
}

pub struct CreateState;
pub struct CreateNoState;

pub struct NoConnection {}

pub struct TablePresent {}
pub struct TableNotPresent {}

pub struct KeyPresent {}
pub struct KeyNotPresent {}

pub struct ValuePresent {}
pub struct ValueNotPresent {}

pub struct UpdateBuilder<'update, TableState, ContionState, WhereState> {
    conn: &'update mut Client,
    table: Vec<&'update str>,
    values: Vec<Condition<'update>>,
    condition: Vec<Option<Condition<'update>>>,
    tablestate: std::marker::PhantomData<TableState>,
    conditionstate: std::marker::PhantomData<ContionState>,
    wherestate: std::marker::PhantomData<WhereState>,
}

pub struct Update<'update> {
    conn: &'update mut Client,
    table: Vec<&'update str>,
    values: Vec<Condition<'update>>,
    condition: Vec<Option<Condition<'update>>>,
}

pub struct Find<'find> {
    coonection: &'find mut Client,
    table: Vec<&'find str>,
    item: Vec<Item<'find>>,
    condition: Vec<Condition<'find>>,
}
pub struct FindBuilder<'find, TableState, KeyState, ValueState> {
    connection: &'find mut Client,
    table: Vec<&'find str>,
    item: Vec<Item<'find>>,
    condition: Vec<Condition<'find>>,
    tablestate: std::marker::PhantomData<TableState>,
    keystate: std::marker::PhantomData<KeyState>,
    valuestate: std::marker::PhantomData<ValueState>,
}

pub struct Delete<'delete> {
    connection: &'delete mut Client,
    table: Vec<&'delete str>,
    condition: Vec<Option<Condtion<'delete>>>,
}

pub struct DeleteBuilder<'delete, TableState, ConditionState> {
    connection: Option<&'delete mut Client>,
    table: Vec<&'delete str>,
    condition: Vec<Option<Condtion<'delete>>>,
    tablestate: std::marker::PhantomData<TableState>,
    condtionstate: std::marker::PhantomData<ConditionState>,
}

pub struct CreateBuilder<'create, TableState, KeyState, ValueState> {
    conn: Option<&'create mut Client>,
    table: Vec<String>,
    key: Vec<&'create [&'create str]>,
    value: Vec<&'create [&'create (dyn ToSql + Sync)]>,
    table_state: std::marker::PhantomData<TableState>,
    key_state: std::marker::PhantomData<KeyState>,
    value_state: std::marker::PhantomData<ValueState>,
}

// #[derive(Debug, Clone)]
pub struct Create<'create> {
    conn: &'create mut Client,
    table: Vec<String>,
    key: Vec<&'create [&'create str]>,
    value: Vec<&'create [&'create (dyn ToSql + Sync)]>,
}
