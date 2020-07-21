use std::env::args;
use std::io::{stdin, BufRead};
use std::net::UdpSocket;

#[derive(Default, Debug)]
struct Params {
    src_address: String,
    dst_address: String,
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
            }
            "-d" => {
                params.dst_address = arg.clone();
                state = "";
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

    let connect_res = socket.connect(params.dst_address.as_str());
    match connect_res {
        Ok(_) => println!("Connected to {dst}", dst = params.dst_address.as_str()),
        Err(e) => panic!(
            "Couldn't connect to {dst}.\n{err}",
            dst = params.dst_address.as_str(),
            err = e
        ),
    }

    for line in stdin().lock().lines() {
        let msg = line.unwrap().clone() + "\n";
        socket.send(msg.as_bytes())?;
    }

    Ok(())
}
