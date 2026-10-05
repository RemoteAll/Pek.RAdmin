//! Pek 管理组件（Rust）：管理系统的“用户 + 权限 + 操作审计”通用数据层。
//!
//! **定位：应用组装件，不是 ORM/基础库核心。**只有实现“管理界面/管理后台”功能的
//! 项目才需要引入本 crate；纯组件用法（DH.RustBase / Pek.RCode / Pek.RRedis 等）不依赖它。
//!
//! - 数据访问基于 [`pek_rcode`]（XCode 模型 + SQLite/多库）；
//! - 会话/令牌与 Web 框架能力仍由 `dhrust`（`net::panel_auth` 等）与各应用装配。
pub mod panel;
