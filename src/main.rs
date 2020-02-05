use std::env;

// トークンの種類
#[derive(Debug)]
enum TokenKind {
    TkReserve(char),  // 記号
    TkNum(u32),       // 整数トークン
    TkEof(char),      // 入力の終わりを表すトークン
}

/*
// エラーを報告するための関数
fn error() {
    .....
}
*/

// 入力文字列をトークナイズしてそれを返す
fn tokenize(formula: &str) -> Vec<TokenKind> {
    let mut token = Vec::new();
    let mut i = 0;

    while i < formula.len(){
        let c: char = formula.chars().nth(i).unwrap();

        // 空白文字をスキップ
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        
        // 加減算
        if c == '+' || c == '-' {
            token.push(TokenKind::TkReserve(c));
            i += 1;
            continue;
        }

        // 数字の場合
        if c.is_ascii_digit() {
            let mut n: u32 = 0;
            n += c.to_digit(10).unwrap();
            i += 1;
            while i < formula.len() {
                if formula.chars().nth(i).unwrap().is_ascii_digit() {
                    n *= 10;
                    n += formula.chars().nth(i).unwrap().to_digit(10).unwrap();
                    i += 1;
                }
                else { break; }
            }
            token.push(TokenKind::TkNum(n));
        }
    }
    token.push(TokenKind::TkEof('e'));
    token
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let formula = &args[1];
    let mut token_vec = Vec::new();

    // トークナイズする
    token_vec = tokenize(formula);

    // アセンブリの前半部分の出力
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut result = 0;

    // アセンブリを出力
    for i in 0..token_vec.len() {
        // result : 0 -> eof, 1 ~ 2 : 演算子, 100 : 数
        let tmp_result = match &token_vec[i]{
            TokenKind::TkReserve(op) => {
                if *op == '+' { 1 }
                else if *op == '-' { 2 }
                else { -1 }
            },
            TokenKind::TkNum(n) => {
                // 式の最初は数でなければならないので、それをチェックして最初のmov命令を出力
                if i == 0 {
                    println!("  mov rax, {}", n);
                }
                else {
                    if result == 1 {
                        println!("  add rax, {}", n);
                    }
                    else if result == 2 {
                        println!("  sub rax, {}", n);
                    }
                }
                100
            },
            TokenKind::TkEof(_c) => 0,
            //_ => -1,  // エラーハンドリング(あとで)
        };
        result = tmp_result;
    }

    println!("  ret");
}