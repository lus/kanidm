#![allow(clippy::unwrap_used)]

use uuid::{uuid, Uuid};

// Built in group and account ranges.
pub const STR_UUID_ADMIN: &str = "00000000-0000-0000-0000-000000000000";
pub const UUID_ADMIN: Uuid = uuid!("00000000-0000-0000-0000-000000000000");
pub const _UUID_IDM_ADMINS: Uuid = uuid!("00000000-0000-0000-0000-000000000001");
pub const _UUID_IDM_PEOPLE_READ_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000002");
pub const _UUID_IDM_PEOPLE_WRITE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000003");
pub const _UUID_IDM_GROUP_WRITE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000004");
pub const _UUID_IDM_ACCOUNT_READ_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000005");
pub const _UUID_IDM_ACCOUNT_WRITE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000006");
pub const _UUID_IDM_RADIUS_SERVERS: Uuid = uuid!("00000000-0000-0000-0000-000000000007");
pub const _UUID_IDM_HP_ACCOUNT_READ_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000008");
pub const _UUID_IDM_HP_ACCOUNT_WRITE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000009");
pub const _UUID_IDM_SCHEMA_MANAGE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000010");
pub const _UUID_IDM_ACP_MANAGE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000011");
pub const _UUID_IDM_HP_GROUP_WRITE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000012");
pub const _UUID_IDM_PEOPLE_MANAGE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000013");
pub const _UUID_IDM_ACCOUNT_MANAGE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000014");
pub const _UUID_IDM_GROUP_MANAGE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000015");
pub const _UUID_IDM_HP_ACCOUNT_MANAGE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000016");
pub const _UUID_IDM_HP_GROUP_MANAGE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000017");
pub const UUID_IDM_ADMIN: Uuid = uuid!("00000000-0000-0000-0000-000000000018");

pub const STR_UUID_SYSTEM_ADMINS: &str = "00000000-0000-0000-0000-000000000000";
pub const UUID_SYSTEM_ADMINS: Uuid = uuid!("00000000-0000-0000-0000-000000000019");
pub const UUID_DOMAIN_ADMINS: Uuid = uuid!("00000000-0000-0000-0000-000000000020");
pub const _UUID_IDM_ACCOUNT_UNIX_EXTEND_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000021");
pub const _UUID_IDM_GROUP_UNIX_EXTEND_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000022");
pub const _UUID_IDM_PEOPLE_ACCOUNT_PASSWORD_IMPORT_PRIV: Uuid =
    uuid!("00000000-0000-0000-0000-000000000023");
pub const _UUID_IDM_PEOPLE_EXTEND_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000024");
pub const _UUID_IDM_HP_ACCOUNT_UNIX_EXTEND_PRIV: Uuid =
    uuid!("00000000-0000-0000-0000-000000000025");
pub const _UUID_IDM_HP_GROUP_UNIX_EXTEND_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000026");
pub const _UUID_IDM_HP_OAUTH2_MANAGE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000027");
pub const _UUID_IDM_HP_PEOPLE_READ_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000028");
pub const _UUID_IDM_HP_PEOPLE_WRITE_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000029");
pub const _UUID_IDM_HP_PEOPLE_EXTEND_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000030");

pub const _UUID_IDM_RADIUS_SECRET_READ_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-000000000032");
pub const _UUID_IDM_RADIUS_SECRET_WRITE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-000000000031");
pub const _UUID_IDM_PEOPLE_SELF_WRITE_MAIL_PRIV: Uuid =
    uuid!("00000000-0000-0000-0000-000000000033");
pub const _UUID_IDM_HP_SERVICE_ACCOUNT_INTO_PERSON_MIGRATE_PRIV: Uuid =
    uuid!("00000000-0000-0000-0000-000000000034");

pub const UUID_IDM_ALL_PERSONS: Uuid = uuid!("00000000-0000-0000-0000-000000000035");
pub const STR_UUID_IDM_ALL_ACCOUNTS: &str = "00000000-0000-0000-0000-000000000036";
pub const UUID_IDM_ALL_ACCOUNTS: Uuid = uuid!("00000000-0000-0000-0000-000000000036");
pub const _UUID_IDM_HP_SYNC_ACCOUNT_MANAGE_PRIV: Uuid =
    uuid!("00000000-0000-0000-0000-000000000037");

