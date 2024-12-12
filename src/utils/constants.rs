use crate::types::index::{Coin, Pool};
use std::collections::HashMap;
use std::string::ToString;
use std::sync::LazyLock;

pub type CoinMap = HashMap<String, Coin>;
pub type PoolMap = HashMap<String, Pool>;

pub struct DeepbookPackageIds {
    pub deepbook_package_id: String,
    pub registry_id: String,
    pub deep_treasury_id: String,
}

pub static TESTNET_PACKAGE_IDS: LazyLock<DeepbookPackageIds> =
    LazyLock::new(|| DeepbookPackageIds {
        deepbook_package_id: String::from(
            "0xcbf4748a965d469ea3a36cf0ccc5743b96c2d0ae6dee0762ed3eca65fac07f7e",
        ),
        registry_id: String::from(
            "0x98dace830ebebd44b7a3331c00750bf758f8a4b17a27380f5bb3fbe68cb984a7",
        ),
        deep_treasury_id: String::from(
            "0x69fffdae0075f8f71f4fa793549c11079266910e8905169845af1f5d00e09dcb",
        ),
    });

pub static MAINNET_PACKAGE_IDS: LazyLock<DeepbookPackageIds> =
    LazyLock::new(|| DeepbookPackageIds {
        deepbook_package_id: String::from(
            "0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809",
        ),
        registry_id: String::from(
            "0xaf16199a2dff736e9f07a845f23c5da6df6f756eddb631aed9d24a93efc4549d",
        ),
        deep_treasury_id: String::from(
            "0x032abf8948dda67a271bcc18e776dbbcfb0d58c8d288a700ff0d5521e57a1ffe",
        ),
    });

pub static TESTNET_COINS: LazyLock<CoinMap> = LazyLock::new(|| {
    CoinMap::from([
    (
        "DEEP".to_string(),
        Coin {
            address: "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8"
                .to_string(),
            coin_type:
                "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP"
                    .to_string(),
            scalar: 1000000,
        },
    ),
    (
        "SUI".to_string(),
        Coin {
            address: "0x0000000000000000000000000000000000000000000000000000000000000002"
                .to_string(),
            coin_type:
                "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
                    .to_string(),
            scalar: 1000000000,
        },
    ),
    (
        "DBUSDC".to_string(),
        Coin {
            address: "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7"
                .to_string(),
            coin_type:
                "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC"
                    .to_string(),
            scalar: 1000000,
        },
    ),
    (
        "DBUSDT".to_string(),
        Coin {
            address: "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7"
                .to_string(),
            coin_type:
                "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDT::DBUSDT"
                    .to_string(),
            scalar: 1000000,
        },
    ),
])
});

