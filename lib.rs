use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{
    initialize_mint, mint_to, transfer, InitializeMint, Mint, MintTo, Token, TokenAccount, Transfer,
};
use anchor_spl::metadata::{
    create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata,
    mpl_token_metadata::types::DataV2,
};

// 常量定义
mod constants {
    pub const CONFIG_SEED: &str = "scenic_config";
    pub const USER_SEED: &str = "user_account";
    pub const USER_COUPON_SEED: &str = "user_coupon";
    pub const SCENIC_SPOT_SEED: &str = "scenic_spot";
    pub const SCENIC_DETAIL_SEED: &str = "scenic_detail";
    pub const REVIEW_SEED: &str = "review";
    pub const COUPON_TEMPLATE_SEED: &str = "coupon_template";
    pub const NFT_SEED: &str = "nft_mint_seed";
    pub const NFT_AUTHORITY_SEED: &str = "nft_authority_seed";
    pub const SCENIC_ID_REGISTRY_SEED: &str = "scenic_id_registry";
    pub const VAULT_AUTHORITY_SEED: &str = "vault-authority";

    pub const INITIAL_LEVEL: u8 = 1;
    pub const INITIAL_REWARD: u64 = 5 * 1_000_000_000;
    pub const LEVEL_2_UPGRADE_COST: u64 = 20 * 1_000_000_000;
    pub const LEVEL_3_UPGRADE_COST: u64 = 30 * 1_000_000_000;
    pub const LEVEL_1_TO_3_UPGRADE_COST: u64 = 40 * 1_000_000_000;

    pub const SCENIC_SPOT_INIT_ID: u64 = 1;
    pub const MAX_SCENIC_NAME_LENGTH: usize = 50;
    pub const MAX_SCENIC_SPOTS: usize = 10;
    pub const MAX_TAGS_LENGTH: usize = 100;
    pub const MAX_REVIEW_CONTENT_LENGTH: usize = 2000;
    pub const MAX_DESCRIPTION_LENGTH: usize = 100;

    pub const _DEFAULT_COUPON_TOKEN_PRICE: u64 = 10 * 1_000_000_000;
    pub const _DEFAULT_COUPON_TOTAL_SUPPLY: u32 = 100;
    pub const TOKEN_DECIMALS: u8 = 9;
    // 默认风景图片URI
    pub const DEFAULT_SCENIC_IMAGE_URI: &str = "https://coffee-official-centipede-389.mypinata.cloud/ipfs/bafybeia2urvtslyvd3562oxwqowk5jt2yavdcvwrqyip5menfzsm4gmgxy";
}

