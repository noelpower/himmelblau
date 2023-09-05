use crate::unix_config::HomeAttr;

pub const DEFAULT_CONFIG_PATH: &str = "/etc/himmelblau/himmelblau.conf";
pub const DEFAULT_SOCK_PATH: &str = "/var/run/himmelblaud/socket";
pub const DEFAULT_DB_PATH: &str = "/var/cache/himmelblau/himmelblau.cache.db";
pub const DEFAULT_HOME_PREFIX: &str = "/home/";
pub const DEFAULT_HOME_ATTR: HomeAttr = HomeAttr::Uuid;
pub const DEFAULT_HOME_ALIAS: Option<HomeAttr> = Some(HomeAttr::Spn);
pub const DEFAULT_SHELL: &str = "/bin/bash";
pub const DEFAULT_ODC_PROVIDER: &str = "odc.officeapps.live.com";
pub const DEFAULT_AUTHORITY_HOST: &str = "login.microsoftonline.com";
pub const DEFAULT_GRAPH: &str = "https://graph.microsoft.com";
pub const DEFAULT_APP_ID: &str = "b743a22d-6705-4147-8670-d92fa515ee2b";
pub const DEFAULT_IDMAP_RANGE: (u32, u32) = (1000000, 6999999);
pub const DEFAULT_CONN_TIMEOUT: u64 = 2;
pub const DEFAULT_CACHE_TIMEOUT: u64 = 15;
pub const DEFAULT_TPM_TCTI_NAME: &str = "device:/dev/tpmrm0";