pub static MAINNET_COINS: LazyLock<CoinMap> = LazyLock::new(|| {
    CoinMap::from([
    (
        "DEEP".to_string(),
        Coin {
            address: "0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270"
                .to_string(),
            coin_type:
                "0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270::deep::DEEP"
                    .to_string(),
            scalar: 1000000,
        },
    ),
    (
        "SUI".to_string(),
        Coin {
            address: "0x0000000000000000000000000000000000000000000000000000000000000002"
                .to_string(),
            coin_type:
                "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
                    .to_string(),
            scalar: 1000000000,
        },
    ),
    (
        "USDC".to_string(),
        Coin {
            address: "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7"
                .to_string(),
            coin_type:
                "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC"
                    .to_string(),
            scalar: 1000000,
        },
    ),
    (
        "WUSDC".to_string(),
        Coin {
            address: "0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf"
                .to_string(),
            coin_type:
                "0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf::coin::COIN"
                    .to_string(),
            scalar: 1000000,
        },
    ),
    (
        "WETH".to_string(),
        Coin {
            address: "0xaf8cd5edc19c4512f4259f0bee101a40d41ebed738ade5874359610ef8eeced5"
                .to_string(),
            coin_type:
                "0xaf8cd5edc19c4512f4259f0bee101a40d41ebed738ade5874359610ef8eeced5::coin::COIN"
                    .to_string(),
            scalar: 100000000,
        },
    ),
    (
        "BETH".to_string(),
        Coin {
            address: "0xd0e89b2af5e4910726fbcd8b8dd37bb79b29e5f83f7491bca830e94f7f226d29"
                .to_string(),
            coin_type:
                "0xd0e89b2af5e4910726fbcd8b8dd37bb79b29e5f83f7491bca830e94f7f226d29::eth::ETH"
                    .to_string(),
            scalar: 100000000,
        },
    ),
    (
        "WBTC".to_string(),
        Coin {
            address: "0x027792d9fed7f9844eb4839566001bb6f6cb4804f66aa2da6fe1ee242d896881"
                .to_string(),
            coin_type:
                "0x027792d9fed7f9844eb4839566001bb6f6cb4804f66aa2da6fe1ee242d896881::coin::COIN"
                    .to_string(),
            scalar: 100000000,
        },
    ),
    (
        "WUSDT".to_string(),
        Coin {
            address: "0xc060006111016b8a020ad5b33834984a437aaa7d3c74c18e09a95d48aceab08c"
                .to_string(),
            coin_type:
                "0xc060006111016b8a020ad5b33834984a437aaa7d3c74c18e09a95d48aceab08c::coin::COIN"
                    .to_string(),
            scalar: 1000000,
        },
    ),
    (
        "NS".to_string(),
        Coin {
            address: "0x5145494a5f5100e645e4b0aa950fa6b68f614e8c59e17bc5ded3495123a79178"
                .to_string(),
            coin_type: "0x5145494a5f5100e645e4b0aa950fa6b68f614e8c59e17bc5ded3495123a79178::ns::NS"
                .to_string(),
            scalar: 1000000,
        },
    ),
    (
        "TYPUS".to_string(),
        Coin {
            address: "0xf82dc05634970553615eef6112a1ac4fb7bf10272bf6cbe0f80ef44a6c489385"
                .to_string(),
            coin_type:
                "0xf82dc05634970553615eef6112a1ac4fb7bf10272bf6cbe0f80ef44a6c489385::typus::TYPUS"
                    .to_string(),
            scalar: 1000000000,
        },
    ),
    (
        "AUSD".to_string(),
        Coin {
            address: "0x2053d08c1e2bd02791056171aab0fd12bd7cd7efad2ab8f6b9c8902f14df2ff2"
                .to_string(),
            coin_type:
                "0x2053d08c1e2bd02791056171aab0fd12bd7cd7efad2ab8f6b9c8902f14df2ff2::ausd::AUSD"
                    .to_string(),
            scalar: 1000000,
        },
    ),
])
});

pub static TESTNET_POOLS: LazyLock<PoolMap> = LazyLock::new(|| {
    PoolMap::from([
        (
            "DEEP_SUI".to_string(),
            Pool {
                address: "0x0d1b1746d220bd5ebac5231c7685480a16f1c707a46306095a4c67dc7ce4dcae"
                    .to_string(),
                base_coin: "DEEP".to_string(),
                quote_coin: "SUI".to_string(),
            },
        ),
        (
            "SUI_DBUSDC".to_string(),
            Pool {
                address: "0x520c89c6c78c566eed0ebf24f854a8c22d8fdd06a6f16ad01f108dad7f1baaea"
                    .to_string(),
                base_coin: "SUI".to_string(),
                quote_coin: "DBUSDC".to_string(),
            },
        ),
        (
            "DEEP_DBUSDC".to_string(),
            Pool {
                address: "0xee4bb0db95dc571b960354713388449f0158317e278ee8cda59ccf3dcd4b5288"
                    .to_string(),
                base_coin: "DEEP".to_string(),
                quote_coin: "DBUSDC".to_string(),
            },
        ),
        (
            "DBUSDT_DBUSDC".to_string(),
            Pool {
                address: "0x69cbb39a3821d681648469ff2a32b4872739d2294d30253ab958f85ace9e0491"
                    .to_string(),
                base_coin: "DBUSDT".to_string(),
                quote_coin: "DBUSDC".to_string(),
            },
        ),
    ])
});

