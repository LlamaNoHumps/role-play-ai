# Role Play AI 架构设计文档

## 1. 项目概述

Role Play AI 是一个基于 Rust 的 AI 角色扮演平台。平台支持用户注册登录、角色创建管理、实时对话、AI辩论等核心功能。

## 2. 技术栈

### 2.1 后端技术栈
- **语言**: Rust (Edition 2024)
- **Web框架**: Axum 0.8.4
- **数据库**: MySQL + SeaORM 1.1.16
- **实时通信**: SocketIO
- **认证**: JWT
- **AI服务**: 七牛云AI API
- **存储**: 七牛云对象存储
- **异步运行时**: Tokio 1.47.1

### 2.2 前端技术栈
- **语言**: HTML5 + JavaScript (ES6+)
- **实时通信**: Socket.IO Client
- **UI**: 原生CSS + 响应式设计
- **音频播放**: Web Audio API

## 3. 系统架构
```
🌐 前端层 Frontend Layer
├── 📱 主界面模块 (Home UI)
├── 💬 对话界面模块 (Chat UI) 
├── 🎭 辩论界面模块 (Debate UI)
├── 👤 用户管理模块 (User UI)
├── ⚡ 实时通信层 (Socket.IO)
│
└──连接至后端服务 ▼

🔧 后端层 Backend Layer
├── API网关层 (Axum Router)
│   ├── 🔐 认证模块 (Auth)
│   ├── ⚙️ 中间件层 (Middleware)
│   └── 🛠️ 路由处理模块 (Handlers)
│
├── 业务逻辑层 (Business Layer)
│   ├── 🤖 AI代理层 (Agents)
│   ├── 📦 存储服务 (Storage)
│   ├── 🔄 Socket处理层 (Sockets)
│   └── 💾 数据层 (Data Layer)
│       ├── 🗄️ 数据库层 (Database)
│       ├── 📊 模型层 (Models)
│       └── ⚡ 缓存层 (Cache)
│
└── 连接至外部服务 ▼

🌍 外部服务层 External Services
├── 🧠 七牛云AI (AI API)
├── ☁️ 七牛云存储 (Qiniu Storage)
├── 🌍 维基百科 (Wikipedia)
└── 🗄️ MySQL数据库 (Database)
```

## 4. 模块规格

### 4.1 认证模块 (Auth Module)
**路径**: `src/server/auth.rs`

**功能**: 
- JWT令牌生成和验证
- 用户身份认证
- 权限控制

**主要组件**:
- `Claims` - JWT声明结构
- `Auth` - 认证服务

### 4.2 数据库模块 (Database Module)
**路径**: `src/database/`

**功能**:
- 数据库连接管理
- 数据模型定义
- CRUD操作

**主要组件**:
- `Database` - 数据库连接和操作
- `models/` - 数据模型定义

### 4.3 AI代理模块 (Agents Module)
**路径**: `src/agents/`

**功能**:
- AI对话生成
- 语音合成与识别
- 角色构建
- 辩论
- 对话摘要总结

**主要组件**:
- `AI` - AI服务客户端
- `Debater` - 辩论
- `RoleBuilder` - 角色构建器
- `Reciter` - 语音合成
- `Recorder` - 语音识别
- `Summarizer` - 对话摘要总结

### 4.4 存储模块 (Storage Module)
**路径**: `src/storage/`

**功能**:
- 文件上传下载
- 对象存储管理

### 4.5 Socket通信模块 (Sockets Module)
**路径**: `src/server/sockets/`

**功能**:
- 实时消息传递
- 语音消息处理
- 房间管理

**主要组件**:
- `message` - 文本消息处理
- `voice` - 语音消息处理
- `join` - 房间管理

## 5. API接口设计

### 5.1 认证接口

#### 5.1.1 用户注册
```
POST /api/auth/register
```

**请求参数**:
```rust
struct RequestParams {
    username: String,      // 用户名
    password: String,      // 密码哈希
    avatar: String,        // 头像URL
}
```

**响应**: `200 OK` 或错误信息

**流程**:
- 查找用户是否已存在
- 创建新用户

#### 5.1.2 用户登录
```
POST /api/auth/login
```

**请求参数**:
```rust
struct RequestParams {
    username: String,      // 用户名
    password: String,      // 密码哈希
}
```

