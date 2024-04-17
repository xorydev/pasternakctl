use std::fs;
use std::io::Write;
use std::net;
use std::process::exit;

fn main() {
    let userhome = std::env::var("HOME").unwrap();
    let configpath = format!("{userhome}/.config/pasternakctl/hosts");
    let configfile = fs::read_to_string(configpath).unwrap();
    let hosts: Vec<&str> = configfile.lines().collect();
    for host in hosts {
      let target = format!("{host}:8492");
      if let Ok(mut stream) = net::TcpStream::connect(target) {
        stream.flush().unwrap();
        drop(stream);
      } else {
        eprintln!("Encountered Error. Most likely could not connect. Please check if all the targets are online.");
      }
    }

}
