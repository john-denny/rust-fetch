use regex::Regex;
use std::fs;
use std::process::Command;

// Function to get the devices uptime
fn get_uptime() -> Result<String, std::io::Error> {
    let output = Command::new("uptime").arg("-p").output()?;

    if output.status.success() {
        let uptime_str = String::from_utf8_lossy(&output.stdout);
        Ok(uptime_str.trim().to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to execute 'uptime': {:?}", output),
        ))
    }
}

// Function to get the current date and time
fn get_date() -> String {
    let output = Command::new("date")
        .output()
        .expect("Unable to get Time Information");
    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

fn main() {
    // Open the OS file
    let os_file: String =
        fs::read_to_string("/etc/os-release").expect("Unable to read OS Details from OS file");

    // Get the Os's "Pretty Name"
    let get_pretty_name_re = Regex::new(r#"PRETTY_NAME="([^"]+)""#).unwrap();

    let mut pretty_name: String = String::new();

    if let Some(captures) = get_pretty_name_re.captures(&os_file) {
        if let Some(matched) = captures.get(1) {
            pretty_name = matched.as_str().to_string();
        }
    }

    // Get the Computer's model name
    let mut model: String = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .expect("Unable to read Computer's Model from file");

    model.pop(); // Remove the newline character from the model number string

    // Slices the uptime
    let uptime = get_uptime().expect("Failed to get uptime");
    let uptime = &uptime[3..];

    let tux = format!(
        r#"
     .---.      {}
    /     \     OS: {}
    \.@-@./     HOST: {}
    /`\_/`\     Uptime: {}
   //  _  \\    
  | \     )|_     
 /`\_`>  <_/ \     
 \__/'---'\__/"#,
        get_date(),
        pretty_name,
        model,
        uptime,
    );

    // Print out the fetch except for the first \n character
    println!("{}", &tux[1..]);
}