**响应**:
```rust
struct ResponseData {
    user_id: i32,          // 用户ID
    username: String,      // 用户名
    avatar: String,        // 头像URL
    token: String,         // JWT令牌
}
```

**流程**:
- 验证用户名和密码
- 生成JWT令牌
- 返回用户信息和令牌

#### 5.1.3 令牌验证
```
GET /api/auth/verify
Authorization: Bearer <token>
```

**响应**:
```rust
struct ResponseData {
    user_id: i32,          // 用户ID
    username: String,      // 用户名
    avatar: String,        // 头像URL
}
```
或401未授权

**流程**:
- 验证JWT令牌
- 返回用户信息

### 5.2 角色管理接口

#### 5.2.1 角色列表
```
GET /api/role/list?offset={offset}&limit={limit}
```

**查询参数**:
- `offset`: 偏移量 (i64)
- `limit`: 限制数量 (i64)

**响应**:
```rust
struct PaginatedResponse {
    items: Vec<RoleData>,   // 角色列表
    total: i64,             // 总数量
    has_more: bool,         // 是否还有更多
}

struct RoleData {
    role_id: i32,              // 角色ID
    user_id: i32,              // 创建者ID
    name: String,              // 角色名称
    description: String,       // 角色描述
    traits: String,            // 角色特征
    image_url: Option<String>, // 角色头像
    gender: String,            // 性别
    age_group: String,         // 年龄组
    voice_type: String,        // 语音类型
}
```

**流程**:
- 分页查询角色列表
- 返回角色数据

#### 5.2.2 角色搜索
```
GET /api/role/search?q={keyword}
```

**查询参数**:
- `q`: 搜索关键词 (String)

**响应**:
```rust
Vec<ResponseItem>

struct ResponseItem {
    role_id: i32,          // 角色ID
    name: String,          // 角色名称
    description: String,   // 角色描述
    traits: String,        // 角色特征
    image_url: String,     // 角色头像
    gender: String,        // 性别
    age_group: String,     // 年龄组
    voice_type: String,    // 语音类型
}
```

**流程**:
- 模糊匹配角色名称和描述进行全局搜索
- 返回匹配结果

#### 5.2.3 创建角色
```
POST /api/role/create
Authorization: Bearer <token>
```

**请求参数**:
```rust
#[derive(Deserialize)]
pub struct RequestParams {
    user_id: i32,          // 创建者ID
    name: String,          // 角色名称
    description: String,   // 角色描述
    traits: String,        // 角色特征
    avatar: String,        // 角色头像
    gender: String,        // 性别
    age_group: String,     // 年龄组
    voice_type: String,    // 语音类型
}
```

**响应**:
```rust
struct ResponseData {
    role_id: i32,          // 角色ID
}
```

**流程**:
- 创建角色
- 返回角色ID

#### 5.2.4 角色详情
```
GET /api/role/details/{role_id}
```

**查询参数**:
- `role_id`: 角色ID (i32)

**响应**:
```rust
struct ResponseData {
    role_id: i32,          // 角色ID
    name: String,          // 角色名
    description: String,   // 角色描述
    traits: String,        // 角色特点
    image_url: String,     // 头像URL
}
```

**流程**:
- 查询角色
- 返回角色详情

#### 5.2.5 生成角色
```
POST /api/role/auto-fill
Authorization: Bearer <token>
```

**请求参数**:
```rust
struct RequestParams {
    name: String,                 // 角色名
    description: Option<String>,  // 角色描述
    traits: Option<String>,       // 角色特点
    gender: Option<String>,       // 性别
    age_group: Option<String>,    // 年龄组
    sid: String,                  // Socket ID
}
```

**响应**:
```rust
struct ResponseData {
    description: String,   // 角色描述
    traits: String,        // 角色特点
    gender: String,        // 性别
    age_group: String,     // 年龄
    voice_type: String,    // 语音类型
}
```

**流程**:
- 根据角色名调用AI生成能用于Wiki查询的精确名称
- 从Wiki上尝试获取摘要
- 根据角色名和摘要调用AI生成角色特征，若摘要为空则按常识生成
- 根据角色特征调用AI选择语音类型
- 根据角色特征调用AI生成角色描述和角色特点
- 返回生成结果

### 5.3 对话管理接口

#### 5.3.1 创建对话
```
POST /api/conversation/new
Authorization: Bearer <token>
```

