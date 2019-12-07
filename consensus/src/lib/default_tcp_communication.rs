use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::channel;
use std::net::TcpStream;
use std::net::TcpListener;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::{Mutex,Arc};
use std::rc::Rc;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use super::communication::{BftCommunication,BftCommunicationMsg};
use std::result::Result;
use std::option::Option::Some;
use flexi_logger::{Logger, opt_format};
use log::*;
use std::io;

pub struct Default_TCP_Communication {
    connections:Arc<Mutex<HashMap<String, Sender<BftCommunicationMsg>>>>,
    sys_nodify_map:Arc<Mutex<HashMap<String, Sender<Box<BftCommunicationMsg>>>>>,
    msg_sender:Sender<Box<BftCommunicationMsg>>
}

impl Default_TCP_Communication {
    pub fn startListen(addr:&str, port:&str) ->(Default_TCP_Communication, Receiver<Box<(BftCommunicationMsg)>>) {
        let mut address_all = String::from(addr);
        address_all.push_str(":");
        address_all.push_str(port);
        let listener = TcpListener::bind(address_all.as_str()).unwrap();

        let (msg_sender, msg_receiver) = channel();
        let connections:HashMap<String, Sender<BftCommunicationMsg>> = HashMap::new();

        let connections_mutex: Arc<Mutex<HashMap<String, Sender<BftCommunicationMsg>>>> = Arc::new(Mutex::new(connections));

//        let mut commincation = Default_TCP_Communication {
//            listener:listener,
//            sender: sender,
//            connections:HashMap::new()
//        };


        let connection_listener = Arc::clone(&connections_mutex);
        let msg_sender_sub = Sender::clone(&msg_sender);
        thread::Builder::new().name("bft_node_listener".to_string()).spawn(move|| {
            let mut thread_index = 0;
            for stream in listener.incoming() {

                info!("receive one connection");

                //
                // read the connection
                let mut stream = stream.unwrap();

                let mut msg_sender_reader = Sender::clone(&msg_sender_sub);
                let millis_100 = Duration::from_millis(200);
                stream.set_read_timeout(Some(millis_100)).expect("set_read_timeout call failed");
                let mut connections = connection_listener.lock().unwrap();
                Default_TCP_Communication::create_new_reader(msg_sender_reader, &mut connections,  stream);
            }
        });


        let nodify_map:HashMap<String, Sender<Box<BftCommunicationMsg>>> = HashMap::new();
        let nodify_map_mutex: Arc<Mutex<HashMap<String, Sender<Box<BftCommunicationMsg>>>>> = Arc::new(Mutex::new(nodify_map));


        let (main_sender, main_receiver) =  channel();
        let nodify_sender_map = Arc::clone(&nodify_map_mutex);
        let connection_receiver = Arc::clone(&connections_mutex);
        thread::Builder::new().name("bft_node_recevier".to_string()).spawn(move|| {
            while (true) {
                let msg_result = msg_receiver.recv();

                if msg_result.is_ok() {
                    let msg_box:Box<BftCommunicationMsg> = msg_result.unwrap();

                    let msg = *msg_box;
                    info!("receive one msg for receiver {}", msg.command);
                    if msg.command.eq_ignore_ascii_case("reply") {
                        let mut notify_map = nodify_sender_map.lock().unwrap();

                        if notify_map.contains_key(&msg.id) {
                            let notify_sender = notify_map.remove(&msg.id).unwrap();
                            let msg_box = Box::new(msg);
                            info!("give one msg for notify");
                            notify_sender.send(msg_box);
                        } else {

                        }

                    } else if msg.command.eq_ignore_ascii_case("disconnection") {
                        let mut connections = connection_receiver.lock().unwrap();
                        connections.remove(&msg.payload);
                    }else {
                        info!("give one msg for main");
                        let msg_box = Box::new(msg);
                        main_sender.send(msg_box);
                    }
                }
            }

        });

        let comminications = Default_TCP_Communication {
            connections:connections_mutex,
            sys_nodify_map:nodify_map_mutex,
            msg_sender:msg_sender,
        };

        return (comminications, main_receiver);
    }

