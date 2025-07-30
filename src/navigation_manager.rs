use crate::config::SSHConfig;
use crate::ui::ScrollManager;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    List,
    AddForm,
    EditForm,
    DeleteDialog,
    SelectImport,
    ProxyConfig,
}

#[derive(Debug, Clone)]
pub struct ImportManager {
    pub candidates: Vec<SSHConfig>,
    pub selected: Vec<bool>,
    pub selected_index: usize,
}

impl ImportManager {
    /// 创建新的导入管理器
    ///
    /// # 返回
    /// 返回初始化的导入管理器
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
            selected: Vec::new(),
            selected_index: 0,
        }
    }

    /// 设置候选配置列表
    ///
    /// # 参数
    /// - `configs`: 候选配置列表
    pub fn set_candidates(&mut self, configs: Vec<SSHConfig>) {
        self.candidates = configs;
        self.selected = vec![false; self.candidates.len()];
        self.selected_index = 0;
    }

    /// 切换当前项的选中状态
    pub fn toggle_current(&mut self) {
        if self.selected_index < self.selected.len() {
            self.selected[self.selected_index] = !self.selected[self.selected_index];
        }
    }

    /// 全选或全不选
    pub fn toggle_all(&mut self) {
        let all_selected = self.selected.iter().all(|&x| x);
        for selected in &mut self.selected {
            *selected = !all_selected;
        }
    }

    /// 获取选中的配置列表
    ///
    /// # 返回
    /// 返回选中的配置列表
    pub fn get_selected_configs(&self) -> Vec<SSHConfig> {
        self.candidates
            .iter()
            .enumerate()
            .filter(|(i, _)| *self.selected.get(*i).unwrap_or(&false))
            .map(|(_, config)| config.clone())
            .collect()
    }

    /// 清空导入管理器
    pub fn clear(&mut self) {
        self.candidates.clear();
        self.selected.clear();
        self.selected_index = 0;
    }

    /// 移动到下一个项目
    pub fn next_item(&mut self) {
        if !self.candidates.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.candidates.len();
        }
    }

    /// 移动到上一个项目
    pub fn previous_item(&mut self) {
        if !self.candidates.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.candidates.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }
}

#[derive(Debug, Clone)]
pub struct NavigationManager {
    pub mode: AppMode,
    pub selected_index: usize,
    pub focus: usize, // 0: list, 1: details

    // 滚动状态管理
    pub scroll_manager: ScrollManager,

    pub import_manager: ImportManager,
}

impl NavigationManager {
    /// 创建新的导航管理器
    ///
    /// # 返回
    /// 返回初始化的导航管理器
    pub fn new() -> Self {
        Self {
            mode: AppMode::List,
            selected_index: 0,
            focus: 0,
            scroll_manager: ScrollManager::new(),
            import_manager: ImportManager::new(),
        }
    }

    /// 设置应用模式
    ///
    /// # 参数
    /// - `mode`: 要设置的应用模式
    pub fn set_mode(&mut self, mode: AppMode) {
        self.mode = mode;
    }

    /// 检查是否为列表模式
    ///
    /// # 返回
    /// 返回 true 表示是列表模式，false 表示不是
    pub fn is_list_mode(&self) -> bool {
        matches!(self.mode, AppMode::List)
    }

    /// 返回到列表模式
    pub fn return_to_list(&mut self) {
        self.mode = AppMode::List;
        self.import_manager.clear();
    }

    /// 移动到下一个项目
    ///
    /// # 参数
    /// - `max_items`: 最大项目数
    pub fn next_item(&mut self, max_items: usize) {
        if max_items > 0 {
            self.selected_index = (self.selected_index + 1) % max_items;
            self.scroll_manager.set_selected_index(self.selected_index);
        }
    }

    /// 移动到上一个项目
    ///
    /// # 参数
    /// - `max_items`: 最大项目数
    pub fn previous_item(&mut self, max_items: usize) {
        if max_items > 0 {
            self.selected_index = if self.selected_index == 0 {
                max_items - 1
            } else {
                self.selected_index - 1
            };
            self.scroll_manager.set_selected_index(self.selected_index);
        }
    }

    /// 切换焦点区域
    ///
    /// # 参数
    /// - `max_focus`: 最大焦点数量
    pub fn toggle_focus(&mut self, max_focus: usize) {
        if max_focus > 0 {
            self.focus = (self.focus + 1) % max_focus;
        }
    }

    /// 获取滚动偏移量
    pub fn get_scroll_offset(&self) -> usize {
        self.scroll_manager.scroll_offset
    }

    /// 更新滚动位置
    ///
    /// # 参数
    /// - `total_items`: 总项目数
    /// - `visible_items`: 可视项目数
    pub fn update_scroll_position(&mut self, total_items: usize, visible_items: usize) {
        self.scroll_manager.set_total_items(total_items);
        self.scroll_manager.set_visible_items(visible_items);
        self.scroll_manager.set_selected_index(self.selected_index);
    }

    /// 获取有效的选中索引
    pub fn get_valid_selected_index(&self, max_items: usize) -> usize {
        if max_items == 0 {
            0
        } else {
            std::cmp::min(self.selected_index, max_items - 1)
        }
    }

    /// 开始导入流程
    pub fn start_import(&mut self, candidates: Vec<SSHConfig>) {
        self.import_manager.set_candidates(candidates);
        self.mode = AppMode::SelectImport;
    }

    /// 导入模式下移动到下一项
    pub fn import_next(&mut self) {
        self.import_manager.next_item();
    }

    /// 导入模式下移动到上一项
    pub fn import_previous(&mut self) {
        self.import_manager.previous_item();
    }

    /// 切换当前导入项的选中状态
    pub fn toggle_import_selection(&mut self) {
        self.import_manager.toggle_current();
    }

    /// 切换所有导入项的选中状态
    pub fn toggle_all_import_selection(&mut self) {
        self.import_manager.toggle_all();
    }

    /// 获取选中的导入项
    pub fn get_selected_imports(&self) -> Vec<SSHConfig> {
        self.import_manager.get_selected_configs()
    }

    /// 取消导入
    pub fn cancel_import(&mut self) {
        self.import_manager.clear();
        self.mode = AppMode::List;
    }
}
