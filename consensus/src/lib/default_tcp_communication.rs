use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::channel;
use std::net::TcpStream;
use std::net::TcpListener;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::boxed::Box;
use super::communication::{BftCommunication,BftCommunicationMsg};

pub struct Default_TCP_Communication {
    listener:TcpListener,
    sender: Sender<Box<BftCommunicationMsg>>
}

impl Default_TCP_Communication {
    pub fn startListen(addr:&str, port:&str) ->Receiver<Box<BftCommunicationMsg>> {
        let mut address_all = String::from(addr);
        address_all.push_str(":");
        address_all.push_str(port);
        let listener = TcpListener::bind(address_all.as_str()).unwrap();

        let (sender, receiver) = channel();

        let commincation = Default_TCP_Communication {
            listener:listener,
            sender: sender
        };

        thread::Builder::new().name("bft_node_listener".to_string()).spawn(move|| {
            for stream in commincation.listener.incoming() {

                println!("receive one connection");
                // read the connection
                let mut stream = stream.unwrap();
                let mut buffer = [0; 2048];

                stream.read(&mut buffer).unwrap();

                let message_str = String::from_utf8_lossy(&buffer[..]);

                println!("receive one msg {}",message_str);

                // check the msg format
                ///  command version leng \n
                ///  body
                let mut lines = message_str.lines();

                // match header
                let mut command = String::new();
                let mut version = String::new();
                let mut payload = String::new();

                match lines.next() {
                    Some(header_str) => {
                        // check header
                        let mut iter = header_str.split_whitespace();
                        let mut i = 0;
                        for token in iter {
                            if i==0 {
                                command.push_str(token);
                            }
                            if i==1 {
                                version.push_str(token);
                            }
                            i += 1;
                        }

                        if(command.is_empty() || version.is_empty()) {
                            // header format error
                            println!("header format error");
                            continue;
                        }

                    }
                    None => {
                        // message format error
                        println!("message format error");
                        continue;
                    }
                }

                match lines.next() {
                    Some(payload_str) => {
                        // check body
                        payload.push_str(payload_str);

                    }
                    None => {
                        // message format error
                        println!("message format error");
                        continue;
                    }
                }

                let communication_msg = BftCommunicationMsg{
                    command:command,
                    version:version,
                    payload:payload
                };

                let box_msg = Box::new(communication_msg);
                println!("send msg to processor");
                commincation.sender.send(box_msg);
            }
        });

        return receiver;
    }

    pub fn sendMessage(address:&str, port:&str, data:BftCommunicationMsg){
        let mut address_all = String::from(address);
        address_all.push_str(":");
        address_all.push_str(port);

        let stream_result = TcpStream::connect(address_all);
        if !stream_result.is_ok() {
            // not connection
            return;
        }

        let mut stream = stream_result.unwrap();
        let data_str = data.to_string();
        //let msg_data = .as_bytes();
        stream.write(data_str.as_bytes());
    }
}