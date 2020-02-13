use crate::TokenType;

pub fn parse(token_vec: &Vec<TokenType>) {
    // アセンブリの前半部分の出力
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // アセンブリを出力
    for i in 0..token_vec.len() {
        match &token_vec[i]{
            TokenType::Num(n) => {
                // 式の最初は数でなければならないので、それをチェックして最初のmov命令を出力
                if i == 0 { println!("  mov rax, {}", n); }
                else { println!("{}",n); }
            },
            TokenType::Plus => {
                print!("  add rax, ");
            },
            TokenType::Minus => {
                print!("  sub rax, ");
            },
            TokenType::Eof =>  {
                println!("  ret");
            },
            _ => (),  // エラーハンドリング(あとで)
        }
    }
}