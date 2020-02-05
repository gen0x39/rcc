use std::env;

// トークンの種類
enum TokenKind {
    TkReserve,  // 記号
    TkNum,      // 整数トークン
    TkEof,      // 入力の終わりを表すトークン
}

// トークン型
struct Token {
    kind: TokenKind, // トークンの型
    val: u32,        // kindがTK_NUMの場合、その数値
    s: char,         // トークン文字列
}

impl Token {
    // コンストラクタ
    fn new(kind: TokenKind, val: u32, s: char) -> Token {
        Token{kind, val, s}
    }
}

/*
// エラーを報告するための関数
fn error() {
    .....
}
*/

// 入力文字列をトークナイズしてそれを返す
fn tokenize(formula: &str) -> Vec<Token> {
    let mut token = Vec::new();

    //println!("input : \"{}\"",formula);

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
            println!("c = {}",c);
            // 第2引数の 0 はダミー
            token.push(Token::new(Tokenkind::TkReserve as u8, 0, c));
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
            token.push(Token::new(Tokenkind::TkNum, n, c));
            println!("n = {}", n);
        }
    }
    token.push(Token::new(Tokenkind::TkEof as u8, 0, 'e'));
    token
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let formula = &args[1];

    // token  Vec< ... > = tokenize(formula);
    let mut token = Vec::new();
    token = tokenize(formula);

    // あ
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    //println!("  mov rax, {}", args[1]);
    println!("  ret");
}