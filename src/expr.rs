pub type Var = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Var(Var),
    App(Box<Expr>, Box<Expr>),
    Abs(Var, Box<Expr>),
    Let(Var, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn var(x: impl Into<String>) -> Expr {
        Expr::Var(x.into())
    }

    pub fn app(e1: Expr, e2: Expr) -> Expr {
        Expr::App(Box::new(e1), Box::new(e2))
    }

    pub fn abs(var: impl Into<String>, e: Expr) -> Expr {
        Expr::Abs(var.into(), Box::new(e))
    }

    pub fn r#let(x: impl Into<String>, e1: Expr, e2: Expr) -> Expr {
        Expr::Let(x.into(), Box::new(e1), Box::new(e2))
    }

    pub fn is_closed(&self) -> bool {
        match self {
            Expr::Var(..) | Expr::App(..) => true,
            Expr::Abs(..) | Expr::Let(..) => false,
        }
    }

    pub fn is_var(&self) -> bool {
        matches!(self, Self::Var(..))
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Var(x) => write!(f, "{}", x),
            Expr::App(e1, e2) => {
                if e1.is_closed() {
                    write!(f, "{}", e1)?
                } else {
                    write!(f, "({})", e1)?
                }

                write!(f, " ")?;

                if e2.is_var() {
                    write!(f, "{}", e2)
                } else {
                    write!(f, "({})", e2)
                }
            }
            Expr::Abs(x, e) => write!(f, "λ{} . {}", x, e),
            Expr::Let(x, e1, e2) => write!(f, "let {} = {} in {}", x, e1, e2),
        }
    }
}
