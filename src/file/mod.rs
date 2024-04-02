//! Crate file provides a file interface.

use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

/// read() is get the data of a file.
///
/// # Examples
/// ```no_run
/// use common_library::file::read;
///
/// let file_name = String::from("test.txt");
/// match read(&file_name) {
///     Ok(_data) => println!("{}", _data),
///     Err(e) => println!("{}", e),
/// }
/// ```
pub fn read(file_name: &String) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;

    let mut data = String::new();

    file.read_to_string(&mut data)?;

    Ok(data)
}

/// write() is write data to file.
///
/// # Examples
/// ```no_run
/// use common_library::file::write;
///
/// let file_name = String::from("test.txt");
/// let data = String::from("test data");
/// match write(&file_name, &data) {
///     Ok(_) => println!("Ok"),
///     Err(e) => println!("{}", e),
/// }
/// ```
pub fn write(file_name: &String, data: &String) -> Result<(), io::Error> {
    let mut file = File::create(file_name)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}

/// remove() is delete a file.
///
/// # Examples
/// ```no_run
/// use common_library::file::remove;
///
/// let file_name = String::from("test.txt");
/// match remove(&file_name) {
///     Ok(_) => println!("Ok"),
///     Err(e) => println!("{}", e),
/// }
/// ```
pub fn remove(file_name: &String) -> Result<(), io::Error> {
    fs::remove_file(file_name)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn read_test() {
        let file_name = String::from("file-test-") + &Uuid::new_v4().to_string();
        let data = String::from("test data");

        match write(&file_name, &data) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e),
        }

        match read(&file_name) {
            Ok(_data) => assert_eq!(_data, data),
            Err(e) => assert!(false, "{}", e),
        }

        match remove(&file_name) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn write_test() {
        read_test();
    }

    #[test]
    fn remove_test() {
        read_test();
    }
}
