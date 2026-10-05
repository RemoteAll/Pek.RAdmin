# Pek.RAdmin

Pek 管理组件（Rust）：**管理系统的"用户 + 权限 + 操作审计"通用数据层**。

- 定位：**应用组装件**（不是 ORM/基础库核心）——只有实现"管理界面/管理后台"的项目才需要引入；
- 数据访问基于 [Pek.RCode](../Pek.RCode)（XCode 模型 + SQLite/多库）；Web 会话能力由 `dhrust` 与各应用装配；
- 消费方：Pek.RAgent（Web 面板）、HlkProductTool（面板用户与记录页）。