**请求参数**:
```rust
struct RequestParams {
    user_id: i32,          // 用户ID
    role_id: i32,          // 角色ID
}
```

**流程**:
- 创建对话

#### 5.3.2 对话列表
```
GET /api/conversation/list?offset={offset}&limit={limit}
Authorization: Bearer <token>
```

**查询参数**:
- `user_id`: 用户ID (i32，必填)
- `offset`: 偏移量 (i64)
- `limit`: 限制数量 (i64)

**响应**:
```rust
struct PaginatedResponse {
    items: Vec<i32>,   // 角色ID列表
    total: i64,        // 总数量
    has_more: bool,    // 是否还有更多
}
```

**流程**:
- 分页查询对话列表
- 返回角色ID列表

#### 5.3.3 对话记录
```
GET /api/conversation/dialogs?role_id={role_id}&limit={limit}
Authorization: Bearer <token>
```

**查询参数**:
- `user_id`: 用户ID (i32，必填)
- `role_id`: 角色ID (i32，必填)
- `offset`: 偏移量 (i64)
- `limit`: 限制数量 (i64)

**响应**:
```rust
struct PaginatedResponse {
    items: Vec<ResponseDataItem>,  // 历史对话列表
    total: i64,                    // 总数量
    has_more: bool,                // 是否还有更多
}

struct ResponseDataItem {
    is_user: bool,          // 是否是用户的发言
    timestamp: i64,         // 时间戳（毫秒）
    text: String,           // 消息文本
    voice: Option<String>,  // 文本的语音
}
```

**流程**:
- 分页查询对话记录
- 返回对话记录

#### 5.3.4 删除对话
```
POST /api/conversation/delete/{user_id}/{role_id}
Authorization: Bearer <token>
```

**路径参数**:
- `user_id`: 用户ID (i32)
- `role_id`: 角色ID (i32)

**流程**:
- 删除对话

### 5.4 辩论管理接口

#### 5.4.1 创建辩论
```
POST /api/debate/new
Authorization: Bearer <token>
```

**请求参数**:
```rust
struct RequestParams {
    user_id: i32,          // 用户ID
    role1_id: i32,         // 正方角色ID
    role2_id: i32,         // 反方角色ID
    topic: String,         // 辩论主题
}
```

**响应**:
```rust
struct ResponseData {
    debate_id: i32,       // 辩论ID
}
```

**流程**:
- 创建辩论
- 返回辩论ID

#### 5.4.2 开始辩论
```
POST /api/debate/start
```

**请求参数**:
```rust
struct RequestParams {
    debate_id: i32,     // 辩论ID
    user_id: i32,       // 用户ID
    role1_id: i32,      // 正方角色ID
    role2_id: i32,      // 反方角色ID
}
```
**响应**:
```rust
struct ResponseData {
    current_speaker_id: i32,    // 当前发言者ID
    response: String,           // 发言内容
    timestamp: i64,             // 时间戳（毫秒）
    voice_url: String,  // 语音URL
}
```

**流程**:
- 根据辩论ID获取辩论信息
- 获取当前发言角色
- 调用AI生成发言内容
- 生成语音
- 消息入库
- 修改当前发言角色
- 返回发言内容

#### 5.4.3 辩论列表
```
POST /api/debate/list
```

**请求参数**:
```rust
struct RequestParams {
    user_id: i32,       // 用户ID
    offset: i64,        // 偏移量
    limit: i64,         // 限制数量
}
```

**响应**:
```rust
struct ResponseData {
    debates: Vec<DebateItem>,  // 历史辩论列表
    total: i64,                // 总数量
    has_more: bool,            // 是否还有更多
}

struct DebateItem {
    id: i32,                     // 辩论ID
    role1_id: i32,               // 正方角色ID
    role2_id: i32,               // 反方角色ID
    topic: String,               // 辩论主题
    last_dialog_timestamp: i64,  // 上次对话时间戳（毫秒）
    current_speaker_id: i32,     // 当前发言角色ID
}
```

**流程**:
- 分页查询辩论列表
- 返回辩论列表

#### 5.4.4 辩论记录
```
POST /api/debate/dialogs
```

**请求参数**:
```rust
struct RequestParams {
    debate_id: i32,     // 辩论ID
    user_id: i32,       // 用户ID
    role1_id: i32,      // 正方角色ID
    role2_id: i32,      // 反方角色ID
    offset: i64,        // 偏移量
    limit: i64,         // 限制数量
}
```

