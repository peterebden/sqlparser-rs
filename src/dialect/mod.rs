mod ansi_sql;
mod generic_sql;
pub mod keywords;
mod postgresql;


pub use self::ansi_sql::AnsiSqlDialect;
pub use self::generic_sql::GenericSqlDialect;
pub use self::postgresql::PostgreSqlDialect;
use to_sql::ToSql;

use sqlast::{
    ASTNode,
    SQLAssignment,
    SQLColumnDef,
    SQLOrderByExpr,
    AlterOperation,
    TableKey,
};

pub trait Dialect {
    /// Get a list of keywords for this dialect
    fn keywords(&self) -> Vec<&'static str>;
    /// Determine if a character is a valid identifier start character
    fn is_identifier_start(&self, ch: char) -> bool;
    /// Determine if a character is a valid identifier character
    fn is_identifier_part(&self, ch: char) -> bool;

    /// convert ast nodes to sql string for each dialect
    fn ast_to_string(&self, ast: &ASTNode) -> String;

    /// convert SQLAssignment to the dialect specific syntax
    fn assignment_to_string(&self, ass: &SQLAssignment) -> String;

    /// convert column def to the dialect specific syntax
    fn column_def_to_string(&self, column_def: &SQLColumnDef) -> String;

    /// convert sql order by to the dialect specific syntax
    fn order_by_to_string(&self, order_by: &SQLOrderByExpr) -> String;

    /// convert sql alter operation to the dialect specific syntax
    fn alter_operation_to_string(&self, alter_op: &AlterOperation) -> String;

    /// convert table key to the dialect's specific syntax
    fn table_key_to_string(&self, table_key: &TableKey) -> String;


}