pub const UUID_IDM_UI_ENABLE_EXPERIMENTAL_FEATURES: Uuid =
    uuid!("00000000-0000-0000-0000-000000000038");
pub const UUID_IDM_ACCOUNT_MAIL_READ_PRIV: Uuid = uuid!("00000000-0000-0000-0000-000000000039");

//
pub const _UUID_IDM_HIGH_PRIVILEGE: Uuid = uuid!("00000000-0000-0000-0000-000000001000");

// Builtin schema
pub const UUID_SCHEMA_ATTR_CLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000000");
pub const UUID_SCHEMA_ATTR_UUID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000001");
pub const UUID_SCHEMA_ATTR_NAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000002");
pub const UUID_SCHEMA_ATTR_SPN: Uuid = uuid!("00000000-0000-0000-0000-ffff00000003");
pub const UUID_SCHEMA_ATTR_DESCRIPTION: Uuid = uuid!("00000000-0000-0000-0000-ffff00000004");
pub const UUID_SCHEMA_ATTR_MULTIVALUE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000005");
pub const UUID_SCHEMA_ATTR_UNIQUE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000047");
pub const UUID_SCHEMA_ATTR_INDEX: Uuid = uuid!("00000000-0000-0000-0000-ffff00000006");
pub const UUID_SCHEMA_ATTR_SYNTAX: Uuid = uuid!("00000000-0000-0000-0000-ffff00000007");
pub const UUID_SCHEMA_ATTR_SYSTEMMAY: Uuid = uuid!("00000000-0000-0000-0000-ffff00000008");
pub const UUID_SCHEMA_ATTR_MAY: Uuid = uuid!("00000000-0000-0000-0000-ffff00000009");
pub const UUID_SCHEMA_ATTR_SYSTEMMUST: Uuid = uuid!("00000000-0000-0000-0000-ffff00000010");
pub const UUID_SCHEMA_ATTR_MUST: Uuid = uuid!("00000000-0000-0000-0000-ffff00000011");
pub const UUID_SCHEMA_ATTR_MEMBEROF: Uuid = uuid!("00000000-0000-0000-0000-ffff00000012");
pub const UUID_SCHEMA_ATTR_MEMBER: Uuid = uuid!("00000000-0000-0000-0000-ffff00000013");
pub const UUID_SCHEMA_ATTR_DIRECTMEMBEROF: Uuid = uuid!("00000000-0000-0000-0000-ffff00000014");
pub const UUID_SCHEMA_ATTR_VERSION: Uuid = uuid!("00000000-0000-0000-0000-ffff00000015");
pub const UUID_SCHEMA_ATTR_DOMAIN: Uuid = uuid!("00000000-0000-0000-0000-ffff00000016");
pub const UUID_SCHEMA_ATTR_ACP_ENABLE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000017");
pub const UUID_SCHEMA_ATTR_ACP_RECEIVER: Uuid = uuid!("00000000-0000-0000-0000-ffff00000018");
pub const UUID_SCHEMA_ATTR_ACP_TARGETSCOPE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000019");
pub const UUID_SCHEMA_ATTR_ACP_SEARCH_ATTR: Uuid = uuid!("00000000-0000-0000-0000-ffff00000020");
pub const UUID_SCHEMA_ATTR_ACP_CREATE_CLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000021");
pub const UUID_SCHEMA_ATTR_ACP_CREATE_ATTR: Uuid = uuid!("00000000-0000-0000-0000-ffff00000022");
pub const UUID_SCHEMA_ATTR_ACP_MODIFY_REMOVEDATTR: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000023");
pub const UUID_SCHEMA_ATTR_ACP_MODIFY_PRESENTATTR: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000024");
pub const UUID_SCHEMA_ATTR_ACP_MODIFY_CLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000025");
pub const UUID_SCHEMA_CLASS_ATTRIBUTETYPE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000026");
pub const UUID_SCHEMA_CLASS_CLASSTYPE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000027");
pub const UUID_SCHEMA_CLASS_OBJECT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000028");
pub const UUID_SCHEMA_CLASS_EXTENSIBLEOBJECT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000029");
pub const UUID_SCHEMA_CLASS_MEMBEROF: Uuid = uuid!("00000000-0000-0000-0000-ffff00000030");
pub const UUID_SCHEMA_CLASS_RECYCLED: Uuid = uuid!("00000000-0000-0000-0000-ffff00000031");
pub const UUID_SCHEMA_CLASS_TOMBSTONE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000032");
pub const UUID_SCHEMA_CLASS_SYSTEM_INFO: Uuid = uuid!("00000000-0000-0000-0000-ffff00000033");
pub const UUID_SCHEMA_CLASS_ACCESS_CONTROL_PROFILE: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000034");
pub const UUID_SCHEMA_CLASS_ACCESS_CONTROL_SEARCH: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000035");
pub const UUID_SCHEMA_CLASS_ACCESS_CONTROL_DELETE: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000036");
pub const UUID_SCHEMA_CLASS_ACCESS_CONTROL_MODIFY: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000037");
pub const UUID_SCHEMA_CLASS_ACCESS_CONTROL_CREATE: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000038");
pub const UUID_SCHEMA_CLASS_SYSTEM: Uuid = uuid!("00000000-0000-0000-0000-ffff00000039");
pub const _UUID_SCHEMA_ATTR_DISPLAYNAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000040");
pub const _UUID_SCHEMA_ATTR_MAIL: Uuid = uuid!("00000000-0000-0000-0000-ffff00000041");
pub const _UUID_SCHEMA_ATTR_SSH_PUBLICKEY: Uuid = uuid!("00000000-0000-0000-0000-ffff00000042");
pub const _UUID_SCHEMA_ATTR_PRIMARY_CREDENTIAL: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000043");
pub const _UUID_SCHEMA_CLASS_PERSON: Uuid = uuid!("00000000-0000-0000-0000-ffff00000044");
pub const _UUID_SCHEMA_CLASS_GROUP: Uuid = uuid!("00000000-0000-0000-0000-ffff00000045");
pub const _UUID_SCHEMA_CLASS_ACCOUNT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000046");
// GAP - 47
pub const UUID_SCHEMA_ATTR_ATTRIBUTENAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000048");
pub const UUID_SCHEMA_ATTR_CLASSNAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000049");
pub const _UUID_SCHEMA_ATTR_LEGALNAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000050");
pub const _UUID_SCHEMA_ATTR_RADIUS_SECRET: Uuid = uuid!("00000000-0000-0000-0000-ffff00000051");
pub const _UUID_SCHEMA_CLASS_DOMAIN_INFO: Uuid = uuid!("00000000-0000-0000-0000-ffff00000052");
pub const _UUID_SCHEMA_ATTR_DOMAIN_NAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000053");
pub const _UUID_SCHEMA_ATTR_DOMAIN_UUID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000054");
pub const _UUID_SCHEMA_ATTR_DOMAIN_SSID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000055");
pub const _UUID_SCHEMA_ATTR_GIDNUMBER: Uuid = uuid!("00000000-0000-0000-0000-ffff00000056");
pub const _UUID_SCHEMA_CLASS_POSIXACCOUNT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000057");
pub const _UUID_SCHEMA_CLASS_POSIXGROUP: Uuid = uuid!("00000000-0000-0000-0000-ffff00000058");
pub const _UUID_SCHEMA_ATTR_BADLIST_PASSWORD: Uuid = uuid!("00000000-0000-0000-0000-ffff00000059");
pub const UUID_SCHEMA_CLASS_SYSTEM_CONFIG: Uuid = uuid!("00000000-0000-0000-0000-ffff00000060");
pub const _UUID_SCHEMA_ATTR_LOGINSHELL: Uuid = uuid!("00000000-0000-0000-0000-ffff00000061");
pub const _UUID_SCHEMA_ATTR_UNIX_PASSWORD: Uuid = uuid!("00000000-0000-0000-0000-ffff00000062");
pub const UUID_SCHEMA_ATTR_LAST_MOD_CID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000063");
pub const UUID_SCHEMA_ATTR_PHANTOM: Uuid = uuid!("00000000-0000-0000-0000-ffff00000064");
pub const UUID_SCHEMA_ATTR_CLAIM: Uuid = uuid!("00000000-0000-0000-0000-ffff00000065");
pub const UUID_SCHEMA_ATTR_PASSWORD_IMPORT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000066");
pub const _UUID_SCHEMA_ATTR_NSUNIQUEID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000067");
pub const UUID_SCHEMA_ATTR_DN: Uuid = uuid!("00000000-0000-0000-0000-ffff00000068");
pub const _UUID_SCHEMA_ATTR_NICE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000069");
pub const UUID_SCHEMA_ATTR_ENTRYUUID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000070");
pub const UUID_SCHEMA_ATTR_OBJECTCLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000071");
pub const _UUID_SCHEMA_ATTR_ACCOUNT_EXPIRE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000072");
pub const _UUID_SCHEMA_ATTR_ACCOUNT_VALID_FROM: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000073");
pub const UUID_SCHEMA_ATTR_ENTRYDN: Uuid = uuid!("00000000-0000-0000-0000-ffff00000074");
pub const UUID_SCHEMA_ATTR_EMAIL: Uuid = uuid!("00000000-0000-0000-0000-ffff00000075");
pub const UUID_SCHEMA_ATTR_EMAILADDRESS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000076");
pub const UUID_SCHEMA_ATTR_KEYS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000077");
pub const UUID_SCHEMA_ATTR_SSHPUBLICKEY: Uuid = uuid!("00000000-0000-0000-0000-ffff00000078");
pub const UUID_SCHEMA_ATTR_UIDNUMBER: Uuid = uuid!("00000000-0000-0000-0000-ffff00000079");
pub const _UUID_SCHEMA_ATTR_OAUTH2_RS_NAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000080");
pub const _UUID_SCHEMA_ATTR_OAUTH2_RS_ORIGIN: Uuid = uuid!("00000000-0000-0000-0000-ffff00000081");
pub const _UUID_SCHEMA_ATTR_OAUTH2_RS_SCOPE_MAP: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000082");
pub const _UUID_SCHEMA_ATTR_OAUTH2_RS_BASIC_SECRET: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000083");
pub const _UUID_SCHEMA_ATTR_OAUTH2_RS_TOKEN_KEY: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000084");
pub const UUID_SCHEMA_CLASS_OAUTH2_RS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000085");
pub const UUID_SCHEMA_CLASS_OAUTH2_RS_BASIC: Uuid = uuid!("00000000-0000-0000-0000-ffff00000086");
pub const UUID_SCHEMA_ATTR_CN: Uuid = uuid!("00000000-0000-0000-0000-ffff00000087");
pub const UUID_SCHEMA_ATTR_DOMAIN_TOKEN_KEY: Uuid = uuid!("00000000-0000-0000-0000-ffff00000088");
pub const _UUID_SCHEMA_ATTR_OAUTH2_RS_IMPLICIT_SCOPES: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000089");
pub const _UUID_SCHEMA_ATTR_ES256_PRIVATE_KEY_DER: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000090");
pub const _UUID_SCHEMA_ATTR_OAUTH2_ALLOW_INSECURE_CLIENT_DISABLE_PKCE: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000091");
pub const _UUID_SCHEMA_ATTR_OAUTH2_JWT_LEGACY_CRYPTO_ENABLE: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000092");
pub const _UUID_SCHEMA_ATTR_RS256_PRIVATE_KEY_DER: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000093");
pub const _UUID_SCHEMA_CLASS_ORGPERSON: Uuid = uuid!("00000000-0000-0000-0000-ffff00000094");
pub const UUID_SCHEMA_ATTR_FERNET_PRIVATE_KEY_STR: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000095");
pub const _UUID_SCHEMA_ATTR_CREDENTIAL_UPDATE_INTENT_TOKEN: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000096");
pub const _UUID_SCHEMA_CLASS_OAUTH2_CONSENT_SCOPE_MAP: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000097");
pub const _UUID_SCHEMA_ATTR_DOMAIN_DISPLAY_NAME: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000098");
pub const _UUID_SCHEMA_ATTR_PASSKEYS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000099");
pub const _UUID_SCHEMA_ATTR_DEVICEKEYS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000100");

