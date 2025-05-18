// 用于表示二元运算符，例如 +, -, *, /, =, <>, AND, OR 等
#[derive(Debug, Clone, PartialEq, Eq, Hash)] // Eq/Hash可能需要根据具体运算符调整
pub enum BinaryOperator {
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // 比较运算符
    Eq,       // =
    NotEq,    // != or <>
    Lt,       // <
    LtEq,     // <=
    Gt,       // >
    GtEq,     // >=

    // 逻辑运算符
    And,      // AND
    Or,       // OR
    // ... 其他你需要的二元运算符
}

// 用于表示一元运算符，例如 -, NOT
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Negate,   // - (例如 -column)
    Not,      // NOT
    // ... 其他一元运算符
}

// 用于表示字面量值
#[derive(Debug, Clone, PartialEq)] // PartialEq 对 f64 可能需要特殊处理
pub enum LiteralValue {
    Null,
    Boolean(bool),
    Integer(i128),
    Float(f64), // 或使用更精确的类型如 rust_decimal::Decimal
    String(String),
    // 可以添加 Date, Timestamp, Interval 等
}

// 核心的表达式枚举 (AST 节点)
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// 字面量值, e.g., 1, 'hello', TRUE, NULL
    Literal(LiteralValue),

    /// 标识符, e.g., column_name, table.column_name
    /// 使用 Vec<String> 来支持 schema.table.column 这种形式
    Identifier(Vec<String>),

    /// 二元运算, e.g., price * quantity, count(*) > 0
    BinaryOp {
        left: Box<Expression>, // 使用 Box 来避免无限递归类型大小
        op: BinaryOperator,
        right: Box<Expression>,
    },

    /// 一元运算, e.g., -price, NOT is_active
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>, // 同样使用 Box
    },

    /// 函数调用, e.g., COUNT(*), SUBSTRING(name, 1, 3), NOW()
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },

    /// 特殊的通配符 '*', 主要用于 COUNT(*) 或 SELECT *
    /// 在表达式解析中，主要用于 COUNT(*) 这类场景
    Wildcard,
}

