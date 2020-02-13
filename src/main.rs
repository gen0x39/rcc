extern crate rcc;

use rcc::parse::parse;
use rcc::token::tokenize;

use std::env;

// エラーを報告するためのマクロ
// Rustでは可変長ひきすがないのでマクロを使う
macro_rules! error {
    () => (eprint!("\n"));
    ($fmt:expr) => {
        (eprint!(concat!($fmt, "\n")));
        std::process::exit(1);
    };
    ($fmt:expr, $($arg:tt)*) => {
        (eprint!(concat!($fmt, "\n"), $($arg)*));
        std::process::exit(1);
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("引数の数を確認してください");
    }

    let formula = &args[1];
    let mut token_vec = Vec::new();

    // tokenize -> Vec<TokenType>
    token_vec = tokenize(formula);

    parse(&token_vec);    
}