use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProxyType {
    None,
    Socks5,
    Http,
}

impl Default for ProxyType {
    /// 获取默认代理类型
    ///
    /// # 返回
    /// 返回默认的代理类型（None）
    fn default() -> Self {
        ProxyType::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxyConfig {
    #[serde(default)]
    pub proxy_type: ProxyType,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    #[serde(default)]
    pub proxy: ProxyConfig,
}

impl ProxyConfig {
    /// 检查代理是否启用
    ///
    /// # 返回
    /// 返回 true 表示代理已启用，false 表示未启用
    pub fn is_enabled(&self) -> bool {
        self.proxy_type != ProxyType::None && !self.host.is_empty()
    }

    /// 获取 SSH 代理命令
    ///
    /// # 返回
    /// 返回代理命令字符串，如果代理未启用则返回 None
    pub fn get_ssh_proxy_command(&self) -> Option<String> {
        if !self.is_enabled() {
            return None;
        }

        let port = self.port.unwrap_or(match self.proxy_type {
            ProxyType::Socks5 => 1080,
            ProxyType::Http => 8080,
            ProxyType::None => return None,
        });

        match self.proxy_type {
            ProxyType::Socks5 => {
                // 使用 nc 通过 SOCKS5 代理连接
                Some(format!("nc -X 5 -x {}:{} %h %p", self.host, port))
            }
            ProxyType::Http => {
                // 使用 nc 通过 HTTP 代理连接
                Some(format!("nc -X connect -x {}:{} %h %p", self.host, port))
            }
            ProxyType::None => None,
        }
    }
} 