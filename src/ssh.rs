use crossterm::{
    cursor::Show,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Write;
use std::process::Command;

use crate::config::SSHConfig;
use crate::proxy::GlobalConfig;

#[derive(Debug, Clone)]
pub struct SSHManager {
    pub global_config: GlobalConfig,
}

impl SSHManager {
    /// 创建新的 SSH 管理器
    ///
    /// # 参数
    /// - `global_config`: 全局配置
    ///
    /// # 返回
    /// 返回初始化的 SSH 管理器
    pub fn new(global_config: GlobalConfig) -> Self {
        Self { global_config }
    }

    /// 建立 SSH 连接
    ///
    /// # 参数
    /// - `config`: SSH 配置
    ///
    /// # 返回
    /// 返回 Result，成功为 Ok(()), 失败为 Err
    pub fn connect(&self, config: &SSHConfig) -> Result<(), Box<dyn std::error::Error>> {
        // 恢复终端设置，退出TUI模式
        disable_raw_mode()?;
        execute!(std::io::stdout(), LeaveAlternateScreen, Show)?;
        std::io::stdout().flush()?;

        println!("\x1b[33m正在连接: {}\x1b[0m", config.address);

        if config.use_global_proxy && self.global_config.proxy.is_enabled() {
            println!(
                "\x1b[33m全局代理:\x1b[0m {:?} {}:{}",
                self.global_config.proxy.proxy_type,
                self.global_config.proxy.host,
                self.global_config.proxy.port.unwrap_or(
                    match self.global_config.proxy.proxy_type {
                        crate::proxy::ProxyType::Socks5 => 1080,
                        crate::proxy::ProxyType::Http => 8080,
                        crate::proxy::ProxyType::None => 0,
                    }
                )
            );
        } else if let Some(proxy) = &config.proxy {
            if proxy.is_enabled() {
                println!(
                    "\x1b[33m自定义代理:\x1b[0m {:?} {}:{}",
                    proxy.proxy_type,
                    proxy.host,
                    proxy.port.unwrap_or(match proxy.proxy_type {
                        crate::proxy::ProxyType::Socks5 => 1080,
                        crate::proxy::ProxyType::Http => 8080,
                        crate::proxy::ProxyType::None => 0,
                    })
                );
            }
        }

        if let Some(pf) = &config.port_forward {
            if pf.enabled {
                println!("\x1b[33m端口转发:\x1b[0m  {} <->  {}", pf.local, pf.remote);
            }
        }

        std::io::stdout().flush()?;

        let mut cmd = Command::new("ssh");

        if let Some(port) = config.port {
            cmd.arg("-p").arg(port.to_string());
        }

        if let Some(key) = &config.key {
            let expanded_key = shellexpand::tilde(key).to_string();
            cmd.arg("-i").arg(expanded_key);
        }

        if config.use_global_proxy {
            if let Some(proxy_cmd) = self.global_config.proxy.get_ssh_proxy_command() {
                cmd.arg("-o").arg(format!("ProxyCommand={proxy_cmd}"));
            }
        } else if let Some(proxy) = &config.proxy {
            if let Some(proxy_cmd) = proxy.get_ssh_proxy_command() {
                cmd.arg("-o").arg(format!("ProxyCommand={proxy_cmd}"));
            }
        }

        if let Some(pf) = &config.port_forward {
            if pf.enabled {
                cmd.arg("-L").arg(format!("{}:{}", pf.local, pf.remote));
            }
        }

        let connection_string = if let Some(user) = &config.user {
            format!("{}@{}", user, config.address)
        } else {
            config.address.clone()
        };
        cmd.arg(connection_string);

        let cmd_str = std::iter::once(cmd.get_program())
            .chain(cmd.get_args())
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("\x1b[33m最终执行命令:\x1b[0m {cmd_str}");
        println!("按 Ctrl+C 取消连接\n");

        cmd.stdin(std::process::Stdio::inherit());
        cmd.stdout(std::process::Stdio::inherit());
        cmd.stderr(std::process::Stdio::inherit());

        // 使用 spawn 和 wait 替代 exec，以实现跨平台
        let status = cmd.spawn()?.wait()?;

        // SSH 进程结束后，重新进入 TUI 模式
        execute!(std::io::stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("SSH 连接失败，退出码: {:?}", status.code()).into())
        }
    }
}
