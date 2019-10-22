use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::from_utf8;

use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interests, Poll, Token};

use communication::BftCommunication;
use bft_node::*;
const SERVER: Token = Token(0);

///基于MIO库的通信模块demo，用户bft通信
///create by liufoyang
struct MioBftCommunication {
    ip:String,
    port:String,
    node:Btf_Node,
    connections:HashMap,
    unique_token: Token,
    poll: Poll
}

impl BftCommunication for MioBftCommunication {
    fn startListen(&mut self, node:Btf_Node) {
        env_logger::init();

        // Create a poll instance.
        let mut poll = Poll::new()?;
        // Create storage for events.
        let mut events = Events::with_capacity(128);

        // Setup the TCP server socket.
        let addr = String::from("127.0.0.1:").push_str(node.get_base_node().port);
        let server = TcpListener::bind(addr)?;

        // Register the server with poll we can receive events for it.
        self.poll.registry()
            .register(&server, SERVER, Interests::READABLE)?;

        // 监听处理
        loop {
            self.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        // Received an event for the TCP server socket.
                        // Accept an connection.
                        let (connection, address) = server.accept()?;
                        println!("Accepted connection from: {}", address);

                        let token = next(&mut self.unique_token);
                        poll.registry().register(
                            &connection,
                            token,
                            Interests::READABLE.add(Interests::WRITABLE),
                        )?;

                        self.connections.insert(address, connection);
                    }
                    token => {
                        // (maybe) received an event for a TCP connection.
                        let done = if let Some(connection) = connections.get_mut(&token) {
                            self.handle_connection_event(&mut self.poll, connection, event)?
                        } else {
                            // Sporadic events happen.
                            false
                        };
                        if done {
                            connections.remove(&token);
                        }
                    }
                }
            }
        }


    }

    fn sendMessage<T: Serialize >(&self, address:&str, port:&str, data:T) {

        let connection:&TcpStream = &self.connections.get(adress);

        // 转成json格式的String
        let dataJsonStr = data;
        match connection.write(data) {

            Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
            Ok(_) => {

                poll.registry()
                    .reregister(&connection, event.token(), Interests::READABLE)?
            }
            Err(ref err) if would_block(err) => {}

            Err(ref err) if interrupted(err) => {
                return self.handle_connection_event(poll, connection, event);
            }

            Err(err) => return Err(err),
        }
    }
}

impl MioBftCommunication {
    fn new(_node:Btf_Node) -> MioBftCommunication {
        let mioBftcommnunication:MioBftCommunication = MioBftCommunication{
            ip:_node.get_base_node().address,
            port:_node.get_base_node().port,
            node:_node,
            connections:HashMap::new(),
            unique_token:Token(SERVER.0 + 1),
            poll: Poll::new()
        };
        return mioBftcommnunication;
    }

    fn next(current: &mut Token) -> Token {
        let next = current.0;
        current.0 += 1;
        Token(next)
    }

    /// Returns `true` if the connection is done.
    fn handle_connection_event(
        poll: &mut Poll,
        connection: &mut TcpStream,
        event: &Event,
    ) -> io::Result<bool> {
        if event.is_writable() {

            match connection.write(DATA) {

                Ok(n) if n < DATA.len() => return Err(io::ErrorKind::WriteZero.into()),
                Ok(_) => {

                    poll.registry()
                        .reregister(&connection, event.token(), Interests::READABLE)?
                }

                Err(ref err) if would_block(err) => {}
                // Got interrupted (how rude!), we'll try again.
                Err(ref err) if interrupted(err) => {
                    return handle_connection_event(poll, connection, event)
                }
                // Other errors we'll consider fatal.
                Err(err) => return Err(err),
            }
        }

        if event.is_readable() {
            let mut connection_closed = false;
            let mut received_data = Vec::with_capacity(4096);
            // We can (maybe) read from the connection.
            loop {
                let mut buf = [0; 256];
                match connection.read(&mut buf) {
                    Ok(0) => {
                        // Reading 0 bytes means the other side has closed the
                        // connection or is done writing, then so are we.
                        connection_closed = true;
                        break;
                    }
                    Ok(n) => received_data.extend_from_slice(&buf[..n]),
                    // Would block "errors" are the OS's way of saying that the
                    // connection is not actually ready to perform this I/O operation.
                    Err(ref err) if would_block(err) => break,
                    Err(ref err) if interrupted(err) => continue,
                    // Other errors we'll consider fatal.
                    Err(err) => return Err(err),
                }
            }

            if let Ok(str_buf) = from_utf8(&received_data) {
                println!("Received data: {}", str_buf.trim_end());
            } else {
                println!("Received (none UTF-8) data: {:?}", &received_data);
            }

            if connection_closed {
                println!("Connection closed");
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn would_block(err: &io::Error) -> bool {
        err.kind() == io::ErrorKind::WouldBlock
    }

    fn interrupted(err: &io::Error) -> bool {
        err.kind() == io::ErrorKind::Interrupted
    }
}