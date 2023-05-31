use std::path::Path;

fn main() {
    let my_file = Path::new("mytile.txt");
    // This yields a Result type of String or an error
    let contents = std::fs::read_to_string(my_file);
    // Let's just handle the error by printing it out
    /*match contents {
        Ok(contents) => println!("File contents: {contents}"),        
        Err(e) => println!("ERROR: {e:#?}"),
    }*/

    // Let's handle individual kinds of errors
    /*match contents {
        Ok(contents) => println!("File contents: {contents}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found"),
            std::io::ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("ERROR: {e:#?}"),
        },
    }*/

    match contents {
        Ok(contents) => println!("File contents: {contents}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found"),
            std::io::ErrorKind::PermissionDenied => println!("Permission denied"),
            std::io::ErrorKind::ConnectionRefused => todo!(),
            std::io::ErrorKind::ConnectionReset => todo!(),
            std::io::ErrorKind::ConnectionAborted => todo!(),
            std::io::ErrorKind::NotConnected => todo!(),
            std::io::ErrorKind::AddrInUse => todo!(),
            std::io::ErrorKind::AddrNotAvailable => todo!(),
            std::io::ErrorKind::BrokenPipe => todo!(),
            std::io::ErrorKind::AlreadyExists => todo!(),
            std::io::ErrorKind::WouldBlock => todo!(),
            std::io::ErrorKind::InvalidInput => todo!(),
            std::io::ErrorKind::InvalidData => todo!(),
            std::io::ErrorKind::TimedOut => todo!(),
            std::io::ErrorKind::WriteZero => todo!(),
            std::io::ErrorKind::Interrupted => todo!(),
            std::io::ErrorKind::Unsupported => todo!(),
            std::io::ErrorKind::UnexpectedEof => todo!(),
            std::io::ErrorKind::OutOfMemory => todo!(),
            std::io::ErrorKind::Other => todo!(),
            _ => todo!(),            
        },
    }
}
