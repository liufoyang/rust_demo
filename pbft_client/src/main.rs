use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::SystemTime;
use std::result::Result;
use std::io;
use std::env;
use std::collections::HashMap;
use std::time::Duration;
use std::io::{Read,Write};

/**
    pbft_client primary_ip, primary_port listener_ip listener_port
*/
fn main() {

    let is_print = false;
    let mut arguments = Vec::new();
    for argument in env::args() {
        arguments.push(argument);
    }

    if arguments.len() < 5 {
        panic!("args need {}", "primary_ip, primary_port listener_ip listener_port");
    }

    let primary_ip = arguments[1].as_str();
    let primary_port = arguments[2].as_str();
    let listener_ip = arguments[3].as_str();
    let listener_port = arguments[4].as_str();

    let mut address_all = String::from(listener_ip);
    address_all.push_str(":");
    address_all.push_str(listener_port);

    let listener = TcpListener::bind(address_all.as_str()).unwrap();

    let (sender, receiver) = channel();

    let (stream_sender, stream_receiver) = channel();
    let mut buffer = [0; 2048];
    let replay_listener =  thread::Builder::new().name("reply_listener".to_string()).spawn(move|| {
        let mut thread_index = 0;
        for stream_accept in listener.incoming() {

            let mut stream = stream_accept.unwrap();
            let connection_name = stream.peer_addr().unwrap().to_string();
            //println!("new connection {}", connection_name);

            //
            let stream_box = Box::new(stream);
            stream_sender.send(stream_box);
        }
    });

    thread::Builder::new().name("reader_thread".to_string()).spawn(move|| {

        let mut stream_vec:Vec<TcpStream> = Vec::new();
        let millis_200 = Duration::from_millis(200);
        while true {

            let stream_result =  stream_receiver.recv_timeout(millis_200);

            if stream_result.is_ok() {
                let stream = * stream_result.unwrap();
                stream_vec.push(stream);
            } else if stream_vec.len()>0 {


                for i in 0..stream_vec.len() {
                    let stream = stream_vec.get_mut(i).unwrap();
                    buffer = [0; 2048];

                    let read_result = stream.read(&mut buffer);
                    if read_result.is_ok() {
                        let lensize = read_result.unwrap();
                        let (left, right) = buffer.split_at(lensize);
                        let mut  reply_str = String::from_utf8_lossy(&left[..]).to_string();
                        let msgList:Vec<&str> = reply_str.split("#end#").collect();

                        for msg in msgList {
                            if msg == "got" ||msg == "" {
                                continue;
                            }
                            if is_print {
                                println!("the reply {}", msg);
                            }
                            let reply_box = Box::new(msg.to_string());
                            sender.send(reply_box);
                        }

                    }
                }
            }
        }

    });

    let mut connection_name = String::from(primary_ip);
    connection_name.push_str(":");
    connection_name.push_str(primary_port);

    if is_print {
        println!("connect to {}", connection_name);
    }
    let stream_result = TcpStream::connect(connection_name.as_str());
    if !stream_result.is_ok() {
        panic!("error: {} {:?}", "can not connection to primary ",stream_result.err());
    }

    let mut stream = stream_result.unwrap();

    let client_id = "one_client";
    let mut msg_num = 1;
    while true {
        let mut input_str = String::new();
        match io::stdin().read_line(&mut input_str) {
            Ok(n) => {
                input_str = input_str.replace("\n", "");

                if input_str.as_str() == "quit"||input_str.as_str() == "q"||input_str.as_str() == "exit" {
                    break;
                }
                ///receiveMsg 1.0  false default_id 30
                ///{"id":"10001","client_id":"bft_client0001","payload":"put second_key=hello_1","timestamp":100000,"status":0,"md5sign":""}
                let mut msgStr = String::from("receiveMsg 1.0  false default_id 30 \n");
                msgStr.push_str("{\"id\":\"");
                msgStr.push_str(msg_num.to_string().as_str());
                msgStr.push_str("\",\"client_id\":\"");
                msgStr.push_str(client_id.clone());
                msgStr.push_str("\",\"payload\":\"");
                msgStr.push_str(input_str.as_str());
                msgStr.push_str("\",\"timestamp\":\"");
                let times = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let timestamp = times.as_millis().to_string();
                msgStr.push_str(timestamp.as_str());
                msgStr.push_str("\",\"status\":\"");
                msgStr.push_str("0");
                msgStr.push_str("\",\"client_ip\":\"");
                msgStr.push_str("10.3.209.97");
                msgStr.push_str("\",\"port\":\"");
                msgStr.push_str("10005");
                msgStr.push_str("\",\"md5sign\":\"");
                msgStr.push_str("sign\"}\n");

                // send client msg to primary node
                stream.write(msgStr.as_bytes());

                stream.flush();

                //println!("send request {}", msgStr);
                // wait for replay
                let mut rece_num = 0;

                while true {
                    let reply_result = receiver.recv();
                    // out put the reply
                    let reply_box = reply_result.unwrap();
                    let result = parse_replay(reply_box.as_str());

                    if timestamp.eq_ignore_ascii_case(result.0.as_str()) {
                        rece_num += 1;

                        if rece_num >= 1 {
                            println!("result {}", result.1);
                            break;
                        }
                    } else {

                    }
                }

            },
            Err(error) => panic!("error: {}", error),
        }
    }

    println!("bye, conse!");
}

fn parse_replay(replay:&str) -> (String,String) {

    let lines: Vec<&str> = replay.split("\n").collect();
    if lines.len()>1 {
        let mut msg_str = lines[1].to_string();
        msg_str = msg_str.replace('{',"").replace('}',"").replace('"',"");

        let items: Vec<&str> = msg_str.split(',').collect();
        let mut item_map:HashMap<String,String> = HashMap::new();
        for i in 0..items.len() {
            let item_str = items[i].clone();

            let keyValue: Vec<&str> = item_str.split(':').collect();
            item_map.insert(keyValue[0].to_string(), keyValue[1].to_string());

        }

        let timestamp = item_map.get("timestamp").unwrap().to_string();
        let payload = item_map.get("payload").unwrap().to_string();
        return (timestamp, payload);
    } else {
        return ("".to_string(),"".to_string());
    }
}