#[error_code]
pub enum ScenicReviewError {
    #[msg("不是管理员，无权执行此操作")]
    NotAdmin,
    #[msg("无效的Oracle签名")]
    InvalidOracle,
    #[msg("用户账户不存在")]
    UserAccountNotFound,
    #[msg("景点不存在")]
    ScenicSpotNotFound,
    #[msg("评价不存在")]
    ReviewNotFound,
    #[msg("优惠券模板不存在")]
    CouponTemplateNotFound,
    #[msg("余额不足，无法完成操作")]
    InsufficientBalance,
    #[msg("优惠券已售罄")]
    CouponSoldOut,
    #[msg("优惠券已过期")]
    CouponExpired,
    #[msg("不能重复评价同一景点")]
    DuplicateReview,
    #[msg("Bump值未找到")]
    BumpNotFound,
    #[msg("无效的优惠券ID")]
    InvalidCouponId,
    #[msg("优惠券已使用")]
    CouponAlreadyUsed,
    #[msg("无效的升级路径")]
    InvalidUpgradePath,
    #[msg("升级资金不足")]
    InsufficientFundsForUpgrade,
    #[msg("未授权更新AI总结")]
    NotAuthorizedForAiUpdate,
    #[msg("AI总结过长")]
    AiSummaryTooLong,
    #[msg("景点名称已存在")]
    ScenicSpotNameDuplicate,
    #[msg("景点名称过长（最大50字符）")]
    ScenicSpotNameTooLong,
    #[msg("无效的景点ID")]
    InvalidScenicSpotId,
    #[msg("景点数量已达上限")]
    ScenicSpotLimitReached,
    #[msg("未找到景点最新详情")]
    ScenicDetailNotFound,
    #[msg("景点标签过长（最大100字符）")]
    ScenicSpotTagsTooLong,
    #[msg("景点描述过长（最大50字符）")]
    ScenicSpotDescriptionTooLong,
    #[msg("无效的代币Mint地址")]
    InvalidTokenMint,
    #[msg("代币Mint已初始化")]
    TokenMintAlreadyInitialized,
    #[msg("评价已处理，无法重复操作")]
    ReviewAlreadyProcessed,
    #[msg("评价状态无效")]
    InvalidReviewStatus,
    #[msg("评价待审核状态才能处理")]
    ReviewNotPending,
    #[msg("无效的用户地址")]
    InvalidUser,
    #[msg("评价内容过长")]
    ReviewContentTooLong,
    #[msg("默认景点已创建，不可重复创建")]
    DefaultSpotsAlreadyCreated,
    #[msg("默认优惠券已创建，不可重复创建")]
    DefaultCouponsAlreadyCreated,
    #[msg("评分无效（必须是1.0-5.0之间的小数）")]
    InvalidRating,
    #[msg("评分最多保留1位小数")]
    InvalidRatingPrecision,
    #[msg("优惠券不存在")]
    CouponNotFound,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct NameToId {
    pub name: String,
    pub id: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct IdToName {
    pub id: u64,
    pub name: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ExtraScenicData {
    pub id: u64,
    pub location: String,
    pub tags: String,
    pub rating: f64,
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ScenicAcceptedReviews {
    pub scenic_id: u64,
    pub count: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ScenicSpotInfo {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub tags: String,
    pub rating: f64,
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct CouponTemplateInfo {
    pub coupon_id: String,
    pub name: String,
    pub description: String,
    pub token_price: u64,
    pub total_supply: u32,
    pub remaining: u32,
    pub expire_date: i64, // 销售截止时间
    pub valid_duration: i64, // 有效时长（秒）
    pub is_expired: bool,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserCouponInfo {
    pub coupon_id: String,
    pub user_pda: Pubkey,
    pub purchase_id: u64,
    pub is_used: bool,
    pub obtain_time: i64,
    pub expire_time: i64, // 过期时间
    pub redeem_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UserReviewItem {
    pub review_pda: Pubkey,
    pub scenic_id: u64,
    pub review_id: u64, // 新增：评价ID
    pub scenic_name: String,
    pub content: String,
    // 改为 f32 类型
    pub rating: f32,
    pub status: ReviewStatus,
    pub reward_amount: u64,
    pub submit_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ScenicReviewStats {
    pub scenic_id: u64,
    pub scenic_name: String,
    pub total_reviews: u32,
    pub accepted_reviews: u32,
    pub average_rating: f64,
    pub pending_reviews: u32,
    pub rejected_reviews: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ReviewStatus {
    #[default]
    Pending,
    Accepted,
    Rejected,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TxType {
    #[default]
    Unknown,
    Expense,
    Income,
}

pub fn account_exists(info: &AccountInfo) -> bool {
    info.lamports() > 0 && !info.data_is_empty()
}

#[event]
pub struct ReviewSubmitted {
    pub user_pda: Pubkey,
    pub scenic_pda: Pubkey,
    pub review_pda: Pubkey,
    pub scenic_id: u64,
    pub review_id: u64, // 新增：评价ID
    pub submit_time: i64,
}

#[event]
pub struct UserLevelUpgraded {
    pub user: Pubkey,
    pub previous_level: u8,
    pub new_level: u8,
    pub tokens_spent: u64,
    pub timestamp: i64,
}



#[event]
pub struct CouponRedeemed {
    pub user: Pubkey,
    pub coupon_id: String,
    pub nft_mint: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AiSummaryUpdated {
    pub scenic_spot_id: Pubkey,
    pub summary_length: u32,
    pub update_slot: u64,
    pub timestamp: i64,
}

#[event]
pub struct CouponTemplateCreated {
    pub coupon_id: String,
    pub name: String,
    pub admin: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ReviewBatchReady {
    pub scenic_pda: Pubkey,
    pub scenic_id: u64,
    pub batch_size: u32,
    pub total_reviews: u32,
    pub trigger_time: i64,
}

#[event]
pub struct ReviewConfirmed {
    pub review_pda: Pubkey,
    pub review_id: u64, // 新增
    pub status: ReviewStatus,
    pub reward_amount: u64,
    pub confirm_time: i64,
}

#[event]
pub struct DefaultScenicSpotsCreated {
    pub admin: Pubkey,
    pub count: u8,
    pub timestamp: i64,
}

#[event]
pub struct DefaultCouponsCreated {
    pub admin: Pubkey,
    pub count: u8,
    pub timestamp: i64,
}

#[account]
#[derive(Default)]
pub struct Config {
    pub admin: Pubkey,
    pub token_mint: Pubkey,
    pub treasury: Pubkey,
    pub oracle: Pubkey,
    pub vault_authority: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    pub default_spots_created: bool,
    pub default_coupons_created: bool,
}

impl Config {
    pub const INIT_SPACE: usize = 32 + 32 + 32 + 32 + 32 + 1 + 1 + 1 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ScenicVersionRecord {
    pub scenic_id: u64,
    pub latest_version: u32,
}

#[account]
#[derive(Default)]
pub struct ScenicIdRegistry {
    pub bump: u8,
    pub next_id: u64,
    pub total_count: u8,
    pub name_to_id: Vec<NameToId>,
    pub id_to_name: Vec<IdToName>,
    pub extra_scenic_data: Vec<ExtraScenicData>,
    pub scenic_accepted_reviews: Vec<ScenicAcceptedReviews>,
    pub scenic_latest_detail_version: Vec<ScenicVersionRecord>,
}

impl ScenicIdRegistry {
    pub const INIT_SPACE: usize = 8
        + 1
        + 8
        + 1
        + (4 + (constants::MAX_SCENIC_NAME_LENGTH + 8) * constants::MAX_SCENIC_SPOTS)
        + (4 + (8 + constants::MAX_SCENIC_NAME_LENGTH) * constants::MAX_SCENIC_SPOTS)
        + (4 + (8 + 100 + 100 + 8 + 50) * constants::MAX_SCENIC_SPOTS)
        + (4 + (8 + 4) * constants::MAX_SCENIC_SPOTS)
        + (4 + (8 + 4) * constants::MAX_SCENIC_SPOTS);
}

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub user_wallet: Pubkey,
    pub level: u8,
    pub reward_per_review: u64,
    pub total_reviews: u32,
    pub accepted_reviews: u32,
    pub created_at: i64,
    pub coupons: Vec<Pubkey>,
}

impl UserAccount {
    pub const SPACE: usize = 32 + 1 + 8 + 4 + 4 + 8 + (4 + 32 * 100);
}

#[account]
#[derive(Default)]
pub struct ScenicSpot {
    pub id: u64,
    pub admin: Pubkey,
    pub name: String,
    pub location: String,
    pub tags: String,
    pub base_rating: f64,
    pub created_at: i64,
    pub is_active: bool,
    pub accepted_review_count: u32,
    pub description: String,
}

impl ScenicSpot {
    pub fn space(name_len: usize, location_len: usize, tags_len: usize) -> usize {
        8 + 8 + 32 + 4 + name_len + 4 + location_len + 4 + tags_len + 8 + 8 + 1 + 4
    }
}

#[account]
#[derive(Default)]
pub struct ScenicDetail {
    pub scenic_spot_id: u64,
    pub scenic_spot_pda: Pubkey,
    pub version: u32,
    pub ai_summary: String,
    pub hash: String,
    pub update_time: i64,
    pub oracle_signature: Pubkey,
    pub source_review_ids: Vec<Pubkey>,
}

impl ScenicDetail {
    pub fn space(summary_len: usize, hash_len: usize) -> usize {
        8 + 8 + 32 + 4 + 4 + summary_len + 4 + hash_len + 8 + 32 + 4 + (32 * 20)
    }
}

#[account]
#[derive(Default)]
pub struct Review {
    pub user_pda: Pubkey,
    pub user_wallet: Pubkey, // 新增：存储用户钱包地址，方便前端查询
    pub scenic_spot_id: u64,
    pub review_id: u64, // 新增：评价ID
    pub scenic_pda: Pubkey,
    pub content: String,
    // 改为 f32 类型，支持小数评分
    pub rating: f32,
    pub status: ReviewStatus,
    pub reward_amount: u64,
    pub submit_time: i64,
    pub process_time: i64,
    pub oracle_signature: Pubkey,
}

impl Review {
    // 更新空间计算：增加 user_wallet (32字节)
    pub fn space(content_len: usize) -> usize {
        32 + 32 + 8 + 8 + 32 + 4 + content_len + 4 + 1 + 8 + 8 + 8 + 32
    }
}

#[account]
#[derive(Default)]
pub struct CouponTemplate {
    pub admin: Pubkey,
    pub coupon_id: String,
    pub name: String,
    pub description: String,
    pub token_price: u64,
    pub total_supply: u32,
    pub remaining: u32,
    pub expire_date: i64, // 销售截止时间
    pub valid_duration: i64, // 有效时长（秒）
}

impl CouponTemplate {
    pub fn space(id_len: usize, name_len: usize, desc_len: usize) -> usize {
        32 + 4 + id_len + 4 + name_len + 4 + desc_len + 8 + 4 + 4 + 8 + 8
    }
}

#[account]
#[derive(Default)]
pub struct UserCoupon {
    pub coupon_id: String,
    pub user_pda: Pubkey,
    pub purchase_id: u64,
    pub is_used: bool,
    pub obtain_time: i64,
    pub expire_time: i64, // 过期时间
    pub redeem_time: i64,
}

impl UserCoupon {
    // 新增：计算账户空间
    pub fn space(coupon_id_len: usize) -> usize {
        8 // 账户判别符
        + (4 + coupon_id_len) // coupon_id
        + 32 // user_pda
        + 8 // purchase_id
        + 1 // is_used
        + 8 // obtain_time
        + 8 // expire_time
        + 8 // redeem_time
    }

    pub fn try_from_account_info(info: &AccountInfo) -> Result<Self> {
        let data = info.try_borrow_data()?;
        Ok(Self::try_deserialize(&mut &data[8..])?)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct TransactionRecordItem {
    pub tx_pda: Pubkey,
    pub tx_type: u8,
    pub amount: u64,
    pub related_id: String,
    pub timestamp: i64,
    pub tx_hash: String,
    pub user_pda: Pubkey,
}

#[account]
#[derive(Default)]
pub struct TransactionRecord {
    pub user_pda: Pubkey,
    pub tx_type: TxType,
    pub tx_name: String,
    pub amount: u64,
    pub related_id: String,
    pub timestamp: i64,
    pub tx_hash: String,
}

impl TransactionRecord {
    pub const SPACE: usize = 8 // 账户判别符
        + 32 // user_pda
        + 1 // tx_type
        + (4 + 100) // tx_name（4字节长度+100字符）
        + 8 // amount
        + (4 + 200) // related_id（4字节长度+200字符）
        + 8 // timestamp
        + (4 + 100); // tx_hash（4字节长度+100字符）

    pub fn try_from_account_info(info: &AccountInfo) -> Result<Self> {
        let data = info.try_borrow_data()?;
        Ok(Self::try_deserialize(&mut &data[..])?)
    }
}

#[account]
#[derive(Default)]
pub struct UserReviewIndex {
    pub user_pda: Pubkey,
    pub review_pdas: Vec<Pubkey>,
    pub bump: u8,
}

impl UserReviewIndex {
    pub const INIT_SPACE: usize = 8 + 32 + 4 + (32 * 50) + 1;
}

#[account]
#[derive(Default)]
pub struct ScenicReviewSummary {
    pub scenic_spot_id: Pubkey,
    pub average_rating: f32,
    pub review_count: u32,
    pub five_star_count: u32,
    pub four_star_count: u32,
    pub three_star_count: u32,
    pub two_star_count: u32,
    pub one_star_count: u32,
    pub ai_summary: String,
    pub last_ai_update_slot: u64,
    pub bump: u8,
}

impl ScenicReviewSummary {
    // 修改：使用 IPFS 方案，只需要存储 CID 或 URL，因此大幅减小长度限制
    // 原来是 2000，现在改为 256 (足够存储 ipfs://Qm... 或 https://...)
    pub const MAX_AI_SUMMARY_LENGTH: usize = 256;

    pub fn space() -> usize {
        8 + 32 + 4 + 4 * 5 + 4 + Self::MAX_AI_SUMMARY_LENGTH + 8 + 1
    }

    pub fn update_rating(&mut self, rating: u8) {
        self.review_count += 1;
        match rating {
            5 => self.five_star_count += 1,
            4 => self.four_star_count += 1,
            3 => self.three_star_count += 1,
            2 => self.two_star_count += 1,
            1 => self.one_star_count += 1,
            _ => {}
        }
        let total_rating = (self.five_star_count * 5
            + self.four_star_count * 4
            + self.three_star_count * 3
            + self.two_star_count * 2
            + self.one_star_count * 1) as f32;
        self.average_rating = total_rating / self.review_count as f32;
    }
}

declare_id!("2QWH1NEeJyo8RB4hGymgsj9jwKNCjumKr46yim7bhk9x");

#[cfg(not(feature = "no-entrypoint"))]
pub mod mpl_token_metadata {
    use anchor_lang::declare_id;
    declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
}

#[program]
pub mod scenic_review_system {
    use super::*;

    pub fn initialize_all(ctx: Context<InitializeAll>, token_mint: Pubkey) -> Result<()> {
        // 校验传入的代币地址和上下文的 token_mint 一致（防止传错）
        require!(
            ctx.accounts.token_mint.key() == token_mint,
            ScenicReviewError::InvalidTokenMint
        );

        let config = &mut ctx.accounts.config;
        config.admin = ctx.accounts.admin.key();
        config.bump = ctx.bumps.config;
        config.vault_bump = ctx.bumps.vault_authority;
        config.token_mint = token_mint; // 赋值为前端传入的代币地址
        config.treasury = ctx.accounts.treasury.key();
        config.oracle = Pubkey::default();
        config.vault_authority = ctx.accounts.vault_authority.key();
        config.default_spots_created = false;
        config.default_coupons_created = false;

        let registry = &mut ctx.accounts.id_registry;
        registry.bump = ctx.bumps.id_registry;
        registry.next_id = constants::SCENIC_SPOT_INIT_ID;
        registry.total_count = 0;
        registry.name_to_id = Vec::new();
        registry.id_to_name = Vec::new();
        registry.extra_scenic_data = Vec::new();
        registry.scenic_accepted_reviews = Vec::new();

        Ok(())
    }

    pub fn create_default_coupons(ctx: Context<CreateDefaultCoupons>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let clock = Clock::get()?;

        require!(
            !config.default_coupons_created,
            ScenicReviewError::DefaultCouponsAlreadyCreated
        );

        let admin_key = ctx.accounts.admin.key();
        let expire_date = clock.unix_timestamp + 30 * 86400;
        let valid_duration = 30 * 86400; // 有效期30天

        let coupon_001 = &mut ctx.accounts.coupon_template_001;
        coupon_001.admin = admin_key;
        coupon_001.coupon_id = "COUPON_001".to_string();
        coupon_001.name = "黄山门票8折券".to_string();
        coupon_001.description = "黄山景区门票8折优惠".to_string();
        coupon_001.token_price = 10 * 1_000_000_000;
        coupon_001.total_supply = 100;
        coupon_001.remaining = 100;
        coupon_001.expire_date = expire_date;
        coupon_001.valid_duration = valid_duration;
        emit!(CouponTemplateCreated {
            coupon_id: coupon_001.coupon_id.clone(),
            name: coupon_001.name.clone(),
            admin: admin_key,
            timestamp: clock.unix_timestamp,
        });

        let coupon_002 = &mut ctx.accounts.coupon_template_002;
        coupon_002.admin = admin_key;
        coupon_002.coupon_id = "COUPON_002".to_string();
        coupon_002.name = "泰山导览免费券".to_string();
        coupon_002.description =
            "泰山景区专业导览服务免费".to_string();
        coupon_002.token_price = 10 * 1_000_000_000;
        coupon_002.total_supply = 50;
        coupon_002.remaining = 50;
        coupon_002.expire_date = expire_date;
        coupon_002.valid_duration = valid_duration;
        emit!(CouponTemplateCreated {
            coupon_id: coupon_002.coupon_id.clone(),
            name: coupon_002.name.clone(),
            admin: admin_key,
            timestamp: clock.unix_timestamp,
        });

        let coupon_003 = &mut ctx.accounts.coupon_template_003;
        coupon_003.admin = admin_key;
        coupon_003.coupon_id = "COUPON_003".to_string();
        coupon_003.name = "桂林游船折扣券".to_string();
        coupon_003.description = "桂林漓江游船9折优惠".to_string();
        coupon_003.token_price = 10 * 1_000_000_000;
        coupon_003.total_supply = 200;
        coupon_003.remaining = 200;
        coupon_003.expire_date = expire_date;
        coupon_003.valid_duration = valid_duration;
        emit!(CouponTemplateCreated {
            coupon_id: coupon_003.coupon_id.clone(),
            name: coupon_003.name.clone(),
            admin: admin_key,
            timestamp: clock.unix_timestamp,
        });

        let coupon_004 = &mut ctx.accounts.coupon_template_004;
        coupon_004.admin = admin_key;
        coupon_004.coupon_id = "COUPON_004".to_string();
        coupon_004.name = "故宫文创满减券".to_string();
        coupon_004.description = "故宫文创产品满100减20".to_string();
        coupon_004.token_price = 20 * 1_000_000_000;
        coupon_004.total_supply = 80;
        coupon_004.remaining = 80;
        coupon_004.expire_date = expire_date;
        coupon_004.valid_duration = valid_duration;
        emit!(CouponTemplateCreated {
            coupon_id: coupon_004.coupon_id.clone(),
            name: coupon_004.name.clone(),
            admin: admin_key,
            timestamp: clock.unix_timestamp,
        });

        config.default_coupons_created = true;

        emit!(DefaultCouponsCreated {
            admin: ctx.accounts.admin.key(),
            count: 4,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    pub fn create_default_scenic_spots(ctx: Context<CreateDefaultScenicSpots>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let registry = &mut ctx.accounts.id_registry;
        let clock = Clock::get()?;

        require!(
            !config.default_spots_created,
            ScenicReviewError::DefaultSpotsAlreadyCreated
        );

        let default_spots = vec![
            (
                1u64, // id
                "Eiffel Tower",
                "Paris, France",
                "Iconic, Night View",
                0.0,
                "Symbol of with panoramic city views",
            ),
            (
                2u64,
                "Huangshan",
                "Anhui, China",
                "Sea of,Sunrise,Plank Roads",
                0.0,
                "Breathtaking with magnificent sea of clouds",
            ),
            (
                3u64,
                "Taj Mahal",
                "Agra, India",
                "White-Marble,Symbol of Love",
                0.0,
                "architectural gem built marble for love",
            ),
            (
                4u64,
                "Sydney Opera House",
                "Sydney, Australia",
                "Shell-Shaped,Coastal Scenery",
                0.0,
                "UNESCO site with sail-like and coastal views",
            ),
            (
                5u64,
                "Statue of Liberty",
                "New York, USA",
                "Symbol of,New York Harbor",
                0.0,
                "Icon of freedom by France to the United States",
            ),
            (
                6u64,
                "Mount Fuji",
                "Honshu, Japan",
                "Cherry,Snow-Capped Peak",
                0.0,
                "Japan's highest with iconic snow-capped summit",
            ),
            (
                7u64,
                "Santorini",
                "Greece",
                "Blue-white,Aegean Sea",
                0.0,
                "Aegean with blue-white buildings and  views",
            ),
            (
                8u64,
                "Giza Pyramids",
                "Cairo, Egypt",
                "Ancient Wonder,Tombs",
                0.0,
                "Ancient Egyptian , one of the Seven Wonders",
            ),
        ];

        let mut spots_accounts = vec![
            &mut ctx.accounts.scenic_spot_1,
            &mut ctx.accounts.scenic_spot_2,
            &mut ctx.accounts.scenic_spot_3,
            &mut ctx.accounts.scenic_spot_4,
            &mut ctx.accounts.scenic_spot_5,
            &mut ctx.accounts.scenic_spot_6,
            &mut ctx.accounts.scenic_spot_7,
            &mut ctx.accounts.scenic_spot_8,
        ];

        let mut current_id = registry.next_id;
        let mut created_count: u64 = 0;

        for (i, (id, name, location, tags, rating, description)) in default_spots.iter().enumerate() {
            require!(
                name.len() <= constants::MAX_SCENIC_NAME_LENGTH,
                ScenicReviewError::ScenicSpotNameTooLong
            );
            require!(
                tags.len() <= constants::MAX_TAGS_LENGTH,
                ScenicReviewError::ScenicSpotTagsTooLong
            );
            require!(
                description.len() <= constants::MAX_DESCRIPTION_LENGTH,
                ScenicReviewError::ScenicSpotDescriptionTooLong
            );

            let name_exists = registry.name_to_id.iter().any(|item| item.name == *name);
            if name_exists {
                msg!("跳过重复景点：{}", name);
                continue;
            }

            // 更新注册表
            registry.name_to_id.push(NameToId {
                name: name.to_string(),
                id: *id,
            });
            registry.id_to_name.push(IdToName {
                id: *id,
                name: name.to_string(),
            });
            registry.extra_scenic_data.push(ExtraScenicData {
                id: *id,
                location: location.to_string(),
                tags: tags.to_string(),
                rating: *rating,
                description: description.to_string(),
            });

            // 初始化具体的景点账户
            let spot = &mut spots_accounts[i];
            spot.id = *id;
            spot.admin = ctx.accounts.admin.key();
            spot.name = name.to_string();
            spot.location = location.to_string();
            spot.tags = tags.to_string();
            spot.base_rating = *rating;
            spot.created_at = clock.unix_timestamp;
            spot.is_active = true;
            spot.accepted_review_count = 0;
            spot.description = description.to_string();

            current_id = id + 1;
            created_count += 1;
        }

        registry.next_id = current_id;
        registry.total_count += created_count as u8;
        config.default_spots_created = true;

        emit!(DefaultScenicSpotsCreated {
            admin: ctx.accounts.admin.key(),
            count: created_count as u8,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    pub fn create_scenic_spot(
        ctx: Context<CreateScenicSpot>,
        name: String,
        location: String,
        rating: f64,
        description: String,
        tags: String,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.id_registry;
        let clock = Clock::get()?;

        // 1. 检查名称是否过长
        require!(
            name.len() <= constants::MAX_SCENIC_NAME_LENGTH,
            ScenicReviewError::ScenicSpotNameTooLong
        );
        require!(
            description.len() <= constants::MAX_DESCRIPTION_LENGTH,
            ScenicReviewError::ScenicSpotDescriptionTooLong
        );
        require!(
            tags.len() <= constants::MAX_TAGS_LENGTH,
            ScenicReviewError::ScenicSpotTagsTooLong
        );

        // 2. 检查名称是否重复
        if registry.name_to_id.iter().any(|item| item.name == name) {
            return err!(ScenicReviewError::ScenicSpotNameDuplicate);
        }

        // 3. 检查景点数量是否达到上限
        require!(
            registry.total_count < (constants::MAX_SCENIC_SPOTS as u8),
            ScenicReviewError::ScenicSpotLimitReached
        );

        // 4. 分配新ID
        let new_id = registry.next_id;
        registry.next_id += 1;
        registry.total_count += 1;

        // 5. 更新注册表
        registry.name_to_id.push(NameToId {
            name: name.clone(),
            id: new_id,
        });
        registry.id_to_name.push(IdToName {
            id: new_id,
            name: name.clone(),
        });
        registry.extra_scenic_data.push(ExtraScenicData {
            id: new_id,
            location: location.clone(),
            tags: tags.clone(),
            rating,
            description: description.clone(),
        });
        registry.scenic_accepted_reviews.push(ScenicAcceptedReviews {
            scenic_id: new_id,
            count: 0,
        });

        // 6. 初始化景点账户
        let scenic_spot = &mut ctx.accounts.scenic_spot;
        scenic_spot.id = new_id;
        scenic_spot.admin = ctx.accounts.admin.key();
        scenic_spot.name = name;
        scenic_spot.location = location;
        scenic_spot.tags = tags;
        scenic_spot.base_rating = rating;
        scenic_spot.created_at = clock.unix_timestamp;
        scenic_spot.is_active = true;
        scenic_spot.accepted_review_count = 0;
        scenic_spot.description = description;

        Ok(())
    }

    pub fn update_scenic_rating(
        ctx: Context<UpdateScenicRating>,
        scenic_id: u64,
        new_rating: f64,
    ) -> Result<()> {
        require!(
            new_rating >= 0.0 && new_rating <= 5.0,
            ScenicReviewError::InvalidRating
        );
        // Verify precision (max 1 decimal place)
        let scaled = (new_rating * 10.0).round();
        require!(
            (scaled / 10.0 - new_rating).abs() < f64::EPSILON,
            ScenicReviewError::InvalidRatingPrecision
        );

        // Update ScenicSpot account
        let scenic_spot = &mut ctx.accounts.scenic_spot;
        scenic_spot.base_rating = new_rating;

        // Update Registry
        let registry = &mut ctx.accounts.id_registry;
        if let Some(extra_data) = registry.extra_scenic_data.iter_mut().find(|d| d.id == scenic_id) {
            extra_data.rating = new_rating;
        } else {
             return err!(ScenicReviewError::ScenicSpotNotFound);
        }

        Ok(())
    }

    //查询景点
    pub fn get_default_scenic_spots(
        ctx: Context<GetDefaultScenicSpots>,
    ) -> Result<Vec<ScenicSpotInfo>> {
        // 手动反序列化 UncheckedAccount 为 ScenicIdRegistry（只读）
        let registry_data = ctx.accounts.id_registry.try_borrow_data()?;
        let registry = ScenicIdRegistry::try_deserialize(&mut &registry_data[..])?;

        // 2. 原有查询逻辑（无修改）
        let mut default_spots = Vec::new();
        for extra_data in &registry.extra_scenic_data {
            let name = registry
                .id_to_name
                .iter()
                .find(|item| item.id == extra_data.id)
                .map(|item| item.name.clone())
                .unwrap_or_default();

            default_spots.push(ScenicSpotInfo {
                id: extra_data.id,
                name,
                location: extra_data.location.clone(),
                tags: extra_data.tags.clone(),
                rating: extra_data.rating,
                description: extra_data.description.clone(),
            });
        }

        Ok(default_spots)
    }

    // init_scenic_spot_account 已移除，功能整合进 create_default_scenic_spots


    pub fn set_oracle(ctx: Context<SetOracle>, oracle: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.oracle = oracle;
        Ok(())
    }
    //创建用户账户
    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.user_wallet = ctx.accounts.user.key();
        user_account.level = constants::INITIAL_LEVEL;
        user_account.reward_per_review = constants::INITIAL_REWARD;
        user_account.total_reviews = 0;
        user_account.accepted_reviews = 0;
        user_account.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
    //升级
    pub fn upgrade_user_level(ctx: Context<UpgradeUserLevel>, new_level: u8) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let current_level = user_account.level;
        let clock = Clock::get()?;

        let (tokens_needed, can_upgrade) = match (current_level, new_level) {
            (1, 2) => (constants::LEVEL_2_UPGRADE_COST, true),
            (2, 3) => (constants::LEVEL_3_UPGRADE_COST as u64, true),
            (1, 3) => (constants::LEVEL_1_TO_3_UPGRADE_COST, true),
            _ => (0, false),
        };

        require!(can_upgrade, ScenicReviewError::InvalidUpgradePath);
        require!(
            ctx.accounts.user_token_account.amount >= tokens_needed,
            ScenicReviewError::InsufficientFundsForUpgrade
        );

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, tokens_needed)?;

        user_account.level = new_level;
        user_account.reward_per_review = match new_level {
            1 => constants::INITIAL_REWARD,
            2 => 10 * 1_000_000_000,
            3 => 15 * 1_000_000_000,
            _ => constants::INITIAL_REWARD,
        };

        let tx_record = &mut ctx.accounts.tx_record;
        tx_record.user_pda = ctx.accounts.user_account.key();
        tx_record.tx_type = TxType::Expense; // 升级消耗代币，属于支出
        tx_record.tx_name = format!("{}_up_{}_consume", current_level, new_level);
        tx_record.amount = tokens_needed;
        tx_record.related_id = format!("level_up_{}_{}", current_level, new_level);
        tx_record.timestamp = Clock::get()?.unix_timestamp;
        tx_record.tx_hash = ctx.accounts.user.key().to_string();

        emit!(UserLevelUpgraded {
            user: ctx.accounts.user.key(),
            previous_level: current_level,
            new_level,
            tokens_spent: tokens_needed,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // 修复旧用户数据的临时指令
    pub fn fix_user_reward_data(ctx: Context<FixUserRewardData>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        
        // 根据当前等级重置奖励金额
        user_account.reward_per_review = match user_account.level {
            1 => constants::INITIAL_REWARD,
            2 => 10 * 1_000_000_000,
            3 => 15 * 1_000_000_000,
            _ => constants::INITIAL_REWARD,
        };
        
        msg!("Fixed user reward to: {}", user_account.reward_per_review);
        Ok(())
    }

    //修改详情
    pub fn update_scenic_detail(
        ctx: Context<UpdateScenicDetail>,
        scenic_id: u64,
        version: u32,
        ai_summary: String,
        hash: String,
        rating: f64,
    ) -> Result<()> {
        require!(
            ctx.accounts.scenic_spot.id == scenic_id,
            ScenicReviewError::InvalidScenicSpotId
        );
        require!(
            ai_summary.len() <= ScenicReviewSummary::MAX_AI_SUMMARY_LENGTH,
            ScenicReviewError::AiSummaryTooLong
        );

        let scenic_detail = &mut ctx.accounts.scenic_detail;
        scenic_detail.scenic_spot_id = scenic_id;
        scenic_detail.scenic_spot_pda = ctx.accounts.scenic_spot.key();
        scenic_detail.version = version;
        scenic_detail.ai_summary = ai_summary;
        scenic_detail.hash = hash;
        scenic_detail.update_time = Clock::get()?.unix_timestamp;
        scenic_detail.oracle_signature = ctx.accounts.oracle.key();

        let mut review_ids = Vec::new();
        for acc in ctx.remaining_accounts.iter() {
            if acc.owner == &crate::ID && acc.data_len() > 0 {
                review_ids.push(*acc.key);
                if review_ids.len() >= 20 {
                    break;
                }
            }
        }
        scenic_detail.source_review_ids = review_ids;

        // 更新景点基础评分
        ctx.accounts.scenic_spot.base_rating = rating;

        // ===== 修正：更新最新版本号（核心修改）=====
        let registry = &mut ctx.accounts.id_registry; // 现在能正常访问 id_registry 了
                                                      // 查找该景点的版本记录，存在则更新，不存在则新增
        let version_entry = registry
            .scenic_latest_detail_version
            .iter_mut()
            .find(|item| item.scenic_id == scenic_id);

        if let Some(entry) = version_entry {
            entry.latest_version = version;
        } else {
            registry
                .scenic_latest_detail_version
                .push(ScenicVersionRecord {
                    scenic_id,
                    latest_version: version,
                });
        }

        // 更新 registry 中的 extra_scenic_data 评分
        if let Some(extra_data) = registry
            .extra_scenic_data
            .iter_mut()
            .find(|item| item.id == scenic_id)
        {
            extra_data.rating = rating;
        } else {
            // 如果不存在，可能需要初始化？但 usually initialization handles it. 
            // Assume it exists or we append if logic allows. 
            // For now just update if exists.
        }

        emit!(AiSummaryUpdated {
            scenic_spot_id: ctx.accounts.scenic_spot.key(),
            summary_length: scenic_detail.ai_summary.len() as u32,
            update_slot: Clock::get()?.slot,
            timestamp: scenic_detail.update_time,
        });

        Ok(())
    }

    // 新函数：只需传scenic_id，自动查最新版本
    //查询详情
    pub fn get_scenic_latest_detail(
        ctx: Context<GetScenicLatestDetail>,
        scenic_id: u64,
    ) -> Result<ScenicDetail> {
        // 核心：调用内部函数，捕获所有错误返回空数据
        match inner_get_scenic_latest_detail(ctx, scenic_id) {
            Ok(detail) => Ok(detail),
            Err(_) => {
                msg!("获取景点最新详情失败，返回空数据（scenic_id: {}）", scenic_id);
                // 自定义空数据（更友好）
                Ok(ScenicDetail {
                    scenic_spot_id: scenic_id,
                    scenic_spot_pda: Pubkey::default(),
                    version: 0,
                    ai_summary: String::new(),
                    hash: String::new(),
                    update_time: 0,
                    oracle_signature: Pubkey::default(),
                    source_review_ids: Vec::new(),
                })
            }
        }
    }

    // 修正1：添加pub(crate)，让同模块内可调用（或直接pub）
    pub(crate) fn inner_get_scenic_latest_detail(
        ctx: Context<GetScenicLatestDetail>,
        scenic_id: u64,
    ) -> Result<ScenicDetail> {
        // 手动校验 scenic_spot 是否初始化
        if !account_exists(&ctx.accounts.scenic_spot.to_account_info()) {
            msg!("景点账户未初始化（scenic_id: {}）", scenic_id);
            return err!(ScenicReviewError::ScenicSpotNotFound);
        }

        // 1. 从注册表中获取该景点的最新版本号
        let registry = &ctx.accounts.id_registry;
        let latest_version = registry
            .scenic_latest_detail_version
            .iter()
            .find(|item| item.scenic_id == scenic_id)
            .map(|item| item.latest_version)
            .ok_or(ScenicReviewError::ScenicDetailNotFound)?;

        // 2. 推导最新版本的ScenicDetail PDA
        let (scenic_detail_pda, _) = Pubkey::find_program_address(
            &[
                constants::SCENIC_DETAIL_SEED.as_bytes(),
                scenic_id.to_be_bytes().as_ref(),
                latest_version.to_be_bytes().as_ref(),
            ],
            ctx.program_id,
        );

        // 3. 手动校验 scenic_detail 是否初始化且匹配PDA
        let scenic_detail_info = &ctx.accounts.scenic_detail.to_account_info();
        if !account_exists(scenic_detail_info) {
            msg!("最新版本详情账户未初始化（scenic_id: {}, version: {}）", scenic_id, latest_version);
            return err!(ScenicReviewError::ScenicDetailNotFound);
        }

        // 修正2：去掉&，直接比较Pubkey值（核心语法修正）
        if scenic_detail_info.key() != scenic_detail_pda {
            msg!("详情账户PDA不匹配（期望: {}, 实际: {}）", scenic_detail_pda, scenic_detail_info.key());
            return err!(ScenicReviewError::ScenicDetailNotFound);
        }

        // 4. 手动反序列化 ScenicDetail（因为是 UncheckedAccount）
        let scenic_detail_data = scenic_detail_info.try_borrow_data()?;
        let scenic_detail = ScenicDetail::try_deserialize(&mut &scenic_detail_data[8..])?;

        Ok(scenic_detail)
    }

    //提交评价
    // 提交评价（rating 改为 f32 类型）
    pub fn submit_review(
        ctx: Context<SubmitReview>,
        scenic_id: u64,
        review_id: u64, // 新增：评价ID（随机数或唯一标识）
        // 改为 f32 类型
        rating: f32,
        content: String,
    ) -> Result<()> {
        // 1. 校验评分范围：1.0 ≤ rating ≤ 5.0
        require!(
            (1.0..=10.0).contains(&rating),
            ScenicReviewError::InvalidRating
        );
        require!(
            ctx.accounts.user_account.user_wallet == ctx.accounts.user.key(),
            ScenicReviewError::InvalidUser
        );
        require!(
            content.len() <= constants::MAX_REVIEW_CONTENT_LENGTH,
            ScenicReviewError::ReviewContentTooLong
        );

        let scenic_id_exists = ctx
            .accounts
            .id_registry
            .id_to_name
            .iter()
            .any(|item| item.id == scenic_id);
        require!(scenic_id_exists, ScenicReviewError::ScenicSpotNotFound);

        let review = &mut ctx.accounts.review;
        review.user_pda = ctx.accounts.user_account.key();
        review.user_wallet = ctx.accounts.user.key(); // 存储用户钱包地址
        review.scenic_spot_id = scenic_id;
        review.review_id = review_id; // 存储 review_id
        review.scenic_pda = Pubkey::default();
        review.content = content;
        // 存储小数评分
        review.rating = rating;
        review.status = ReviewStatus::default();
        review.reward_amount = 0;
        review.submit_time = Clock::get()?.unix_timestamp;
        review.process_time = 0;
        review.oracle_signature = Pubkey::default();

        ctx.accounts.user_account.total_reviews += 1;

        emit!(ReviewSubmitted {
            user_pda: ctx.accounts.user_account.key(),
            scenic_pda: Pubkey::default(),
            review_pda: review.key(),
            scenic_id,
            review_id, // 发出事件
            submit_time: review.submit_time,
        });

        Ok(())
    }
    //oracle确认评价
    pub fn oracle_confirm_review(
        ctx: Context<OracleConfirmReview>,
        scenic_id: u64,
        review_id: u64, // 新增：评价ID
        status: u8,
    ) -> Result<()> {
        let program_id = ctx.program_id;

        let review_status = match status {
            1 => ReviewStatus::Accepted,
            2 => ReviewStatus::Rejected,
            _ => return err!(ScenicReviewError::InvalidReviewStatus),
        };

        let review = &mut ctx.accounts.review;
        let user_account = &mut ctx.accounts.user_account;
        let clock = Clock::get()?;

        // ========== 新增：从 ID Registry 获取景点名称（不依赖未初始化的 ScenicSpot PDA） ==========
        // 使用单独的代码块限制借用范围，并 Clone 字符串，避免后续的可变借用冲突
        let scenic_name = {
            let registry = &ctx.accounts.id_registry;
            registry
                .id_to_name
                .iter()
                .find(|item| item.id == scenic_id)
                .map(|item| item.name.clone())
                .ok_or(ScenicReviewError::ScenicSpotNotFound)?
        };

        require!(
            review.status == ReviewStatus::Pending,
            ScenicReviewError::ReviewAlreadyProcessed
        );
        require!(
            review.scenic_spot_id == scenic_id,
            ScenicReviewError::InvalidScenicSpotId
        );

        review.status = review_status;
        review.process_time = clock.unix_timestamp;
        review.oracle_signature = ctx.accounts.oracle.key();

        if review_status == ReviewStatus::Accepted {
            // 自动修复旧数据的精度问题：如果奖励金额小于 1_000_000，说明没有乘以 10^9
            if user_account.reward_per_review < 1_000_000 {
                user_account.reward_per_review = user_account.reward_per_review.checked_mul(1_000_000_000).unwrap();
                msg!("Auto-fixed user reward precision to: {}", user_account.reward_per_review);
            }

            require!(
                ctx.accounts.treasury.amount >= user_account.reward_per_review,
                ScenicReviewError::InsufficientBalance
            );

            // 修改：ATA Authority 是 user_wallet，而不是 user_account PDA
            if !account_exists(&ctx.accounts.user_token_account.to_account_info()) {
                let cpi_ctx = CpiContext::new(
                    ctx.accounts.associated_token_program.to_account_info(),
                    anchor_spl::associated_token::Create {
                        payer: ctx.accounts.oracle.to_account_info(),
                        associated_token: ctx.accounts.user_token_account.to_account_info(),
                        authority: ctx.accounts.user_wallet.to_account_info(), // 使用 user_wallet
                        mint: ctx.accounts.token_mint.to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                        token_program: ctx.accounts.token_program.to_account_info(),
                    },
                );
                anchor_spl::associated_token::create(cpi_ctx)?;
            }

            let vault_seeds = &[
                constants::VAULT_AUTHORITY_SEED.as_bytes(),
                &[ctx.accounts.config.vault_bump],
            ];
            let signer_seeds = &[&vault_seeds[..]];

            let cpi_context = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
                signer_seeds,
            );

            transfer(cpi_context, user_account.reward_per_review)?;

            let registry = &mut ctx.accounts.id_registry;
            if let Some(item) = registry
                .scenic_accepted_reviews
                .iter_mut()
                .find(|item| item.scenic_id == scenic_id)
            {
                item.count += 1;
                if item.count % 20 == 0 {
                    let (scenic_pda, _) = Pubkey::find_program_address(
                        &[
                            constants::SCENIC_SPOT_SEED.as_bytes(),
                            &scenic_id.to_be_bytes(),
                        ],
                        ctx.program_id,
                    );
                    emit!(ReviewBatchReady {
                        scenic_pda,
                        scenic_id,
                        batch_size: 20,
                        total_reviews: item.count,
                        trigger_time: clock.unix_timestamp,
                    });
                }
            } else {
                registry
                    .scenic_accepted_reviews
                    .push(ScenicAcceptedReviews {
                        scenic_id,
                        count: 1,
                    });
            }

            user_account.accepted_reviews += 1;
            review.reward_amount = user_account.reward_per_review;

            let tx_record = &mut ctx.accounts.tx_record;
            // 移除手动 PDA 检查，因为 Context 中已经通过 seeds 进行了初始化和检查
            // 且 Context 中的 seeds 与此处手动计算的 seeds 不一致（Context 中无 timestamp）
            // 如果需要使用 timestamp 保证唯一性，已经在 submit_review 中通过 review_id 保证了 review PDA 的唯一性
            // 而 tx_record 是与 review 一一对应的，所以也是唯一的
            
            tx_record.user_pda = user_account.key();
            tx_record.tx_type = TxType::Income;
            tx_record.amount = user_account.reward_per_review;
            tx_record.tx_name = format!("{}_adopt", scenic_name);
            // 修复后：用 to_string() 生成Base58格式的评价ID
            tx_record.related_id = format!("review_{}", review.key().to_string());
            tx_record.timestamp = clock.unix_timestamp;
            tx_record.tx_hash = ctx.accounts.oracle.key().to_string();
        }

        emit!(ReviewConfirmed {
            review_pda: review.key(),
            review_id, // 使用传入的参数
            status: review_status,
            reward_amount: review.reward_amount,
            confirm_time: clock.unix_timestamp,
        });

        Ok(())
    }


    //核销优惠券
    pub fn redeem_coupon(ctx: Context<RedeemCoupon>, coupon_id: String, purchase_id: u64) -> Result<()> {
        let user_coupon = &mut ctx.accounts.user_coupon;
        require!(!user_coupon.is_used, ScenicReviewError::CouponAlreadyUsed);
        require!(
            user_coupon.coupon_id == coupon_id,
            ScenicReviewError::InvalidCouponId
        );
        require!(
            user_coupon.purchase_id == purchase_id,
            ScenicReviewError::InvalidCouponId
        );
        // 新增：检查是否过期
        require!(
            Clock::get()?.unix_timestamp <= user_coupon.expire_time,
            ScenicReviewError::CouponExpired
        );

        user_coupon.is_used = true;
        user_coupon.redeem_time = Clock::get()?.unix_timestamp;

        let nft_mint_info = ctx.accounts.nft_mint.to_account_info();
        let (_, nft_authority_bump) = Pubkey::find_program_address(
            &[constants::NFT_AUTHORITY_SEED.as_bytes()],
            ctx.program_id,
        );
        let authority_seeds = &[
            constants::NFT_AUTHORITY_SEED.as_bytes(),
            &[nft_authority_bump],
        ];
        let authority_signer_seeds = &[&authority_seeds[..]];

        // Mint账户已在账户约束阶段完成初始化（authority/decimals），无需重复初始化

        let cpi_ctx = CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.user.to_account_info(),
                associated_token: ctx.accounts.user_nft_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                mint: nft_mint_info.clone(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );
        anchor_spl::associated_token::create(cpi_ctx)?;

        let cpi_mint_to_accounts = MintTo {
            mint: nft_mint_info.clone(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.nft_authority.to_account_info(),
        };
        let cpi_mint_to_program = ctx.accounts.token_program.to_account_info();
        mint_to(
            CpiContext::new_with_signer(
                cpi_mint_to_program,
                cpi_mint_to_accounts,
                authority_signer_seeds,
            ),
            1,
        )?;

        // Create Metadata Account
        let coupon_template = &ctx.accounts.coupon_template;
        let metadata_accounts = CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata.to_account_info(),
            mint: ctx.accounts.nft_mint.to_account_info(),
            mint_authority: ctx.accounts.nft_authority.to_account_info(),
            payer: ctx.accounts.user.to_account_info(),
            update_authority: ctx.accounts.nft_authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let data_v2 = DataV2 {
            name: coupon_template.name.clone(),
            symbol: "SCENIC".to_string(),
            uri: constants::DEFAULT_SCENIC_IMAGE_URI.to_string(), // 使用默认风景图片URI
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metadata_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            metadata_accounts,
            authority_signer_seeds,
        );

        create_metadata_accounts_v3(
            metadata_ctx,
            data_v2,
            true, // is_mutable
            true, // update_authority_is_signer
            None, // collection details
        )?;

        emit!(CouponRedeemed {
            user: ctx.accounts.user.key(),
            coupon_id: coupon_id.clone(),
            nft_mint: ctx.accounts.nft_mint.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn create_coupon(
        ctx: Context<CreateCoupon>,
        coupon_id: String,
        name: String,
        description: String,
        token_price: u64,
        total_supply: u32,
        expire_date: i64,
        valid_duration: i64, // 新增参数
    ) -> Result<()> {
        let coupon = &mut ctx.accounts.coupon_template;
        coupon.admin = ctx.accounts.admin.key();
        coupon.coupon_id = coupon_id.clone();
        coupon.name = name.clone();
        coupon.description = description;
        coupon.token_price = token_price;
        coupon.total_supply = total_supply;
        coupon.remaining = total_supply;
        coupon.expire_date = expire_date;
        coupon.valid_duration = valid_duration;

        emit!(CouponTemplateCreated {
            coupon_id: coupon_id,
            name: name,
            admin: ctx.accounts.admin.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // 更新优惠券模板
    pub fn update_coupon_template(
        ctx: Context<UpdateCouponTemplate>,
        _coupon_id: String,
        valid_duration: i64,
        expire_date: i64,
        total_supply: u32,
    ) -> Result<()> {
        let coupon = &mut ctx.accounts.coupon_template;
        coupon.valid_duration = valid_duration;
        coupon.expire_date = expire_date;
        coupon.total_supply = total_supply;
        coupon.remaining = total_supply;
        Ok(())
    }

    // 手动同步/修复景点评价计数
    pub fn sync_scenic_review_count(
        ctx: Context<SyncScenicReviewCount>,
        scenic_id: u64,
        correct_count: u32,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.id_registry;
        let clock = Clock::get()?;

        if let Some(item) = registry
            .scenic_accepted_reviews
            .iter_mut()
            .find(|item| item.scenic_id == scenic_id)
        {
            item.count = correct_count;
        } else {
            registry
                .scenic_accepted_reviews
                .push(ScenicAcceptedReviews {
                    scenic_id,
                    count: correct_count,
                });
        }
        
        // 如果设置为 20 的倍数，手动触发事件
        if correct_count > 0 && correct_count % 20 == 0 {
             emit!(ReviewBatchReady {
                scenic_pda: Pubkey::default(),
                scenic_id,
                batch_size: 20,
                total_reviews: correct_count,
                trigger_time: clock.unix_timestamp,
            });
        }

        Ok(())
    }

    //购买优惠券
    pub fn buy_coupon(
        ctx: Context<BuyCoupon>,
        coupon_id: String,
        purchase_id: u64,
        amount: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let program_id = ctx.program_id;

        let coupon_template = &mut ctx.accounts.coupon_template;
        let user_coupon = &mut ctx.accounts.user_coupon;
        let user_account = &mut ctx.accounts.user_account; // 改为可变引用
        let user_account_key = user_account.key();

        let coupon_name = &coupon_template.name.clone();
        // 使用前端传入的金额
        let token_price = amount;

        // Transfer tokens from user to treasury
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, token_price)?;

        // ========== 原有校验逻辑（不变） ==========
        if coupon_template.coupon_id != coupon_id {
            return err!(ScenicReviewError::InvalidCouponId);
        }
        if clock.unix_timestamp >= coupon_template.expire_date {
            return err!(ScenicReviewError::CouponExpired);
        }
        if coupon_template.remaining == 0 {
            return err!(ScenicReviewError::CouponSoldOut);
        }
        coupon_template.remaining -= 1;

        // ========== 原有 user_coupon 赋值（不变） ==========
        user_coupon.coupon_id = coupon_id.clone();
        user_coupon.user_pda = user_account_key;
        user_coupon.purchase_id = purchase_id;
        user_coupon.is_used = false;
        user_coupon.obtain_time = clock.unix_timestamp;
        // 新增：计算过期时间
        user_coupon.expire_time = clock.unix_timestamp + coupon_template.valid_duration;
        user_coupon.redeem_time = 0;

        // ========== 原有 PDA 验证逻辑（不变） ==========
        let (expected_template_pda, _) = Pubkey::find_program_address(
            &[
                constants::COUPON_TEMPLATE_SEED.as_bytes(),
                coupon_id.as_bytes(),
            ],
            &program_id,
        );
        if ctx.accounts.coupon_template.key() != expected_template_pda {
            return err!(ScenicReviewError::InvalidCouponId);
        }
        let (expected_user_coupon_pda, _) = Pubkey::find_program_address(
            &[
                constants::USER_COUPON_SEED.as_bytes(),
                user_account_key.as_ref(),
                coupon_id.as_bytes(),
                purchase_id.to_be_bytes().as_ref(),
            ],
            &program_id,
        );
        if ctx.accounts.user_coupon.key() != expected_user_coupon_pda {
            return err!(ScenicReviewError::InvalidCouponId);
        }
        let (tx_record_pda, _tx_record_bump) = Pubkey::find_program_address(
            &[
                b"tx_record",
                user_account_key.as_ref(),
                coupon_id.as_bytes(),
                purchase_id.to_be_bytes().as_ref(),
            ],
            &program_id,
        );
        require!(
            tx_record_pda == ctx.accounts.tx_record.key(),
            ScenicReviewError::BumpNotFound
        );

        // ========== 原有 tx_record 赋值（不变） ==========
        let tx_name = format!("buy_coupon_{}", coupon_name);
        require!(
            tx_name.len() <= 100,
            ScenicReviewError::ReviewContentTooLong
        );
        let related_id = format!(
            "coupon_{}_user_{}_pid_{}",
            coupon_id,
            user_account_key.to_string(),
            purchase_id
        );
        require!(
            related_id.len() <= 200,
            ScenicReviewError::ReviewContentTooLong
        );
        let tx_hash = ctx.accounts.user.key().to_string();
        require!(
            tx_hash.len() <= 100,
            ScenicReviewError::ReviewContentTooLong
        );
        let tx_record = &mut ctx.accounts.tx_record;
        tx_record.user_pda = user_account_key;
        tx_record.tx_type = TxType::Expense;
        tx_record.tx_name = tx_name;
        tx_record.amount = token_price;
        tx_record.related_id = related_id;
        tx_record.timestamp = clock.unix_timestamp;
        tx_record.tx_hash = tx_hash;

        // ========== 新增：将 user_coupon PDA 写入 UserAccount 的 coupons 列表 ==========
        // 1. 检查是否已存在该优惠券（避免重复添加）
        if !user_account.coupons.contains(&expected_user_coupon_pda) {
            // 2. 添加优惠券 PDA 到用户列表
            user_account.coupons.push(expected_user_coupon_pda);
        }

        Ok(())
    }

    pub fn update_ai_summary(ctx: Context<UpdateAiSummary>, summary: String) -> Result<()> {
        require!(
            summary.len() <= ScenicReviewSummary::MAX_AI_SUMMARY_LENGTH,
            ScenicReviewError::AiSummaryTooLong
        );

        let review_summary = &mut ctx.accounts.scenic_review_summary;
        review_summary.ai_summary = summary;
        review_summary.last_ai_update_slot = Clock::get()?.slot;

        emit!(AiSummaryUpdated {
            scenic_spot_id: review_summary.scenic_spot_id,
            summary_length: review_summary.ai_summary.len() as u32,
            update_slot: review_summary.last_ai_update_slot,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn get_default_coupon_templates(
        ctx: Context<GetDefaultCouponTemplates>,
    ) -> Result<Vec<CouponTemplateInfo>> {
        let clock = Clock::get()?;
        let mut default_coupons = Vec::new();

        let coupon_templates = vec![
            &ctx.accounts.coupon_template_001,
            &ctx.accounts.coupon_template_002,
            &ctx.accounts.coupon_template_003,
            &ctx.accounts.coupon_template_004,
        ];

        for template in coupon_templates {
            let is_expired = clock.unix_timestamp >= template.expire_date;
            default_coupons.push(CouponTemplateInfo {
                coupon_id: template.coupon_id.clone(),
                name: template.name.clone(),
                description: template.description.clone(),
                token_price: template.token_price,
                total_supply: template.total_supply,
                remaining: template.remaining,
                expire_date: template.expire_date,
                valid_duration: template.valid_duration, // 新增
                is_expired,
            });
        }

        Ok(default_coupons)
    }

    pub fn get_user_coupons(ctx: Context<GetUserCoupons>) -> Result<Vec<UserCouponInfo>> {
        let program_id = ctx.program_id;
        let user_account = &ctx.accounts.user_account;
        let mut user_coupons_info = Vec::new();

        for coupon_pda in &user_account.coupons {
            // 修复1：解引用 coupon_pda
            let coupon_account_info = ctx
                .remaining_accounts
                .iter()
                .find(|acc| acc.key() == *coupon_pda)
                // 修复2：使用正确的错误变体
                .ok_or(ScenicReviewError::CouponNotFound)?;

            if coupon_account_info.owner != program_id {
                continue;
            }

            // 修复3：解析 UserCoupon 数据（二选一）
            // 方式1：使用手动实现的 try_from_account_info
            let user_coupon = UserCoupon::try_from_account_info(coupon_account_info)?;
            // 方式2：使用 Anchor 内置解析
            // let user_coupon = Account::<UserCoupon>::try_from(coupon_account_info)?.into_inner();

            user_coupons_info.push(UserCouponInfo {
                coupon_id: user_coupon.coupon_id,
                user_pda: user_coupon.user_pda,
                purchase_id: user_coupon.purchase_id,
                is_used: user_coupon.is_used,
                obtain_time: user_coupon.obtain_time,
                expire_time: user_coupon.expire_time, // 新增
                redeem_time: user_coupon.redeem_time,
            });
        }

        Ok(user_coupons_info)
    }

    //查询用户评价记录
    pub fn get_user_review_list(ctx: Context<GetUserReviewList>) -> Result<Vec<UserReviewItem>> {
        // 新增：校验剩余账户均为只读
        for acc in ctx.remaining_accounts.iter() {
            require!(!acc.is_writable, ScenicReviewError::InvalidUser);
        }

        let user_account_key = ctx.accounts.user_account.key();
        let registry = &ctx.accounts.id_registry;
        let program_id = ctx.program_id;
        let mut user_reviews = Vec::new();

        // 原有逻辑不变...
        for acc in ctx.remaining_accounts.iter() {
            if acc.owner != program_id || acc.data_is_empty() || acc.is_writable {
                continue;
            }

            let review = match Review::try_deserialize(&mut acc.data.borrow().as_ref()) {
                Ok(r) => r,
                Err(e) => {
                    msg!("⚠️ 跳过无效评价账户 {}: {}", acc.key(), e);
                    continue;
                }
            };

            if review.user_pda != user_account_key {
                continue;
            }

            let scenic_name = registry
                .id_to_name
                .iter()
                .find(|item| item.id == review.scenic_spot_id)
                .map(|item| item.name.clone())
                .unwrap_or_else(|| format!("未知景点({})", review.scenic_spot_id));

            user_reviews.push(UserReviewItem {
                review_pda: acc.key(),
                scenic_id: review.scenic_spot_id,
                review_id: review.review_id, // 新增
                scenic_name,
                content: review.content.clone(),
                rating: review.rating,
                status: review.status,
                reward_amount: review.reward_amount,
                submit_time: review.submit_time,
            });
        }

        Ok(user_reviews)
    }

    pub fn get_scenic_review_statistics(
        ctx: Context<GetScenicReviewStats>,
        scenic_id: u64,
    ) -> Result<ScenicReviewStats> {
        let registry = &ctx.accounts.id_registry;
        let program_id = ctx.program_id;

        let scenic_name = registry
            .id_to_name
            .iter()
            .find(|item| item.id == scenic_id)
            .map(|item| item.name.clone())
            .ok_or(ScenicReviewError::ScenicSpotNotFound)?;

        let mut total_reviews = 0;
        let mut accepted_reviews = 0;
        let mut pending_reviews = 0;
        let mut rejected_reviews = 0;
        let mut total_rating = 0.0;

        for acc in ctx.remaining_accounts.iter() {
            if acc.owner != program_id || acc.data_is_empty() || acc.is_writable {
                continue;
            }

            let review = match Review::try_deserialize(&mut acc.data.borrow().as_ref()) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if review.scenic_spot_id != scenic_id {
                continue;
            }

            total_reviews += 1;
            match review.status {
                ReviewStatus::Accepted => {
                    accepted_reviews += 1;
                    total_rating += review.rating;
                }
                ReviewStatus::Pending => pending_reviews += 1,
                ReviewStatus::Rejected => rejected_reviews += 1,
            }
        }

        // 修复：将 else 分支的 f64 转为 f32
        let average_rating = if accepted_reviews > 0 {
            total_rating / accepted_reviews as f32
        } else {
            registry
                .extra_scenic_data
                .iter()
                .find(|item| item.id == scenic_id)
                .map(|item| item.rating as f32) // 关键：转为 f32
                .unwrap_or(0.0)
        };

        let stats = ScenicReviewStats {
            scenic_id,
            scenic_name,
            total_reviews,
            accepted_reviews,
            average_rating: ((average_rating as f64) * 10.0).round() / 10.0,
            pending_reviews,
            rejected_reviews,
        };

        Ok(stats)
    }

}

#[derive(Accounts)]
pub struct CreateDefaultScenicSpots<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump,
        has_one = admin @ ScenicReviewError::NotAdmin
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub id_registry: Box<Account<'info, ScenicIdRegistry>>,

    // 预定义8个景点账户
    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(50, 50, 100),
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), 1u64.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot_1: Box<Account<'info, ScenicSpot>>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(50, 50, 100),
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), 2u64.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot_2: Box<Account<'info, ScenicSpot>>,

    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(50, 50, 100),
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), 3u64.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot_3: Box<Account<'info, ScenicSpot>>,

    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(50, 50, 100),
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), 4u64.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot_4: Box<Account<'info, ScenicSpot>>,

    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(50, 50, 100),
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), 5u64.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot_5: Box<Account<'info, ScenicSpot>>,

    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(50, 50, 100),
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), 6u64.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot_6: Box<Account<'info, ScenicSpot>>,

    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(50, 50, 100),
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), 7u64.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot_7: Box<Account<'info, ScenicSpot>>,

    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(50, 50, 100),
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), 8u64.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot_8: Box<Account<'info, ScenicSpot>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
}

#[derive(Accounts)]
pub struct CreateScenicSpot<'info> {
    #[account(mut, has_one = admin)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump = id_registry.bump
    )]
    pub id_registry: Box<Account<'info, ScenicIdRegistry>>,

    #[account(
        init,
        payer = admin,
        space = 8 + ScenicSpot::space(
            constants::MAX_SCENIC_NAME_LENGTH,
            constants::MAX_DESCRIPTION_LENGTH,
            constants::MAX_TAGS_LENGTH
        ),
        seeds = [
            constants::SCENIC_SPOT_SEED.as_bytes(),
            &(id_registry.next_id).to_be_bytes()
        ],
        bump
    )]
    pub scenic_spot: Box<Account<'info, ScenicSpot>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(scenic_id: u64)]
pub struct UpdateScenicRating<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump = id_registry.bump
    )]
    pub id_registry: Box<Account<'info, ScenicIdRegistry>>,

    #[account(
        mut,
        seeds = [
            constants::SCENIC_SPOT_SEED.as_bytes(),
            &scenic_id.to_be_bytes()
        ],
        bump,
        has_one = admin
    )]
    pub scenic_spot: Box<Account<'info, ScenicSpot>>,
}

#[derive(Accounts)]
pub struct InitializeAll<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        payer = admin,
        space = 8 + ScenicIdRegistry::INIT_SPACE,
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump
    )]
    pub id_registry: Box<Account<'info, ScenicIdRegistry>>,

    #[account(
        seeds = [constants::VAULT_AUTHORITY_SEED.as_bytes()],
        bump
    )]
    /// CHECK: PDA仅作为签名者
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = token_mint.decimals == constants::TOKEN_DECIMALS @ ScenicReviewError::InvalidTokenMint
    
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = admin,
        token::mint = token_mint,
        token::authority = vault_authority,
        seeds = [b"treasury_vault", token_mint.key().as_ref()],
        bump,
        rent_exempt = enforce
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct GetDefaultScenicSpots<'info> {
    pub id_registry: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CreateDefaultCoupons<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump,
        has_one = admin @ ScenicReviewError::NotAdmin
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        payer = admin,
        space = 8 + CouponTemplate::space(8, 50, 200),
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), b"COUPON_001"],
        bump
    )]
    pub coupon_template_001: Box<Account<'info, CouponTemplate>>,

    #[account(
        init,
        payer = admin,
        space = 8 + CouponTemplate::space(8, 50, 200),
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), b"COUPON_002"],
        bump
    )]
    pub coupon_template_002: Box<Account<'info, CouponTemplate>>,

    #[account(
        init,
        payer = admin,
        space = 8 + CouponTemplate::space(8, 50, 200),
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), b"COUPON_003"],
        bump
    )]
    pub coupon_template_003: Box<Account<'info, CouponTemplate>>,

    #[account(
        init,
        payer = admin,
        space = 8 + CouponTemplate::space(8, 50, 200),
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), b"COUPON_004"],
        bump
    )]
    pub coupon_template_004: Box<Account<'info, CouponTemplate>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// InitScenicSpotAccount struct removed