pub const UUID_SCHEMA_ATTR_SYSTEMSUPPLEMENTS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000101");
pub const UUID_SCHEMA_ATTR_SUPPLEMENTS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000102");
pub const UUID_SCHEMA_ATTR_SYSTEMEXCLUDES: Uuid = uuid!("00000000-0000-0000-0000-ffff00000103");
pub const UUID_SCHEMA_ATTR_EXCLUDES: Uuid = uuid!("00000000-0000-0000-0000-ffff00000104");
pub const UUID_SCHEMA_ATTR_SCOPE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000105");
pub const UUID_SCHEMA_CLASS_SERVICE_ACCOUNT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000106");
pub const _UUID_SCHEMA_CLASS_DYNGROUP: Uuid = uuid!("00000000-0000-0000-0000-ffff00000107");
pub const _UUID_SCHEMA_ATTR_DYNGROUP_FILTER: Uuid = uuid!("00000000-0000-0000-0000-ffff00000108");
pub const _UUID_SCHEMA_ATTR_OAUTH2_PREFERR_SHORT_USERNAME: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000109");
pub const _UUID_SCHEMA_ATTR_JWS_ES256_PRIVATE_KEY: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000110");
pub const _UUID_SCHEMA_ATTR_API_TOKEN_SESSION: Uuid = uuid!("00000000-0000-0000-0000-ffff00000111");
pub const _UUID_SCHEMA_ATTR_OAUTH2_RS_SUP_SCOPE_MAP: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000112");
pub const _UUID_SCHEMA_ATTR_USER_AUTH_TOKEN_SESSION: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000113");
pub const _UUID_SCHEMA_CLASS_SYNC_ACCOUNT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000114");
pub const _UUID_SCHEMA_ATTR_SYNC_TOKEN_SESSION: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000115");
pub const _UUID_SCHEMA_ATTR_SYNC_COOKIE: Uuid = uuid!("00000000-0000-0000-0000-ffff00000116");
pub const _UUID_SCHEMA_ATTR_OAUTH2_SESSION: Uuid = uuid!("00000000-0000-0000-0000-ffff00000117");
pub const UUID_SCHEMA_ATTR_ACP_RECEIVER_GROUP: Uuid = uuid!("00000000-0000-0000-0000-ffff00000118");
pub const _UUID_SCHEMA_ATTR_GRANT_UI_HINT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000119");
pub const _UUID_SCHEMA_ATTR_OAUTH2_RS_ORIGIN_LANDING: Uuid =
    uuid!("00000000-0000-0000-0000-ffff00000120");

