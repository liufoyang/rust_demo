use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::channel;
use std::net::TcpStream;
use std::net::TcpListener;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::boxed::Box;
use super::communication::{BftCommunication,BftCommunicationMsg};
use std::result::Result;

pub struct Default_TCP_Communication {
    listener:TcpListener,
    sender: Sender<Box<(BftCommunicationMsg, TcpStream)>>
}

impl Default_TCP_Communication {
    pub fn startListen(addr:&str, port:&str) ->Receiver<Box<(BftCommunicationMsg, TcpStream)>> {
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

                let lensize = stream.read(&mut buffer).unwrap();
                let (left, right)  = buffer.split_at(lensize);
                let message_str = String::from_utf8_lossy(&left[..]);

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

                let box_msg = Box::new((communication_msg,stream));
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

        let stream_result = TcpStream::connect(address_all.as_str());
        if !stream_result.is_ok() {
            // not connection
            println!("connection {} fail {:?}", address_all, stream_result.err());
            return;
        }

        let mut stream = stream_result.unwrap();
        let data_str = data.to_string();
        //let msg_data = .as_bytes();
        stream.write(data_str.as_bytes());

        println!("send finish {} {}", address_all, data_str);
    }

    pub fn sendMessageWithReply(address:&str, port:&str, data:BftCommunicationMsg) ->Result<String, &'static str>{
        let mut address_all = String::from(address);
        address_all.push_str(":");
        address_all.push_str(port);

        let stream_result = TcpStream::connect(address_all.as_str());
        if !stream_result.is_ok() {
            // not connection
            println!("connection {} fail {:?}", address_all, stream_result.err());
            return Err("connection fail");
        }

        let mut stream = stream_result.unwrap();
        let data_str = data.to_string();
        //let msg_data = .as_bytes();
        stream.write(data_str.as_bytes());

        println!("send finish {} {}", address_all, data_str);

        let mut buffer = [0; 2048];

        let read_result = stream.read(&mut buffer);
        if !read_result.is_ok() {
            println!("send finish {:?}", read_result.err());
            return Err("read stram error");
        } else {
            let (left, right)  = buffer.split_at(read_result.unwrap());
            let reply_str = String::from_utf8_lossy(&left[..]);
            return Ok(reply_str.to_string());
        }

    }
}