#[derive(Accounts)]
pub struct SetOracle<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump,
        has_one = admin
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + UserAccount::SPACE,
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(new_level: u8)]
pub struct UpgradeUserLevel<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        associated_token::mint = config.token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"treasury_vault", config.token_mint.key().as_ref()],
        bump,
        token::mint = config.token_mint,
        token::authority = config.vault_authority
    )]
    pub treasury: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer = user,
        space = 8 + TransactionRecord::SPACE,
        seeds = [
            b"tx_record",          // 固定前缀
            user_account.key().as_ref(), // 用户账户PDA
            new_level.to_be_bytes().as_ref() // 新等级确保唯一
        ],
        bump
    )]
    pub tx_record: Account<'info, TransactionRecord>,
}

#[derive(Accounts)]
#[instruction(scenic_id: u64, version: u32, ai_summary: String, hash: String, rating: f64)]
pub struct UpdateScenicDetail<'info> {
    #[account(mut)]
    pub oracle: Signer<'info>,
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump,
        constraint = config.oracle == oracle.key() @ ScenicReviewError::InvalidOracle
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), scenic_id.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_spot: Box<Account<'info, ScenicSpot>>,
    #[account(
        init,
        payer = oracle,
        space = 8 + ScenicDetail::space(ai_summary.len(), hash.len()),
        seeds = [constants::SCENIC_DETAIL_SEED.as_bytes(), scenic_id.to_be_bytes().as_ref(), version.to_be_bytes().as_ref()],
        bump
    )]
    pub scenic_detail: Box<Account<'info, ScenicDetail>>,
    // 新增：添加 ScenicIdRegistry 账户（只读，用于更新最新版本号）
    #[account(
        mut,
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump = id_registry.bump
    )]
    pub id_registry: Box<Account<'info, ScenicIdRegistry>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// 重构上下文结构体：移除version入参
