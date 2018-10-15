use dialect::Dialect;

pub trait ToSql{

    fn to_sql(&self, dialect: &Dialect ) -> String;
}

enum SQLElement{
    ASTNode(ASTNode),
    SQLAssignment(SQLAssignment),
    SQLOrderByExpr(SQLOrderByExpr),
}
