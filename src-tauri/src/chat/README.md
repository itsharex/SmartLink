# SmartLink 聊天模块数据流程

## 概述

SmartLink 聊天功能采用了分层设计，从前端到后端形成了完整的数据流程，确保消息传递的安全和可靠。以下是详细的数据流程：

## 前端发送消息流程

1. **用户界面** -> **Tauri 调用** -> **ChatCommands** -> **ChatManager** -> **数据库存储**

   具体步骤：
   - 用户在界面上输入消息并点击发送
   - 前端通过 `invoke("send_message", {...})` 调用 Tauri 命令
   - `chat_commands.rs` 接收请求并转发给 ChatManager
   - ChatManager 处理消息（可能包括加密）
   - 消息被存储到 MongoDB 数据库

2. **加密流程**（当启用端到端加密时）：
   - **ChatManager** -> **KeyManager** -> **EncryptionService**
   - 使用接收者的公钥和发送者的私钥生成共享密钥
   - 使用 AES-GCM 算法加密消息内容
   - 加密后的消息和随机数（nonce）被存储

## 接收消息流程

1. **主动获取消息**：
   - **前端轮询** -> **Tauri 调用** -> **ChatCommands** -> **ChatManager** -> **数据库查询**
   - 前端调用 `get_messages` 从服务器获取最新消息
   - ChatManager 从数据库检索消息
   - 如果消息是加密的，ChatManager 解密后返回给前端

2. **WebSocket 实时通知**：
   - **WebSocket 服务器** -> **前端接收** -> **本地状态更新**
   - WebSocket 服务器推送新消息通知（不包含消息内容，只有元数据）
   - 前端收到通知后调用 `get_messages` 获取完整消息
   - 前端更新本地状态和 UI 显示

3. **解密流程**（当启用端到端加密时）：
   - **ChatManager** -> **SessionKeyStore** -> **EncryptionService**
   - 使用接收者与发送者的共享密钥
   - 解密消息内容后返回给前端

## 消息状态更新流程

1. **更新已读/已送达状态**：
   - **前端操作** -> **Tauri 调用** -> **ChatCommands** -> **ChatManager** -> **数据库更新**
   - 前端调用 `update_message_status` 或相关方法
   - ChatManager 验证请求并更新数据库中的消息状态
   - 状态变更通过 WebSocket 通知原发送者

2. **批量标记已读**：
   - **前端操作** -> **Tauri 调用** -> **ChatCommands** -> **ChatManager** -> **数据库批量更新**
   - 前端调用 `mark_conversation_read` 方法
   - ChatManager 批量更新该会话中的所有未读消息状态

## 群组管理流程

1. **创建群组**：
   - **前端操作** -> **Tauri 调用** -> **ChatCommands** -> **ChatManager** -> **数据库创建**
   - ChatManager 创建新的会话记录
   - 如果启用加密，为所有参与者创建密钥对

2. **添加/移除成员**：
   - **前端操作** -> **Tauri 调用** -> **ChatCommands** -> **ChatManager** -> **数据库更新**
   - ChatManager 更新会话参与者列表
   - 如果启用加密，管理加密密钥（添加新密钥或吊销现有密钥）

## 安全考量

- **密钥管理**：密钥生成和存储均在本地完成，不经过服务器
- **消息加密**：使用 X25519 进行密钥交换，AES-GCM 进行消息加密
- **权限验证**：每个操作都验证发起者是否有权限执行该操作

## 数据存储

- **会话数据**：存储在 MongoDB `conversations` 集合中
- **消息数据**：存储在 MongoDB `messages` 集合中
- **密钥数据**：仅存储在内存中，确保安全性