#[derive(Accounts)]
#[instruction(scenic_id: u64)] // 仅传scenic_id
pub struct GetScenicLatestDetail<'info> {
    pub user: AccountInfo<'info>,
    #[account(
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    // 关键修改1：从 Account 改为 UncheckedAccount，跳过自动初始化校验
    #[account(
        seeds = [constants::SCENIC_SPOT_SEED.as_bytes(), scenic_id.to_be_bytes().as_ref()],
        bump
    )]
    /// CHECK: 手动校验账户是否初始化，容错返回空数据
    pub scenic_spot: UncheckedAccount<'info>,

    #[account(
        // 这里的PDA由前端根据「scenic_id + 临时版本0」推导（仅占位，合约内会校验是否是最新版本的PDA）
        seeds = [constants::SCENIC_DETAIL_SEED.as_bytes(), scenic_id.to_be_bytes().as_ref(), 0u32.to_be_bytes().as_ref()],
        bump
    )]
    // 关键修改2：scenic_detail 也改为 UncheckedAccount，避免未初始化抛错
    /// CHECK: 手动校验账户是否初始化，容错返回空数据
    pub scenic_detail: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump = id_registry.bump
    )]
    pub id_registry: Account<'info, ScenicIdRegistry>,
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
// rating 参数类型改为 f32
#[instruction(scenic_id: u64, review_id: u64, rating: f32, content: String)]
pub struct SubmitReview<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump,
        constraint = user_account.user_wallet != Pubkey::default() @ ScenicReviewError::UserAccountNotFound
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump = id_registry.bump
    )]
    pub id_registry: Account<'info, ScenicIdRegistry>,
    #[account(
        init,
        payer = user,
        space = 8 + Review::space(content.len()),
        seeds = [
            constants::REVIEW_SEED.as_bytes(), 
            user_account.key().as_ref(), 
            scenic_id.to_be_bytes().as_ref(),
            review_id.to_be_bytes().as_ref()
        ],
        bump
    )]
    pub review: Account<'info, Review>,
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(scenic_id: u64, review_id: u64, status: u8)]
pub struct OracleConfirmReview<'info> {
    #[account(mut)]
    pub oracle: Signer<'info>,
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump,
        constraint = config.oracle == oracle.key() @ ScenicReviewError::InvalidOracle
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        constraint = user_account.user_wallet == user_wallet.key() @ ScenicReviewError::InvalidUser
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// CHECK: 验证用户钱包地址匹配
    pub user_wallet: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [
            constants::REVIEW_SEED.as_bytes(), 
            user_account.key().as_ref(), 
            scenic_id.to_be_bytes().as_ref(),
            review_id.to_be_bytes().as_ref()
        ],
        bump,
        constraint = review.status == ReviewStatus::Pending @ ScenicReviewError::ReviewNotPending
    )]
    pub review: Box<Account<'info, Review>>,
    #[account(
        mut,
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump = id_registry.bump
    )]
    pub id_registry: Box<Account<'info, ScenicIdRegistry>>,
    #[account(
        seeds = [constants::VAULT_AUTHORITY_SEED.as_bytes()],
        bump = config.vault_bump
    )]
    pub vault_authority: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"treasury_vault", config.token_mint.key().as_ref()],
        bump,
        token::mint = config.token_mint,
        token::authority = config.vault_authority
    )]
    pub treasury: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = config.token_mint,
        associated_token::authority = user_wallet
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        address = config.token_mint @ ScenicReviewError::InvalidTokenMint
    )]
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        init,
        payer = oracle, // 由Oracle支付创建账户的SOL
        space = 8 + TransactionRecord::SPACE,
        // PDA种子：固定前缀 + 用户PDA + 评价PDA + 时间戳（保证唯一）
        seeds = [
            b"tx_record",
            user_account.key().as_ref(),
            review.key().as_ref()
        ],
        bump
    )]
    pub tx_record: Account<'info, TransactionRecord>,
}



