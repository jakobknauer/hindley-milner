pub type Var = String;

#[derive(Debug)]
pub enum Expr {
    Var(Var),
    App(Box<Expr>, Box<Expr>),
    Abs(Var, Box<Expr>),
    Let(Var, Box<Expr>, Box<Expr>),
}