pub const UUID_SCHEMA_ATTR_SYNC_EXTERNAL_ID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000121");
pub const UUID_SCHEMA_ATTR_SYNC_PARENT_UUID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000122");
pub const UUID_SCHEMA_CLASS_SYNC_OBJECT: Uuid = uuid!("00000000-0000-0000-0000-ffff00000123");
pub const UUID_SCHEMA_ATTR_SYNC_CLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000124");
pub const UUID_SCHEMA_ATTR_SYNC_ALLOWED: Uuid = uuid!("00000000-0000-0000-0000-ffff00000125");

// System and domain infos
// I'd like to strongly criticise william of the past for making poor choices about these allocations.
pub const UUID_SYSTEM_INFO: Uuid = uuid!("00000000-0000-0000-0000-ffffff000001");
pub const STR_UUID_DOMAIN_INFO: &str = "00000000-0000-0000-0000-ffffff000025";
pub const UUID_DOMAIN_INFO: Uuid = uuid!("00000000-0000-0000-0000-ffffff000025");

// DO NOT allocate here, allocate below.

// Access controls
// skip 00 / 01 - see system info
pub const _UUID_IDM_ADMINS_ACP_RECYCLE_SEARCH_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000002");
pub const _UUID_IDM_ADMINS_ACP_REVIVE_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000003");
pub const _UUID_IDM_SELF_ACP_READ_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000004");
pub const _UUID_IDM_ALL_ACP_READ_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000006");
pub const _UUID_IDM_ACP_PEOPLE_READ_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000007");
pub const _UUID_IDM_ACP_PEOPLE_WRITE_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000008");
pub const _UUID_IDM_ACP_GROUP_WRITE_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000009");
pub const _UUID_IDM_ACP_ACCOUNT_READ_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000010");
pub const _UUID_IDM_ACP_ACCOUNT_WRITE_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000011");
pub const _UUID_IDM_ACP_ACCOUNT_MANAGE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000012");
pub const _UUID_IDM_ACP_PEOPLE_MANAGE_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000013");
pub const _UUID_IDM_ACP_RADIUS_SERVERS_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000014");
pub const _UUID_IDM_ACP_HP_ACCOUNT_READ_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000015");
pub const _UUID_IDM_ACP_HP_ACCOUNT_WRITE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000016");
pub const _UUID_IDM_ACP_HP_GROUP_WRITE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000017");
pub const _UUID_IDM_ACP_SCHEMA_WRITE_ATTRS_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000018");
pub const _UUID_IDM_ACP_ACP_MANAGE_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000019");
pub const _UUID_IDM_ACP_SCHEMA_WRITE_CLASSES_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000020");
pub const _UUID_IDM_SELF_ACP_WRITE_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000021");
pub const _UUID_IDM_ACP_GROUP_MANAGE_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000022");
pub const _UUID_IDM_ACP_HP_ACCOUNT_MANAGE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000023");
pub const _UUID_IDM_ACP_HP_GROUP_MANAGE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000024");
// Skip 25 - see domain info.
pub const _UUID_IDM_ACP_DOMAIN_ADMIN_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000026");