#[derive(Accounts)]
#[instruction(coupon_id: String, name: String, description: String, token_price: u64, total_supply: u32, expire_date: i64)]
pub struct CreateCoupon<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump,
        has_one = admin
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        space = 8 + CouponTemplate::space(coupon_id.len(), name.len(), description.len()),
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), coupon_id.as_bytes()],
        bump
    )]
    pub coupon_template: Account<'info, CouponTemplate>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(coupon_id: String, purchase_id: u64)]
pub struct RedeemCoupon<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [constants::USER_COUPON_SEED.as_bytes(), user_account.key().as_ref(), coupon_id.as_bytes(), purchase_id.to_be_bytes().as_ref()],
        bump
    )]
    pub user_coupon: Account<'info, UserCoupon>,
    #[account(
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), coupon_id.as_bytes()],
        bump
    )]
    pub coupon_template: Account<'info, CouponTemplate>,
    #[account(
        init,
        payer = user,
        mint::authority = nft_authority,
        mint::decimals = 0,
        mint::freeze_authority = nft_authority,
        seeds = [constants::NFT_SEED.as_bytes(), user.key().as_ref(), coupon_id.as_ref(), purchase_id.to_be_bytes().as_ref()],
        bump
    )]
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: 由前端传入Metaplex元数据PDA（无需本程序派生/初始化）
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: 在指令中创建用户的NFT ATA，避免在Mint未初始化前由运行时创建导致失败
    #[account(mut)]
    pub user_nft_account: UncheckedAccount<'info>,
    /// CHECK: NFT创建权限PDA
    #[account(
        seeds = [constants::NFT_AUTHORITY_SEED.as_bytes()],
        bump
    )]
    pub nft_authority: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// 1. 给 BuyCoupon 上下文添加 instruction 注解，传入 coupon_id
