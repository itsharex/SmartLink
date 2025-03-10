# SmartLink 聊天功能使用流程

本文档详细介绍了 SmartLink 聊天功能的每个主要函数的使用流程，包括初始化、创建会话、发送消息、接收消息等各个环节的详细说明。

## 1. 用户认证流程

### 1.1 用户登录
- 前端调用 `auth::commands::login` 函数进行用户登录
- 登录成功后，后端调用 `auth::utils::set_current_user_id` 设置当前用户ID
- 此时发送 `user_logged_in` 事件，触发 WebSocket 连接的建立

### 1.2 用户身份验证
- 每次聊天操作前，通过 `get_current_user_id` 函数验证用户是否已登录
- 如果用户未登录，返回错误提示

## 2. 聊天初始化流程

### 2.1 建立 WebSocket 连接
1. **前端调用 `initialize_chat_connection`**：
   ```javascript
   await invoke('initialize_chat_connection', { 
     userId: currentUser.id
   });
   ```
   
2. **后端处理**：
   - `WebSocketManager` 创建一个通道用于与前端通信
   - `ChatManager` 将用户注册为在线用户
   - 获取并发送该用户的离线消息
   - 通知其他用户该用户上线

3. **前端监听 WebSocket 事件**：
   ```javascript
   await listen('chat_event', (event) => {
     // 处理聊天事件
     handleChatEvent(event.payload);
   });
   ```

## 3. 会话管理流程

### 3.1 获取会话列表
1. **前端调用 `get_conversations`**：
   ```javascript
   const conversations = await invoke('get_conversations');
   ```

2. **后端处理**：
   - 验证用户身份（`get_current_user_id`）
   - `ChatManager.get_conversations` 从数据库获取用户参与的所有会话
   - 返回会话列表，按最后消息时间排序

### 3.2 创建新会话
1. **前端调用 `create_conversation`**：
   ```javascript
   const newConversation = await invoke('create_conversation', {
     request: {
       conversation_type: 'Direct', // 或 'Group'
       participants: [currentUser.id, otherUserId],
       name: groupName, // 可选，用于群聊
       avatar_url: avatarUrl // 可选
     }
   });
   ```

2. **后端处理**：
   - 验证用户身份
   - `ChatManager.create_conversation` 创建新会话记录
   - 广播 `ConversationUpdated` 事件给所有参与者
   - 返回新创建的会话数据

## 4. 消息收发流程

### 4.1 发送消息
1. **前端调用 `send_message`**：
   ```javascript
   const message = await invoke('send_message', {
     request: {
       conversation_id: conversationId,
       content: messageText,
       content_type: 'Text' // 或其他类型
     }
   });
   ```

2. **后端处理**：
   - 验证用户身份及会话参与权限
   - `ChatManager.send_message` 将消息保存到数据库
   - 广播 `NewMessage` 事件给所有参与者
   - 更新会话的最后消息信息
   - 返回新发送的消息数据

### 4.2 获取消息历史
1. **前端调用 `get_messages`**：
   ```javascript
   const messages = await invoke('get_messages', {
     request: {
       conversation_id: conversationId,
       limit: 20, // 分页大小
       before_id: oldestMessageId // 用于加载更多，可选
     }
   });
   ```

2. **后端处理**：
   - 验证用户身份及会话参与权限
   - `ChatManager.get_messages` 从数据库获取指定会话的消息
   - 分页返回消息列表，支持加载更早的消息

### 4.3 接收实时消息
1. **WebSocket 事件处理**：
   - 前端监听 `chat_event` 事件
   - 当收到 `NewMessage` 类型事件时，将消息添加到对应会话的消息列表中
   - 更新会话的最后消息信息

## 5. 已读状态流程

### 5.1 标记消息已读
1. **前端调用 `mark_as_read`**：
   ```javascript
   await invoke('mark_as_read', {
     request: {
       conversation_id: conversationId,
       message_id: messageId
     }
   });
   ```

