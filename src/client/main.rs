use std::env::args;
use std::io::{stdin, BufRead};
use std::net::UdpSocket;
use std::thread;
use std::str;

const BUF_SIZE: usize = 4096;

#[derive(Default, Debug)]
struct Params {
    src_address: String,
    dst_address: String,
    mode: String,
}

fn parse_args() -> std::io::Result<Params> {
    let args: Vec<String> = args().collect();

    let mut params: Params = Default::default();

    let mut state = "";
    for arg in args.iter().skip(1) {
        match state {
            // Search for a state
            "" => {
                match arg.as_str() {
                    "-s" => state = "-s",
                    "-d" => state = "-d",
                    "-m" => state = "-m",
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Argument {arg} is not supported", arg = arg),
                        ))
                    }
                };
            }
            "-s" => {
                params.src_address = arg.clone();
                state = "";
            },
            "-d" => {
                params.dst_address = arg.clone();
                state = "";
            },
            "-m" => {
                params.mode = arg.clone()
            }
            _ => (),
        };
    }

    if params.src_address.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Source address not specified!",
        ));
    }

    if params.dst_address.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Destination address not specified! {:?}", params),
        ));
    }

    Ok(params)
}

fn spawn_recv_loop(socket: UdpSocket) {
    thread::spawn(move || {
        let mut buf: Vec<u8> = vec![0; BUF_SIZE];
        loop {
            let res = socket.recv(&mut buf);
            match res {
                Ok(_nb_chars) => {
                    let word = str::from_utf8(&buf);
                    match word {
                        Ok(text) => print!("{}", text),
                        Err(e) => println!("Can't convert buffer to utf8 string. {:?}", e)
                    };
                },
                Err(e) => println!("Can't receive message from the server. {:?}", e)
            }
        }
    });
}

fn connect_handler(socket: &UdpSocket, address: &str) {
    let connect_res = socket.connect(address);
    match connect_res {
        Ok(_) => println!("Connected to {}", address),
        Err(e) => panic!(
            "Couldn't connect to {dst}.\n{err}",
            dst = address,
            err = e
        ),
    }

    socket.send(format!("Connected to {}", address).as_bytes()).expect("Couldn't send message" );
}

fn main() -> std::io::Result<()> {
    let params = parse_args()?;

    let socket = UdpSocket::bind(params.src_address.as_str());
    let socket = match socket {
        Ok(s) => {
            println!(
                "Successfully binded {src}!",
                src = params.src_address.as_str()
            );
            s
        }
        Err(e) => panic!(
            "Couldn't bind to {src}!\n{err}",
            src = params.src_address.as_str(),
            err = e
        ),
    };

    connect_handler(&socket, params.dst_address.as_str());

    // Get another client address from the server
    let mut buf: Vec<u8> = vec![0; BUF_SIZE];
    let res = socket.recv(&mut buf);
    let client_address = match res {
        Ok(_) => match str::from_utf8(&buf) {
            Ok(text) => text.trim_matches(char::from(0)),
            Err(_) => panic!("Error decoding client address")
        },
        Err(_) => panic!("Error obtaining client address")
    };
    println!("Got client address {} of len {}", client_address, client_address.len());

    connect_handler(&socket, client_address);

    spawn_recv_loop(socket.try_clone()?);

    for line in stdin(). lock(). lines() {
        let msg = line.unwrap().clone() + "\n";
        socket.send(msg.as_bytes())?;
    }

    Ok(())
}
