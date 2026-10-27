//! Value objects for the auth domain.

#[macro_use]
pub(crate) mod id_macros;

// UUID ID types
id_macros::uuid_id!(UserId, serde);
id_macros::uuid_id!(RefreshTokenId);
id_macros::uuid_id!(VerificationCodeId);
id_macros::uuid_id!(ResetCodeId);
id_macros::uuid_id!(UserAgentId, serde);
id_macros::uuid_id!(TierChangeId, serde);
id_macros::uuid_id!(RefundSettlementId, serde);
id_macros::uuid_id!(RefundAttemptId, serde);

pub mod email;

pub mod user_role;

pub use email::Email;

pub use user_role::UserRole;
