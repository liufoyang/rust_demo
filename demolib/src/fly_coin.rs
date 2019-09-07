use crate::fly_result::Result;
use crate::fly_result::Error;
pub struct FlyCoin{
    amount:u64,
    currency: String,
    ownId:String,
}

impl FlyCoin{

    pub fn get_amount(&self) -> u64{
        return self.amount;
    }

    pub fn get_currency(&self) -> &str {
        return self.currency.as_str();
    }

    pub fn get_ownId(&self) -> &str {
        return self.ownId.as_str();
    }

    pub fn create(amountNum:u64, currencyCode:String, ownAdId:String) -> FlyCoin{
        let coin = FlyCoin {
            amount:amountNum,
            currency:currencyCode,
            ownId:ownAdId
        };
        return coin;
    }

    /**
    转移币，新建一个币，老的币因为ownShip放进来了， 函数结束币就自动回收销毁了。
    */
    pub fn moveTo(self, toAddress:String) -> Result<FlyCoin>{
        if toAddress == self.ownId {
            let coin = FlyCoin {
                amount: 0,
                currency: self.currency.clone(),
                ownId: toAddress
            };
            return Result::FAIL{ error:Error::new("FAIL","can not give coin to self"), item:self};
        } else {
            // 检查没问题，这里直接转移self的amount， currenty的ownership， self再也不能使用了
            let coin = FlyCoin {
                amount:self.amount,
                currency:self.currency,
                ownId:toAddress
            };

            return Result::SUCCESS{item:coin};
        }


    }

    pub fn add(mut self, mut anotherCoin:FlyCoin) -> Result<(FlyCoin,FlyCoin)> {
        if self.ownId != anotherCoin.ownId {
            return Result::FAIL {error:Error::new("FAIL","can add the coin to not same account"), item:(self, anotherCoin)};

        }
        if self.currency != anotherCoin.currency {
            return Result::FAIL {error:Error::new("FAIL","can add the coin to not same currency"), item:(self, anotherCoin)};
        }

        self.amount += anotherCoin.amount;
        anotherCoin.amount = 0;
//        let coin = FlyCoin {
//            amount:self.amount + anotherCoin.amount,
//            currency:self.currency,
//            ownId:self.ownId
//        };

        // 相加成功，返回新的币
        return Result::SUCCESS{item:(self, anotherCoin)};

    }

    pub fn sub(mut self, mut subAmount:u64) -> Result<(FlyCoin,FlyCoin)>  {

        if self.amount < subAmount {
            let anotherCoin = FlyCoin {
                amount:0,
                currency: self.currency.clone(),
                ownId: self.ownId.clone(),
            };
            return Result::FAIL {error:Error::new("FAIL","can sub the coin less than"), item:(self, anotherCoin)};
        }

        self.amount -= subAmount;
        let anotherCoin = FlyCoin {
            amount:subAmount,
            currency: self.currency.clone(),
            ownId: self.ownId.clone(),
        };

        // 减少成功，返回新的币
        return Result::SUCCESS{item:(self, anotherCoin)};

    }
}

#[test]
pub fn test_lfy() {
    let mut coinA = FlyCoin::create(100, String::from("CNY"), String::from("address_A"));

    let moveResult = coinA.moveTo(String::from("address_B"));

//        let coin2  = match moveResult {
//            Result::SUCCESS{code, msg, item} => item,
//            Result::FAIL{code, msg, credit, debit} => credit,
//        };

    let mut coinB:FlyCoin  = moveResult.unwrap();
//        if let Result::SUCCESS{code, msg, item} = moveResult {
//            //
//            coinB = item;
//        } else if let Result::FAIL{code, msg, credit, debit} = moveResult {
//            // 如果是失败， 退回给A， B得到的是空.
//            coinB = credit;
//        }
    println!("move coin1 to address_B ");

    // 转移成功， 这里再使用coin1就回报错
    // coin1.moveTo("address_B")
    let coinC = FlyCoin::create(50, String::from("CNY"), String::from("address_B"));

    let addResult = coinB.add(coinC);
    let (coinB, coinC) = addResult.unwrap();
    println!("add coinB to coinC fail {}, {}", coinB.amount, coinC.amount);

}