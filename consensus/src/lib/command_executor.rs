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
//use std::convert::From::from;

///只是做一个直接写log，写businessfile的保存命令结果。
//
pub struct Command_Executor {
    threadpools:ThreadPool,
    logfiles:Vec<File>,
    busifiles:Vec<File>,
    size:usize
}

impl Command_Executor {
    pub fn new(_size: usize) ->Command_Executor {
        //let mut pools_vec = Vec::with_capacity(_size);
        let mut logfile_vec = Vec::with_capacity(_size);
        let mut busi_vec = Vec::with_capacity(_size);

        let logFileName = "bft_log_record.log";
        let recordFileName = "bft_business_record.log";
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
            size:_size
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
        let mut busifile = self.busifiles.get_mut(index).unwrap();

        let command = v[0].clone();

        let value:&str = v[2].clone();

        let mut s = String::from(payload);
        s.push_str("\n");
        let mut  buf = s.as_ref();

        logfile.write(buf);
        logfile.flush();
        println!("write the log file {}", s);

//        let mut comm = String::from(key);
//        comm.push_str(":");
//        comm.push_str(value);
//        comm.push_str("\n");
//        let mut  buf = comm.as_ref();
//        busifile.write(buf);
//        busifile.flush();
//        println!("write the business file {}", comm);

    }
}