**响应**:
```rust
struct ResponseData {
    debate_id: i32,            // 辩论ID
    topic: String,             // 主题
    role1_id: i32,             // 正方角色ID
    role2_id: i32,             // 反方角色ID
    current_speaker_id: i32,   // 当前发言角色ID
    dialogs: Vec<DialogItem>,  // 历史对话列表
    total: i64,                // 总数量
    has_more: bool,            // 是否还有更多
}

struct DialogItem {
    id: i32,                // 辩论ID
    role_id: i32,           // 角色ID
    timestamp: i64,         // 时间戳（毫秒）
    text: String,           // 消息文本
    voice: Option<String>,  // 语音URL
}
```

**流程**:
- 分页查询辩论记录
- 返回辩论记录

#### 5.4.5 删除辩论
```
POST /api/debate/delete
```

**请求参数**:
```rust
struct RequestParams {
    user_id: i32,          // 用户ID
    debate_id: i32,        // 辩论ID
}
```

**响应**:
```rust
struct ResponseData {
    success: bool,        // 是否删除成功
    message: String,      // 提示信息
}
```

**流程**:
- 删除辩论
- 删除辩论的所有对话
- 返回删除结果

### 5.5 用户管理接口

#### 5.5.1 用户资料
```
GET /api/user/profile
PUT /api/user/profile
Authorization: Bearer <token>
```

**请求参数 (PUT)**:
```rust
#[derive(Deserialize)]
pub struct RequestParams {
    username: String,    // 用户名
    avatar: String,      // 头像URL
}
```

**响应**:
```rust
struct ResponseData {
    new_password: String,       // 新密码哈希
    current_password: String,   // 当前密码哈希
}
```

**流程 (GET)**:
- 从JWT中获取用户
- 返回用户信息

**流程 (PUT)**:
- 从JWT中获取用户
- 检查当前密码哈希是否正确
- 更新密码哈希

#### 5.5.2 更新头像
```
POST /api/user/avatar
Authorization: Bearer <token>
```

**请求参数**:
```rust
struct RequestData {
    avatar_url: String,    // 新头像URL
}
```

**响应**:
```rust
struct ResponseData {
    success: bool,        // 是否更新成功
    message: String,      // 提示信息
}
```

**流程**:
- 从JWT中获取用户
- 更新用户头像URL
- 返回更新结果

#### 5.5.3 用户角色列表
```
GET /api/user/roles
Authorization: Bearer <token>
```

**响应**:
```rust
Vec<RoleData>

struct RoleData {
    id: i32,                   // 角色ID
    name: String,              // 角色名称
    description: String,       // 角色描述
    traits: String,            // 角色特征
    image: String,             // 角色头像
}
```

**流程**:
- 从JWT中获取用户
- 获取用户创建的角色列表
- 返回角色信息列表

#### 5.5.4 删除用户角色
```
DELETE /api/user/role/delete/{role_id}
Authorization: Bearer <token>
```

**路径参数**:
- `role_id`: 角色ID (i32)

**响应**: `200 OK` 或错误信息

**流程**:
- 从JWT中获取用户
- 验证角色是否属于用户
- 删除角色和相关对话、辩论等数据

#### 5.5.5 删除用户所有对话
```
DELETE /api/user/conversations/delete
Authorization: Bearer <token>
```

**响应**:
```rust
struct ResponseData {
    deleted_count: u64,    // 删除的对话数量
}
```

**流程**:
- 从JWT中获取用户
- 删除用户的所有对话记录
- 返回删除数量

#### 5.5.6 删除用户所有辩论
```
POST /api/user/debates/delete
Authorization: Bearer <token>
```

**请求参数**:
```rust
struct RequestParams {
    user_id: i32,         // 用户ID
}
```

**响应**:
```rust
struct ResponseData {
    success: bool,        // 是否删除成功
    message: String,      // 提示信息
    deleted_count: i32,   // 删除的辩论数量
}
```

**流程**:
- 删除用户的所有辩论记录
- 返回删除结果

### 5.6 文件上传接口

#### 5.6.1 文件上传
```
POST /api/upload
Authorization: Bearer <token>
X-File-Name: {filename}
```

**请求**: Bytes

**响应**:
```rust
struct ResponseData {
    file_url: String,    // 文件访问URL
}
```

