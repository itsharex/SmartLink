# Contacts 模块文档

## 概述

Contacts 模块负责管理用户之间的联系人关系，包括添加好友、管理好友请求、收藏联系人等功能。该模块采用独立的数据集合来存储用户关系，实现了灵活的联系人管理功能。

## 数据结构

模块使用以下关键数据集合:
- `users`: 存储用户基本信息
- `contacts`: 存储用户之间的联系人关系
- `friend_requests`: 存储好友请求信息

## 功能调用流程

### 1. 搜索用户
```
前端 -> search_users命令 -> db::search_users -> MongoDB查询(users集合) -> 返回用户列表
```

### 2. 好友请求流程
```
前端 -> send_friend_request命令 -> db::create_friend_request -> 
  检查是否已是好友 -> 
  检查是否已有待处理请求 -> 
  创建好友请求记录(friend_requests集合) -> 
  返回结果
```

### 3. 获取好友请求
```
前端 -> get_friend_requests命令 -> db::get_friend_requests -> 
  查询待处理请求(friend_requests集合) -> 
  获取发送者信息(users集合) -> 
  返回请求列表
```

### 4. 接受好友请求
```
前端 -> accept_friend_request命令 -> db::accept_friend_request -> 
  更新请求状态为accepted -> 
  创建双向联系人关系(contacts集合中创建两条记录) -> 
  返回结果
```

### 5. 拒绝好友请求
```
前端 -> reject_friend_request命令 -> db::reject_friend_request -> 
  更新请求状态为rejected -> 
  返回结果
```

### 6. 获取联系人列表
```
前端 -> get_contacts命令 -> db::get_user_contacts -> 
  获取用户的联系人记录(contacts集合) -> 
  查询每个联系人的详细信息(users集合) -> 
  返回联系人列表
```

### 7. 管理收藏联系人
```
添加收藏:
前端 -> add_contact_to_favorites命令 -> db::add_contact_to_favorites -> 
  更新联系人is_favorite字段为true -> 
  返回结果

移除收藏:
前端 -> remove_contact_from_favorites命令 -> db::remove_contact_from_favorites -> 
  更新联系人is_favorite字段为false -> 
  返回结果

获取收藏联系人:
前端 -> get_favorite_contacts命令 -> db::get_favorite_contacts -> 
  查询用户收藏的联系人(contacts集合) -> 
  获取每个联系人的详细信息(users集合) -> 
  返回收藏联系人列表
```

## 数据模型说明

- 联系人关系是双向的，在 `contacts` 集合中以两条记录表示
- 每个联系人关系可以包含标签、收藏状态等个性化设置
- 好友请求状态可以是 pending、accepted 或 rejected

## 实现特点

- 使用分离的数据集合设计，符合数据库最佳实践
- 实现双向好友关系，每个用户可独立管理联系人设置
- 支持联系人标签和收藏功能，便于个性化管理
- 检查重复添加和请求，避免数据冗余