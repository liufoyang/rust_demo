mod fly_coin {
    enum Result<T> {
        SUCCESS(String, String, T),
        FAIL(String, String)
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
            coin;
        }

        /**
        转移币，新建一个币，老的币因为ownShip放进来了， 函数结束币就自动回收销毁了。
        */
        fn moveTo(self, toAddress:String) -> Result{
            if(toAddress == self.ownAccountId) {
                Result::FAIL("FAIL", "can move the coin to same account");
            }

            let coin = FlyCoin {
                amount:self.amount,
                currency:self.currency,
                ownAccountId:toAddress
            };

            Result::SUCCESS("SUCCESS","Move coin to new account", coin);
        }

        fn add(self, anotherCoin:FlyCoin) -> Result {
            if self.ownAccountId != anotherCoin.ownAccountId {
                Result::FAIL("FAIL", "can add the coin to not same account");
            }
            if self.currency != anotherCoin.currency {
                Result::FAIL("FAIL", "can add the coin to not same currency");
            }

            let coin = FlyCoin {
                amount:self.amount + anotherCoin.amount,
                currency:self.currency,
                ownAccountId:toAddress
            };

            // 相加成功，返回新的币
            Result::SUCCESS("SUCCESS","Move coin to new account", coin);

        }

        fn sub(self, anotherCoin:FlyCoin) -> Result {
            if self.ownAccountId != anotherCoin.ownAccountId {
                Result::FAIL("FAIL", "can sub the coin to not same account");
            }
            if self.currency != anotherCoin.currency {
                Result::FAIL("FAIL", "can sub the coin to not same currency");
            }

            if self.amount < anotherCoin.amount {
                Result::FAIL("FAIL", "can sub the coin less than");
            }

            let coin = FlyCoin {
                amount:self.amount - anotherCoin.amount,
                currency:self.currency,
                ownAccountId:toAddress
            };

            // 减少成功，返回新的币
            Result::SUCCESS("SUCCESS","Move coin to new account", coin);

        }
    }

    fn test_lfy() {
        let coin1 = FlyCoin::create(100, "CNY", "address_A");
        let coin2 = coin1.moveTo("address_B");
        println!("move coin1 to address_B ");

        // 这里再使用coin1就回报错
        // coin1.moveTo("address_B")
        let coin3 = FlyCoin::create(50, "CNY", "address_B");

        let coin4 = coin2.add(coin3);
        println!("add coin3 to coin2 become to coin4 ");
    }
}

