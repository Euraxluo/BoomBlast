use near_contract_standards::non_fungible_token::events::{NftBurn, NftMint};
use near_contract_standards::{
    impl_non_fungible_token_approval, impl_non_fungible_token_core,
    impl_non_fungible_token_enumeration,
};
use near_sdk::collections::{LookupSet, UnorderedSet};
use rand::seq::SliceRandom;
use rhai::{Dynamic, Engine, EvalAltResult, FnPtr, FuncArgs, ImmutableString};
use std::cmp::Ordering;

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
// 定义合约根结构, 一个项目中只能有一个根结构
#[near_bindgen]
// 实现 borsh 序列化, 实现不可用的 `default` 方法以通过编译
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // 合约账号
    owner_id: AccountId,
    // 自增 token id
    token_id: u64,
    // 存储所有的卡牌
    tokens: NonFungibleToken,
    // 自增卡片id
    card_id: u64,
    // 存储所有的卡片
    cards: LookupMap<u64, Card>,
    // 用于存储卡片id和卡片名称的映射
    card_map: LookupMap<String, u64>,
    // 自增对局id
    game_id: u64,
    // 存储所有关联的游戏状态
    games: LookupMap<u64, Game>,
    // 存储玩家的状态
    player_status: LookupMap<AccountId, PlayerStatus>,
    // 激活的游戏
    active_games: Vec<u64>,
    // 预设的游玩模式
    game_mode: LookupMap<String, Vec<u64>>,
    // TODO:匹配队列
    match_queue: LookupSet<AccountId>,
}

// 游戏中卡池的状态
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Deck {
    // 当前卡牌库中交互玩家数
    pub num_of_players: usize,
    // 当前卡牌库
    pub cards: Vec<Card>,
}

// 游戏中玩家的状态
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Player {
    // 玩家id
    name: AccountId,
    //玩家手牌
    hand: Vec<Card>,
    // 玩家状态
    active: bool,
    // 玩家投票数据的存储
    mode_vote: String,
}

// 游戏状态
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Game {
    // 游戏对局id
    game_id: u64,
    // 当前对局卡牌库
    deck: Deck,
    // 弃牌堆
    discard_pile: Vec<Card>,
    // 游戏玩家列表
    players: Vec<Player>,
    //最终胜利者
    winner: String,
    // 当前轮次玩家
    current_player: String,
    // 当前轮次
    turn_count: usize,
    // 当前玩家需要摸的牌数
    cards_to_draw: usize,
    // 下一位玩家需要摸的牌数
    next_player_cards_to_draw: usize,
}

impl FuncArgs for Game {
    fn parse<ARGS: Extend<Dynamic>>(self, args: &mut ARGS) {
        // args.extend(Some(self.game_id.into()));
        // args.extend(Some(self.deck.clone().into()));
        args.extend(Some(self.discard_pile.clone().into()));
        args.extend(Some(self.players.clone().into()));
        args.extend(Some(self.winner.clone().into()));
        args.extend(Some(self.current_player.clone().into()));
        // args.extend(Some(self.turn_count.into()));
        // args.extend(Some(self.cards_to_draw.into()));
        // args.extend(Some(self.next_player_cards_to_draw.into()));
    }
}
// impl FuncArgs for Deck {
//     fn parse<ARGS: Extend<Dynamic>>(self, args: &mut ARGS) {
//         args.extend(Some(self.num_of_players.into()));
//         args.extend(Some(self.cards.into()));
//     }
// }
// 卡牌信息
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    // 卡牌id
    card_id: u64,
    // 卡牌名
    card_name: String,
    // 卡牌描述
    card_description: String,
    // 卡牌图片
    card_image: String,
    // 卡牌类型
    card_type: String,
    // 卡牌行为
    card_action: String,
    // 卡牌状态
    card_status: CardStatus,
}

