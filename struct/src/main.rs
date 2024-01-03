#[derive(Debug, PartialEq, Eq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  French,
  German,
  Czech
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::French, message: String::from("Bonjour WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::German, message: String::from("Hallo WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Czech, message: String::from("Ahoj WasmEdge!") };
  v.push(g);

  let check_lang = Lang::Czech;

  for e in v {
    if e.lang == check_lang {
      println!("{:?} {}", e.lang, e.message);
    }
  }
}
