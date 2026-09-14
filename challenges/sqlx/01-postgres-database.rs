/// PostgreSQL database driver.
#[derive(Debug)]
pub struct Postgres;

impl Database for Postgres {
    type Connection = PgConnection;

    type TransactionManager = PgTransactionManager;

    type Row = PgRow;

    type QueryResult = PgQueryResult;

    type Column = PgColumn;

    type TypeInfo = PgTypeInfo;

    type Value = PgValue;
    type ValueRef<'r> = PgValueRef<'r>;

    type Arguments = PgArguments;
    type ArgumentBuffer = PgArgumentBuffer;

    type Statement = PgStatement;

    const NAME: &'static str = "PostgreSQL";

    const URL_SCHEMES: &'static [&'static str] = &["postgres", "postgresql"];
}

impl HasStatementCache for Postgres {}
