use boxy::app::App;
use dropbox_sdk::{files, UserAuthClient};
use std::collections::VecDeque;

fn main() {
  let app = App::new();
  list_directory(&app.client, "/", false)
    .unwrap()
    .unwrap()
    .into_iter()
    .for_each(|f| println!("{:?}", &f));
}

// The following code is the same as `demo.rs` in `dropbox-sdk-rust` (https://github.com/dropbox/dropbox-sdk-rust/blob/master/examples/demo.rs)
// It is used temporarily to verify the operation.
fn list_directory<'a, T: UserAuthClient>(
  client: &'a T,
  path: &str,
  recursive: bool,
) -> dropbox_sdk::Result<Result<DirectoryIterator<'a, T>, files::ListFolderError>> {
  assert!(
    path.starts_with('/'),
    "path needs to be absolute (start with a '/')"
  );
  let requested_path = if path == "/" {
    // Root folder should be requested as empty string
    String::new()
  } else {
    path.to_owned()
  };
  match files::list_folder(
    client,
    &files::ListFolderArg::new(requested_path).with_recursive(recursive),
  ) {
    Ok(Ok(result)) => {
      let cursor = if result.has_more {
        Some(result.cursor)
      } else {
        None
      };

      Ok(Ok(DirectoryIterator {
        client,
        cursor,
        buffer: result.entries.into(),
      }))
    }
    Ok(Err(e)) => Ok(Err(e)),
    Err(e) => Err(e),
  }
}

struct DirectoryIterator<'a, T: UserAuthClient> {
  client: &'a T,
  buffer: VecDeque<files::Metadata>,
  cursor: Option<String>,
}

impl<'a, T: UserAuthClient> Iterator for DirectoryIterator<'a, T> {
  type Item = dropbox_sdk::Result<Result<files::Metadata, files::ListFolderContinueError>>;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(entry) = self.buffer.pop_front() {
      Some(Ok(Ok(entry)))
    } else if let Some(cursor) = self.cursor.take() {
      match files::list_folder_continue(self.client, &files::ListFolderContinueArg::new(cursor)) {
        Ok(Ok(result)) => {
          self.buffer.extend(result.entries.into_iter());
          if result.has_more {
            self.cursor = Some(result.cursor);
          }
          self.buffer.pop_front().map(|entry| Ok(Ok(entry)))
        }
        Ok(Err(e)) => Some(Ok(Err(e))),
        Err(e) => Some(Err(e)),
      }
    } else {
      None
    }
  }
}
