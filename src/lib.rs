pub mod token;
pub mod parse;

// トークンの種類
/*
#[derive(Debug)]
pub enum TokenKind {
    TkReserve(char),  // 記号
    TkNum(u32),       // 整数トークン
    TkEof(char),      // 入力の終わりを表すトークン
}*/

#[derive(Debug)]
pub enum TokenType {
    Num(u32),         // 整数トークン
    Plus,               // +
    Minus,              // -
    Eof,              // 入力の終わりを表すトークン
}