mod ansi_sql;
mod generic_sql;
pub mod keywords;
mod postgresql;


pub use self::ansi_sql::AnsiSqlDialect;
pub use self::generic_sql::GenericSqlDialect;
pub use self::postgresql::PostgreSqlDialect;
use sqlast::ASTNode;

pub trait Dialect {
    /// Get a list of keywords for this dialect
    fn keywords(&self) -> Vec<&'static str>;
    /// Determine if a character is a valid identifier start character
    fn is_identifier_start(&self, ch: char) -> bool;
    /// Determine if a character is a valid identifier character
    fn is_identifier_part(&self, ch: char) -> bool;

    /// convert ast nodes to sql string for each dialect
    fn ast_to_string(&self, ast: &ASTNode) -> String;

}
