use boxy::app::App;
use boxy::command::ls::ls;
use dropbox_sdk::files::Metadata;

fn main() {
  let app = App::new();
  let ret = ls(&app.client, None, None);
  ret.iter().for_each(|f| match f {
    Metadata::File(f) => println!("{:?}", &f.name),
    Metadata::Folder(f) => println!("{:?}", &f.name),
    Metadata::Deleted(f) => println!("{:?}", &f.name),
  })
}
