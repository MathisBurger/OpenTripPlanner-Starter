use std::process::{Command, Stdio};
use rusty_cli::flags::flag::Flags;
use std::str::FromStr;

pub(crate) fn executor(_flags: Flags) {

    let output = Command::new("ps")
        .arg("aux")
        .stdout(Stdio::piped())
        .output()
        .expect("Cannot run ps");
    let res = output.stdout;
    let string_result = String::from_utf8(res).unwrap();
    let processes = string_result.split("\n").collect::<Vec<&str>>();
    for process in processes {
        if process.contains("java -Xmx")
            && process.contains("-jar otp-2.")
            && process.contains("-shaded.jar")
            && process.contains("--serve") {
            let mut process_split = process.split(" ").collect::<Vec<&str>>();
            process_split.remove(0);
            let mut i: usize = 0;
            while process_split.get(i).unwrap().to_string() == "".to_string() {
                i+=1;
            }
            let process_id = process_split.get(i).unwrap().to_string();
            Command::new("kill")
                .arg(process_id)
                .spawn()
                .expect("Cannot kill OTP");
            println!("Stopped all OTP instances");
        }
    }
}