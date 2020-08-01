use std::io::{stdin, BufRead};
use std::net::UdpSocket;
use std::str;
use std::thread;

use clap::{App, Arg};

const BUF_SIZE: usize = 4096;

struct Params {
    src_address: String,
    dst_address: String,
    protocol: String, // TODO: Replace with enum
}

fn parse_args() -> Params {
    let matches = App::new("Hole punching: client")
        .arg(
            Arg::with_name("src_address")
                .short('s')
                .long("src-address")
                .about("Address to bind client on (X.X.X.X:port)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("dst_address")
                .short('d')
                .long("dst-address")
                .about("Server address (X.X.X.X:port)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("protocol")
                .short('p')
                .long("protocol")
                .about("Which protocol to use")
                .possible_values(&["udp"])
                .default_value("udp")
                .takes_value(true),
        )
        .get_matches();

    return Params {
        src_address: matches.value_of("src_address").unwrap().to_string(),
        dst_address: matches.value_of("dst_address").unwrap().to_string(),
        protocol: matches.value_of("protocol").unwrap().to_string(),
    };
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
                        Err(e) => println!("Can't convert buffer to utf8 string. {:?}", e),
                    };
                }
                Err(e) => println!("Can't receive message from the server. {:?}", e),
            }
            buf.iter_mut().map(|x| *x = 0).count();
        }
    });
}

fn connect_handler(socket: &UdpSocket, address: &str) {
    let connect_res = socket.connect(address);
    match connect_res {
        Ok(_) => println!("Connected to {}", address),
        Err(e) => panic!("Couldn't connect to {dst}.\n{err}", dst = address, err = e),
    }

    socket
        .send(format!("Connected to {}", address).as_bytes())
        .expect("Couldn't send message");
}

fn main() -> std::io::Result<()> {
    let params = parse_args();

    println!("Using protocol {}", params.protocol.to_uppercase());

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
            Err(_) => panic!("Error decoding client address"),
        },
        Err(_) => panic!("Error obtaining client address"),
    };
    println!(
        "Got client address {} of len {}",
        client_address,
        client_address.len()
    );

    connect_handler(&socket, client_address);

    spawn_recv_loop(socket.try_clone()?);

    for line in stdin().lock().lines() {
        let msg = line.unwrap().clone() + "\n";
        socket.send(msg.as_bytes())?;
    }

    Ok(())
}
