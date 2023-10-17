use crate::ast::expression::Expr;

pub struct Select {
    distinct: bool,
    straight_join_hint: bool,
    from: Vec<TableExpr>,
    select: Vec<SelectExpr>,
    where_: Expr,
    with: Option<With>,
    groupby: Vec<Expr>,
    have: Expr,
    windows: Vec<NamedWindow>,
    orderby: Vec<Order>,
    limit: Option<Limit>,
    offset: Option<Offset>,
    lock: Option<Lock>,
    into: Option<SelectInto>,
}

pub struct Identifier {
    value: String
}

/// TableName represents a table
pub struct TableName {
    idents: Vec<String>
}

/// With contains the lists of common table expression and specifies if it is recursive or not
pub struct With {
    recursive: bool,
    ctes: Vec<CommonTableExpr>,
}

/// CommonTableExpr is the structure for supporting common table expressions
pub struct CommonTableExpr {
    pub alias: TableAlias,
    pub materialized: bool,
    pub select: Select,
}

/// TableAlias represents table alias with optional columns list
pub struct TableAlias {
    /// alias name
    pub name: String,
    /// alias columns
    pub columns: Vec<String>,
}

/// Subquery represents a subquery used as an value expression
pub struct Subquery {
    select: Select,
}

/// SelectItem represents SELECT expressions 
pub enum SelectExpr {
    /// SelectAlias defines a 'expr' or 'expr as a'
    SelectAlias(Expr, Option<String>),
    /// StarExpr defines a '*' or 'table.*' expression
    StarExpr(Option<TableName>, Option<StarModifier>), 
}

/// TODO
pub struct StarModifier {
}

/// TODO :docs
pub enum TableExpr {
    SimpleTable(TableName, Option<TableAlias>),
    JoinTable(Box<TableExpr>, JoinType, Box<TableExpr>, JoinCriteria),
    Subquery(Select, Option<TableAlias>),
}

pub enum JoinType {
    CrossJoin,
    Inner(JoinCriteria),
    LeftOuter(JoinCriteria),
    RightOuter(JoinCriteria),
    FullOuter(JoinCriteria),
    /// LEFT SEMI (non-standard)
    LeftSemi(JoinCriteria),
    /// RIGHT SEMI (non-standard)
    RightSemi(JoinCriteria),
    /// LEFT ANTI (non-standard)
    LeftAnti(JoinCriteria),
    /// RIGHT ANTI (non-standard)
    RightAnti(JoinCriteria),
}

pub enum JoinCriteria {
    Natural,
    On(Expr),
    Using(Vec<Identifier>),
}

pub struct Order {
	expr: Expr,
	direction: Option<OrderDirection>,
    collate: Option<Collate>,
    nulls_order: Option<NullsOrder>,
}

pub struct Collate {
}

pub enum OrderDirection {
    Asc,
    Desc,
}

pub enum NullsOrder {
    First,
    Last,
}

pub enum Limit {
    Limit(Expr),
    LimitAll,
    Fetch(FetchPosition, FetchValue, RowVariant, Option<FetchModifier>),
}

pub struct FetchValue {
    value: Expr,
    percent: bool,
}

pub enum FetchPosition {
    First,
    Next,
}

pub enum FetchModifier {
    Only,
    WithTies
}

pub struct Offset {
    offset: Expr,
    row: Option<RowVariant>,
}

pub enum RowVariant {
    Row,
    Rows,
}

pub enum Lock {
    ReadOnly,
    LockList(Vec<LockItem>),
}

pub struct LockItem {
    type_: LockType,
    relations: Vec<String>,
    modifier: LockModifier,
}

pub enum LockType {
    Update,
    Share,
    KeyShare,
    NoKeyUpdate,
}

pub enum LockModifier {
    Nowait,
    SkipLocked,
}

pub struct NamedWindow {
    ident: String,
}

pub struct WindowSpec {
    name: Option<String>,
    partition: Vec<Expr>,
    orderby: Vec<Order>,
    frame: Option<WindowFrame>,
}

pub struct WindowFrame {
    start: WindowFrameBound,
    end: Option<WindowFrameBound>,
    exclude: Option<WindowFrameExclusion>,
}

pub enum WindowFrameUnit {
    Range,
    Rows,
}

pub enum WindowFrameBound {
    CurrentRow,
    UnboundedPreceding,
    UnboundedFollowing,
    Preceding(Expr),
    Following(Expr),
}

pub enum WindowFrameExclusion {
    Group,
    Ties,
    CurrentRow,
    NoOthers,
}

pub struct SelectIntoTable {
    pub temporary: bool,
    pub unlogged: bool,
    pub table: bool,
    pub name: Identifier,
}

// SelectInto is a struct that represent the INTO part of a select query
pub struct SelectInto {
    Type         SelectIntoType
    FileName     string
    Charset      ColumnCharset
    FormatOption string
    ExportOption string
    Manifest     string
    Overwrite    string
}

pub enum SelectInto {
    Table(),
    Variables,
    OutFile,
    DumpFile,
}
