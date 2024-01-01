use near_contract_standards::non_fungible_token::events::{NftBurn, NftMint};
use near_contract_standards::{
    impl_non_fungible_token_approval, impl_non_fungible_token_core,
    impl_non_fungible_token_enumeration,
};
use near_sdk::collections::UnorderedSet;

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
    // 存储所有的卡牌
    tokens: NonFungibleToken,
    // 使用全局自增 id 作为 NFT id
    unique_id: u64,
    // 存储卡片id
    card_id: u64,
    // 存储
    cards: LookupMap<u64, Card>,
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Card {
    pub card_id: u64,
    pub card_name: String,
    pub card_description: String,
    pub card_image: String,
    pub card_type: String,
    pub card_action: String,
    pub card_status: CardStatus,
}
/// 卡片状态
#[derive(BorshSerialize, BorshStorageKey, BorshDeserialize)]
pub enum CardStatus {
    // 禁用
    Disabled,
    // 可使用
    Available,
}
// 存储键
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Approval,
    Enumeration,
    TokenMetadata,
    NonFungibleToken,
    // Secret,
    Card,
}

#[near_bindgen]
impl Contract {
    // 初始化合约
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        let mut contract = Self {
            owner_id: owner_id.clone(),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            unique_id: 0,
            card_id: 1,
            cards: LookupMap::new(StorageKey::Card),
        };
        return contract;
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

    // 只有合约所有者可以新增卡片类型
    pub fn add_card(
        &mut self,
        card_name: String,
        card_description: String,
        card_image: String,
        card_type: String,
        card_action: String,
    ) {
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
        );
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

    // user mint NFT
    pub fn mint(
        &mut self,
        account_id: AccountId,
        metadata: TokenMetadata,
        memo: Option<String>,
        // secret: String,
    ) {
        let token_id = self.next_id().to_string();
        // self.internal_mint(&account_id, &token_id, &metadata, memo, secret);
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
impl Contract {
    pub(crate) fn next_id(&mut self) -> u64 {
        self.unique_id += 1;
        self.unique_id
    }

    fn init_card(&mut self) {
        // 示例用法
        add_card!(self,"ExplodingKitten","Exploding Kitten(爆炸猫，需要打出拆除卡，否则出局)","image","type",
            "
            |game| {
            };
            "
        );
        add_card!(self, "Defuse", "Defuse(拆除卡)", "image", "type",
            "
            |game| {
            };
            "
        );

        add_card!(self, "Catermelon", "Catermelon(普通西瓜卡)", "image", "type",
            "
            |game| {
            };
            "
        );
        add_card!(self, "HairyPotatoCat", "Hairy Potato Cat(普通土豆猫卡)", "", "",
            "
            |game| {
            };
            "
        );

        add_card!(self, "RainbowCat", "Rainbow Cat(普通彩虹猫卡)", "", "",
            "
            |game| {
            };
            "
        );

        add_card!(self, "BeardCat", "Beard Cat(普通胡子猫卡)", "", "",
            "
            |game| {
            };
            "
        );

        add_card!(self, "Tacocat", "Tacocat(普通墨西哥卷饼猫卡)", "", "",
            "
            |game| {
            };
            "
        );

        add_card!(self, "Favor", "Favor(不知道啥用)", "", "",
            "
            |game| {
            };
            "
        );

        add_card!(self, "Skip", "Skip(跳过卡)", "", "",
            "
            |game| {
                game.cards_to_draw -= 1;
            };
            "
        );

        add_card!(self, "Attack", "Attack(攻击卡，下一位玩家需要多摸两张牌)", "", "",
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

        add_card!(self, "SeeTheFuture", "See The Future(看三张牌)", "", "",
            "
            |game| {
                game.show(game.deck.top_three());
            };
            "
        );

        add_card!(self, "Nope", "Nope(否定卡,阻止其他玩家的行动)", "", "",
            "
            |game| {
            };
            "
        );

        add_card!(self, "Shuffle", "Shuffle(洗牌卡，重新洗牌)", "", "",
            "
            |game| {
                game.deck.shuffle_deck();
            };
            "
        );


    }

    // 只有合约所有者可以新增卡片类型
    pub(crate) fn internal_add_card(
        &mut self,
        card_name: String,
        card_description: String,
        card_image: String,
        card_type: String,
        card_action: String,
    ) {
        let card = Card {
            card_id: self.card_id,
            card_name,
            card_description,
            card_image,
            card_type,
            card_action,
            card_status: CardStatus::Available,
        };
        self.cards.insert(self.card_id, card);
        self.card_id += 1;
    }

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

    pub(crate) fn internal_mint(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
        metadata: &TokenMetadata,
        memo: Option<String>,
        // secret: String,
    ) {
        // 添加 token_id -> token_owner_id 映射
        self.tokens.owner_by_id.insert(token_id, account_id);
        // 添加 secret
        // self.set_account_description(token_id.clone(), secret);
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
