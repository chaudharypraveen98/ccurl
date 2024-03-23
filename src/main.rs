mod cli;
mod constanst;
use cli::get_arguments;
use constanst::{DEFAULT_PORT, PROTOCOL_STRING};
use std::io::{Read, Write};
use std::net::TcpStream;

fn parse_url(url: &str) -> (&str, &str, &str, String) {
    let (protocol, rest) = url.split_once("://").unwrap();
    let (temp_hostname, pathname) = rest.split_once("/").unwrap();
    let (hostname,port) = if temp_hostname.contains(":") {
        temp_hostname.split_once(":").expect("Invalid hostname")
    } else{
        (temp_hostname,DEFAULT_PORT)
    };
    let socket_addr = format!("{}:{}", hostname, port);
    let protocol_str = PROTOCOL_STRING
        .get(protocol)
        .expect("invalid protocol");

    (protocol_str, hostname, pathname, socket_addr)
}
fn populate_get_request(
    protocol: &str,
    host: &str,
    path: &str,
    data: Option<&String>,
    method: Option<&String>,
    headers: Vec<&str>,
) -> String {
    let default_method = String::from("GET");
    let method = method.unwrap_or(&default_method);
    let mut res = String::new();
    res += &format!("{} /{} {}\r\n", method, path, protocol);
    res += &format!("Host: {}\r\n", host);
    res += "Accept: */*\r\n";
    res += "Connection: close\r\n";

    if method == "POST" || method == "PUT" {
        if headers.len() > 0 {
            for head in headers {
                res += head;
            }
            res += "\r\n"
        } else {
            res += "Content-Type: application/json\r\n";
        }
        if let Some(data_str) = data {
            let data_bytes = data_str.as_bytes();
            res += &format!("Content-Length: {}\r\n\r\n", data_bytes.len());
            res += data_str;
            res += "\r\n";
        }
    }

    res += "\r\n";
    res
}

fn parse_resp(resp: &str) -> (&str, &str) {
    let (response_header, response_data) = resp.split_once("\r\n\r\n").unwrap();
    (response_header, response_data)
}

fn main() {
    let matches = get_arguments();

    // argument matching
    let verbose_enabled = matches.contains_id("verbose") && matches.get_flag("verbose");
    let url = matches.get_one::<String>("url").unwrap();
    let data = matches.get_one::<String>("data");
    let method = matches.get_one::<String>("x-method");
    let headers: Vec<&str> = matches
        .get_many::<String>("headers")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    let (protocol, hostname, pathname, socket_addr) = parse_url(url);
    let buffer_str = populate_get_request(protocol, hostname, &pathname, data, method, headers);

    let tcp_socket = TcpStream::connect(socket_addr);

    match tcp_socket {
        Ok(mut stream) => {
            if verbose_enabled {
                let lines = buffer_str.lines();
                for line in lines {
                    println!("> {}", line)
                }
            }
            stream
                .write_all(buffer_str.as_bytes())
                .expect("Failed to write data to stream");

            // initialising the buffer, reads data from the stream and stores it in the buffer.
            let mut buffer = [0; 1024];
            stream
                .read(&mut buffer)
                .expect("Failed to read from response from host!");

            // converts buffer data into a UTF-8 enccoded string (lossy ensures invalid data can be truncated).
            let response = String::from_utf8_lossy(&buffer[..]);

            // dividing the response headers and body
            let (response_header, response_data) = parse_resp(&response);
            if verbose_enabled {
                let lines = response_header.split("\r\n");
                for line in lines {
                    println!("< {}", line)
                }
            }
            println!("{}", response_data);
        }
        Err(e) => {
            eprintln!("Failed to establish connection: {}", e);
        }
    }

    // Ok(())
}
