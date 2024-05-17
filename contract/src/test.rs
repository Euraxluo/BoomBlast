#[cfg(test)]
mod nft_test {
    use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApproval;

    use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
    use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
    use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
    use near_contract_standards::non_fungible_token::TokenId;

    use near_sdk::json_types::U128;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, ONE_NEAR, ONE_YOCTO};

    use crate::contract::Contract;

    fn owner() -> AccountId {
        "owner.near".parse().unwrap()
    }

    fn alice() -> AccountId {
        "alice.near".parse().unwrap()
    }

    fn bob() -> AccountId {
        "bob.near".parse().unwrap()
    }

    fn token(token_id: TokenId) -> TokenMetadata {
        // title: string|null, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
        // description: string|null, // free-form description
        // media: string|null, // URL to associated media, preferably to decentralized, content-addressed storage
        // media_hash: string|null, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
        // copies: number|null, // number of copies of this set of metadata in existence when token was minted.
        // issued_at: number|null, // When token was issued or minted, Unix epoch in milliseconds
        // expires_at: number|null, // When token expires, Unix epoch in milliseconds
        // starts_at: number|null, // When token starts being valid, Unix epoch in milliseconds
        // updated_at: number|null, // When token was last updated, Unix epoch in milliseconds
        // extra: string|null, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
        // reference: string|null, // URL to an off-chain JSON file with more info.
        // reference_hash: string|null // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        TokenMetadata {
            title: Some(format!("HelloNFT #{}", token_id)),
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_mint_transfer_burn() {
        let mut contract = Contract::init(owner());

        let token_id_1 = "1".to_string();
        let token_1 = token(token_id_1.clone());
        let token_id_2 = "2".to_string();
        let token_2: TokenMetadata = token(token_id_2.clone());

        // --------------------------------- 给 Bob mint NFT ---------------------------------------

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        contract.mint(bob(), token_1, None);
        contract.mint(bob(), token_2, None);

        assert_eq!(
            contract.nft_token(token_id_1.clone()).unwrap().owner_id,
            bob()
        );
        assert_eq!(
            contract.nft_token(token_id_2.clone()).unwrap().owner_id,
            bob()
        );
        assert_eq!(contract.nft_total_supply(), U128(2));

        // -------------------------------- Bob 给 Alice 转 NFT -------------------------------------

        // `nft_transfer` 调用需要附加 1 yocto NEAR
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(bob())
            .attached_deposit(ONE_YOCTO)
            .build());

        contract.nft_transfer(alice(), token_id_1.clone(), None, None);

        assert_eq!(
            contract.nft_token(token_id_1.clone()).unwrap().owner_id,
            alice()
        );
        assert_eq!(
            contract.nft_token(token_id_2.clone()).unwrap().owner_id,
            bob()
        );
        assert_eq!(contract.nft_total_supply(), U128(2));

        // ---------------------------------- 销毁 Bob 的 NFT ---------------------------------------

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        contract.burn(bob(), token_id_2.clone(), None);

        assert_eq!(contract.nft_token(token_id_1).unwrap().owner_id, alice());
        assert!(contract.nft_token(token_id_2).is_none());
        assert_eq!(contract.nft_total_supply(), U128(1));
    }

    #[test]
    fn test_approve_transfer() {
        let mut contract = Contract::init(owner());

        let token_id = "1".to_string();
        let token = token(token_id.clone());

        // --------------------------------- 给 Bob mint NFT ---------------------------------------

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        contract.mint(bob(), token, None);

        assert_eq!(
            contract.nft_token(token_id.clone()).unwrap().owner_id,
            bob()
        );

        // ------------------------------- Bob 授权 NFT 给 Alice ------------------------------------

        // `nft_approve` 需要附加一些 NEAR 作为被授权账户的存储费
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(bob())
            .attached_deposit(ONE_NEAR / 100) // 附加 0.01 NEAR
            .build());

        contract.nft_approve(token_id.clone(), alice(), None);

        assert!(contract.nft_is_approved(token_id.clone(), alice(), None));

        // ---------------------------- Alice 通过授权把 Bob 的NFT 转给自己 ---------------------------

        // `nft_transfer` 调用需要附加 1 yocto NEAR
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(alice())
            .attached_deposit(ONE_YOCTO)
            .build());

        contract.nft_transfer(alice(), token_id.clone(), None, None);

        assert_eq!(
            contract.nft_token(token_id.clone()).unwrap().owner_id,
            alice()
        );
        assert!(!contract.nft_is_approved(token_id, alice(), None));
    }
}

