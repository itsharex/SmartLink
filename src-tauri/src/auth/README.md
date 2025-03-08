# SmartLink认证模块流程解析

## 1. 用户注册流程

```
前端调用register_user命令 -> commands.rs:register_user函数 
-> 获取AuthManager实例 -> AuthManager::register_user 
-> 检查用户是否存在(UserDatabase::user_exists) 
-> 创建新用户(User::new_with_email) 
-> 哈希密码(bcrypt::hash) 
-> 存储用户(UserDatabase::create_user) 
-> 生成JWT令牌(generate_token) 
-> 返回LoginResponse到前端
```

## 2. 邮箱密码登录流程

```
前端调用login_with_email命令 -> commands.rs:login_with_email函数 
-> 获取AuthManager实例 -> AuthManager::login_with_email 
-> 获取用户和密码哈希(UserDatabase::get_user_with_password) 
-> 验证密码(bcrypt::verify) 
-> 设置当前用户 
-> 更新最后登录时间(UserDatabase::update_last_login) 
-> 生成JWT令牌(generate_token) 
-> 返回LoginResponse到前端
```

## 3. OAuth登录URL获取流程

```
前端调用get_oauth_url命令 -> commands.rs:get_oauth_url函数 
-> 解析OAuth提供商类型(parse_oauth_provider) 
-> 获取AuthManager实例 -> AuthManager::login_with_oauth 
-> OAuthManager::get_authorization_url 
-> 创建PKCE挑战和验证器 
-> 构建授权URL和CSRF令牌 
-> 存储CSRF令牌和PKCE验证器 
-> 返回授权URL到前端
```

## 4. OAuth回调处理流程

```
前端调用handle_oauth_callback命令 -> commands.rs:handle_oauth_callback函数 
-> 解析OAuth提供商类型(parse_oauth_provider) 
-> 获取AuthManager实例 -> AuthManager::handle_oauth_callback 
-> OAuthManager::handle_callback 
-> 交换授权码获取令牌(client.exchange_code) 
-> 获取用户信息(fetch_google_user_info/fetch_github_user_info) 
-> 检查用户是否存在(UserDatabase::find_user_by_oauth_id) 
-> 如存在:更新OAuth信息(UserDatabase::update_oauth_info) 
-> 如不存在:创建新用户(User::new_with_oauth -> UserDatabase::create_user_with_oauth) 
-> 更新最后登录时间(UserDatabase::update_last_login) 
-> 生成JWT令牌(generate_token) 
-> 返回LoginResponse到前端
```

## 5. 获取当前用户流程

```
前端调用get_current_user命令 -> commands.rs:get_current_user函数 
-> 获取AuthManager实例 -> AuthManager::get_current_user 
-> 检查当前用户是否设置 
-> 返回用户信息或Unauthenticated错误到前端
```

## 6. 登出流程

```
前端调用logout命令 -> commands.rs:logout函数 
-> 获取AuthManager实例 -> AuthManager::logout 
-> 清除当前用户信息 
-> 返回成功结果到前端
```

## 7. 刷新令牌流程

```
前端调用refresh_token命令 -> commands.rs:refresh_token函数 
-> 获取AuthManager实例 -> AuthManager::get_current_user 
-> 生成新JWT令牌(generate_token) 
-> 返回LoginResponse到前端
```

## 8. 初始化认证系统流程

```
Tauri应用启动 -> main.rs -> app.setup调用commands.rs:init函数 
-> 从环境变量读取配置信息 
-> 初始化MongoDB客户端(AuthManager::init) 
-> 设置全局AUTH_MANAGER实例 
-> 配置OAuth客户端 
-> 应用继续启动
```

## 9. MongoDB日期时间处理流程

```
当需要存储日期时间到MongoDB时: 
chrono::DateTime<Utc> -> utils.rs:datetime_to_bson 
-> 提取时间戳毫秒值 -> mongodb::bson::DateTime::from_millis 
-> 创建Bson::DateTime -> 插入MongoDB文档

当从MongoDB读取日期时间时:
Bson::DateTime -> utils.rs:bson_to_datetime 
-> 提取时间戳毫秒值 -> 计算秒和纳秒 
-> chrono::DateTime::from_timestamp 
-> 返回chrono::DateTime<Utc>
```

## 核心数据流路径

1. **用户认证**：前端 -> Tauri命令 -> AuthManager -> UserDatabase -> MongoDB
2. **OAuth流程**：前端 -> Tauri命令 -> AuthManager -> OAuthManager -> 外部OAuth服务 -> 数据返回 -> UserDatabase -> MongoDB
3. **令牌生成**：AuthManager -> 生成JWT -> 返回前端
4. **日期处理**：chrono DateTime <-> utils.rs转换函数 <-> MongoDB BSON DateTime

这些流程展示了SmartLink认证模块的数据如何在各组件间流动，从前端一直到数据库，以及如何处理外部服务(如OAuth提供商)的交互。