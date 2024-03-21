## Build Your Own curl
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

### 1. Getting the cli arguments.
We will be using library for [Clap](https://crates.io/crates/clap) - A simple-to-use, efficient, and full-featured library for parsing command line arguments and subcommands.
   

### 2. Creation of socket connection

### 3. Sending the request

### 4. Parsing the response


Hurray!! We have able to make our own curl.