fn main() {
  for i in 0..10 {
    // match i  {
    //   0 => println!("Hello WasmEdge!"),
    //   1 => println!("Howdy WasmEdge!"),
    //   2 => println!("Howdy WasmEdge!"),
    //   3 => println!("Hola WasmEdge!"),
    //   4 => println!("Bonjour WasmEdge!"),
    //   5 => println!("guten tag WasmEdge!"),
    //   6 => println!("WasmEdge 你好!"),
    //   7 => println!("こんにちは  WasmEdge!"),
    //   _ => println!("Salve WasmEdge!"),
    // }
    if i==0 { println!("Hello WasmEdge!") }
    else if i==1 { println!("Howdy WasmEdge!") }
    else if i==2 { println!("Howdy WasmEdge!") }
    else if i==3 { println!("Hola WasmEdge!") }
    else if i==4 { println!("Bonjour WasmEdge!") }
    else if i==5 { println!("guten tag WasmEdge!") }
    else if i==6 { println!("WasmEdge 你好!") }
    else if i==7 { println!("こんにちは  WasmEdge!") }
    else { println!("Salve WasmEdge!") }
  }
}
