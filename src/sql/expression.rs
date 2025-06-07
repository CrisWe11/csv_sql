#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // 比较运算符
    Eq,    // =
    NotEq, // != or <>
    Lt,    // <
    LtEq,  // <=
    Gt,    // >
    GtEq,  // >=

    // 逻辑运算符
    And, // AND
    Or,  // OR
}

// 用于表示一元运算符，例如 -, NOT
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnOp {
    Negate, // - (例如 -column)
    Not,    // NOT
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
#[derive(Debug, PartialEq)]
pub enum Exp<'a> {
    /// 字面量值, e.g., 1, 'hello', TRUE, NULL
    Literal(LiteralValue),

    /// 标识符, e.g., column_name, table.column_name
    Identifier { parts: Vec<&'a str>, org: &'a str },

    /// 二元运算, e.g., price * quantity, count(*) > 0
    BinaryOp {
        left: Box<Expression<'a>>, // 使用 Box 来避免无限递归类型大小
        op: BinaryOperator<'a>,
        right: Box<Expression<'a>>,
    },

    /// 一元运算, e.g., -price, NOT is_active
    UnaryOp {
        op: UnaryOperator<'a>,
        expr: Box<Expression<'a>>, // 同样使用 Box
    },

    /// 函数调用, e.g., COUNT(*), SUBSTRING(name, 1, 3), NOW()
    FunctionCall {
        name: &'a str,
        args: Vec<Expression<'a>>,
    },

    /// 特殊的通配符 '*', 主要用于 COUNT(*) 或 SELECT *
    /// 在表达式解析中，主要用于 COUNT(*) 这类场景
    Wildcard,
}

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    expression: Exp<'a>,
    org: &'a str,
}

impl<'a> Expression<'a> {
    pub fn get_org(self) -> String {
        String::from(self.org)
    }

    pub fn new(expression: Exp<'a>, org: &'a str) -> Expression<'a> {
        Self { expression, org }
    }

    pub fn new_function_call(name: &'a str, args: Vec<Expression<'a>>, org: &'a str) -> Expression<'a> {
        Self::new(Exp::FunctionCall { name, args }, org)
    }

    pub fn new_literal(literal_value: LiteralValue, org: &'a str) -> Expression<'a> {
        Self::new(Exp::Literal(literal_value), org)
    }

    pub fn new_binary_op(left: Expression<'a>, op: BinaryOperator<'a>, right: Expression<'a>, org: &'a str) -> Self {
        Self::new(Exp::BinaryOp {left: Box::new(left), op, right: Box::new(right)}, org)
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperator<'a> {
    operator: BinOp,
    org: &'a str,
}

impl<'a> BinaryOperator<'a> {
    pub fn new(operator: BinOp, org: &'a str) -> BinaryOperator<'a> {
        BinaryOperator { operator, org }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnaryOperator<'a> {
    operator: UnOp,
    org: &'a str,
}

impl<'a> UnaryOperator<'a> {
    pub fn new(operator: UnOp, org: &'a str) -> UnaryOperator<'a> {
        UnaryOperator { operator, org }
    }
}
