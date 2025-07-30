use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub is_error: bool,
    pub created_at: Instant,
}

impl Message {
    /// 创建成功消息
    ///
    /// # 参数
    /// - `content`: 消息内容
    ///
    /// # 返回
    /// 返回成功消息实例
    pub fn success(content: String) -> Self {
        Self {
            content,
            is_error: false,
            created_at: Instant::now(),
        }
    }

    /// 创建错误消息
    ///
    /// # 参数
    /// - `content`: 消息内容
    ///
    /// # 返回
    /// 返回错误消息实例
    pub fn error(content: String) -> Self {
        Self {
            content,
            is_error: true,
            created_at: Instant::now(),
        }
    }

    /// 检查消息是否已过期
    ///
    /// # 参数
    /// - `timeout`: 超时时间
    ///
    /// # 返回
    /// 返回 true 表示已过期，false 表示未过期
    pub fn is_expired(&self, timeout: Duration) -> bool {
        self.created_at.elapsed() >= timeout
    }
}

#[derive(Debug, Clone)]
pub struct MessageManager {
    pub current_message: Option<Message>,
    pub timeout: Duration,
}

impl MessageManager {
    /// 创建新的消息管理器
    ///
    /// # 返回
    /// 返回初始化的消息管理器
    pub fn new() -> Self {
        Self {
            current_message: None,
            timeout: Duration::from_secs(1),
        }
    }

    /// 设置成功消息
    ///
    /// # 参数
    /// - `content`: 消息内容
    pub fn set_success_message(&mut self, content: String) {
        self.current_message = Some(Message::success(content));
    }

    /// 设置错误消息
    ///
    /// # 参数
    /// - `content`: 消息内容
    pub fn set_error_message(&mut self, content: String) {
        self.current_message = Some(Message::error(content));
    }

    /// 清空当前消息
    pub fn clear_message(&mut self) {
        self.current_message = None;
    }

    /// 获取当前消息
    ///
    /// # 返回
    /// 返回当前消息的引用，如果没有则为 None
    pub fn get_message(&self) -> Option<&Message> {
        self.current_message.as_ref()
    }

    /// 检查并清理过期消息
    pub fn check_and_clear_expired(&mut self) {
        if let Some(message) = &self.current_message {
            if message.is_expired(self.timeout) {
                self.current_message = None;
            }
        }
    }
}