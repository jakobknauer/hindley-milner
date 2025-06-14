#[macro_export]
macro_rules! expect_token {
    ($parser:expr, $pat:pat, $expected:expr) => {{
        let token = $parser.consume()?;
        let $pat = token else {
            return unexpected_token_error(&token, $expected);
        };
        token
    }};
}

#[macro_export]
macro_rules! expect_variable {
    ($parser:expr, $expected:expr) => {{
        let token = $parser.consume()?;
        let Var(x) = token else {
            return unexpected_token_error(&token, $expected);
        };
        x
    }};
}

#[macro_export]
macro_rules! consume_and_return {
    ($tokenizer:expr, $token:expr) => {{
        $tokenizer.consume();
        $token
    }};
}
