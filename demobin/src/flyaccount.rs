use crate::flycoin::FlyCoin;
//extern crate flycoin as coin;
mod fly_account {
    enum Result {
        SUCCESS { code: String, msg: String},
        SUCCESS_WITHDRAW { code: String, msg: String, coin:FlyCoin },
        FAIL { code: String, msg: String}
    }

    struct FlyAccount {
        balance: u64 ,
        available_balance: u64,
        frozen_balance: u64,
        currency: String,
        ownAccount_id: String,
    }

    impl FlyAccount {
        fn create(owdAddress:&String) -> Result{
             // 对输入的公钥做md5后，作为账号地址

             let accountItem:FlyAccount = FlyAccount{
                 balance: 0,
                 available_balance: 0,
                 frozen_balance: 0,
                 currency: "CNY",
                 ownAccount_id: owdAddress.clone(),
             };
             let result = Result{code: String::from("FAIL"), msg: String::from("can move the coin to same account"),account:accountItem };
             return result;
        }


        fn deposit(&mut self, coin:flycoin::FlyAccount) ->Result {
            self.balance =self.balance + flycoin.amount;

            let result = Result::SUCCESS {code:String::from("SUCCESS"),msg:String::from("Move coin to new account")};
            return result;
        }

        fn withdrawal(&mut self, withdraw_amout:u64) ->Result  {
            if self.available_balance < withdraw_amout {
                let result = Result::FAIL {code:String::from("FAIL"),msg:String::from("acount available balance not enough")};
                return result;
            }

            self.balance =self.available_balance - withdraw_amout;
            self.balance =self.available_balance + self.frozen_balance;

            let coinA:FlyCoin = FlyCoin{
                amount:withdraw_amout,
                currency:self.currency.clone(),
                ownAccountId:self.ownAccount_id.clone()
            };
            let result = Result::SUCCESS_WITHDRAW {code:String::from("SUCCESS"),msg:String::from("Move coin to new account"),coin:coinA};
            return result;
        }
    }
}