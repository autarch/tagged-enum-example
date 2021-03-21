use serde::Deserialize;

const DOC: &str = r#"
{
   "Root":{
      "first":{
         "Foo":{
            "size":42,
            "color":"blue"
         }
      },
      "second":{
         "Bar":{
            "mood":"indigo",
            "car":"Super"
         }
      },
      "actions":[
         {
            "Run":{
               "speed":84
            }
         },
         {
            "Sleep":{
               "hours":8
            }
         }
      ]
   }
}
"#;

#[derive(Debug, Deserialize)]
struct Root {
    first: Foo,
    second: Option<Bar>,
    actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
struct Foo {
    size: i8,
    color: String,
}

#[derive(Debug, Deserialize)]
struct Bar {
    mood: String,
    car: String,
}

#[derive(Debug, Deserialize)]
enum Action {
    Run { speed: i64 },
    Sleep { hours: i8 },
}

fn main() {
    let output: Root = serde_json::from_str(DOC).expect("parsed");
    println!("{:#?}", output);
}
