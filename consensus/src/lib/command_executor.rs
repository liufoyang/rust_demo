use std::fs::File;
use std::str::FromStr;
use std::string::String;
use std::vec::Vec;
use std::io::Write;
use std::convert::Infallible;
use std::result::Result;
use super::threadpool::ThreadPool;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
//use std::convert::From::from;

///只是做一个直接写log，写businessfile的保存命令结果。
///
struct Command_Executor {
    threadpools:Vec<ThreadPool>,
    logfiles:Vec<String>,
    busifiles:Vec<String>,
    size:usize
}

impl Command_Executor {
    fn new(_size: usize) ->Command_Executor {
        let mut pools_vec = Vec::with_capacity(_size);
        let mut logfile_vec = Vec::with_capacity(_size);
        let mut busi_vec = Vec::with_capacity(_size);

        let logFileName = "bft_log_record.log";
        let recordFileName = "bft_record.log";
        for i in 0.._size {
            let pool = ThreadPool::new(1);
            pools_vec.push(pool);

            let index = i.to_string();

            let mut logfile_name = String::from(index.as_str());
            logfile_name.push_str(logFileName);
            logfile_vec.push(logfile_name);

            let mut recordfile_name = String::from(index.as_str());
            recordfile_name.push_str(recordFileName);
            busi_vec.push(recordfile_name);
        }

        let executor =  Command_Executor {
            threadpools:pools_vec,
            logfiles:logfile_vec,
            busifiles:busi_vec,
            size:_size
        };

        return executor;
    }

    // payload format command key value
    fn execute(& mut self,payload:&'static str) {
        let v: Vec<&str> = payload.split(' ').collect();


        // 计算key的hash取模匹配文件index值。
        let mut hasher = DefaultHasher::new();
        let key = v[1];
        hasher.write(key.as_ref());
        let hashValue = hasher.finish() as usize;
        let siez_str = self.size;
        let index = hashValue%self.size;

        {
            let command = v[0].clone();

            let key2:&'static str = key.clone();
            let value:&'static str = v[2].clone();
            let logfile_name: &str = self.logfiles.get(index).unwrap().as_str();
            let comfile_name:&str = self.busifiles.get(index).unwrap().as_str();
            let threadpool = self.threadpools.get(index).unwrap();

            //doExecute(threadpool,logfile_name,command, comfile_name, key, value);

            let temp_exec = move |_threadpool:&ThreadPool, _logfile_name, _payload, _comfile_name, _key, _value| {
//                _threadpool.execute(||{
//
//                });

                let mut logfile = File::create(_logfile_name).unwrap();
                let s = String::from(_payload);
                let mut  buf = s.as_ref();

                logfile.write_all(buf);

                let mut recordfile = File::create(_comfile_name).unwrap();
                let mut comm = String::from(_key);
                comm.push_str(":");
                comm.push_str(_value);
                let mut  buf = comm.as_ref();
                recordfile.write_all(buf);

            };
            temp_exec(threadpool,logfile_name,command, comfile_name, key2, value);

        }


    }



}

fn savelog(logfile_name:&'static str, payload:&'static str) {
    let mut logfile = File::create(logfile_name).unwrap();
    let s = String::from(payload);
    let mut  buf = s.as_ref();

    logfile.write_all(buf);
}

fn commandSetValue(recordfile_name:&'static str, key:&'static str, value:&'static str) {
    let mut recordfile = File::create(recordfile_name).unwrap();
    let mut comm = String::from(key);
    comm.push_str(":");
    comm.push_str(value);
    let mut  buf = comm.as_ref();
    recordfile.write_all(buf);
}