#[cfg(test)]
mod rhai_test {
    use std::fmt::Display;

    use rhai::{Engine, EvalAltResult, FnPtr};
    #[test]
    fn test_demo() -> Result<(), Box<EvalAltResult>> {
        let engine = Engine::new();

        engine.run(r#"print("hello, world!")"#)?;

        let result = engine.eval::<i64>("40 + 2")?;

        println!("The Answer: {result}"); // prints 42
        assert!(result == 42);
        Ok(())
    }

    #[test]
    fn test_demo2() -> Result<(), Box<EvalAltResult>> {
        let engine = Engine::new();

        // This script creates a closure which captures a variable and returns it.
        let ast = engine.compile(
            "
                let x = 18;
    
                // The following closure captures 'x'
                return |a, b| {
                    x += 1;         // x is incremented each time
                    (x + a) * b
                };
            ",
        )?;

        let closure = engine.eval_ast::<FnPtr>(&ast)?;

        // Create a closure by encapsulating the `Engine`, `AST` and `FnPtr`.
        // In a real application, you'd be handling errors.
        let func = move |x: i64, y: i64| -> i64 { closure.call(&engine, &ast, (x, y)).unwrap() };

        // Now we can call `func` anywhere just like a normal function!
        let r1 = func(1, 2);

        // Notice that each call to `func` returns a different value
        // because the captured `x` is always changing!
        let r2 = func(1, 2);
        let r3 = func(3, 2);

        println!("The Answers: {r1}, {r2}, {r3}");
        assert!(r1 == 40);
        assert!(r2 == 42);
        assert!(r3 == 48);
        Ok(())
    }

    #[derive(Clone, Debug, Copy)]
    pub struct TestStruct {
        pub value: i64,
    }

    impl TestStruct {
        pub fn new() -> TestStruct {
            TestStruct { value: 0 }
        }

        pub fn increment(&mut self) -> i64 {
            self.value += 1;
            self.value
        }
        pub fn get_value(&mut self) -> i64 {
            self.value
        }
        pub fn set_value(&mut self, value: i64) {
            self.value = value;
        }
    }
    fn add(x: &mut TestStruct, y: i64) -> i64 {
        x.value += y;
        x.increment();
        x.value
    }
    #[test]
    fn test_demo3() -> Result<(), Box<EvalAltResult>> {
        let mut engine = Engine::new();
        // 注册 TestStruct 到 Rhai 的 Engine 中
        engine.register_type::<TestStruct>();

        // 注册 TestStruct 的 new 方法
        engine.register_fn("add", add);
        // This script creates a closure which captures a variable and returns it.
        let ast = engine.compile(
            "
            return |a, b| {
                add(a, b)
            };
            ",
        )?;
        let closure = engine.eval_ast::<FnPtr>(&ast)?;

        // Create a closure by encapsulating the `Engine`, `AST` and `FnPtr`.
        // In a real application, you'd be handling errors.
        let add =
            move |x: TestStruct, y: i64| -> i64 { closure.call(&engine, &ast, (x, y)).unwrap() };
        let mut x = TestStruct::new();
        x.value = 18;
        let r3 = add(x, 2);

        println!("The Answers:  {r3}"); // prints 40, 42, 44
        assert!(r3 == 21);
        Ok(())
    }

    #[test]
    fn test_demo4() -> Result<(), Box<EvalAltResult>> {
        let mut engine = Engine::new();
        // 注册 TestStruct 到 Rhai 的 Engine 中
        engine.register_type::<TestStruct>();
        engine.register_get_set("value", TestStruct::get_value, TestStruct::set_value);
        engine.register_fn("increment", TestStruct::increment);
        // 注册 TestStruct 的 new 方法
        engine.register_fn("add", add);

        // This script creates a closure which captures a variable and returns it.
        let ast = engine.compile(
            "
            return |a, b| {
                a.value += b;
                a.increment();
                a.value
            }
            ",
        )?;
        let closure = engine.eval_ast::<FnPtr>(&ast)?;

        // Create a closure by encapsulating the `Engine`, `AST` and `FnPtr`.
        // In a real application, you'd be handling errors.
        let add = move |x: &mut TestStruct, y: i64| -> i64 {
            closure.call(&engine, &ast, (x.clone(), y)).unwrap()
        };
        let mut x = TestStruct::new();
        x.value = 18;
        let r1 = add(&mut x, 2);

        println!("The Answers:  {r1}");
        assert!(r1 == 21);
        println!("The Answers:  {:?}", x);
        Ok(())
    }

    #[test]
    fn test_demo5() -> Result<(), Box<EvalAltResult>> {
        #[derive(Debug, Clone)]
        struct TestStruct {
            x: i64,
        }

        impl TestStruct {
            pub fn new() -> Self {
                Self { x: 1 }
            }
            pub fn update(&mut self) {
                self.x += 1000;
            }
            pub fn calculate(&mut self, data: i64) -> i64 {
                self.x * data
            }
            pub fn get_x(&mut self) -> i64 {
                self.x
            }
            pub fn set_x(&mut self, value: i64) {
                self.x = value;
            }
        }

        let mut engine = Engine::new();

        engine
            .register_type_with_name::<TestStruct>("TestStruct")
            .register_fn("new_ts", TestStruct::new)
            .register_fn("update", TestStruct::update)
            .register_fn("calc", TestStruct::calculate)
            .register_get_set("x", TestStruct::get_x, TestStruct::set_x);

        let result = engine.eval::<i64>(
            "
            let x = new_ts();
            x.x = 42;
            x.update();
            x.calc(x.x)
        ",
        )?;

        println!("result: {result}"); // prints 1085764

        Ok(())
    }
}

#[cfg(test)]
mod game_test {
    use std::fmt::format;

