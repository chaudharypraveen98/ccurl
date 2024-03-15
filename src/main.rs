mod constanst;
use constanst::{DEFAULT_PORT, METHOD_TYPES, PROTOCOL_STRING};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use clap::{Arg, Command};
use tokio::{self, stream};

fn parse_url(url: &str) -> (&str, &str, &str, &str,String) {
    let (temp_protocol, rest) = url.split_once("://").unwrap();
    let (mut hostname, pathname) = rest.split_once("/").unwrap();
    let socket_addr = hostname.to_string();
    let mut port = DEFAULT_PORT;
    if hostname.contains(":") {
        (hostname, port) = hostname.split_once(":").expect("Invalid hostname");
    }
    let mut protocol = PROTOCOL_STRING
        .get("http")
        .expect("Unable to find http in protocol string hashmap");
    if "http" != temp_protocol && PROTOCOL_STRING.contains_key(temp_protocol) {
        protocol = PROTOCOL_STRING
            .get(temp_protocol)
            .expect("protocol string not defined")
    }
    return (protocol, hostname, port, pathname,socket_addr);
}
fn populate_get_request(
    protocol: &str,
    host: &str,
    path: &str,
    data: Option<&String>,
    method: Option<&String>,
) -> String {
    let default_method = String::from("GET");
    let method = method.unwrap_or(&default_method);
    let mut res = String::new();
    res += &format!("{} /{} {}\r\n", method, path, "HTTP/1.1");
    res += &format!("Host: {}\r\n", host);
    res += "Accept: */*\r\n";
    res += "Connection: close\r\n";

    if method == "POST" || method == "PUT" {
        res += "Content-Type: application/json\r\n";
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

fn handle_client(mut stream: TcpStream) {
    // this is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // this line reads data from the stream and stores it in the buffer.
    stream
        .read(&mut buffer)
        .expect("Failed to read from client!");
    // this line converts the data in the buffer into a UTF-8 enccoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response!");
}
fn send_get(mut stream: TcpStream) {
    let response = "Hello, Client!".as_bytes();
    stream
        .write_all(response)
        .expect("Failed to write response!");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Ccurl - custom curl")
        .about("It helps to make http methods")
        .version("1.0")
        .disable_version_flag(true)
        .author("Praveen Chaudhary <chaudharypraveen98@gmail.com>")
        .arg(Arg::new("url").index(1).required(true))
        .arg(
            Arg::new("x-method")
                .help("Http method which you want to use")
                .long("x-method")
                .short('X'),
        )
        .arg(
            Arg::new("data")
                .help("Payload you want to send with the request")
                .long("data")
                .short('d'),
        )
        .arg(
            Arg::new("headers")
                .help("Request header")
                .long("header")
                .short('H'),
        )
        .arg(
            Arg::new("verbose")
                .help("verbose mode")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();
    let verbose_enabled = matches.contains_id("verbose") && matches.get_flag("verbose");

    let url = matches.get_one::<String>("url").unwrap();
    let data = matches.get_one::<String>("data");
    let mut headers = "";
    if let Some(input_headers) = matches.get_one::<String>("data") {
        headers = input_headers;
    };
    let method = matches.get_one::<String>("x-method");
    let (protocol, hostname, port, pathname,socket_addr) = parse_url(url);
    // fn populate_get_request(protocol: &str, host: &str, path: &[&str], data: Option<(&str, &str)>, method: Option<&str>) -> String {
    let buffer_str = populate_get_request(protocol, hostname, &pathname, data, method);
    let tcp_socket = TcpStream::connect(socket_addr);
    match tcp_socket {
        Ok(mut stream) => {
            println!("Sending request {}",buffer_str);
            stream.write(buffer_str.as_bytes())?;
            let mut buffer = [0; 1024];
            // this line reads data from the stream and stores it in the buffer.
            stream
                .read(&mut buffer)
                .expect("Failed to read from response from host!");
            // this line converts the data in the buffer into a UTF-8 enccoded string.
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("{}", request);

            // std::thread::spawn(|| handle_client(stream,data));
        }
        Err(e) => {
            eprintln!("Failed to establish connection: {}", e);
            // stderr - standard error stream
        }
    }
    // let method = match given_method {
    //     "POST" => Method::POST,
    //     "DELETE" => Method::DELETE,
    //     "PUT" => Method::PUT,
    //     "PATCH" => Method::PATCH,
    //     "GET" => Method::GET,
    //     _ => {
    //         eprintln!("INVALID METHOD");
    //         panic!("INVALID METHOD")
    //     }
    // };
    // let client = Client::new();
    // let response = client.request(method, url).send().await.unwrap();
    // let response = get_response(client, url, method).await;
    // let issue_list_url = Url::parse(url)?;
    // let transport_protocol: HashMap<&str, &str> = [("http", "HTTP/1.1"), ("https", "HTTP/2.0")]
    //     .iter()
    //     .cloned()
    //     .collect();

    // let schema = issue_list_url.scheme();
    // let mut protocol = "HTTP/1.1";
    // if let Some(value) = transport_protocol.get(schema) {
    //     protocol = value;
    // }
    // if verbose_enabled {
    //     println!(
    //         "> {} {} {}\n> Host: {}\n> Accept: */*\n>",
    //         given_method,
    //         issue_list_url.path(),
    //         protocol,
    //         issue_list_url.host().unwrap().to_string()
    //     );
    //     println!("< {}: {}", protocol, response.status());
    //     for (name, value) in response.headers().iter() {
    //         println!("< {}: {:?}", name, value);
    //     }
    // }
    // if response.status().is_success() {
    //     println!("{:?}", response.text().await?);
    // }

    Ok(())
}
