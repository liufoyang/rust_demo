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

///只是做一个直接写log，写businessfile的保存命令结果。
///
struct Command_Executor {
    threadpools:Vec<ThreadPool>,
    logfiles:Vec<File>,
    busifiles:Vec<File>,
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
            let mut logfile = File::create(logfile_name.as_str()).unwrap();
            logfile_vec.push(logfile);

            let mut recordfile_name = String::from(index.as_str());
            recordfile_name.push_str(recordFileName);
            let recordfile = File::create(recordfile_name.as_str()).unwrap();
            busi_vec.push(recordfile);
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
    fn execute(& mut self,payload:&str) {
        let v: Vec<&str> = payload.split(' ').collect();

        let command = v[0];
        let key = v[1];
        let value = v[2];

        // 计算key的hash取模匹配文件index值。
        let mut hasher = DefaultHasher::new();
        hasher.write(key.as_ref());
        let hashValue = hasher.finish();
        let index = hashValue%self.size;

        self.threadpools[index].execute(move||{
            let mut logfile =  self.busifiles.get(index).unwrap();
            savelog(logfile, payload);
            let mut comfile = self.busifiles.get(index).unwrap();
            commandSetValue(comfile, key, value);
        });


    }
}

fn savelog( mut file:&File, payload:&str) {
    let s = String::from(payload);
    let mut  buf = s.as_ref();

    file.write_all(buf);
}

fn commandSetValue(mut file:&File, key:&str, value:&str) {
    let mut comm = String::from(key);
    comm.push_str(":");
    comm.push_str(value);
    let mut  buf = comm.as_ref();
    file.write_all(buf);
}



