use dialect::Dialect;

use dialect::keywords::*;
use dialect::generic_sql::GenericSqlDialect;
use sqlast::{
    ASTNode,
    SQLAssignment,
    SQLColumnDef,
    SQLOrderByExpr,
    AlterOperation,
    TableKey,
};

pub struct PostgreSqlDialect {}

impl Dialect for PostgreSqlDialect {
    fn keywords(&self) -> Vec<&'static str> {
        return vec![
            ALTER, ONLY, SELECT, FROM, WHERE, LIMIT, ORDER, GROUP, BY, HAVING, UNION, ALL, INSERT,
            INTO, UPDATE, DELETE, IN, IS, NULL, SET, CREATE, EXTERNAL, TABLE, ASC, DESC, AND, OR,
            NOT, AS, STORED, CSV, WITH, WITHOUT, ROW, // SQL types
            CHAR, CHARACTER, VARYING, LARGE, VARCHAR, CLOB, BINARY, VARBINARY, BLOB, FLOAT, REAL,
            DOUBLE, PRECISION, INT, INTEGER, SMALLINT, BIGINT, NUMERIC, DECIMAL, DEC, BOOLEAN,
            DATE, TIME, TIMESTAMP, VALUES, DEFAULT, ZONE, REGCLASS, TEXT, BYTEA, TRUE, FALSE, COPY,
            STDIN, PRIMARY, KEY, UNIQUE, UUID, ADD, CONSTRAINT, FOREIGN, REFERENCES,
        ];
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '@'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z')
            || (ch >= 'A' && ch <= 'Z')
            || (ch >= '0' && ch <= '9')
            || ch == '@'
            || ch == '_'
    }

    fn ast_to_string(&self, ast: &ASTNode) -> String {
        let dialect = GenericSqlDialect {};
        dialect.ast_to_string(ast)
    }

    fn assignment_to_string(&self, ass: &SQLAssignment) -> String {
        panic!("not yet implemented")
    }

    fn column_def_to_string(&self, column_def: &SQLColumnDef) -> String {
        panic!("not yet implemented")
    }

    fn order_by_to_string(&self, order_by: &SQLOrderByExpr) -> String {
        panic!("not yet implemented")
    }

    fn alter_operation_to_string(&self, alter_op: &AlterOperation) -> String {
        panic!("not yet implemented")
    }

    fn table_key_to_string(&self, table_key: &TableKey) -> String {
        panic!("not yet implemented")
    }

}

#[cfg(test)]
mod test{

use crate::dialect::{
    Dialect,
    PostgreSqlDialect};
use crate::sqlast::*;
use crate::sqlparser::*;
use crate::sqltokenizer::*;

    #[test]
    fn parse_simple_select() {
        let sql = String::from("SELECT id, fname, lname FROM customer WHERE id = 1 LIMIT 5");
        let pg = PostgreSqlDialect {};
        let ast = parse_sql(&sql, &pg);
        let to_sql = pg.ast_to_string(&ast);
        println!("sql: {}", sql);
        assert_eq!(sql, to_sql);
    }

    fn parse_sql(sql: &str, dialect: &Dialect) -> ASTNode {
        debug!("sql: {}", sql);
        let mut parser = parser(sql, dialect);
        let ast = parser.parse().unwrap();
        ast
    }

    fn parser(sql: &str, dialect: &Dialect) -> Parser {
        let mut tokenizer = Tokenizer::new(dialect, &sql);
        let tokens = tokenizer.tokenize().unwrap();
        debug!("tokens: {:#?}", tokens);
        Parser::new(tokens)
    }
}

