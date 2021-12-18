use boxy::app::{App, Metadata};
use boxy::command::ls::ls;

fn main() {
  let app = App::new();
  let ret = ls(&app.client, None, None);
  ret.iter().for_each(|f| {
    let metadata = Metadata::from(f.to_owned());
    println!("{:?}", metadata.to_string());
  })
}
