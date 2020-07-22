use std::net::UdpSocket;

const BUF_SIZE: usize = 4096;

fn main() {
    let listener = UdpSocket::bind("0.0.0.0:6543").unwrap();
    let mut buf: Vec<u8> = vec![0; BUF_SIZE];

    println!("Server started listening!");
    let first_address = listener.peek_from(&mut buf).unwrap().1;
    println!(
        "Connection established. Client has IP:PORT {}:{}",
        first_address.ip(),
        first_address.port()
    );

    loop {
        listener.recv_from(&mut buf).unwrap();
        print!("{}", String::from_utf8_lossy(&buf));
        listener.send_to(&buf, first_address).unwrap();
    }
}
