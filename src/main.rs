use std::{
    io::{Read, Write},
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
    time::Instant,
};

fn server(path: PathBuf) {
    let mut f = std::fs::File::open(path).unwrap();
    let size = f.metadata().unwrap().len();

    let socket = std::net::TcpListener::bind("0.0.0.0:0").unwrap();

    println!("Listening on {}", socket.local_addr().unwrap());

    let (mut tx, _) = socket.accept().unwrap();

    tx.write_all(&u64::to_le_bytes(size)).unwrap();

    let start = Instant::now();
    std::io::copy(&mut f, &mut tx).unwrap();
    let start = start.elapsed();
    println!(
        "File sent in {start:?} ({} bytes/s)",
        size as f64 / start.as_secs_f64()
    );
}

fn client() {
    let ip_port = {
        print!("Enter peer IP address and port: ");
        std::io::stdout().lock().flush().unwrap();

        let mut addr = String::new();
        std::io::stdin().read_line(&mut addr).unwrap();

        SocketAddr::from_str(addr.trim()).unwrap()
    };

    let mut rx = std::net::TcpStream::connect(ip_port).unwrap();

    let mut size = [0u8; 8];
    rx.read_exact(&mut size).unwrap();
    let size = u64::from_le_bytes(size);

    let mut f = std::fs::File::create(std::env::temp_dir().join("received.bin")).unwrap();

    let start = Instant::now();
    std::io::copy(&mut rx.take(size), &mut f).unwrap();
    let start = start.elapsed();
    println!(
        "File received in {start:?} ({} bytes/s)",
        size as f64 / start.as_secs_f64()
    );
}

fn main() {
    let path = {
        print!("Enter file path or press enter to receive: ");
        std::io::stdout().lock().flush().unwrap();

        let mut path = String::new();
        std::io::stdin().read_line(&mut path).unwrap();

        let path = path.trim();

        if path.is_empty() {
            None
        } else {
            Some(PathBuf::from(path))
        }
    };

    if let Some(path) = path {
        server(path);
    } else {
        client();
    }
}
