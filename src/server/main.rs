use std::net::{SocketAddr, UdpSocket};

const BUF_SIZE: usize = 1024;

fn get_clients_info(listener: &UdpSocket) -> (SocketAddr, SocketAddr) {
    let mut tmp_buf: Vec<u8> = Vec::with_capacity(BUF_SIZE);
    let first_address = listener.recv_from(&mut tmp_buf).unwrap().1;
    #[cfg(debug_assertions)]
    println!(
        "First client connection established. {}",
        first_address.to_string(),
    );

    let second_address = loop {
        let addr = listener.recv_from(&mut tmp_buf).unwrap().1;
        if addr != first_address {
            break addr;
        }
    };

    #[cfg(debug_assertions)]
    println!(
        "Second client connection established. {}",
        second_address.to_string(),
    );

    (first_address, second_address)
}

fn main() {
    let listener = UdpSocket::bind("0.0.0.0:6543").unwrap();

    println!("Server started listening!");

    loop {
        let (first_client, second_client) = get_clients_info(&listener);

        listener
            .send_to(first_client.to_string().as_bytes(), second_client)
            .expect(format!("Error to send second client {}", second_client.to_string()).as_str());
        listener
            .send_to(second_client.to_string().as_bytes(), first_client)
            .expect(format!("Error to send second client {}", second_client.to_string()).as_str());
    }
}