#[derive(Accounts)]
#[instruction(coupon_id: String, purchase_id: u64)] // 必须传入 coupon_id 与 purchase_id
pub struct BuyCoupon<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump,
        mut
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(
        mut,
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), coupon_id.as_bytes()],
        bump
    )]
    pub coupon_template: Box<Account<'info, CouponTemplate>>,
    #[account(
        init,
        payer = user,
        seeds = [
            constants::USER_COUPON_SEED.as_bytes(), 
            user_account.key().as_ref(), 
            coupon_id.as_bytes(),
            purchase_id.to_be_bytes().as_ref()
        ],
        bump,
        space = 8 + UserCoupon::space(coupon_id.len())
    )]
    pub user_coupon: Box<Account<'info, UserCoupon>>,
    pub system_program: Program<'info, System>,
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        associated_token::mint = config.token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"treasury_vault", config.token_mint.key().as_ref()],
        bump,
        token::mint = config.token_mint,
        token::authority = config.vault_authority
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        init,
        payer = user,
        space = TransactionRecord::SPACE,
        seeds = [
            b"tx_record",
            user_account.key().as_ref(),
            coupon_id.as_bytes(),
            purchase_id.to_be_bytes().as_ref()
        ],
        bump
    )]
    pub tx_record: Box<Account<'info, TransactionRecord>>,
}

