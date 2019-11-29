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
    logfiles:Vec<File>,
    busifiles:Vec<File>,
    datafileNames:Vec<String>,
    size:usize,
    valueMap:HashMap<String, String>,
}

impl Command_Executor {
    pub fn new(_size: usize) ->Command_Executor {
        //let mut pools_vec = Vec::with_capacity(_size);
        let mut logfile_vec = Vec::with_capacity(_size);
        let mut busi_vec = Vec::with_capacity(_size);
        let mut data_vec = Vec::with_capacity(2);

        let logFileName = "bft_log_record.log";
        let recordFileName = "bft_business_record.log";
        let checkpointName0 = "0bft_checkpoint_record.log";
        let checkpointName1 = "1bft_checkpoint_record.log";

        data_vec.push(checkpointName0.to_string());
        data_vec.push(checkpointName1.to_string());

        let pool = ThreadPool::new(_size);
        for i in 0.._size {
            let index = i.to_string();

            let mut logfile_name = String::from(index.as_str());
            logfile_name.push_str(logFileName);

            let mut logfile_result = OpenOptions::new().append(true).open(logfile_name.clone());
            match logfile_result {
                Ok(logfile) => {
                    logfile_vec.push(logfile);
                },
                Err(err) => {
                    let mut logfile = File::create(logfile_name).unwrap();
                    logfile_vec.push(logfile);

                }
            }


            let mut recordfile_name = String::from(index.as_str());
            recordfile_name.push_str(recordFileName);

            let mut logfile_result = OpenOptions::new().append(true).open(recordfile_name.clone());
            match logfile_result {
                Ok(busifile) => {
                    busi_vec.push(busifile);
                },
                Err(err) => {
                    let mut busifile = File::create(recordfile_name).unwrap();
                    busi_vec.push(busifile);

                }
            }

        }

        let executor =  Command_Executor {
            threadpools:pool,
            logfiles:logfile_vec,
            busifiles:busi_vec,
            datafileNames:data_vec,
            size:_size,
            valueMap:HashMap::new(),
        };

        return executor;
    }

    // payload format command key value
    pub fn savelog(&mut self,payload: &str) {
        let v: Vec<&str> = payload.split(' ').collect();


        // 计算key的hash取模匹配文件index值。
        let mut hasher = DefaultHasher::new();
        let key = v[1];
        hasher.write(key.as_ref());
        let hashValue = hasher.finish() as usize;
        let siez_str = self.size;
        let index = hashValue%self.size;
        let mut logfile = self.logfiles.get_mut(index).unwrap();

        let command = v[0].clone();

        let value:&str = v[2].clone();

        let mut s = String::from(payload);
        s.push_str("\n");
        let mut  buf = s.as_ref();

        logfile.write(buf);
        logfile.flush();
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

        let mut busifile = self.busifiles.get_mut(0).unwrap();
        let mut out = String::from(command);
        out.push_str("\n");
        let mut  buf = out.as_ref();
        busifile.write(buf);
        busifile.flush();
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
        let fileName = self.datafileNames.get(check_point_file_index).unwrap();

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

        let mut busifile = self.busifiles.get_mut(0).unwrap();
        let mut buf = add_check_point_line.as_str().as_bytes();
        busifile.write(buf);
        busifile.flush();

        return Some("add check point".to_string());
    }
}






