use libc::calloc;
use libc::isspace;
use libc::strtol;
use std::env;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_char;
use std::process;
use std::ptr;
use libc::isdigit;

// トークンの種類
#[derive(PartialOrd, PartialEq)]
enum TokenKind {
  TK_RESERVED, // 記号
  TK_NUM,      // 整数トークン
  TK_EOF,      // 入力の終わりを表すトークン
}

// トークン型
struct Token {
  kind: TokenKind,          // トークンの型
  next: Box<Option<Token>>, // 次の入力トークン
  val: i8,                  // kindがTK_NUMの場合、その数値
  str: String,              // トークン文字列
}

// 次のトークンが期待している記号のときには、次のトークンを返す
// それ以外の場合にはNoneを返す
fn consume(op: char, token: Token) -> Option<Token> {
  if token.kind != TokenKind::TK_RESERVED || token.str.chars().nth(0).unwrap() != op {
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
  if token.kind != TokenKind::TK_RESERVED || token.str.chars().nth(0).unwrap() != op {
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
  if token.kind != TokenKind::TK_NUM {
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
  return token.kind == TokenKind::TK_EOF;
}

// 新しいトークンを作成してcurに繋げる
fn new_token(kind: TokenKind, cur: *const Token, str: String) -> Box<Token> {
  Box::new(Token {
    kind: kind,
    str: str,
    next: Box::new(None),
    val: 0,
  })
}

// 入力文字列pをトークナイズしてそれを返す
fn tokenize(p: &str) -> Token {
  let head: Token;
  // head.next = NULL;
  let cur = &head;

  // while *p as u8 != b'\0' {
    p.chars().fold('', |mut acc: Box<Token>, c:char| {
    let ptr = c;
    // 空白文字をスキップ
    match ptr {
      it if isspace(it as i32) != 0 => {
        return acc;
      }
      '+' | '-' => return new_token(TokenKind::TK_RESERVED, cur, String::from(c)),
      it if isdigit(c) != 0 => {
        let token = new_token(TokenKind::TK_NUM, acc, String::from(c));
        token.val = c as i8;
        return token;
      }
      _ => eprint!("トークナイズできません"),
    }
  });

  new_token(TK_EOF, cur, p);
  return head.next;
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