pub static MAINNET_POOLS: LazyLock<PoolMap> = LazyLock::new(|| {
    PoolMap::from([
        (
            "DEEP_SUI".to_string(),
            Pool {
                address: "0xb663828d6217467c8a1838a03793da896cbe745b150ebd57d82f814ca579fc22"
                    .to_string(),
                base_coin: "DEEP".to_string(),
                quote_coin: "SUI".to_string(),
            },
        ),
        (
            "SUI_USDC".to_string(),
            Pool {
                address: "0xe05dafb5133bcffb8d59f4e12465dc0e9faeaa05e3e342a08fe135800e3e4407"
                    .to_string(),
                base_coin: "SUI".to_string(),
                quote_coin: "USDC".to_string(),
            },
        ),
        (
            "DEEP_USDC".to_string(),
            Pool {
                address: "0xf948981b806057580f91622417534f491da5f61aeaf33d0ed8e69fd5691c95ce"
                    .to_string(),
                base_coin: "DEEP".to_string(),
                quote_coin: "USDC".to_string(),
            },
        ),
        (
            "WUSDT_USDC".to_string(),
            Pool {
                address: "0x4e2ca3988246e1d50b9bf209abb9c1cbfec65bd95afdacc620a36c67bdb8452f"
                    .to_string(),
                base_coin: "WUSDT".to_string(),
                quote_coin: "USDC".to_string(),
            },
        ),
        (
            "WUSDC_USDC".to_string(),
            Pool {
                address: "0xa0b9ebefb38c963fd115f52d71fa64501b79d1adcb5270563f92ce0442376545"
                    .to_string(),
                base_coin: "WUSDC".to_string(),
                quote_coin: "USDC".to_string(),
            },
        ),
        (
            "BETH_USDC".to_string(),
            Pool {
                address: "0x1109352b9112717bd2a7c3eb9a416fff1ba6951760f5bdd5424cf5e4e5b3e65c"
                    .to_string(),
                base_coin: "BETH".to_string(),
                quote_coin: "USDC".to_string(),
            },
        ),
        (
            "NS_USDC".to_string(),
            Pool {
                address: "0x0c0fdd4008740d81a8a7d4281322aee71a1b62c449eb5b142656753d89ebc060"
                    .to_string(),
                base_coin: "NS".to_string(),
                quote_coin: "USDC".to_string(),
            },
        ),
        (
            "NS_SUI".to_string(),
            Pool {
                address: "0x27c4fdb3b846aa3ae4a65ef5127a309aa3c1f466671471a806d8912a18b253e8"
                    .to_string(),
                base_coin: "NS".to_string(),
                quote_coin: "SUI".to_string(),
            },
        ),
        (
            "TYPUS_SUI".to_string(),
            Pool {
                address: "0xe8e56f377ab5a261449b92ac42c8ddaacd5671e9fec2179d7933dd1a91200eec"
                    .to_string(),
                base_coin: "TYPUS".to_string(),
                quote_coin: "SUI".to_string(),
            },
        ),
        (
            "SUI_AUSD".to_string(),
            Pool {
                address: "0x183df694ebc852a5f90a959f0f563b82ac9691e42357e9a9fe961d71a1b809c8"
                    .to_string(),
                base_coin: "SUI".to_string(),
                quote_coin: "AUSD".to_string(),
            },
        ),
        (
            "AUSD_USDC".to_string(),
            Pool {
                address: "0x5661fc7f88fbeb8cb881150a810758cf13700bb4e1f31274a244581b37c303c3"
                    .to_string(),
                base_coin: "AUSD".to_string(),
                quote_coin: "USDC".to_string(),
            },
        ),
    ])
});
