//! OpenAPI 文档定义
//!
//! 使用 utoipa 生成 OpenAPI 3.0 文档

#[cfg(feature = "swagger")]
use utoipa::OpenApi;

/// API 文档
///
/// 包含所有公开的 API 端点文档
#[cfg(feature = "swagger")]
#[derive(OpenApi)]
#[openapi(
    info(
        title = "AkShare API",
        description = "期货和股票数据 RESTful API 服务\n\n数据来源：新浪财经、100ppi、99期货网等",
        version = "1.0.0",
        contact(
            name = "AkShare",
            url = "https://github.com/akfamily/akshare"
        )
    ),
    servers(
        (url = "/api/v1", description = "API v1")
    ),
    tags(
        (name = "health", description = "健康检查接口"),
        (name = "ak", description = "AkShare 模块接口"),
        (name = "bank", description = "银保监会/金融监管数据接口"),
        (name = "bond", description = "债券与国债收益率接口"),
        (name = "currency", description = "外汇与货币牌价接口"),
        (name = "cal", description = "波动率估算与算法接口"),
        (name = "stocks", description = "股票数据接口"),
        (name = "futures", description = "期货数据接口"),
        (name = "futures-main", description = "主力连续合约接口"),
        (name = "futures-rank", description = "持仓排名接口"),
        (name = "futures-warehouse", description = "仓单日报接口"),
    ),
    paths(
        // 健康检查
        crate::handlers::health::health_check,
        // AK 接口
        crate::handlers::ak::get_info,
        crate::handlers::ak::get_article_epu_index,
        crate::handlers::ak::get_fred_md,
        crate::handlers::ak::get_fred_qd,
        crate::handlers::ak::get_article_oman_rv,
        crate::handlers::ak::get_article_oman_rv_short,
        crate::handlers::ak::get_article_rlab_rv,
        // 银行/金融监管接口
        crate::handlers::ak::bank::get_fjcf_total_num,
        crate::handlers::ak::bank::get_fjcf_total_page,
        crate::handlers::ak::bank::get_fjcf_list,
        crate::handlers::ak::bank::get_fjcf_detail,
        // 债券接口
        crate::handlers::ak::bond::get_bond_zh_cov_spot,
        crate::handlers::ak::bond::get_bond_gb_zh_sina,
        crate::handlers::ak::bond::get_bond_gb_us_sina,
        crate::handlers::ak::bond::get_bond_zh_us_rate,
        crate::handlers::ak::bond::get_bond_sh_buy_back,
        crate::handlers::ak::bond::get_bond_sz_buy_back,
        crate::handlers::ak::bond::get_bond_cb_index_jsl,
        crate::handlers::ak::bond::get_bond_cb_redeem_jsl,
        crate::handlers::ak::bond::get_bond_cb_profile_sina,
        crate::handlers::ak::bond::get_bond_cov_comparison,
        // 外汇与货币接口
        crate::handlers::ak::currency::get_currency_boc_sina,
        // 量化计算接口
        crate::handlers::ak::cal::calculate_volatility_yz,
        // 股票接口
        crate::handlers::stock::list_stocks,
        crate::handlers::stock::get_stock_info,
        crate::handlers::stock::get_stock_history,
        // 期货基础接口
        crate::handlers::futures::list_futures,
        crate::handlers::futures::get_futures_info,
        crate::handlers::futures::get_exchanges,
        crate::handlers::futures::get_symbol_mark,
        crate::handlers::futures::get_exchange_symbols,
        crate::handlers::futures::get_multiple_futures,
        crate::handlers::futures::get_history,
        crate::handlers::futures::get_contract_detail,
        // 主力合约
        crate::handlers::futures::get_main_contracts,
        // 持仓排名
        crate::handlers::futures::get_rank_shfe,
        crate::handlers::futures::get_rank_cffex,
        crate::handlers::futures::get_rank_dce,
        crate::handlers::futures::get_rank_czce,
        crate::handlers::futures::get_rank_gfex,
        crate::handlers::futures::get_rank_sum_data,
        crate::handlers::futures::get_rank_sum_daily_data,
        // 仓单日报
        crate::handlers::futures::get_warehouse_czce,
        crate::handlers::futures::get_warehouse_dce,
        crate::handlers::futures::get_warehouse_shfe,
        crate::handlers::futures::get_warehouse_gfex,
    ),
    components(
        schemas(
            crate::models::ApiResponse<String>,
            crate::models::AkInfo,
            crate::models::AkQuery,
            crate::models::EpuIndexItem,
            crate::models::EpuIndexQuery,
            crate::models::FredItem,
            crate::models::FredQuery,
            crate::models::VolatilityItem,
            crate::models::OmanRvQuery,
            crate::models::OmanRvShortQuery,
            crate::models::RlabRvQuery,
            crate::models::ak::bank::BankFjcfQuery,
            crate::models::ak::bank::BankFjcfListItem,
            crate::models::ak::bank::BankFjcfDetailItem,
            crate::models::ak::bond::BondQuery,
            crate::models::ak::bond::BondZhCovSpotItem,
            crate::models::ak::bond::BondGbKlineItem,
            crate::models::ak::bond::BondZhUsRateItem,
            crate::models::ak::bond::BondBuyBackItem,
            crate::models::ak::bond::BondJslItem,
            crate::models::ak::bond::BondCbProfileItem,
            crate::models::ak::bond::BondCovComparisonItem,
            crate::models::ak::currency::CurrencyBocQuery,
            crate::models::ak::currency::CurrencyBocItem,
            crate::models::ak::cal::OhlcItem,
            crate::models::ak::cal::YangZhangVolatilityResult,
            crate::models::StockInfo,
            crate::models::StockQuery,
            crate::models::StockHistoryData,
            crate::models::FuturesInfo,
            crate::models::FuturesQuery,
            crate::models::FuturesHistoryData,
            crate::models::FuturesSymbolMark,
            crate::models::FuturesContractDetail,
            crate::models::FuturesMainContract,
            crate::models::FuturesMainDailyData,
            crate::models::FuturesMainQuery,
            crate::models::FuturesHoldPosition,
            crate::models::FuturesHoldPosQuery,
            crate::models::ForeignFuturesHistData,
            crate::models::ForeignFuturesDetail,
            crate::models::FuturesFeesInfo,
            crate::models::FuturesCommInfo,
            crate::models::FuturesCommQuery,
            crate::models::FuturesRule,
            crate::models::FuturesRuleQuery,
            crate::models::Futures99Symbol,
            crate::models::FuturesInventory99,
            crate::models::FuturesInventory99Query,
            crate::models::FuturesSpotPrice,
            crate::models::FuturesSpotPriceQuery,
            crate::models::FuturesSpotPricePrevious,
            crate::models::FuturesSpotPricePreviousQuery,
            crate::models::FuturesSpotPriceDailyQuery,
            crate::models::RankTableQuery,
            crate::models::RankSumDailyQuery,
            crate::models::RankTableResponse,
            crate::models::RankSum,
            crate::models::CzceWarehouseReceiptResponse,
            crate::models::DceWarehouseReceipt,
            crate::models::ShfeWarehouseReceiptResponse,
            crate::models::GfexWarehouseReceiptResponse,
        )
    )
)]
pub struct ApiDoc;
