use std::process;
use std::env;
use libc::strtol;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

fn main() {
  let args: Vec<String> = env::args().collect();
  
  if args.len() != 2 {
    eprint!("引数の個数が正しくありません\n");
    process::exit(1);
  }

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
        },
        '-' => {
          p = p.offset(1);
          println!("  sub rax, {}", strtol(p, &mut p as *mut *mut i8, 10));
        },
        _ => panic!("予期しない文字です: {}", *p),
      }
    }
  }
  
  println!("  ret");
}