    use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApproval;

    use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
    use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
    use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
    use near_contract_standards::non_fungible_token::TokenId;

    use near_sdk::json_types::U128;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, ONE_NEAR, ONE_YOCTO};

    use crate::contract::Contract;

    fn owner() -> AccountId {
        "owner.near".parse().unwrap()
    }

    fn alice() -> AccountId {
        "alice.near".parse().unwrap()
    }

    fn bob() -> AccountId {
        "bob.near".parse().unwrap()
    }

    fn token(user: String, card: u64) -> TokenMetadata {
        TokenMetadata {
            title: Some(format!("NFT #{}", user)),
            description: None,
            media: Some(format!("http://images.com/{}", card)),
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: Some(format!("{}", card)),
            reference: None,
            reference_hash: None,
        }
    }

    /// 测试游戏创建
    #[test]
    fn test_create_game() {
        let mut contract = Contract::init(owner());
        // --------------------------------- 给 Bob mint NFT ---------------------------------------

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        let game_id = contract.create_game();
        println!("game_id: {:?}", game_id);
        let game = contract.get_game(game_id);
        println!("game: {:?}", game);
        let active_games = contract.active_games();
        println!("active_games: {:?}", active_games);
        assert!(active_games.contains(&game_id));
        assert!(active_games.len() == 1);
    }

