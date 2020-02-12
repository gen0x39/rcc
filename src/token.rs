use crate::TokenKind;

pub fn tokenize(formula: &str) -> Vec<TokenKind> {
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
            continue;
        }
        
        //error!("トークナイズできません");

    }
    token.push(TokenKind::TkEof('e'));
    token
}