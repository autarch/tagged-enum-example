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
enum RootWrapper {
    Root(Root),
}

#[derive(Debug, Deserialize)]
struct Root {
    first: FooWrapper,
    second: Option<BarWrapper>,
    actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
enum FooWrapper {
    Foo(Foo),
}

#[derive(Debug, Deserialize)]
struct Foo {
    size: i8,
    color: String,
}

#[derive(Debug, Deserialize)]
enum BarWrapper {
    Bar(Bar),
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
    let output: RootWrapper = serde_json::from_str(DOC).expect("parsed");
    println!("{:#?}", output);
    let RootWrapper::Root(root) = output;
    let FooWrapper::Foo(foo) = root.first;
    println!("{:#?}", foo);
}