pub const STR_UUID_SYSTEM_CONFIG: &str = "00000000-0000-0000-0000-ffffff000027";
pub const UUID_SYSTEM_CONFIG: Uuid = uuid!("00000000-0000-0000-0000-ffffff000027");

pub const _UUID_IDM_ACP_SYSTEM_CONFIG_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000028");
pub const _UUID_IDM_ACP_ACCOUNT_UNIX_EXTEND_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000029");
pub const _UUID_IDM_ACP_GROUP_UNIX_EXTEND_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000030");
pub const _UUID_IDM_ACP_PEOPLE_ACCOUNT_PASSWORD_IMPORT_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000031");
pub const _UUID_IDM_ACP_PEOPLE_EXTEND_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000032");
pub const _UUID_IDM_HP_ACP_ACCOUNT_UNIX_EXTEND_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000033");
pub const _UUID_IDM_HP_ACP_GROUP_UNIX_EXTEND_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000034");
pub const _UUID_IDM_HP_ACP_OAUTH2_MANAGE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000035");
pub const _UUID_IDM_ACP_HP_PEOPLE_READ_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000036");
pub const _UUID_IDM_ACP_HP_PEOPLE_WRITE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000037");
pub const _UUID_IDM_ACP_HP_PEOPLE_EXTEND_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000038");
pub const _UUID_IDM_ACP_RADIUS_SECRET_READ_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000039");
pub const _UUID_IDM_ACP_RADIUS_SECRET_WRITE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000040");
pub const _UUID_IDM_PEOPLE_SELF_ACP_WRITE_MAIL_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000041");
pub const _UUID_IDM_HP_ACP_SERVICE_ACCOUNT_INTO_PERSON_MIGRATE_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000042");
pub const _UUID_IDM_ACP_OAUTH2_READ_PRIV_V1: Uuid = uuid!("00000000-0000-0000-0000-ffffff000043");
pub const _UUID_IDM_HP_ACP_SYNC_ACCOUNT_MANAGE_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000044");
pub const UUID_IDM_ACP_ACCOUNT_MAIL_READ_PRIV_V1: Uuid =
    uuid!("00000000-0000-0000-0000-ffffff000045");

// End of system ranges
pub const UUID_DOES_NOT_EXIST: Uuid = uuid!("00000000-0000-0000-0000-fffffffffffe");
pub const UUID_ANONYMOUS: Uuid = uuid!("00000000-0000-0000-0000-ffffffffffff");