impl Card {
    fn action(&self, game: Game) -> Result<(), Box<EvalAltResult>> {
        let mut engine = Engine::new();
        // 注册 GameStruct 到 Rhai 的 Engine 中
        engine.register_type::<Game>();

        // 注册 GameStruct 的 new 方法
        engine.register_fn("shuffle_deck", Game::shuffle_deck);
        // This script creates a closure which captures a variable and returns it.
        let card_action_ast = engine.compile(self.card_action.as_str())?;
        let closure = engine.eval_ast::<FnPtr>(&card_action_ast)?;

        // Create a closure by encapsulating the `Engine`, `AST` and `FnPtr`.
        let action = move |g: Game| -> Result<(), Box<EvalAltResult>> {
            closure.call(&engine, &card_action_ast, g)
        };

        // Execute the closure and handle any errors
        action(game)?;

        Ok(())
    }
}
/// 卡片状态
#[derive(
    BorshSerialize, BorshStorageKey, BorshDeserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum CardStatus {
    // 整个BoomBlast中禁用
    Disabled,
    // 整个BoomBlast可使用
    Available,

    // 整个Game的手牌中
    InHand,
    // 整个Game的牌堆中
    InDeck,
    // 整个Game的弃牌堆中
    InDiscardPile,
    // 已准备
    // Ready,
    // 已加入
    // Joined,
    // 已退出
    // Left,
    // 已离线
    // Offline,
    // 已掉线
    // Disconnected,
    // 已断开连接
    // Disconnected,
    // 已连接
    // Connected,
    // 已匹配
    // Matched,
    // 已准备
    // Ready,
    // 已开始
    // Started,
    // 已结束
    // Ended,
    // 已取消
    // Canceled,
}

/// 卡片状态
#[derive(
    BorshSerialize, BorshStorageKey, BorshDeserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum PlayerStatus {
    // 空闲
    IDLE,
    // 队列中
    Queued,
    // 游戏中
    Playing(u64),
    // 加入游戏
    Joined(u64),
    JoinedActive(u64),
    // 准备完成
    Ready(u64),
}
// 存储键
#[derive(BorshSerialize, BorshStorageKey, Debug)]
pub enum StorageKey {
    Approval,
    Enumeration,
    TokenMetadata,
    NonFungibleToken,
    Card,
    CardMap,
    Game,
    Player,
    AccountId,
    PlayerStatus,
    GameMode,
    MatchQueue,
}
macro_rules! extend_cards {
    ($deck:expr, $card:expr, $count:expr) => {
        for _ in 0..$count {
            $deck.cards.push($card);
        }
    };
}

