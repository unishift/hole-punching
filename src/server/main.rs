use std::net::{SocketAddr, UdpSocket};

const BUF_SIZE: usize = 1024;

fn get_clients_info(listener: &UdpSocket) -> (SocketAddr, SocketAddr) {
    let mut tmp_buf: Vec<u8> = Vec::with_capacity(BUF_SIZE);
    let first_address = listener.recv_from(&mut tmp_buf).unwrap().1;
    #[cfg(debug_assertions)]
    println!(
        "First client connection established. ip:port {}:{}",
        first_address.ip(),
        first_address.port()
    );

    let second_address = loop {
        let addr = listener.recv_from(&mut tmp_buf).unwrap().1;
        if addr != first_address {
            break addr;
        }
    };

    #[cfg(debug_assertions)]
    println!(
        "Second client connection established. ip:port {}:{}",
        second_address.ip(),
        second_address.port()
    );

    (first_address, second_address)
}

fn main() {
    let listener = UdpSocket::bind("0.0.0.0:6543").unwrap();

    println!("Server started listening!");

    let (first_client, second_client) = get_clients_info(&listener);

    loop {
        let mut buf: Vec<u8> = vec![0; BUF_SIZE];
        let recv_addr = listener.recv_from(&mut buf).unwrap().1;

        #[cfg(debug_assertions)]
        print!("Data: {}", String::from_utf8_lossy(&buf));

        let sent_data_size = if recv_addr == first_client {
            listener.send_to(&buf, second_client).unwrap()
        } else if recv_addr == second_client {
            listener.send_to(&buf, first_client).unwrap()
        }
        else {
            0
        };
        #[cfg(debug_assertions)]
        println!("Bytes of sent data: {}", sent_data_size);
    }
}
