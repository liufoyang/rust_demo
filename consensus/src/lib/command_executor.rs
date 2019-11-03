use std::fs::File;
use std::vec::Vec;
use threadpool::ThreadPool;
///只是做一个直接写log，写businessfile的保存命令结果。
///
struct Command_Executor {
    threadpools:Vec<ThreadPool>,
    logfiles:Vec<File>,
    busifiles:Vec<File>,
    size:usize
}

impl Command_Executor {
    fn new(_size: usize) {
        let mut pools_vec = Vec::with_capacity(_size);
        let mut logfile_vec = Vec::with_capacity(_size);
        let mut busi_vec = Vec::with_capacity(_size);

        let logFileName = "bft_log_record.log";
        let recordFileName = "bft_record.log";
        for i in 0.._size {
            let pool = ThreadPool::new(1);
            pools_vec.push(pool);

            let index = i.to_string();
            let logfile_name = &index + &logFileName;
            let logfile = File::create(logfile_name)?;
            logfile_vec.push(logfile);

            let recordfile_name = &index + &recordFileName;
            let recordfile = File::create(recordfile_name)?;
            busi_vec.push(recordfile);
        }

        command_executor {
            threadpools:pools_vec,
            logfiles:logfile_vec,
            busifiles:busi_vec,
            size:_size
        }
    }

    // payload format command key value
    fn execute(self, payload:&str) {
        let v: Vec<&str> = payload.split(' ').collect();

        let command = v[0];
        let key = v[1];
        let value = v[2];

        // 计算key的hash取模匹配文件index值。
        let mut hasher = DefaultHasher::new();
        hasher.write(key);
        let hashValue = hasher.finish();
        let index = hashValue%self.size;

        self.threadpools[index].execute(move||{
            savelog(self.logfiles[index], payload);
            commandSetValue(self.busifiles[index], key, value);
        });


    }
}

fn savelog(file:File, payload:&str) {
    file.write_all(payload)?;
}

fn commandSetValue(file:File, key:&str, value:&str) {
    let s = &key + &value;
    file.write_all(s);
}



