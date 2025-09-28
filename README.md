# role-play-ai
AI角色扮演平台

- 项目架构见[architecture.md](https://github.com/LlamaNoHumps/role-play-ai/blob/master/architecture.md)
- 演示Demo见[role-play-ai-demo.mp4](https://github.com/LlamaNoHumps/role-play-ai/blob/master/role-play-ai-demo.mp4)
- 问题回答见[answer.md](https://github.com/LlamaNoHumps/role-play-ai/blob/master/answer.md)

## 功能
- 自定义创建角色
- 输入角色名后，自动生成角色，自动选择语音音色
- 通过文字或语音与角色对话，角色回复文字和对应语音
- 创建辩论，观摩两个不同世界观的角色产生的碰撞

## 获取API授权
本项目需要使用七牛云的API。

- 访问[https://portal.qiniu.com/signup](https://portal.qiniu.com/signup)开通七牛开发者帐号
- 访问[https://portal.qiniu.com/user/key](https://portal.qiniu.com/user/key)获取`Access Key`和`Secret Key`
- 访问[https://portal.qiniu.com/ai-inference/api-key](https://portal.qiniu.com/ai-inference/api-key)获取AI大模型推理`API Key`

项目用到七牛云的服务
- Kodo对象存储
- LLM实时推理
- ASR语音识别
- TTS语音合成

项目仅使用LLM实时推理的基础功能，没有加入额外参数，因此没有使用模型自带的Agent。

## 部署方法
使用`docker compose`部署。

项目使用本地部署的`MySql`作为后端数据库。

首先修改`docker-compose.yml`中的配置。

修改`role-play-ai`服务的环境变量
- `QINIU_ACCESS_KEY`修改为七牛云`Access Key`
- `QINIU_SECRET_KEY`修改为七牛云`Secret Key`
- `QINIU_AI_API_KEY`修改为七牛云AI大模型推理`API Key`

默认映射主机的`8080`端口，如果有端口冲突，请修改`role-play-ai`的端口映射。

其余配置均为默认即可，不需要修改。

运行
```sh
docker compose up -d
```

项目需要编译，构建完成需要5-10分钟。