    fn create_new_reader(msg_sender: Sender<Box<BftCommunicationMsg>>, connections: &mut HashMap<String, Sender<BftCommunicationMsg>>, mut stream: TcpStream) {

        let (readerSender, readerReceiver) = channel();
        connections.insert(stream.peer_addr().unwrap().to_string(), readerSender);
        let connection_name = stream.peer_addr().unwrap().to_string();
        let mut name = String::from("bft_node_readwritor_");
        name.push_str(connection_name.as_str());

        thread::Builder::new().name(name).spawn(move || {
            let millis_100 = Duration::from_millis(200);
            while true {
                let mut buffer = [0; 2048];

                let read_result = stream.read(&mut buffer);

                let mut check_heart = false;
                if read_result.is_ok() {
                    let lensize = read_result.unwrap();

                    if lensize<= 0 {
                        thread::sleep(millis_100);
                        check_heart = true;
                    } else {
                        let (left, right) = buffer.split_at(lensize);
                        // just say hello.
                        if left == b"got" {
                            continue;
                        }
                        let message_str = String::from_utf8_lossy(&left[..]);

                        info!("receive one msg {}", message_str);

                        // check the msg format
                        ///  command version leng \n
                        ///  body
                        let mut lines = message_str.lines();

                        // match header
                        let mut command = String::new();
                        let mut version = String::new();
                        let mut payload = String::new();
                        let mut is_sys = false;
                        let mut id = String::new();

                        match lines.next() {
                            Some(header_str) => {
                                // check header
                                let mut iter = header_str.split_whitespace();
                                let mut i = 0;
                                for token in iter {
                                    if i == 0 {
                                        command.push_str(token);
                                    }
                                    if i == 1 {
                                        version.push_str(token);
                                    }

                                    if i == 2 {
                                        is_sys = token.eq_ignore_ascii_case("true");
                                    }

                                    if i == 3 {
                                        id.push_str(token);
                                    }
                                    i += 1;
                                }

                                if (command.is_empty() || version.is_empty()) {
                                    // header format error
                                    warn!("header format error");
                                    continue;
                                }
                            }
                            None => {
                                // message format error
                                warn!("message format error");
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
                                warn!("message format error");
                                continue;
                            }
                        }

                        let communication_msg = BftCommunicationMsg {
                            id: id,
                            is_sys: is_sys,
                            command: command,
                            version: version,
                            payload: payload,
                            from: stream.peer_addr().unwrap().to_string()
                        };

                        let box_msg = Box::new(communication_msg);
                        info!("send msg to processor");
                        msg_sender.send(box_msg);

                        stream.write(b"got");
                        stream.flush();
                    }
                } else {
                    check_heart = true;
                }

                if check_heart {
                    let hit_result:io::Result<usize> = stream.write(b"got");

                    if hit_result.is_err() {
                        let stream_result = TcpStream::connect(connection_name.as_str());
                        if !stream_result.is_ok() {
                            // not connection
                            error!("connection {} fail {:?}", connection_name, stream_result.err());

                            // send disconnection msg;
                            let communication_msg = BftCommunicationMsg {
                                id: "disconnection_id".to_string(),
                                is_sys: false,
                                command: "disconnection".to_string(),
                                version: "v1.0".to_string(),
                                payload: connection_name.clone(),
                                from: connection_name.clone()
                            };

                            let box_msg = Box::new(communication_msg);
                            info!("send disconnection msg to processor and quit");
                            msg_sender.send(box_msg);
                            break;
                        } else {
                            info!("reconnection {}", connection_name);
                            stream = stream_result.unwrap();
                        }
                    }

                }
                let send_msg_result: Result<BftCommunicationMsg, TryRecvError> = readerReceiver.try_recv();
                if (send_msg_result.is_ok()) {
                    let data: BftCommunicationMsg = send_msg_result.unwrap();
                    let data_str = data.to_string();
                    //let msg_data = .as_bytes();
                    stream.write(data_str.as_bytes());
                    stream.flush();
                    info!("send finish {}", data_str);
                }


            }

            //return Ok("sender finish job".to_string());
        });
    }


    pub fn sendMessage(&mut self, address:&str, port:&str, data:BftCommunicationMsg, isAsync:bool) -> Option<BftCommunicationMsg>{
        let mut address_all = String::from(address);
        address_all.push_str(":");
        address_all.push_str(port);

        if isAsync {
            return self.sendMessageAsync(address_all, data);
        } else {
            return self.sendMessageSync(address_all, data);
        }


    }

