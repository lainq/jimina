use std::{
  collections::HashMap,
  env,
  fs::{self, File, OpenOptions},
  io::Write,
  path::Path,
  time::{SystemTime, UNIX_EPOCH},
};

/// Get the already stored content from the data
/// file. Data is stored in csv format
fn get_stored_content(path: &Path) -> String {
  let does_exists = path.exists();
  if !does_exists {
    File::create(path).expect("Something unexpected happened while reading stored content");
    String::new()
  } else {
    let file_name = path.display().to_string();
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read {file_name}"))
  }
}

/// A function for throwing errors related
/// to invalid value in the file
fn invalid_content_error(msg: &str, lc: usize, fname: &String) {
  panic!("Error in {fname} at {lc}: {msg}");
}

/// Convert the stored content into a hashmap
fn parse_contents_into_map(content: &str, fname: &String) -> HashMap<String, u128> {
  let mut map: HashMap<String, u128> = HashMap::new();
  for (i, line) in content.lines().enumerate() {
    if line.trim().is_empty() {
      continue;
    }
    // Split the lines by commas and obtaint he first and second
    // value and insert them into the hashmap
    let split: Vec<&str> = line.split(',').collect();
    if split.len() < 2 {
      invalid_content_error("Insufficient amount of values", i + 1, fname);
    } else {
      let left = split[0];
      let right = split[1].parse::<u128>(); // Returns a result
      match right {
        Ok(value) => {
          map.insert(String::from(left), value);
        }
        Err(_) => invalid_content_error("Invalid value", i + 1, fname),
      }
    }
  }
  map
}

fn get_current_time_as_ms() -> u128 {
  let start = SystemTime::now();
  start
    .duration_since(UNIX_EPOCH)
    .unwrap_or_else(|_| {
      panic!("Error while retrieving time");
    })
    .as_millis()
}

// Return the hashmap contents as a string
// format: [key], [value]\n
fn get_hashmap_as_string(hm: &HashMap<String, u128>) -> String {
  let mut value = String::new();
  for (key, v) in hm.iter() {
    value.push_str(format!("{key},{v}").as_str());
    value.push('\n');
  }
  value
}

// Update the data file with the new data
fn update_file(path: &Path, content: String) {
  let mut file = OpenOptions::new().write(true).open(path).unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

// Convert milliseconds into a string
// Mainly used inorder to display the time difference between
// now and last action
// format: [day] days [hour] hours [minute] minutes [second] seconds
fn milliseconds_to_string(ms: u128) -> String {
  let mut seconds = ms / 1000;
  let mut minutes = seconds / 60;
  let mut hours = minutes / 60;
  let days = hours / 24;

  seconds %= 60;
  minutes %= 60;
  hours %= 24;

  format!(
    "{} days {:02} hours {:02} minutes {:02} seconds",
    days, hours, minutes, seconds
  )
}

fn get_time_difference_as_str(t: u128) -> String {
  let time_diff = get_current_time_as_ms() - t;
  milliseconds_to_string(time_diff)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Provide an argument with the program");
    std::process::exit(1);
  }

  // The file path where all the app data is stored
  // file: ~/.daysince.json
  let fpath = home::home_dir()
    .expect("Failed to locate the home directory")
    .join(".daysince.json");
  let command = &args[1];

  let fname_as_str = fpath.display().to_string();
  let mut content = parse_contents_into_map(&get_stored_content(&fpath), &fname_as_str);
  if command == "did" {
    if args.len() < 3 {
      panic!("Tf did you do dawg");
    }
    let what = &args[2];
    let time = get_current_time_as_ms();

    // Update the value if the key already exists in the map
    // If it doesnt already exist, add the key and set the value to time
    (*content.entry(what.clone()).or_insert(time)) = time;
    update_file(&fpath, get_hashmap_as_string(&content));
    println!("Ok");
  } else {
    match content.get(command) {
      Some(value) => {
        println!(
          "[You havent done {command} for]\n {}",
          get_time_difference_as_str(*value)
        );
      }
      None => {
        println!("You never did {command}");
      }
    }
  }
}
