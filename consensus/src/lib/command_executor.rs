use std::fs::File;
use std::fs::OpenOptions;
use std::str::FromStr;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write, Seek, SeekFrom};
use std::convert::Infallible;
use std::result::Result;
use super::threadpool::ThreadPool;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::collections::HashMap;
//use std::convert::From::from;

///只是做一个直接写log，写businessfile的保存命令结果。
//
pub struct Command_Executor {
    threadpools:ThreadPool,
    msglogfiles:File,
    busifiles:File,
    datafileName:String,
    size:usize,
    valueMap:HashMap<String, String>,
}

impl Command_Executor {
    pub fn new(msglogfile_name:&str, datafile_name:&str, busifile_name:&str) ->Command_Executor {
        //let mut pools_vec = Vec::with_capacity(_size);
        let mut logfile_result = OpenOptions::new().append(true).open(msglogfile_name.to_string());
        if logfile_result.is_err() {
            logfile_result = File::create(msglogfile_name.to_string());
        }

        let logfile: File = logfile_result.unwrap();

        let mut busifile_result = OpenOptions::new().append(true).open(busifile_name.to_string());
        if busifile_result.is_err() {
            busifile_result  = File::create(busifile_name.to_string());
        }
        let busifile:File = busifile_result.unwrap();

        let executor =  Command_Executor {
            threadpools:ThreadPool::new(1),
            msglogfiles:logfile,
            busifiles:busifile,
            datafileName:datafile_name.to_string(),
            size:1,
            valueMap:HashMap::new(),
        };

        return executor;
    }

    // record the msg log 记录消息日志
    pub fn savelog(&mut self,payload: &str) {
        let v: Vec<&str> = payload.split(' ').collect();


        // TODO 计算key的hash取模匹配文件index值, 分组消息日志

        let mut s = String::from(payload);
        s.push_str("\n");
        let mut  buf = s.as_ref();
        self.msglogfiles.write(buf);
        self.msglogfiles.flush();

        println!("write the log file {}", s);


    }

    /// the command for key value, 3 command put key = value, delete key, get key
    /// key is path split by /,  value is string max length 1024;
    pub fn command_execute(&mut self,command: &str) -> Option<String> {

        let mut result:Option<String> = Option::None;

        let mut commandKey = "";

        if command.starts_with("put") {
            commandKey = "put";
        }else if command.starts_with("delete") {
            commandKey = "delete";
        }else if command.starts_with("get") {
            commandKey = "get";
        } else {
            println!("not valid command {}", command);
            return result;
        }

        let mut keyValueStr = command.replace(commandKey, "");

        if !keyValueStr.contains("=") {
            println!("not valid command {}", command);
            return result;
        }
        let playloads:Vec<&str> = keyValueStr.split('=').collect();;

        let key = playloads[0].trim();
        let value = playloads[1].trim();

        let mut out = String::from(command);
        out.push_str("\n");
        let mut  buf = out.as_ref();
        self.busifiles.write(buf);
        self.busifiles.flush();
        println!("write the business file {}", command);

        if commandKey == "put" {
            self.valueMap.insert(key.to_string(), value.to_string());
        } else if commandKey == "delete" {
            result = self.valueMap.remove(key.clone());
        }

        let keyStr = key.to_string();
        if self.valueMap.contains_key(&keyStr) {
            let value_str = self.valueMap.get(&keyStr).unwrap();
            result = Some(value_str.to_string());
        }

        return result;

    }

    pub fn save_check_point(&mut self,check_point_num: &u64) -> Option<String> {

        let check_point_file_index = ((*check_point_num)%2) as usize;
        let mut fileName = String::from(self.datafileName.as_str());
        fileName.push_str(check_point_file_index.to_string().as_str());
        fileName.push_str(".log");

        let file_result = File::create(fileName.as_str());
        if !file_result.is_ok() {
            return None;
        }

        let mut file = file_result.unwrap();
        for (key, value) in self.valueMap.iter() {
            let mut line = String::from(key.as_str());
            line.push_str("=");
            line.push_str(value.as_str());
            line.push_str("\n");

            file.write_all(line.as_str().as_bytes());
        }

        file.flush();

        // save the command
        let mut add_check_point_line = String::from("checkpoint ");
        add_check_point_line.push_str(check_point_file_index.to_string().as_str());
        add_check_point_line.push_str("\n");

        let mut buf = add_check_point_line.as_str().as_bytes();
        self.busifiles.write(buf);
        self.busifiles.flush();

        return Some("add check point".to_string());
    }
}






