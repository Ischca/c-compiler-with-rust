use libc::isdigit;
use libc::isspace;
use libc::strtol;
use std::env;
use std::ffi::CString;
use std::process;

// トークンの種類
#[derive(PartialOrd, PartialEq)]
enum TokenKind {
    TkReserved, // 記号
    TkNum,      // 整数トークン
    TkEof,      // 入力の終わりを表すトークン
}

// トークン型
struct Token {
    kind: TokenKind,          // トークンの型
    next: Box<Option<Token>>, // 次の入力トークン
    val: i8,                  // kindがTkNumの場合、その数値
    str: String,              // トークン文字列
}

impl Token {
    fn Empty() -> Token {
        todo!()
    }
}

// 次のトークンが期待している記号のときには、次のトークンを返す
// それ以外の場合にはNoneを返す
fn consume(op: char, token: Token) -> Option<Token> {
    if token.kind != TokenKind::TkReserved || token.str.chars().nth(0).unwrap() != op {
        None
    } else {
        match *token.next {
            Some(t) => Some(t),
            None => None,
        }
    }
}

// 次のトークンが期待している記号のときには、トークンを1つ読み進める。
// それ以外の場合にはエラーを報告する。
fn expect(op: char, token: Token) -> Option<Token> {
    if token.kind != TokenKind::TkReserved || token.str.chars().nth(0).unwrap() != op {
        eprintln!("'{}'ではありません", op);
        None
    } else {
        match *token.next {
            Some(t) => Some(t),
            None => None,
        }
    }
}

// 次のトークンが数値の場合、トークンを1つ読み進めてその数値を返す。
// それ以外の場合にはエラーを報告する。
fn expect_number(token: Token) -> Option<(i8, Token)> {
    if token.kind != TokenKind::TkNum {
        eprintln!("数ではありません");
        None
    } else {
        match *token.next {
            Some(t) => Some((token.val, t)),
            None => None,
        }
    }
}

fn at_eof(token: Token) -> bool {
    return token.kind == TokenKind::TkEof;
}

// 新しいトークンを作成してcurに繋げる
fn new_token(kind: TokenKind, cur: *const Token, str: String, val: i8) -> Box<Token> {
    Box::new(Token {
        kind: kind,
        str: str,
        next: Box::new(None),
        val: val,
    })
}

// 入力文字列pをトークナイズしてそれを返す
fn tokenize(p: String) -> Option<Token> {
    let head: Token = Token::Empty();
    // head.next = NULL;
    let cur = &head;

    // while *p as u8 != b'\0' {
    p.chars()
        .fold(Box::new(Token::Empty()), |mut acc: Box<Token>, c: char| {
            let ptr = c;
            // 空白文字をスキップ
            unsafe {
                match ptr {
                    it if isspace(it as i32) != 0 => {
                        return acc;
                    }
                    '+' | '-' => return new_token(TokenKind::TkReserved, cur, String::from(c), 0),
                    it if isdigit(c as i32) != 0 => {
                        let token = new_token(TokenKind::TkNum, &*acc, String::from(c), c as i8);
                        return token;
                    }
                    _ => {
                        eprint!("トークナイズできません");
                        Box::new(Token::Empty())
                    }
                }
            }
        });

    new_token(TokenKind::TkEof, cur, p, 0);
    return *head.next;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprint!("引数の個数が正しくありません\n");
        process::exit(1);
    }

    // 現在着目しているトークン
    let token: Token;

    let mut p = CString::new(args[1].clone()).expect("error").into_raw();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    unsafe {
        println!("  mov rax, {}", strtol(p, &mut p as *mut *mut i8, 10));

        while *p as u8 != b'\0' {
            let ptr = *p as u8 as char;
            match ptr {
                '+' => {
                    p = p.offset(1);
                    println!("  add rax, {}", strtol(p, &mut p as *mut *mut i8, 10));
                }
                '-' => {
                    p = p.offset(1);
                    println!("  sub rax, {}", strtol(p, &mut p as *mut *mut i8, 10));
                }
                _ => panic!("予期しない文字です: {}", *p),
            }
        }
    }
    println!("  ret");
}
