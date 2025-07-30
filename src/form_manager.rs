use crate::config::SSHConfig;
use crate::forms::FormData;
use crate::proxy::{ProxyConfig, ProxyType};
use crate::ui::ScrollManager;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FormManager {
    pub form_data: FormData,
    pub editing_host: Option<String>,

    // 滚动状态管理
    pub scroll_manager: ScrollManager,
}

impl FormManager {
    /// 创建新的表单管理器
    ///
    /// # 返回
    /// 返回初始化的表单管理器
    pub fn new() -> Self {
        Self {
            form_data: FormData::new(),
            editing_host: None,
            scroll_manager: ScrollManager::new(),
        }
    }

    /// 开始添加表单
    pub fn start_add_form(&mut self) {
        self.editing_host = None;
        self.form_data = FormData::new();
    }

    /// 开始编辑表单
    ///
    /// # 参数
    /// - `config`: 要编辑的 SSH 配置
    pub fn start_edit_form(&mut self, config: &SSHConfig) {
        self.editing_host = Some(config.alias.clone());
        self.form_data = FormData::from_config(config);
    }

    /// 清空表单数据
    pub fn clear(&mut self) {
        self.editing_host = None;
        self.form_data = FormData::new();
    }

    /// 检查是否正在编辑
    ///
    /// # 返回
    /// 返回 true 表示正在编辑，false 表示添加新配置
    pub fn is_editing(&self) -> bool {
        self.editing_host.is_some()
    }

    /// 获取正在编辑的主机别名
    ///
    /// # 返回
    /// 返回正在编辑的主机别名，如果没有则为 None
    pub fn get_editing_host(&self) -> Option<&String> {
        self.editing_host.as_ref()
    }

    /// 验证并创建 SSH 配置
    ///
    /// # 返回
    /// 返回 Result，成功为 SSHConfig，失败为包含错误信息的 Err
    pub fn validate_and_create_config(&self) -> Result<SSHConfig, String> {
        self.form_data.to_ssh_config()
    }

    // 代理配置相关方法
    /// 开始代理配置
    ///
    /// # 参数
    /// - `global_proxy`: 全局代理配置
    pub fn start_proxy_config(&mut self, global_proxy: &ProxyConfig) {
        self.form_data = FormData::new();
        self.form_data.data.insert(
            "global_proxy_type".to_string(),
            format!("{:?}", global_proxy.proxy_type),
        );
        self.form_data
            .data
            .insert("global_proxy_host".to_string(), global_proxy.host.clone());
        if let Some(port) = global_proxy.port {
            self.form_data
                .data
                .insert("global_proxy_port".to_string(), port.to_string());
        }
        if let Some(username) = &global_proxy.username {
            self.form_data
                .data
                .insert("global_proxy_username".to_string(), username.clone());
        }
        if let Some(password) = &global_proxy.password {
            self.form_data
                .data
                .insert("global_proxy_password".to_string(), password.clone());
        }
        // 重置字段索引和光标位置
        self.form_data.current_field = 0;
        self.form_data.cursor_position = 0;
    }

    /// 创建代理配置
    ///
    /// # 返回
    /// 返回从表单数据创建的代理配置
    pub fn create_proxy_config(&self) -> Result<ProxyConfig, String> {
        // 验证全局代理配置
        self.form_data.validate_global_proxy()?;

        let proxy_type = self
            .form_data
            .data
            .get("global_proxy_type")
            .map(|t| match t.as_str() {
                "Socks5" => ProxyType::Socks5,
                "Http" => ProxyType::Http,
                _ => ProxyType::None,
            })
            .unwrap_or(ProxyType::None);

        let host = self
            .form_data
            .data
            .get("global_proxy_host")
            .cloned()
            .unwrap_or_default();

        let port = self
            .form_data
            .data
            .get("global_proxy_port")
            .and_then(|p| p.parse::<u16>().ok());

        let username = self
            .form_data
            .data
            .get("global_proxy_username")
            .filter(|u| !u.is_empty())
            .cloned();

        let password = self
            .form_data
            .data
            .get("global_proxy_password")
            .filter(|p| !p.is_empty())
            .cloned();

        Ok(ProxyConfig {
            proxy_type,
            host,
            port,
            username,
            password,
        })
    }

    // 字段操作方法的代理
    /// 移动到下一个字段
    pub fn next_field(&mut self) {
        self.form_data.next_field();
        self.scroll_manager
            .set_selected_index(self.form_data.current_field);
        self.scroll_manager.update_scroll_position();
    }

    /// 移动到上一个字段
    pub fn previous_field(&mut self) {
        self.form_data.previous_field();
        self.scroll_manager
            .set_selected_index(self.form_data.current_field);
        self.scroll_manager.update_scroll_position();
    }

    /// 光标左移
    pub fn move_cursor_left(&mut self) {
        self.form_data.move_cursor_left();
    }

    /// 光标右移
    pub fn move_cursor_right(&mut self) {
        self.form_data.move_cursor_right();
    }

    /// 光标移动到开头
    pub fn move_cursor_to_start(&mut self) {
        self.form_data.move_cursor_to_start();
    }

    /// 光标移动到结尾
    pub fn move_cursor_to_end(&mut self) {
        self.form_data.move_cursor_to_end();
    }

    /// 插入字符
    ///
    /// # 参数
    /// - `c`: 要插入的字符
    pub fn insert_char(&mut self, c: char) {
        self.form_data.insert_char(c);
    }

    /// 删除字符
    pub fn delete_char(&mut self) {
        self.form_data.delete_char();
    }

    /// 向前删除字符
    pub fn delete_char_forward(&mut self) {
        self.form_data.delete_char_forward();
    }

    /// 清空当前字段
    pub fn clear_current_field(&mut self) {
        self.form_data.clear_current_field();
    }

    /// 切换复选框状态
    pub fn toggle_checkbox(&mut self) {
        self.form_data.toggle_checkbox();
    }

    /// 切换代理选项
    pub fn toggle_proxy_option(&mut self) {
        self.form_data.toggle_proxy_option();
    }

    // 访问器方法
    /// 获取当前字段索引
    ///
    /// # 返回
    /// 返回当前字段索引
    pub fn get_current_field(&self) -> usize {
        self.form_data.current_field
    }

    /// 获取当前光标位置
    ///
    /// # 返回
    /// 返回当前光标位置
    pub fn get_cursor_position(&self) -> usize {
        self.form_data.cursor_position
    }

    /// 获取表单数据
    ///
    /// # 返回
    /// 返回表单数据的引用
    pub fn get_form_data(&self) -> &HashMap<String, String> {
        &self.form_data.data
    }

    /// 设置可视区域大小
    ///
    /// # 参数
    /// - `visible_fields`: 可视区域能显示的字段数量
    pub fn set_visible_fields(&mut self, visible_fields: usize) {
        self.scroll_manager.set_visible_items(visible_fields);
    }

    /// 更新滚动位置，确保当前字段在可视区域内
    pub fn update_scroll_position(&mut self) {
        self.scroll_manager
            .set_selected_index(self.form_data.current_field);
        self.scroll_manager.update_scroll_position();
    }

    /// 获取滚动信息
    ///
    /// # 返回
    /// 返回 (当前滚动位置, 可视区域大小, 总字段数)
    pub fn get_scroll_info(&self) -> (usize, usize, usize) {
        self.scroll_manager.get_scroll_info()
    }
}
