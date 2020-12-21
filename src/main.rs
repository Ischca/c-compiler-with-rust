use std::process;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  
  if args.len() != 2 {
    eprint!("引数の個数が正しくありません\n");
    process::exit(1);
  }

  println!(".intel_syntax noprefix");
  println!(".globl main");
  println!("main:");
  println!("  mov rax, {}", args[1]);
  println!("  ret");
}
