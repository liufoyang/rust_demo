use crate::fly_coin::FlyCoin;
use rand::{rngs::OsRng, Rng};
pub const ADDRESS_LENGTH: usize = 32;

struct FlyAccount{
    own_id: String,
    address:[u8; ADDRESS_LENGTH],
    sequence_num: u64,
    balance: u64,
    reserve_balance: u64,
    frozen_balance: u64,
}

impl FlyAccount {
    fn new(ads:[u8; ADDRESS_LENGTH], ownId:String) -> FlyAccount {
        let account = FlyAccount{
            address:ads,
            sequence_num: 0,
            balance:0,
            reserve_balance:0,
            frozen_balance:0,
            own_id:ownId,
        };

        return account;
    }

    pub fn get_balance(&self) -> u64{
        return self.balance;
    }

    pub fn get_sequence_num(&self) -> u64{
        return self.sequence_num;
    }
    /// save some coin to account;
    fn deposit(mut self, coin:FlyCoin) -> Result<FlyAccount, &'static str> {

        if self.own_id.as_str() != coin.get_ownId() {
           return  Result::Err("not your coin, can not deposit");
        }

        self.balance += coin.get_amount();
        self.sequence_num+= 1;
        return Result::Ok(self);
    }

    /// withdraw some coin
    fn withdraw(mut self, amount:u64) -> Result<FlyAccount, &'static str> {
        if self.balance < amount {
            return  Result::Err("balance not enough, can not withdraw");
        }
        self.sequence_num+= 1;
        self.balance -= amount;

        return Result::Ok(self);
    }
    /// transfer some coin to another account;
    fn transfer(mut self, mut accountB:FlyAccount, amount:u64) ->Result<(FlyAccount, FlyAccount), &'static str> {
        if self.balance < amount {
            return  Result::Err("balance not enough, can not transfer");
        }
        self.sequence_num+= 1;
        self.balance -= amount;
        accountB.sequence_num+= 1;
        accountB.balance += amount;

        return  Result::Ok((self, accountB));
    }

    fn reserve(self, accountB:u64) ->Result<FlyAccount, FlyAccount> {
        return Result::Ok(self);
    }

    fn account_address(&self) -> &str {
        return "not finish";
    }
}

#[cfg(test)]
pub mod test_lfy_account {
    use super::FlyCoin;
    use rand::{rngs::OsRng, Rng};
    use super::FlyAccount;

    #[test]
    pub fn test_account_base() {

        /// create two account;
        ///
        let mut rng = OsRng::new().expect("can't access OsRng");
        let buf: [u8; 32] = rng.gen();
        let mut accountBob = FlyAccount::new(buf, String::from("Bob"));

        let buf2: [u8; 32] = rng.gen();
        let mut accountAlan = FlyAccount::new(buf2, String::from("Alan"));

        let mut coinBob = FlyCoin::create(101, String::from("CNY"), String::from("Bob"));
        let mut coinAlan = FlyCoin::create(202, String::from("CNY"), String::from("Alan"));


        /// deposit to two account;
        let bobResult =  accountBob.deposit(coinBob);
//        accountBob = match bobResult {
//            Result::Ok(account) => account,
//            Result::Err(error_msg) => accountBob,
//        };

        accountBob = bobResult.unwrap();

        let alanResult = accountAlan.deposit(coinAlan);
        accountAlan = alanResult.unwrap();

        assert_eq!(accountAlan.get_balance(), 202);
        assert_eq!(accountBob.get_balance(), 101);
        assert_eq!(accountAlan.get_sequence_num(), 1);
        assert_eq!(accountBob.get_sequence_num(), 1);

        /// transfer from Alan to Bob;
        let trasnferResult =  accountAlan.transfer(accountBob, 50);
        let(accountAlan, accountBob) = trasnferResult.unwrap();

        assert_eq!(accountAlan.get_balance(), 152);
        assert_eq!(accountBob.get_balance(), 151);
        assert_eq!(accountAlan.get_sequence_num(), 2);
        assert_eq!(accountBob.get_sequence_num(), 2);
    }
}