**流程**:
- 从Headers中解析文件名
- 用uuid重命名文件
- 上传文件到七牛云对象存储
- 返回文件访问URL

## 6. 实时通信接口 (Socket.IO)

### 6.1 连接管理

#### 6.1.1 连接事件
**事件**:
- connect
- disconnect

#### 6.1.2 加入房间
**事件**:
- join

**数据**:
```rust
room: String    // 房间名
```

### 6.2 消息通信

#### 6.2.1 发送文本消息
后端监听

**事件**:
- `message`

**数据**:
```rust
struct MessageData {
    user_id: i32,    // 用户ID
    role_id: i32,    // 角色ID
    timestamp: i64,  // 时间戳（毫秒）
    text: String,    // 消息文本
}
```

**流程**:
- 消息入库
- 获取角色信息
- 获取对话历史及摘要
- 调用AI生成回复
- 生成语音
- 向前端发送`message`事件，传输消息信息

#### 6.2.2 接收回复消息
前端监听

**事件**:
- `message`

**数据**:
```javascript
{
    role_id: number,     // 角色ID
    timestamp: number,   // 时间戳（毫秒）
    text: string,        // 消息文本
    voice_url: string    // 语音URL
}
```

#### 6.2.3 发送语音消息
后端监听

**事件**:
- `voice`

**数据**:
```rust
struct MessageData {
    id: i32,              // 消息ID
    user_id: i32,         // 用户ID
    role_id: i32,         // 角色ID
    timestamp: i64,       // 时间戳（毫秒）
    voice_url: String,    // 语音URL
}
```

**流程**:
- 语音转文字
- 消息入库
- 向前端发送`update_message`事件，传输消息文本
- 获取角色信息
- 获取对话历史及摘要
- 调用AI生成回复
- 生成语音
- 向前端发送`message`事件，传输消息信息

#### 6.2.4 消息更新通知
前端监听

**事件**:
- `update_message`

**数据**:
```javascript
{
    id: number,        // 消息ID
    text: string,      // 消息文本
}
```

## 7. 数据模型

### 7.1 用户模型 (users)
```rust
struct Model {
    id: i32,                    // 主键
    username: String,           // 用户名
    password_hash: String,      // 密码哈希
    image: String,              // 头像URL
    jwt_secret: String,         // JWT密钥
}
```

### 7.2 角色模型 (roles)
```rust
struct Model {
    id: i32,                    // 主键
    user_id: i32,               // 创建者ID
    name: String,               // 角色名称
    description: String,        // 角色描述
    traits: String,             // 角色特征
    image: String,              // 角色头像URL
    gender: String,             // 性别
    age_group: String,          // 年龄组
    voice_type: String,         // 语音类型
}
```

### 7.3 对话模型 (conversations)
```rust
struct Model {
    id: i32,                    // 主键
    user_id: i32,               // 用户ID
    role_id: i32,               // 角色ID
    last_dialog_timestamp: i64, // 最后对话时间戳（毫秒）
    history: String,            // 对话历史摘要
}
```

### 7.4 对话记录模板 (conversation_template)
用于生成用户对话记录表，生成的表名格式为`conv_{user_id}_{role_id}`
```rust
struct Model {
    id: i32,                    // 主键
    is_user: bool,              // 是否为用户消息
    timestamp: i64,             // 时间戳（毫秒）
    text: String,               // 消息文本
    voice: Option<String>,      // 语音URL
}
```

### 7.5 辩论模型 (debates)
```rust
struct Model {
    id: i32,                     // 主键
    user_id: i32,                // 用户ID
    role1_id: i32,               // 正方角色ID
    role2_id: i32,               // 反方角色ID
    topic: String,               // 辩论主题
    table_name: String,          // 辩论记录表名
    last_dialog_timestamp: i64,  // 最后发言时间戳（毫秒）
    history: String,             // 辩论历史摘要
    current_speaker_id: i32,     // 当前发言角色ID
}
```

### 7.6 辩论记录模板 (debate_template)
用于生成用户辩论记录表，生成的表名格式为`debate_{user_id}_{role1_id}_{role2_id}_{uuid}`
```rust
struct Model {
    id: i32,                    // 主键
    role_id: i32,               // 发言角色ID
    timestamp: i64,             // 时间戳（毫秒）
    text: String,               // 发言内容
    voice: Option<String>,      // 语音URL
}
```

## 8. 前后端交互逻辑

