pub mod fly_coin {
    enum Result<T> {
        SUCCESS{code:String, msg:String, item:T},
        FAIL{code:String, msg:String, credit:T, debit:T}
    }

    struct FlyCoin{
        amount:u64,
        currency: String,
        ownAccountId:String,
    }

    impl FlyCoin{
        fn create(amountNum:u64, currencyCode:String, ownAdId:String) -> FlyCoin{
            let coin = FlyCoin {
                amount:amountNum,
                currency:currencyCode,
                ownAccountId:ownAdId
            };
            return coin;
        }

        /**
        转移币，新建一个币，老的币因为ownShip放进来了， 函数结束币就自动回收销毁了。
        */
        fn moveTo(self, toAddress:String) -> Result<FlyCoin>{
            if toAddress == self.ownAccountId {
                let coin = FlyCoin {
                    amount: 0,
                    currency: self.currency.clone(),
                    ownAccountId: toAddress
                };
                return Result::FAIL { code: String::from("FAIL"), msg: String::from("can move the coin to same account"), credit: self, debit: coin };
            } else {
                // 检查没问题，这里直接转移self的amount， currenty的ownership， self再也不能使用了
                let coin = FlyCoin {
                    amount:self.amount,
                    currency:self.currency,
                    ownAccountId:toAddress
                };

                return Result::SUCCESS{code:String::from("SUCCESS"),msg:String::from("Move coin to new account"), item:coin};
            }


        }

        fn add(self, anotherCoin:FlyCoin) -> Result<FlyCoin> {
            if self.ownAccountId != anotherCoin.ownAccountId {
                return Result::FAIL {code:String::from("FAIL"), msg:String::from("can add the coin to not same account"), credit:self, debit:anotherCoin};

            }
            if self.currency != anotherCoin.currency {
                return  Result::FAIL {code:String::from("FAIL"), msg:String::from("can add the coin to not same currency"),credit:self, debit:anotherCoin};

            }

            let coin = FlyCoin {
                amount:self.amount + anotherCoin.amount,
                currency:self.currency,
                ownAccountId:self.ownAccountId
            };

            // 相加成功，返回新的币
            return Result::SUCCESS{code:String::from("SUCCESS"),msg:String::from("Move coin to new account"), item:coin };

        }

        fn sub(self, anotherCoin:FlyCoin) -> Result<FlyCoin> {
            if self.ownAccountId != anotherCoin.ownAccountId {
                return  Result::FAIL {code:String::from("FAIL"), msg:String::from("can sub the coin to not same account"),credit:self, debit:anotherCoin};
            }
            if self.currency != anotherCoin.currency {
                return  Result::FAIL {code:String::from("FAIL"), msg:String::from("can sub the coin to not same currency"),credit:self, debit:anotherCoin};
            }

            if self.amount < anotherCoin.amount {
                return  Result::FAIL {code:String::from("FAIL"), msg:String::from("can sub the coin less than"),credit:self, debit:anotherCoin};
            }

            let coin = FlyCoin {
                amount:self.amount - anotherCoin.amount,
                currency:self.currency,
                ownAccountId:self.ownAccountId
            };

            // 减少成功，返回新的币
            return  Result::SUCCESS{code:String::from("SUCCESS"),msg:String::from("Move coin to new account"), item:coin};

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

        let mut coinB:FlyCoin  = match moveResult {
            Result::SUCCESS{code, msg, item} => item,
            Result::FAIL{code, msg, credit, debit} => credit,
        };
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
        let coinD  = match addResult {
            Result::SUCCESS{code, msg, item} => item,
            Result::FAIL{code, msg, credit, debit} => credit,
        };
        println!("add coin3 to coin2 become to coinD {}", coinD.amount);
    }
}