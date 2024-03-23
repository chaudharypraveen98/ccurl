# Build Your Own curl
We are going to build curl from scratch by accepting the coding challenge posted on [Coding Challenges FYI](https://codingchallenges.fyi/challenges/challenge-curl/).

Before moving ahead, you must need to know how tcp client server connections works.

<img src="https://media.geeksforgeeks.org/wp-content/uploads/Socket_server-1.png" alt="client server socket connection">

You can read more about [GeeksforGeeks](https://www.geeksforgeeks.org/tcp-server-client-implementation-in-c/).

On Server Side : -
1. **Socket**:- Socket object to expose our endpoints.
2. **setsockopt**:- This functions set the extra options for the sockets if needed.
3. **Bind**:- It binds the socket with the ip and port.
4. **Listen**:- Socket is now in *listening* state, and listen at the specified port for the incoming connection request. Here it queue incoming client request to connect. The second argument specifies the maximum number of request it can queue.
5. **Accept**:- In this phase, server calls accept function, it initiates the 3-way handshaking. The client sends **SYN** packet to the server, server responds back with the **SYN-ACK** packet and blocks(wait) the connection, until it finally get the **ACK** packet.
6. **Send/Recv**:- Once **ACK** received from client, communication can proceed to and fro.

On Client Side : -
1. **Socket initialization**:- In this setup, socket is defined with all the configuration needed to connect.
2. **Connect**:- In this phase, client call the connect function, which sends the SYN packet to the server with the intent to connect.
3. **Send/Recv**: Once connection is established, when client can send and receive the data.


We are doing to acheive in few steps: - 
1. Getting the cli arguments
2. Creation of socket connection
3. Sending the request
4. Parsing the response

## 1. Getting the cli arguments.
We will be using library for [Clap](https://crates.io/crates/clap) - A simple-to-use, efficient, and full-featured library for parsing command line arguments and subcommands.

Tha clap library provides two different ways to build parse object. First is the **Builder** pattern(creational design pattern to create complex things by step by step process) and second **Derive** pattern in which library automatically generate code based on the macros.

We are using **Builder** pattern for our cli tool.

But you can implement **Derive pattern** at [doc.rs/clap/_derive](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html)

'[cli.rs]("./src/cli.rs")'
```
use clap::{Arg, ArgMatches, Command};

pub fn get_arguments()-> ArgMatches{
    Command::new("Ccurl - custom curl")
        .about("It helps to make http methods")
        .version("1.0")
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
                .short('H')
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("verbose")
                .help("verbose mode")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}
```
Firstly, we have define the basic info like about, author and  version. 

We have defined all the arguments need for our own curl. We have made one positional required argument **url**.

### Argument matching or parsing
Clap makes it easier to match arguments.  
For **verbose**, we have used [action](https://docs.rs/clap/4.5.3/clap/struct.Arg.html#method.action) method `.action(clap::ArgAction::SetTrue)`  because it will not contain any subsequent value.
For **headers**, similarly we have used [action](https://docs.rs/clap/4.5.3/clap/struct.Arg.html#method.action) method `.action(ArgAction::Append)`, **Append** will append new values to the previous value if any value have already encountered.
For others, we have simply used [get_one](https://docs.rs/clap/4.5.3/clap/struct.ArgMatches.html#method.get_one) method to get the value.

```
let verbose_enabled = matches.contains_id("verbose") && matches.get_flag("verbose");
    let url = matches.get_one::<String>("url").unwrap();
    let data = matches.get_one::<String>("data");
    let method = matches.get_one::<String>("x-method");
    let headers: Vec<&str> = matches
        .get_many::<String>("headers")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();
```
## 2. Drafting request with input information [ HTTP 1.1 - RFC9110.]
We will be using the [RFC9110](https://datatracker.ietf.org/doc/html/rfc9110) for HTTP 1.1 client. 

we will start with empty string, and append all the information needed for the Request according to RFC.

```
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
    ....
    ....
    ....
    res += "\r\n";
    res
}
```

For **PUT** and **POST**, we need to add headers and data.

```
fn populate_get_request(
    protocol: &str,
    host: &str,
    path: &str,
    data: Option<&String>,
    method: Option<&String>,
    headers: Vec<&str>,
) -> String {
    ....
    ....

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

    ....
    res
}
```
According to RFC, for post or post we need to provide **Content-Length** and **Content-Type** header.


So now we have complete request string. Let's move to socket connection, sending this request string to the server.

```
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
```

## 3. Creation of socket connection
We will using the standard [rust network library](https://doc.rust-lang.org/std/net/) for socket connection with the host server.

```
fn main() {
    ....
    ....

    let tcp_socket = TcpStream::connect(socket_addr);

    match tcp_socket {
        Ok(mut stream) => {
            ....
            ....
        }
        Err(e) => {
            eprintln!("Failed to establish connection: {}", e);
        }
    }
    ....
    ....
}

```
Once we are successfully connected, we can listen and send your own request to server.

## 4. Sending the request
1. First we have check if verbose mode is enabled, then we print out the request.
2. We have used the [write_all](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all) to ensure our that our whole buffer is added to the stream.
3. Create a new empty buffer, and provide this buffer to the stream, to read the response data from the host.
4. Converts that bytes string to UTF-8 string using the [from_utf8_lossy](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy).
5. Print the response header and body.
```
fn main() {
    ....
    ....

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
    ....
    ....
}

```

## 5. Time for Testing
cli - `cargo run -- http://eu.httpbin.org:80/get`
response - 
```
{
  "args": {}, 
  "headers": {
    "Accept": "*/*", 
    "Host": "eu.httpbin.org", 
    "X-Amzn-Trace-Id": "Root=1-65fec214-25771a3e732101c433ce67a7"
  }, 
  "origin": "49.36.177.79", 
  "url": "http://eu.httpbin.org/get"
}
```

Similarly, you can test others.

**Hurray!! We have able to make our own curl.**