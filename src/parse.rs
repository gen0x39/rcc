use crate::TokenKind;

pub fn parse(token_vec: &Vec<TokenKind>) {
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