#[near_bindgen]
impl Contract {
    // 初始化合约
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        let mut contract = Self {
            owner_id: owner_id.clone(),
            token_id: 0,
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            card_id: 0,
            cards: LookupMap::new(StorageKey::Card),
            card_map: LookupMap::new(StorageKey::CardMap),
            game_id: 0,
            games: LookupMap::new(StorageKey::Game),
            active_games: Vec::new(),
            player_status: LookupMap::new(StorageKey::PlayerStatus),
            game_mode: LookupMap::new(StorageKey::GameMode),
            match_queue: LookupSet::new(StorageKey::MatchQueue),
        };
        contract.init_card();
        contract.init_game_mode();
        return contract;
    }

    /// 获取一个游戏对局信息
    pub fn get_game(&self, game_id: u64) -> Game {
        // 如果玩家已经在游戏中，那么直接返回游戏id
        if let Some(game) = self.games.get(&game_id) {
            return game.clone();
        }
        env::panic_str("The game does not exist.");
    }

    /// 寻找游戏，如果当前有激活状态的游戏，那么返回第一个游戏，
    /// 如果当前没有激活状态的游戏，那么就创建一个新游戏，然后返回该新游戏
    pub fn find_game(&mut self) -> u64 {
        if self.active_games.is_empty() {
            return self.create_game();
        }
        self.active_games[0]
    }

    /// 获取可游玩的游戏列表,这是一个view函数
    pub fn active_games(&self) -> Vec<u64> {
        self.active_games.clone()
    }

    /// 创建空游戏，并且增加游戏ID，然后将该游戏放入 active_games 列表中
    pub fn create_game(&mut self) -> u64 {
        // 游戏ID自增
        self.game_id += 1;
        // 创建一个游戏
        let game_value = Game {
            // 对局id
            game_id: self.game_id,
            // 空牌堆
            deck: Deck {
                num_of_players: 0,
                cards: Vec::new(),
            },
            // 空弃牌堆
            discard_pile: Vec::new(),
            // 空玩家列表
            players: Vec::new(),
            // 胜利玩家
            winner: "".to_string(),
            // 当前玩家
            current_player: "".to_string(),
            // 当前轮次
            turn_count: 0,
            // 当前玩家需要摸的牌数
            cards_to_draw: 0,
            // 下一位玩家需要摸的牌数
            next_player_cards_to_draw: 0,
        };
        // 添加游戏到索引中
        self.games.insert(self.game_id, game_value);
        // 添加游戏到游戏列表中
        self.active_games.push(self.game_id);
        // 返回这局的游戏
        self.game_id
    }

    /// 获取某个玩家状态
    pub fn player_states(&self, player: &AccountId) -> PlayerStatus {
        self.player_status
            .get(player)
            .expect("Player does not exist.")
            .clone()
    }
    /// TODO：加入别人创建的游戏大厅，需要使用凭证加入，并且这个凭证是由零知识证明生成的
    pub fn join_lobby(&mut self, game_certificate: u64, player: &AccountId) {}

    /// 寻找游戏并加入加入游戏
    /// 该方式用于队列匹配
    /// 只有玩家自己可以调用该方法
    pub fn join_game(&mut self) -> u64 {
        let player = env::signer_account_id();
        // 如果玩家已经在游戏中，那么直接返回游戏id
        if let Some(PlayerStatus::Joined(game_id)) = self.player_status.get(&player) {
            return game_id.clone();
        }

        // 寻找游戏
        let game_id = self.find_game();
        // 更新游戏
        if let Some(game) = self.games.get_mut(&game_id) {
            // 创建玩家
            let player_value = Player {
                name: player.clone(),
                hand: Vec::new(),
                active: false,
                mode_vote: "".to_string(),
            };
            // 添加玩家到游戏中
            (*game).players.push(player_value);
            //  改变玩家状态
            self.player_status
                .insert(player.clone(), PlayerStatus::Joined(game_id.clone()));
        }
        game_id
    }

    /// 离开游戏
    pub fn leave_game(&mut self) {
        let player = env::signer_account_id();
        match self.player_status.get(&player) {
            // 当用户状态为加入房间时，离开没有其他副作用
            Some(PlayerStatus::Joined(game_id)) => {
                if let Some(game) = self.games.get_mut(game_id) {
                    // 查找满足条件的玩家索引
                    if let Some(index) = game.players.iter().position(|p| p.name == player.clone())
                    {
                        // 根据索引删除玩家
                        game.players.remove(index);
                        //  改变玩家状态
                        self.player_status
                            .insert(player.clone(), PlayerStatus::IDLE);
                    }
                }
            }
            // 当用户状态为准备完成时，离开时直接游戏中的玩家
            Some(PlayerStatus::Ready(game_id)) => {
                if let Some(game) = self.games.get_mut(game_id) {
                    // 查找满足条件的玩家索引
                    if let Some(index) = game.players.iter().position(|p| p.name == player.clone())
                    {
                        // 根据索引删除玩家
                        game.players.remove(index);
                        //  改变玩家状态
                        self.player_status
                            .insert(player.clone(), PlayerStatus::IDLE);
                    }
                }
            }
            // 当用户状态为游戏中时，离开时游戏中的玩家状态为离开
            Some(PlayerStatus::Playing(game_id)) => {
                if let Some(game) = self.games.get_mut(game_id) {
                    // 查找满足条件的玩家索引
                    if let Some(index) = game.players.iter().position(|p| p.name == player.clone())
                    {
                        // 将游戏中该玩家的状态设置为不活跃
                        game.players.get_mut(index).unwrap().active = false;
                        // 改变玩家状态
                        self.player_status
                            .insert(player.clone(), PlayerStatus::IDLE);
                    }
                }
            }
            _ => {
                // Handle other cases if needed
            }
        }
    }

    /// 准备游戏
    /// 玩家从自己的所有卡牌中选择3张卡牌，你可以传三个一样的tokenID，表示你选这个token作为你的牌，然后准备游戏
    /// 进入对局后，所有玩家都会摸到5张卡牌，也就是牌堆会发2个牌给每个用户
    pub fn prepare_game(&mut self, token_ids: Vec<TokenId>, game_mode: String) -> u64 {
        require!(
            token_ids.len() == 3,
            "You need to select 3 cards to prepare the game."
        );
        let player = env::signer_account_id();
        // 只处理玩家已经加入游戏的情况
        if let Some(&PlayerStatus::Joined(game_id)) = self.player_status.get(&player.clone()) {
            // 获取当前用户的所有token,并且判断是不是所有的token都是属于当前用户的
            if let Some(tokens_per_owner) = &self.tokens.tokens_per_owner {
                // 成功获取当前用户的所有token
                if let Some(user_total_token_ids) = tokens_per_owner.get(&player) {
                    // 判断是不是所有的token都是属于当前用户的
                    if token_ids
                        .iter()
                        .all(|_token_id| user_total_token_ids.contains(&_token_id))
                    {
                        // 如果都是属于当前用户的，那么就可以调整当前玩家的手牌状态
                        // 设置当前玩家的初始手牌
                        let current_hand_cards: Vec<Card> = token_ids
                            .iter()
                            .map(|_input_token_id| {
                                let token_metadata = self
                                    .tokens
                                    .token_metadata_by_id
                                    .as_ref()
                                    .unwrap()
                                    .get(&_input_token_id)
                                    .unwrap();
                                let token_extra_card_id = token_metadata
                                    .extra
                                    .as_ref()
                                    .expect("BoomBlast extra set failed.")
                                    .parse::<u64>()
                                    .unwrap();
                                let card = self.cards.get(&token_extra_card_id).unwrap();

                                // 如果卡片已经在整个对局中禁用，那么就抛出异常
                                if card.card_status == CardStatus::Disabled {
                                    env::panic_str(
                                        format!(
                                            "The card {} has been disabled throughout the game",
                                            card.card_name
                                        )
                                        .as_str(),
                                    );
                                }
                                Card {
                                    card_id: token_extra_card_id,
                                    card_name: card.card_name.clone(),
                                    card_description: card.card_description.clone(),
                                    card_image: token_metadata.media.clone().unwrap(),
                                    card_type: card.card_type.clone(),
                                    card_action: card.card_action.clone(),
                                    card_status: CardStatus::InHand,
                                }
                            })
                            .collect();

                        // 更新游戏中当前玩家的手牌和玩家的投票
                        if let Some(game) = self.games.get_mut(&game_id) {
                            // 查找满足条件的玩家索引
                            if let Some(_player_index) =
                                game.players.iter().position(|p| p.name == player.clone())
                            {
                                // 设置当前玩家的手牌
                                game.players[_player_index].hand = current_hand_cards;
                                // 激活当前玩家状态
                                game.players[_player_index].active = true;
                                game.players[_player_index].mode_vote = game_mode;
                            }
                            println!("ready game: {:?}", game);
                            // 改变当前玩家状态为准备完成
                            self.player_status
                                .insert(player.clone(), PlayerStatus::Ready(game_id));

                            // 判断当前游戏的状态，如果所有玩家都准备好了，那么就设置当前游戏为游戏中状态
                            if game.players.iter().all(|p| p.active == true) {
                                self.active_game();
                            }
                        }
                        return game_id;
                    }
                }
                env::panic_str("Did not have any BoomBlastNFT.");
            }
        }
        env::panic_str("Did not join any game.");
    }

    pub fn play(&mut self, game_id: u64, card_id: u64) {
        let player = env::signer_account_id();
        // 确保当前玩家还处于游戏中状态
        require!(
            self.player_status.get(&player).unwrap() == &PlayerStatus::Playing(game_id),
            "You are not in the game."
        );
        let mut game = self
            .games
            .get_mut(&game_id)
            .expect(format!("Game {} does not exist.", game_id).as_str());
        // 确保当前游戏轮到当前玩家出牌
        require!(
            game.current_player == player.to_string(),
            "It's not your turn."
        );
        // 确保当前玩家在游戏里面还活着
        require!(
            game.players
                .iter()
                .find(|p| p.name == player)
                .unwrap()
                .active,
            "You are dead."
        );
        // 获取当前打出的卡牌
        let played_card = game
            .players
            .iter()
            .find(|p| p.name == player)
            .unwrap()
            .hand
            .iter()
            .find(|card| card.card_id == card_id)
            .unwrap();

        // 更新游戏的弃牌堆
        game.discard_pile.push(played_card.clone());

        let new_game = game.clone();
        // 执行卡牌的行为
        played_card.action(new_game);
    }

    // 只有合约所有者可以禁用卡片类型
    pub fn disable_card(&mut self, card_id: u64) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        if let Some(card) = self.cards.get_mut(&card_id) {
            (*card).card_status = CardStatus::Disabled;
        }
    }
    // 获取卡片详细信息
    pub fn get_card(&self, card_id: u64) -> Card {
        self.cards
            .get(&card_id)
            .expect("Card does not exist.")
            .clone()
    }
    // 只有合约所有者可以新增卡片类型
    pub fn add_card(
        &mut self,
        card_name: String,
        card_description: String,
        card_image: String,
        card_type: String,
        card_action: String,
    ) -> u64 {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.internal_add_card(
            card_name,
            card_description,
            card_image,
            card_type,
            card_action,
        )
    }
    // 只有合约所有者可以添加新的游戏模式
    pub fn add_game_mode(&mut self, game_mode: String, game_ids: Vec<u64>) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.game_mode.insert(game_mode, game_ids);
    }
    // 获取游戏模式
    pub fn get_game_mode(&self, game_mode: String) -> Vec<u64> {
        self.game_mode
            .get(&game_mode)
            .expect("Game mode does not exist.")
            .clone()
    }

    // predecessor_account burn NFT
    pub fn burn(&mut self, account_id: AccountId, token_id: TokenId, memo: Option<String>) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.internal_burn(&account_id, &token_id, memo);
    }

    // predecessor_account transfer NFT
    pub fn transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.nft_transfer(receiver_id, token_id, approval_id, memo)
    }

    /// 每个用户都可以mint自己的nft
    /// 每个nft会关联一个卡牌类型
    pub fn mint(&mut self, account_id: AccountId, metadata: TokenMetadata, memo: Option<String>) {
        let token_id = self.next_token_id().to_string();
        self.internal_mint(&account_id, &token_id, &metadata, memo);
    }
}

