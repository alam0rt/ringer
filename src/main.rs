use std::io::{BufRead, BufReader, Error};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 1079);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr();

    println!("Listening on {:?}", port);

    let (tcp_stream, _addr) = listener.accept()?;
    let mut stream = BufReader::new(tcp_stream);
    let mut input = String::new();

    stream.read_line(&mut input)?;

    /*
       The Finger query specification is defined:

        {Q1}    ::= [{W}|{W}{S}{U}]{C}

        {Q2}    ::= [{W}{S}][{U}]{H}{C}

        {U}     ::= username

        {H}     ::= @hostname | @hostname{H}

        {W}     ::= /W

        {S}     ::= <SP> | <SP>{S}

        {C}     ::= <CRLF>
    */

    let parsed = Input::new(input);
    println!("{:?}", parsed);

    Ok(())
}

#[derive(Debug)]
struct Input {
    verbose: bool,
    user: Option<String>,
    host: Option<String>,
}

impl Input {
    pub fn new(input: String) -> Result<Input, String> {
        let mut ret = Input {
            user: None,
            host: None,
            verbose: false,
        };

        // split up \W sam@host to "\W", "sam", "host"
        let mut args: Vec<&str> = input.split_whitespace().collect();
	println!("{:?}",args);

        match args.starts_with(&[VERBOSE]) {
            true => {
                ret.verbose = true;
                args.remove(0);
            }
            false => {
                ret.verbose = false;
            }
        }

	println!("{:?}",args);

        match args.len() {
            0 => Ok(ret),
            1 => {
                let query: Vec<&str> = args[0].split('@').collect();
		println!("{:?}",query);
                match query.len() {
                    1 => {
                        ret.user.replace(query[0].to_string());
                        Ok(ret)
                    }
                    2 => {
                        ret.user.replace(query[0].to_string());
                        ret.host.replace(query[1].to_string());
                        Ok(ret)
                    }
                    _ => Err("malformed query: must be <user>@<host>".to_string()),
                }
            }
            _ => return Err("too many args provided".to_string()),
        }
    }
}

#[allow(dead_code)]
const VERBOSE: &str = "\\W";
