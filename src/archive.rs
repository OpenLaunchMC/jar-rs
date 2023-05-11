pub mod archive {
  use zip::{ZipArchive,read , result , write};
  use std::fs::File;
  
  pub async fn read_jar_file(path : String) -> Result <ZipArchive,> {
    // Open Zip File
    let file = File::open(path).unwrap();
    let jar = ZipArchive::new(file).unwrap();
    Ok(jar);
  }

  
}