2. **后端处理**：
   - 验证用户身份及会话参与权限
   - `ChatManager.mark_as_read` 更新消息的已读状态
   - 检查并更新会话的最后消息已读状态
   - 广播 `MessageRead` 事件给所有参与者

### 5.2 处理已读通知
1. **WebSocket 事件处理**：
   - 前端监听 `chat_event` 事件
   - 当收到 `MessageRead` 类型事件时，更新对应消息的已读状态
   - 更新UI显示（如已读标记）

## 6. 输入状态流程

### 6.1 发送输入状态
1. **前端调用 `update_typing_status`**：
   ```javascript
   await invoke('update_typing_status', {
     conversation_id: conversationId,
     is_typing: true // 或 false
   });
   ```

2. **后端处理**：
   - 验证用户身份及会话参与权限
   - `ChatManager.update_typing_status` 广播 `ParticipantTyping` 事件给所有参与者

### 6.2 处理输入状态通知
1. **WebSocket 事件处理**：
   - 前端监听 `chat_event` 事件
   - 当收到 `ParticipantTyping` 类型事件时，更新对应用户的输入状态
   - 在UI中显示"XXX正在输入..."

## 7. 用户在线状态流程

### 7.1 上线通知
1. **用户登录后自动处理**：
   - `ChatManager.register_user` 将用户标记为在线
   - 广播 `ParticipantOnlineStatus` 事件（status: "online"）给相关会话参与者

### 7.2 离线通知
1. **用户登出或关闭应用时自动处理**：
   - `ChatManager.unregister_user` 将用户标记为离线
   - 广播 `ParticipantOnlineStatus` 事件（status: "offline"）给相关会话参与者

### 7.3 处理在线状态通知
1. **WebSocket 事件处理**：
   - 前端监听 `chat_event` 事件
   - 当收到 `ParticipantOnlineStatus` 类型事件时，更新对应用户的在线状态
   - 在UI中显示用户在线状态（如绿点）

## 8. 聊天WebSocket事件类型汇总

| 事件类型 | 说明 | 数据结构 |
|---------|------|---------|
| `NewMessage` | 新消息 | `{ message: Message }` |
| `MessageRead` | 消息已读 | `{ message_id: String, user_id: String, timestamp: DateTime }` |
| `ConversationUpdated` | 会话更新 | `{ action: "created"/"updated", conversation: Conversation }` |
| `ParticipantTyping` | 输入状态 | `{ user_id: String, is_typing: boolean }` |
| `ParticipantOnlineStatus` | 在线状态 | `{ user_id: String, status: "online"/"offline" }` |

## 9. 完整流程示例

### 9.1 私聊流程
1. 用户A登录应用，建立WebSocket连接
2. 用户A获取会话列表（`get_conversations`）
3. 用户A创建与用户B的私聊（`create_conversation`）
4. 用户B收到新会话通知（WebSocket `ConversationUpdated` 事件）
5. 用户A发送消息（`send_message`）
6. 用户B收到新消息通知（WebSocket `NewMessage` 事件）
7. 用户B打开会话，加载消息历史（`get_messages`）
8. 用户B查看消息，标记为已读（`mark_as_read`）
9. 用户A收到已读通知（WebSocket `MessageRead` 事件）
10. 用户B开始输入回复，发送输入状态（`update_typing_status`, is_typing=true）
11. 用户A收到输入状态通知（WebSocket `ParticipantTyping` 事件）
12. 用户B发送回复消息（`send_message`）
13. 用户B停止输入，发送输入状态（`update_typing_status`, is_typing=false）

### 9.2 群聊流程
1. 用户A创建群聊，邀请用户B、C（`create_conversation`，type=Group）
2. 用户B、C收到新会话通知（WebSocket `ConversationUpdated` 事件）
3. 用户A发送群消息（`send_message`）
4. 用户B、C收到新消息通知（WebSocket `NewMessage` 事件）
5. 用户B查看消息，标记为已读（`mark_as_read`）
6. 用户A、C收到已读通知（WebSocket `MessageRead` 事件）
7. 群聊消息显示部分已读状态
8. 用户C也查看消息并标记为已读
9. 群聊消息更新为全部已读状态