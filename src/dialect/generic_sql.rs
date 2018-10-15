use dialect::Dialect;

use dialect::keywords::*;
use sqlast::ASTNode;
use sqlast::ToSql;

pub struct GenericSqlDialect {}

impl Dialect for GenericSqlDialect {
    fn keywords(&self) -> Vec<&'static str> {
        return vec![
            SELECT, FROM, WHERE, LIMIT, ORDER, GROUP, BY, HAVING, UNION, ALL, INSERT, INTO, UPDATE,
            DELETE, IN, IS, NULL, SET, CREATE, EXTERNAL, TABLE, ASC, DESC, AND, OR, NOT, AS,
            STORED, CSV, PARQUET, LOCATION, WITH, WITHOUT, HEADER, ROW, // SQL types
            CHAR, CHARACTER, VARYING, LARGE, OBJECT, VARCHAR, CLOB, BINARY, VARBINARY, BLOB, FLOAT,
            REAL, DOUBLE, PRECISION, INT, INTEGER, SMALLINT, BIGINT, NUMERIC, DECIMAL, DEC,
            BOOLEAN, DATE, TIME, TIMESTAMP,
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
        match ast {
            ASTNode::SQLIdentifier(s) => s.to_string(),
            ASTNode::SQLWildcard => "*".to_string(),
            ASTNode::SQLCompoundIdentifier(s) => s.join("."),
            ASTNode::SQLAssignment(ass) => ass.to_sql(self),
            ASTNode::SQLIsNull(ast) => format!("{} IS NULL", ast.to_sql(self)),
            ASTNode::SQLIsNotNull(ast) => format!("{} IS NOT NULL", ast.to_sql(self)),
            ASTNode::SQLBinaryExpr { left, op, right } => format!(
                "{} {} {}",
                left.to_sql(self),
                op.to_string(),
                right.to_sql(self)
            ),
            ASTNode::SQLCast { expr, data_type } => format!(
                "CAST({} AS {})",
                expr.to_sql(self),
                data_type.to_string()
            ),
            ASTNode::SQLNested(ast) => format!("({})", ast.to_sql(self)),
            ASTNode::SQLUnary { operator, rex } => {
                format!("{} {}", operator.to_string(), rex.to_sql(self))
            }
            ASTNode::SQLValue(v) => v.to_string(),
            ASTNode::SQLFunction { id, args } => format!(
                "{}({})",
                id,
                args.iter()
                    .map(|a| a.to_sql(self))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            ASTNode::SQLSelect {
                projection,
                relation,
                selection,
                order_by,
                group_by,
                having,
                limit,
            } => {
                let mut s = format!(
                    "SELECT {}",
                    projection
                        .iter()
                        .map(|p| p.to_sql(self))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                if let Some(relation) = relation {
                    s += &format!(" FROM {}", relation.to_sql(self));
                }
                if let Some(selection) = selection {
                    s += &format!(" WHERE {}", selection.to_sql(self));
                }
                if let Some(group_by) = group_by {
                    s += &format!(
                        " GROUP BY {}",
                        group_by
                            .iter()
                            .map(|g| g.to_sql(self))
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                }
                if let Some(having) = having {
                    s += &format!(" HAVING {}", having.to_sql(self));
                }
                if let Some(order_by) = order_by {
                    s += &format!(
                        " ORDER BY {}",
                        order_by
                            .iter()
                            .map(|o| o.to_sql(self))
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                }
                if let Some(limit) = limit {
                    s += &format!(" LIMIT {}", limit.to_sql(self));
                }
                s
            }
            ASTNode::SQLInsert {
                table_name,
                columns,
                values,
            } => {
                let mut s = format!("INSERT INTO {}", table_name);
                if columns.len() > 0 {
                    s += &format!(" ({})", columns.join(", "));
                }
                if values.len() > 0 {
                    s += &format!(
                        " VALUES({})",
                        values
                            .iter()
                            .map(|row| row
                                .iter()
                                .map(|c| c.to_sql(self))
                                .collect::<Vec<String>>()
                                .join(", "))
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                }
                s
            }
            ASTNode::SQLCopy {
                table_name,
                columns,
                values,
            } => {
                let mut s = format!("COPY {}", table_name);
                if columns.len() > 0 {
                    s += &format!(
                        " ({})",
                        columns
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                }
                s += " FROM stdin; ";
                if values.len() > 0 {
                    s += &format!(
                        "\n{}",
                        values
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join("\t")
                    );
                }
                s += "\n\\.";
                s
            }
            ASTNode::SQLUpdate {
                table_name,
                assignments,
                selection,
            } => {
                let mut s = format!("UPDATE {}", table_name);
                if assignments.len() > 0 {
                    s += &format!(
                        "{}",
                        assignments
                            .iter()
                            .map(|ass| ass.to_sql(self))
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                }
                if let Some(selection) = selection {
                    s += &format!(" WHERE {}", selection.to_sql(self));
                }
                s
            }
            ASTNode::SQLDelete {
                relation,
                selection,
            } => {
                let mut s = String::from("DELETE");
                if let Some(relation) = relation {
                    s += &format!(" FROM {}", relation.to_sql(self));
                }
                if let Some(selection) = selection {
                    s += &format!(" WHERE {}", selection.to_sql(self));
                }
                s
            }
            ASTNode::SQLCreateTable { name, columns } => format!(
                "CREATE TABLE {} ({})",
                name,
                columns
                    .iter()
                    .map(|c| c.to_sql(self))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            ASTNode::SQLAlterTable { name, operation } => {
                format!("ALTER TABLE {} {}", name, operation.to_sql(self))
            }
        }
    }

}