#[derive(Accounts)]
#[instruction(scenic_id: u64)]
pub struct SyncScenicReviewCount<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump = id_registry.bump
    )]
    pub id_registry: Account<'info, ScenicIdRegistry>,
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump,
        has_one = admin @ ScenicReviewError::NotAdmin
    )]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
#[instruction(coupon_id: String)]
pub struct UpdateCouponTemplate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), coupon_id.as_bytes()],
        bump,
        has_one = admin
    )]
    pub coupon_template: Account<'info, CouponTemplate>,
}

#[derive(Accounts)]
pub struct UpdateAiSummary<'info> {
    #[account(mut)]
    pub oracle_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"scenic_review_summary", scenic_review_summary.scenic_spot_id.as_ref()],
        bump = scenic_review_summary.bump,
    )]
    pub scenic_review_summary: Account<'info, ScenicReviewSummary>,
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump,
        constraint = config.oracle == oracle_authority.key() @ ScenicReviewError::NotAuthorizedForAiUpdate
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetUserCoupons<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>, // 读取用户账户的 coupons 列表
}

#[derive(Accounts)]
pub struct GetDefaultCouponTemplates<'info> {
    /// CHECK: 查询者账户（仅读取，无需签名）
    pub user: AccountInfo<'info>,
    #[account(
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>, // 验证用户账户存在（非签名）
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>, // 程序配置验证

    // 4个默认优惠券模板PDA（只读）
    #[account(
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), b"COUPON_001"],
        bump
    )]
    pub coupon_template_001: Account<'info, CouponTemplate>,
    #[account(
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), b"COUPON_002"],
        bump
    )]
    pub coupon_template_002: Account<'info, CouponTemplate>,
    #[account(
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), b"COUPON_003"],
        bump
    )]
    pub coupon_template_003: Account<'info, CouponTemplate>,
    #[account(
        seeds = [constants::COUPON_TEMPLATE_SEED.as_bytes(), b"COUPON_004"],
        bump
    )]
    pub coupon_template_004: Account<'info, CouponTemplate>,
}