### 8.1 注册
- 前端上传头像到`/api/upload`获取头像URL，若使用默认头像则跳过
- 前端发送注册请求到 `/api/auth/register`
- 后端处理后入库
- 前端跳转登陆，自动填入用户名

### 8.2 登陆
- 前端发送用户名和密码哈希到 `/api/auth/login`
- 后端验证后返回用户信息和JWT令牌
- 前端存储JWT令牌

### 8.3 退出登陆
- 前端清除JWT令牌

### 8.4 主界面
- 前端通过JWT令牌请求`/api/auth/verify`验证并获取用户信息
- 前端通过分页请求`/api/role/list`获取角色列表
- 前端通过分页请求`/api/conversation/list`获取对话列表
- 前端通过分页请求`/api/debate/list`获取辩论列表

### 8.5 角色搜索
- 前端将搜索框中的关键词发送到`/api/role/search`
- 后端返回匹配的角色列表
- 前端展示搜索结果

### 8.6 角色创建
- 前端上传头像到`/api/upload`获取头像URL，若使用默认头像则跳过
- 前端请求`/api/role/auto-fill`根据角色名自动生成角色信息，若不使用则跳过
- 前端发送角色信息到`/api/role/create`
- 后端处理后入库

### 8.7 对话
- 前端通过Socket.IO连接服务器并加入角色房间
- 若发送语音消息，则前端调用`/api/upload`上传用户语音消息获取语音URL，并发送语音消息到`voice`事件；前端监听`update_message`事件将语音转文字后的消息文本更新到消息框
- 若发送文本消息，则前端发送文本消息到`message`事件
- 后端处理消息，调用AI生成回复并生成语音
- 前端监听`message`事件接收回复消息并自动播放语音

### 8.8 辩论
- 用户填写辩论主题
- 用户选择正反方角色，前端调用`/api/role/list`分页获取角色列表供用户选择，或调用`/api/role/search`搜索角色
- 前端发送创建辩论请求到`/api/debate/new`
- 后端处理后入库并返回辩论ID
- 用户点击开始辩论按钮，前端发送请求到`/api/debate/start`
- 后端处理消息，调用AI生成回复并生成语音
- 前端接收回复消息并自动播放语音
- 前端继续发送请求到`/api/debate/start`，并重复上述步骤，直到用户点击结束辩论按钮

### 8.9 用户设置
- 前端通过JWT令牌请求`/api/user/profile`获取用户信息
- 前端上传新头像到`/api/upload`获取头像URL
- 前端发送更新请求到`/api/user/avatar`更新头像
- 前端发送更新请求到`/api/user/profile`更新用户名和密码
- 前端请求`/api/user/roles`获取用户创建的角色列表
- 前端发送删除请求到`/api/user/role/delete/{role_id}`删除角色
- 前端发送删除请求到`/api/user/conversations/delete`删除所有对话
- 前端发送删除请求到`/api/user/debates/delete`删除所有辩论

## 9. 环境变量
- `PORT`: 服务器监听端口 (默认: 8080)
- `TRACING_LEVEL`: 日志级别 (默认: info)
- `QINIU_ACCESS_KEY`: 七牛云 Access Key
- `QINIU_SECRET_KEY`: 七牛云 Secret Key
- `QINIU_AI_API_KEY`: 七牛云 AI 大模型推理 API Key
- `QINIU_LLM_MODEL`: 七牛云 AI 大模型名称 (默认: deepseek/deepseek-v3.1-terminus)
- `QINIU_LLM_THINKING_MODEL`: 七牛云 AI 大模型推理模型名称 (默认: deepseek-r1-0528)
- `MYSQL_USERNAME`: MySQL 用户名
- `MYSQL_PASSWORD`: MySQL 密码
- `MYSQL_ENDPOINT`: MySQL 连接地址

## 10. 安全考虑

### 10.1 认证安全
- JWT令牌有效期7天
- 每个用户独立的JWT密钥
- 密码使用SHA-256哈希存储

### 10.2 API安全
- 所有敏感接口需要Bearer token认证
- SQL注入防护 (SeaORM)

### 10.3 文件上传安全
- 文件大小限制50MB
- 七牛云安全存储

## 11. 性能优化

### 11.1 数据传输优化
- 分页查询减少数据传输

### 11.2 并发处理
- 异步IO处理
- 独立协程处理历史摘要
