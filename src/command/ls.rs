use dropbox_sdk::{
  files::{list_folder, list_folder_continue, ListFolderArg, ListFolderContinueArg, Metadata},
  UserAuthClient,
};

pub fn ls<'a, T: UserAuthClient>(
  client: &T,
  path: Option<String>,
  cursor: Option<String>,
) -> Vec<Metadata> {
  let response = match cursor {
    Some(cursor) => list_folder_continue(client, &ListFolderContinueArg::new(cursor))
      .unwrap()
      .unwrap(),
    None => {
      let path = path.unwrap_or(String::new());
      list_folder(client, &ListFolderArg::new(path))
        .unwrap()
        .unwrap()
    }
  };
  let mut metadata: Vec<Metadata> = response.entries.into();
  if response.has_more {
    metadata.extend(ls(client, None, Some(response.cursor)));
  }
  metadata
}
