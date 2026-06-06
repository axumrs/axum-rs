# AXUM 中文网

鉴于 nodejs 对性能的严重影响，我们将 AXUM 中文网由之前的 nodejs 工具链进行渲染，改为由 axum 直接对 UI 进行渲染。

我们将使用以下技术栈：

- axum
- postgres
- askama
- rust embed
- htmx
- tailwind css（with basecoat ui）
- alpine.js

以此为契机，我们：

- 对 AXUM 中文网进行再次重构。为了保持职责明晰，我们依然将整站划分为几个独立部分，并由不同的子域名提供服务
    - API 服务：提供全站的 RESTFul API 服务，域名：`api.axum.eu.org`
    - WEB 服务：就是直接呈现给用户的服务，域名：`axum.eu.org`
    - OAuth 服务：提供 OAuth 2.0 接入服务，域名：`oauth.axum.eu.org`
    - 后台：提供后台管理，只有管理员可操作，域名：`admin.axum.eu.org`
- 除了完成对已有功能的重构，我们还将增加以下功能
    - 站内通知：使用 SSE 技术，对订单、支付、订阅等进行系统级的站内通知
    - OAuth
        - 我们将实现并开放 OAuth 2.0, 通过接入我们的 OAuth 2.0，你也可以使用 AXUM 中文网的登录功能
        - 我们将把通过邮箱注册的功能进行关闭（免费的邮件服务送达率太低），而改由第三方 OAuth 2.0 登录，比如 github