    /// 测试两个玩家匹配游戏
    #[test]
    fn test_find_game() {
        let mut contract = Contract::init(owner());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        // --------------------------------- 两个玩家都进行匹配 ---------------------------------------
        // 然后两个玩家都进行匹配
        // 这里 逻辑是这样的，每个玩家在点击页面后，都会调用 find_game 方法，然后在 find_game 方法中，
        // 会遍历所有的游戏，然后找到第一个满足条件的游戏，然后进行匹配
        // 但如果当前没有游戏，那么就会去创建一个游戏。
        // find game 本质上就是返回一个activate game list

        let find1 = contract.find_game();
        let find2 = contract.find_game();
        println!("find1: {:?}", find1);
        println!("find2: {:?}", find2);
        assert_eq!(find1, find2);

        // --------------------------------- 两个玩家分别创建游戏 ---------------------------------------
        let game_1 = contract.create_game();
        let game_value1 = contract.get_game(game_1);
        println!("game_1: {:?}", game_1);
        println!("game_value1: {:?}", game_value1);

        let game_2 = contract.create_game();
        let game_value2 = contract.get_game(game_2);
        println!("game_2: {:?}", game_2);
        println!("game_value2: {:?}", game_value2);

        assert_ne!(game_1, game_2);
    }

    /// 测试两个玩家匹配游戏
    #[test]
    fn test_match_game() {
        let mut contract = Contract::init(owner());

        println!("--------------------------------- 两个玩家分别进行匹配 ---------------------------------------");
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(bob())
            .build());
        let game_1 = contract.join_game();
        let game_value1 = contract.get_game(game_1);
        println!("game_1: {:?}", game_1);
        println!("game_value1: {:?}", game_value1);
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(alice())
            .build());
        let game_2 = contract.join_game();
        let game_2 = contract.join_game();
        let game_2 = contract.join_game();
        let game_2 = contract.join_game();
        let game_2 = contract.join_game();
        let game_value2 = contract.get_game(game_2);
        println!("game_2: {:?}", game_2);
        println!("game_value2: {:?}", game_value2);
        assert_eq!(game_1, game_2);

        println!(
            "--------------------------------- 离开游戏 ---------------------------------------"
        );
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(bob())
            .build());
        contract.leave_game();
        contract.leave_game();
        contract.leave_game();
        contract.leave_game();
        let game_value1 = contract.get_game(game_1);
        println!("game_value1: {:?}", game_value1);

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(alice())
            .build());
        contract.leave_game();
        let game_value2 = contract.get_game(game_1);
        println!("game_value2: {:?}", game_value2);
    }

    /// 测试添加卡片和添加游戏模式
    #[test]
    fn test_add_card_and_add_game_mode() {
        let mut contract = Contract::init(owner());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .build());

        let game_mode_id = "game_mode_1".to_string();
        let game_mode_card = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        contract.add_game_mode(game_mode_id.clone(), game_mode_card.clone());
        println!("game_mode_id: {:?}", game_mode_id);
        let game_mode = contract.get_game_mode(game_mode_id);
        println!("game_mode: {:?}", game_mode);
        assert_eq!(game_mode, game_mode_card);

        let card_id = contract.add_card(
            "card_name".to_string(),
            "card_des".to_string(),
            "card_img".to_string(),
            "card_type  ".to_string(),
            "card_type".to_string(),
        );
        println!("card_id: {:?}", card_id);
        let card = contract.get_card(card_id);
        println!("card: {:?}", card);
    }

    /// 测试多个玩家加入游戏后，完成游戏设置后，开始游戏
    #[test]
    fn test_start_game() {
        let mut contract = Contract::init(owner());

        println!("--------------------------------- 两个玩家分别进行匹配 ---------------------------------------");
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(bob())
            .build());
        let game_1 = contract.join_game();
        let game_value1 = contract.get_game(game_1);
        println!("game_1: {:?}", game_1);
        println!("game_value1: {:?}", game_value1);
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(alice())
            .build());
        let game_2 = contract.join_game();
        let game_2 = contract.join_game();
        let game_2 = contract.join_game();
        let game_2 = contract.join_game();
        let game_2 = contract.join_game();
        let game_value2 = contract.get_game(game_2);
        println!("game_2: {:?}", game_2);
        println!("game_value2: {:?}", game_value2);
        assert_eq!(game_1, game_2);

        println!(
            "--------------------------------- 离开游戏 ---------------------------------------"
        );
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(bob())
            .build());
        contract.leave_game();
        contract.leave_game();
        contract.leave_game();
        contract.leave_game();
        let game_value1 = contract.get_game(game_1);
        println!("game_value1: {:?}", game_value1);

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(alice())
            .build());
        contract.leave_game();

        let game_value2 = contract.get_game(game_1);
        println!("game_value2: {:?}", game_value2);
    }

    /// 测试多个玩家加入游戏后，开始游戏玩耍
    #[test]
    fn test_play_game() {
        let mut contract: Contract = Contract::init(owner());

        println!("--------------------------------- 两个玩家分别mint自己的卡牌并进行匹配 ---------------------------------------");
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(bob())
            .build());

        println!("--------------------------------- 玩家 bob 操作 ---------------------------------------");
        for _ in 0..3 {
            contract.mint(
                bob(),
                token(
                    bob().to_string(),
                    contract.get_card_id("Shuffle".to_string()),
                ),
                Some("bob mint".to_string()),
            );
        }

        // 获取用户的所有token
        let tokens_1 = contract.nft_tokens_for_owner(bob(), None, None);
        println!("tokens: {:?}", tokens_1);
        let game_1 = contract.join_game();
        let game_value1 = contract.get_game(game_1);
        println!("game: {:?}", game_1);
        println!("game_value: {:?}", game_value1);

        println!("--------------------------------- 玩家alice操作 ---------------------------------------");
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(alice())
            .build());

        for _ in 0..3 {
            contract.mint(
                alice(),
                token(
                    alice().to_string(),
                    contract.get_card_id("Shuffle".to_string()),
                ),
                Some("alice mint".to_string()),
            );
        }

        // 获取用户的所有token
        let tokens_2 = contract.nft_tokens_for_owner(alice(), None, None);
        println!("tokens: {:?}", tokens_2);
        let game_2 = contract.join_game();
        let game_value2 = contract.get_game(game_2);
        println!("game: {:?}", game_2);
        println!("game_value: {:?}", game_value2);
        assert_eq!(game_1, game_2);

        println!("--------------------------------- 配置     游戏 ---------------------------------------");

        println!("--------------------------------- 玩家 bob 操作 ---------------------------------------");
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner())
            .signer_account_id(bob())
            .build());
        contract.prepare_game(
            tokens_1.into_iter().map(|x| x.token_id).collect(),
            "basic".to_string(),
        );
        // 打印玩家状态
        println!("--------------------------------- 玩家alice操作 ---------------------------------------");
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(alice())
            .signer_account_id(alice())
            .build());
        contract.prepare_game(
            tokens_2.into_iter().map(|x| x.token_id).collect(),
            "basic".to_string(),
        );

        println!("--------------------------------- 玩家     状态 ---------------------------------------");
        println!("status {:?}", contract.player_states(&alice()));
        println!("status {:?}", contract.player_states(&alice()));
    }
}
#[cfg(test)]
mod test_some_code {

    use std::cmp::Ordering;

    fn binary_search<T: Ord>(vec: &[T], target: &T) -> Option<usize> {
        let mut low = 0;
        let mut high = vec.len();

        while low < high {
            let mid = low + (high - low) / 2;
            match vec[mid].cmp(target) {
                Ordering::Equal => return Some(mid),
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }

        None
    }

    fn delete_element<T: Ord>(vec: &mut Vec<T>, target: T) {
        if let Some(index) = binary_search(vec, &target) {
            vec.remove(index);
        }
    }
    #[test]
    fn bs() {
        let mut sorted_vec = vec![1, 2, 3, 4, 5, 7, 8, 8, 9, 10, 1000];
        println!("Original Vec: {:?}", sorted_vec);

        delete_element(&mut sorted_vec, 10);
        println!("Vec after deleting 3: {:?}", sorted_vec);
    }
}