// ------------------------------------- 合约内部方法 ------------------------------------------------

// 定义一个宏，用于展开 add_card 的调用
macro_rules! add_card {
    ($self:ident, $name:expr, $description:expr, $image:expr, $card_type:expr, $action:expr) => {
        $self.internal_add_card(
            $name.to_string(),
            $description.to_string(),
            $image.to_string(),
            $card_type.to_string(),
            $action.to_string(),
        );
    };
}
impl Game {
    // 洗牌
    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle_deck();
    }
}
impl Deck {
    /// 洗牌
    pub fn shuffle_deck(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl Contract {
    // 设置所有没有设置好的内容
    // 1. 设置游戏的卡牌库
    // 4. 设置游戏的当前玩家
    // 5. 设置游戏的轮次
    // 6. 设置游戏的当前玩家需要摸的牌数
    // 7. 设置游戏的下一位玩家需要摸的牌数
    // 8. 设置所有玩家状态切换至游戏中
    pub fn active_game(&mut self) {
        let player = env::signer_account_id();
        // 只处理玩家已经加入游戏的情况
        if let Some(&PlayerStatus::Ready(game_id)) = self.player_status.get(&player.clone()) {
            // 先处理所有的投票信息，这样才能知道怎么设置当前游戏的卡牌库是什么
            let game = self
                .games
                .get(&game_id)
                .expect(format!("Game {} does not exist.", game_id).as_str());
            let vote = self.vote_result(game);
            // 通过投票结果设置游戏的卡牌库
            let vote_cards = self.cards_to_deck(
                self.game_mode
                    .get(&vote)
                    .expect(format!("Game mode {} does not exist.", vote).as_str())
                    .clone(),
                vec![],
            );

            // 获取核心卡牌
            let kernel_cards = self.cards_to_deck(
                vec![
                    self.get_card_id("ExplodingKitten".to_string()),
                    self.get_card_id("Defuse".to_string()),
                ],
                vec![game.players.len() - 1, game.players.len()],
            );

            println!("kernel_cards: {:?}", kernel_cards);
            // 修改游戏配置
            if let Some(game) = self.games.get_mut(&game_id) {
                // 设置游戏的牌堆
                {
                    // 设置游戏的卡牌库
                    game.deck.cards = vote_cards;
                    // 添加核心卡牌
                    game.deck.cards.extend(kernel_cards);
                    // 洗牌
                    game.deck.shuffle_deck();
                }

                // 设置游戏
                {
                    // 设置游戏的当前玩家
                    game.current_player = game.players[0].name.to_string();
                    // 设置游戏的轮次
                    game.turn_count = 1;
                    // 设置游戏的当前玩家需要摸的牌数
                    game.cards_to_draw = 2;
                    // 设置游戏的下一位玩家需要摸的牌数
                    game.next_player_cards_to_draw = 0;
                }
                // 设置玩家
                {
                    // 设置所有玩家状态切换至游戏中
                    for player in &mut game.players {
                        self.player_status
                            .insert(player.name.clone(), PlayerStatus::Playing(game_id));
                        println!("deck cards: {:?}", game.deck.cards.len());
                        player.hand.extend(
                            (0..=1)
                                .map(|_| game.deck.cards.pop().unwrap())
                                .collect::<Vec<Card>>(),
                        );
                        player.hand.sort_by_key(|card| card.card_id);
                    }
                }
            }
        }
    }

    /// 获取游戏的投票游戏模式
    pub fn vote_result(&self, game: &Game) -> String {
        // 模式投票数量必须大于0
        require!(
            game.players.len() > 0,
            "Game mode vote,player count must be greater than 0."
        );
        // 获取所有的投票信息
        let mode_votes = game
            .players
            .iter()
            .map(|p| p.mode_vote.clone())
            .collect::<Vec<String>>();
        // 获取投票最多的模式，如果模式数量均等，那么就获取第一个模式
        let mut mode = mode_votes[0].clone();
        let mut mode_count = 0;
        for vote in &mode_votes {
            let vote_count = mode_votes.iter().filter(|v| *v == vote).count();
            if vote_count > mode_count {
                mode = vote.clone();
                mode_count = vote_count;
            }
        }
        return mode;
    }
    /// 设置卡片为牌堆中
    pub fn cards_to_deck(&self, card_ids: Vec<u64>, nums: Vec<usize>) -> Vec<Card> {
        let mut card_nums = nums;
        if card_nums.is_empty() {
            card_nums = vec![1; card_ids.len()];
        }
        require!(
            card_ids.len() == card_nums.len(),
            "Card id and card num must be equal."
        );
        card_ids
            .iter()
            .zip(card_nums)
            .flat_map(|(card_id, num)| {
                self.cards.get(card_id).map(|card| {
                    // 如果卡片已经在整个游戏中禁用，那么就抛出异常
                    if card.card_status == CardStatus::Disabled {
                        env::panic_str(&format!(
                            "The card {} has been disabled throughout the game",
                            card.card_name
                        ));
                    }

                    vec![
                        Card {
                            card_id: card.card_id,
                            card_name: card.card_name.clone(),
                            card_description: card.card_description.clone(),
                            card_image: card.card_image.clone(),
                            card_type: card.card_type.clone(),
                            card_action: card.card_action.clone(),
                            card_status: CardStatus::InDeck,
                        };
                        num
                    ]
                })
            })
            .flatten()
            .collect()
    }
    /// 二分搜索
    fn binary_search<T: Ord>(&self, vec: &[T], target: &T) -> Option<usize> {
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
    /// 删除元素，主要用于删除玩家列表元素
    fn delete_element_bs<T: Ord>(&mut self, vec: &mut Vec<T>, target: T) {
        if let Some(index) = self.binary_search(vec, &target) {
            vec.remove(index);
        }
    }

    pub(crate) fn next_token_id(&mut self) -> u64 {
        self.token_id += 1;
        self.token_id
    }

    /// 初始化游玩模式
    fn init_game_mode(&mut self) {
        self.game_mode
            .insert("basic".to_string(), (0..=1).collect());
        self.game_mode
            .insert("classic".to_string(), (0..=14).collect());
    }

    /// 初始化卡片
    fn init_card(&mut self) {
        // 示例用法
        add_card!(
            self,
            "ExplodingKitten",
            "Exploding Kitten(爆炸猫，需要打出拆除卡，否则出局)",
            "image",
            "type",
            "
            |game| {
            };
            "
        );
        add_card!(
            self,
            "Defuse",
            "Defuse(拆除卡)",
            "image",
            "type",
            "
            |game| {
            };
            "
        );

        add_card!(
            self,
            "Catermelon",
            "Catermelon(普通西瓜卡)",
            "image",
            "type",
            "
            |game| {
            };
            "
        );
        add_card!(
            self,
            "HairyPotatoCat",
            "Hairy Potato Cat(普通土豆猫卡)",
            "",
            "",
            "
            |game| {
            };
            "
        );

        add_card!(
            self,
            "RainbowCat",
            "Rainbow Cat(普通彩虹猫卡)",
            "",
            "",
            "
            |game| {
            };
            "
        );

        add_card!(
            self,
            "BeardCat",
            "Beard Cat(普通胡子猫卡)",
            "",
            "",
            "
            |game| {
            };
            "
        );

        add_card!(
            self,
            "Tacocat",
            "Tacocat(普通墨西哥卷饼猫卡)",
            "",
            "",
            "
            |game| {
            };
            "
        );

        add_card!(
            self,
            "Favor",
            "Favor(不知道啥用)",
            "",
            "",
            "
            |game| {
            };
            "
        );

        add_card!(
            self,
            "Skip",
            "Skip(跳过卡)",
            "",
            "",
            "
            |game| {
                game.cards_to_draw -= 1;
            };
            "
        );

        add_card!(
            self,
            "Attack",
            "Attack(攻击卡，下一位玩家需要多摸两张牌)",
            "",
            "",
            "
            |game| {
                if game.cards_to_draw == 1 {
                    game.next_player_cards_to_draw = 2;
                } else {
                    game.next_player_cards_to_draw = game.cards_to_draw + 2;
                }
                game.cards_to_draw = 0;
            };
            "
        );

        add_card!(
            self,
            "SeeTheFuture",
            "See The Future(看三张牌)",
            "",
            "",
            "
            |game| {
                game.show(game.deck.top_three());
            };
            "
        );

        add_card!(
            self,
            "Nope",
            "Nope(否定卡,阻止其他玩家的行动)",
            "",
            "",
            "
            |game| {
            };
            "
        );

        add_card!(
            self,
            "Shuffle",
            "Shuffle(洗牌卡，重新洗牌)",
            "",
            "",
            "
            |game| {
                game.deck.shuffle_deck();
            };
            "
        );
    }
    pub(crate) fn get_card_id(&self, card_name: String) -> u64 {
        self.card_map
            .get(&card_name)
            .expect("Card does not exist.")
            .clone()
    }
    // 只有合约所有者可以新增卡片类型
    pub(crate) fn internal_add_card(
        &mut self,
        card_name: String,
        card_description: String,
        card_image: String,
        card_type: String,
        card_action: String,
    ) -> u64 {
        self.card_id += 1;
        let card = Card {
            card_id: self.card_id,
            card_name: card_name.clone(),
            card_description,
            card_image,
            card_type,
            card_action,
            card_status: CardStatus::Available,
        };
        self.cards.insert(self.card_id, card);
        self.card_map.insert(card_name, self.card_id);
        self.card_id
    }

    // 内部的burn方法
    pub(crate) fn internal_burn(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
        memo: Option<String>,
    ) {
        // 移除 token_id -> token_owner_id 映射
        self.tokens.owner_by_id.remove(token_id);

        // 更新或移除 token_owner_id -> token_ids 映射
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            if let Some(mut token_ids) = tokens_per_owner.remove(account_id) {
                token_ids.remove(token_id);
                if !token_ids.is_empty() {
                    tokens_per_owner.insert(account_id, &token_ids);
                }
            }
        };

        // 移除 token_id -> token_metadata 映射
        if let Some(token_metadata_by_id) = &mut self.tokens.token_metadata_by_id {
            token_metadata_by_id.remove(token_id);
        }

        // 移除 token_id -> approval_ids 映射
        if let Some(approvals_by_id) = &mut self.tokens.approvals_by_id {
            approvals_by_id.remove(token_id);
        }

        // 移除 token_id -> next_approval_id 映射
        if let Some(next_approval_id_by_id) = &mut self.tokens.next_approval_id_by_id {
            next_approval_id_by_id.remove(token_id);
        }

        // 打印标准 log
        NftBurn {
            owner_id: account_id,
            token_ids: &[token_id],
            authorized_id: Some(account_id),
            memo: memo.as_deref(),
        }
        .emit();
    }

    // 内部的mint方法
    pub(crate) fn internal_mint(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
        metadata: &TokenMetadata,
        memo: Option<String>,
    ) {
        // 添加 token_id -> token_owner_id 映射
        self.tokens.owner_by_id.insert(token_id, account_id);
        // 更新或添加 token_owner_id -> token_ids 映射
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(account_id).unwrap_or_else(|| {
                UnorderedSet::new(
                    near_contract_standards::non_fungible_token::core::StorageKey::TokensPerOwner {
                        account_hash: env::sha256(&account_id.try_to_vec().unwrap()), // 也可以用 `account_id.as_bytes()`, 但使用 borsh 字节更加通用
                    },
                )
            });
            token_ids.insert(token_id);
            tokens_per_owner.insert(account_id, &token_ids);
        }

        // 添加 token_id -> token_metadata 映射
        if let Some(token_metadata_by_id) = &mut self.tokens.token_metadata_by_id {
            token_metadata_by_id.insert(token_id, metadata);
        }

        // 打印标准 log
        NftMint {
            owner_id: account_id,
            token_ids: &[token_id],
            memo: memo.as_deref(),
        }
        .emit();
    }
}

// 为合约实现NRP171
impl_non_fungible_token_core!(Contract, tokens);
// 为合约实现NEP178
impl_non_fungible_token_approval!(Contract, tokens);
//为合约实现NEP181
impl_non_fungible_token_enumeration!(Contract, tokens);

// 为合约实现 NEP177
#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "BoomBlastV1".to_string(),
            symbol: "BoomBlastNFT".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        }
    }
}
