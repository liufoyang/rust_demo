/// define the result enum for fly block chain system
pub struct Error{
    code:String,
    msg:String
}

impl Error {
    pub fn new(_code:&str, _msg:&str) -> Error{
        let error = Error{
            code: String::from(_code),
            msg: String::from(_msg),
        };
        return error;
    }
}

pub enum Result<T> {
    SUCCESS{item:T},
    FAIL{error:Error, item:T},
}

impl <T>Result<T> {
    pub fn unwrap(self) -> T {
        let item_resutn = match self {
            Result::SUCCESS{item} => item,
            Result::FAIL{error, item} => item,
        };
        return item_resutn;
    }
}