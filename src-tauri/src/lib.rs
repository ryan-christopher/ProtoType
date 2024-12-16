use rdev::{listen, EventType, Event};
use std::thread;
use tauri::Manager;

#[tauri::command]
fn pass_key(curr_key: &str){
    println!("{curr_key}");
}
/*
#[tauri::command]
fn get_data(handle: tauri::AppHandle) -> String {
  // find path
  let resource_path = handle.path().resolve("resources/settings.json", tauri::path::BaseDirectory::Resource);

  println!("resource path:");
  println!("{}", resource_path.expect("REASON").display());
  println!("-------");

  //let file = std::fs::File::open(&resource_path).unwrap();

  //let testfile: serde_json::Value = serde_json::from_reader(file).unwrap();

  //testfile.get("testkey").unwrap().to_string()
  "test".to_string()
}
*/


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let _handler = thread::spawn(|| {
    // This will block.
    if let Err(error) = listen(callback) {
      println!("Error: {:?}", error)
    }
    fn callback(event: Event) {
      if let EventType::KeyPress(_) = event.event_type{
        match event.name {
          Some(string) => pass_key(&string),
          None => (),
      }
      }
    }
  });

  tauri::Builder::default()
    .setup(|app| {
      let resource_path = app.path().resolve("resources/settings.json", tauri::path::BaseDirectory::Resource)?;
      let file = std::fs::File::open(&resource_path).unwrap();
      let testfile: serde_json::Value = serde_json::from_reader(file).unwrap();
      println!("{}", testfile);
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![pass_key])
    //.invoke_handler(tauri::generate_handler![get_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