// ========== 新增：两个查询函数的上下文结构体 ==========
#[derive(Accounts)]
pub struct GetUserReviewList<'info> {
    /// CHECK: 被查询用户的钱包地址（仅只读，无需签名）
    pub target_user: UncheckedAccount<'info>,

    #[account(
        seeds = [constants::USER_SEED.as_bytes(), target_user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump
    )]
    pub id_registry: Account<'info, ScenicIdRegistry>,
}

#[derive(Accounts)]
pub struct GetScenicReviewStats<'info> {
    /// CHECK: 查询者账户（只读，无需签名）
    pub user: AccountInfo<'info>,
    #[account(
        seeds = [constants::SCENIC_ID_REGISTRY_SEED.as_bytes()],
        bump = id_registry.bump
    )]
    pub id_registry: Account<'info, ScenicIdRegistry>, // 景点注册表（只读）
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>, // 程序配置（只读）
                                        // 剩余账户：该景点的所有Review PDA（客户端传入，只读）
}
#[derive(Accounts)]
pub struct GetUserTransactionHistory<'info> {
    /// CHECK: 被查询的用户账户（只读，无需签名）
    pub target_user: AccountInfo<'info>,
    #[account(
        seeds = [constants::USER_SEED.as_bytes(), target_user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>, // 关联用户账户（只读）
    #[account(
        seeds = [constants::CONFIG_SEED.as_bytes()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>, // 程序配置（只读）
                                        // 剩余账户：用户的所有TransactionRecord PDA（客户端传入，只读）
}

#[derive(Accounts)]
pub struct FixUserRewardData<'info> {
    #[account(
        mut,
        seeds = [constants::USER_SEED.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}
