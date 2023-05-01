pub mod data_stream;
pub mod types;
pub mod status;
pub mod ftp;

pub use self::ftp::FtpStream;
pub use self::types::FtpError;

// #[macro_use]
// extern crate lazy_static;
// extern crate chrono;
// extern crate regex;

pub type Result<T> = std::result::Result<T, FtpError>;

fn main() {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();

    let _ = ftp_stream.login("ftpuser", "Password123").unwrap();

    println!("Current directory: {}", ftp_stream.pwd().unwrap());
}
