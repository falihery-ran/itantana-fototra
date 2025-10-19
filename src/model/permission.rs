pub mod error;

pub type Permission = String;

pub const ALL_PERMISSIONS: &[&str] = &[
    "user:create",
    "user:update",
    "user:delete",
    "user:find",
    "user:find_one",
    "user_internet:create",
    "user_internet:delete",
    "user_internet:find",
    "user_permission:create",
    "user_permission:delete",
    "user_permission:find",
    "user_password:create",
    "user_password:match",
];