    fn sendMessageAsync(&self, connection_name:String, data:BftCommunicationMsg) -> Option<BftCommunicationMsg>{
        // find the sender
        let connection_sender = Arc::clone(&self.connections);

        let mut connections = connection_sender.lock().unwrap();

        if connections.contains_key(&connection_name) {
            let sender = connections.get(&connection_name).unwrap();
            sender.send(data);
        } else {

            let stream_result = TcpStream::connect(connection_name.as_str());
            if !stream_result.is_ok() {
                // not connection
                error!("connection {} fail {:?}", connection_name, stream_result.err());
                return None;
            }
            info!("connection to {} success", connection_name);

            let millis_100 = Duration::from_millis(200);
            let mut stream = stream_result.unwrap();
            stream.set_read_timeout(Some(millis_100)).expect("set_read_timeout call failed");

            let msg_sender_sub = Sender::clone(&self.msg_sender);
            Default_TCP_Communication::create_new_reader(msg_sender_sub, &mut connections, stream);

            let sender = connections.get(&connection_name).unwrap();

            sender.send(data);
        }

        return None;
    }

    fn sendMessageSync(&mut self, connection_name:String, mut data:BftCommunicationMsg) -> Option<BftCommunicationMsg>{

        data.id = String::from("one_time");
        // find the sender
        let mut connection_sender = Arc::clone(&self.connections);
        let reply_recevier = self.applySysNotify(data.id.as_str());

        let mut has_send = false;
        if !has_send {
            // todo not need to lock at send
            let mut connections = connection_sender.lock().unwrap();

            if connections.contains_key(&connection_name) {
                let sender = connections.get(&connection_name).unwrap();

                sender.send(data);
                has_send = true

            } else {
                let stream_result = TcpStream::connect(connection_name.as_str());
                if !stream_result.is_ok() {
                    // not connection
                    error!("connection {} fail {:?}", connection_name, stream_result.err());
                    return None;
                }
                info!("connection to {} success", connection_name);
                let msg_sender_sub = Sender::clone(&self.msg_sender);

                let millis_100 = Duration::from_millis(200);
                let mut stream = stream_result.unwrap();
                stream.set_read_timeout(Some(millis_100)).expect("set_read_timeout call failed");
                Default_TCP_Communication::create_new_reader(msg_sender_sub, &mut connections, stream);

                let sender = connections.get(&connection_name).unwrap();

                sender.send(data);
                has_send = true;
            }
        }

        if has_send {
            let reply_result = reply_recevier.recv();
            if reply_result.is_ok() {
                let replay_msg_box:Box<BftCommunicationMsg> = reply_result.unwrap();
                return Some(*replay_msg_box);
            } else {
                return None;
            }
        } else {
            return None;
        }

    }

    fn applySysNotify(&mut self, msg_id:&str) -> Receiver<Box<BftCommunicationMsg>> {
        let (notify_sender, notify_receiver) = channel();

        let nodify_sender_map = Arc::clone(&self.sys_nodify_map);
        let mut notify_map = nodify_sender_map.lock().unwrap();

        notify_map.insert(msg_id.to_string(), notify_sender);

        return notify_receiver;
    }

//    pub fn sendMessageWithReply(address:&str, port:&str, data:BftCommunicationMsg) ->Result<String, &'static str>{
//        let mut address_all = String::from(address);
//        address_all.push_str(":");
//        address_all.push_str(port);
//
//        let stream_result = TcpStream::connect(address_all.as_str());
//        if !stream_result.is_ok() {
//            // not connection
//            println!("connection {} fail {:?}", address_all, stream_result.err());
//            return Err("connection fail");
//        }
//
//        let mut stream = stream_result.unwrap();
//        let data_str = data.to_string();
//        //let msg_data = .as_bytes();
//        stream.write(data_str.as_bytes());
//
//        println!("send finish {} {}", address_all, data_str);
//
//        let mut buffer = [0; 2048];
//
//        let read_result = stream.read(&mut buffer);
//        if !read_result.is_ok() {
//            println!("send finish {:?}", read_result.err());
//            return Err("read stram error");
//        } else {
//            let (left, right)  = buffer.split_at(read_result.unwrap());
//            let reply_str = String::from_utf8_lossy(&left[..]);
//            return Ok(reply_str.to_string());
//        }
//
//    }
}