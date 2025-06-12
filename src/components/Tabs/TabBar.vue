<template>
  <div class="tab-bar">
    <!-- 左侧：面板切换、插件选择器和刷新按钮 -->
    <div class="left-section">
      <div class="panel-toggle">
        <el-button type="default" @click="handleTogglePanel"
          :icon="settingsStore.leftPanelVisible ? ArrowLeft : ArrowRight" size="small" circle
          :title="settingsStore.leftPanelVisible ? '收起配置面板' : '展开配置面板'" />
      </div>
    </div>
    <!-- 固定标签页 -->
    <div v-if="pinnedTabs.length > 0" class="pinned-tabs">
      <TabItem v-for="tab in pinnedTabs" :key="tab.id" :tab="tab" :is-pinned="true" @click="handleTabClick(tab.id)"
        @close="() => handleTabClose(tab.id)" @context-menu="handleTabContextMenu(tab, $event)" />
      <div class="tab-divider"></div>
    </div>

    <!-- 普通标签页 -->
    <div class="normal-tabs" ref="normalTabsContainer">
      <TabItem v-for="tab in activeTabs" :key="tab.id" :tab="tab" :is-pinned="false" @click="handleTabClick(tab.id)"
        @close="() => handleTabClose(tab.id)" @context-menu="handleTabContextMenu(tab, $event)" />
    </div>

    <!-- 新建标签页按钮 -->
    <div class="tab-actions">
      <el-button link :icon="Plus" size="small" class="new-tab-btn" :disabled="!canCreateNewTab" title="新建标签页"
        @click="showNewTabDialog = true" />

      <!-- 标签页管理按钮 -->
      <el-button link :icon="More" size="small" class="manage-tab-btn" title="标签页管理"
        @click="showTabManageDialog = true" />
      <div class="right-section">
        <el-button type="default" @click="handlePluginManager" :icon="Box" size="small" circle title="插件管理" />
        <el-button type="default" @click="handleSettings" :icon="Setting" size="small" circle title="系统设置" />
      </div>
    </div>

    <!-- 标签页右键菜单 -->
    <TabContextMenu v-model:visible="contextMenuVisible" :tab="contextMenuTab" :position="contextMenuPosition"
      @rename="handleTabRename" @pin="handleTabPin" @close="() => handleTabClose('')" @close-others="handleCloseOthers"
      @close-to-right="handleCloseToRight" />

    <SystemSettings v-model="showSettings" />
    <PluginManager v-model="showPluginManager" />

    <!-- 新建标签页弹窗 -->
    <el-dialog v-model="showNewTabDialog" title="新建标签页" width="400px" :before-close="handleNewTabDialogClose">
      <div class="new-tab-dialog">
        <div class="dialog-description">
          <p>选择要创建的插件类型：</p>
        </div>
        <div class="plugin-list">
          <div v-for="plugin in availablePlugins" :key="plugin.id" class="plugin-item"
            :class="{ disabled: plugin.disabled }" @click="!plugin.disabled && handleCreateTab(plugin.id)">
            <el-icon v-if="plugin.icon" class="plugin-icon">
              <component :is="plugin.icon" />
            </el-icon>
            <div class="plugin-info">
              <div class="plugin-name">{{ plugin.name }}</div>
              <div class="plugin-description">{{ plugin.description }}</div>
            </div>
          </div>
        </div>
      </div>
      <template #footer>
        <el-button type="success" @click="handleOpenPluginManager">
          <el-icon>
            <Box />
          </el-icon>
          插件管理
        </el-button>
        <el-button @click="showNewTabDialog = false">取消</el-button>
      </template>
    </el-dialog>

    <!-- 标签页管理弹窗 -->
    <el-dialog v-model="showTabManageDialog" title="标签页管理" width="350px" :before-close="handleTabManageDialogClose">
      <div class="tab-manage-dialog">
        <div class="manage-actions">
          <el-button type="danger" :disabled="tabCount === 0" @click="handleManageAction('close-all')" block>
            <el-icon>
              <Close />
            </el-icon>
            关闭所有标签页
          </el-button>
          <el-button type="warning" :disabled="tabCount <= 1" @click="handleManageAction('close-others')" block>
            <el-icon>
              <Close />
            </el-icon>
            关闭其他标签页
          </el-button>
          <el-button type="info" :disabled="activeTabs.length === 0" @click="handleManageAction('close-unpinned')"
            block>
            <el-icon>
              <Close />
            </el-icon>
            关闭未固定标签页
          </el-button>
          <el-divider />
          <el-button type="primary" @click="handleManageAction('restore-session')" block>
            <el-icon>
              <Refresh />
            </el-icon>
            恢复会话
          </el-button>
        </div>
      </div>
      <template #footer>
        <el-button @click="showTabManageDialog = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Plus, More, Setting, ArrowLeft, ArrowRight, Box, Close, Refresh } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useTabManagerStore, type Tab } from '@/stores/tabManager'
import { usePluginStore } from '@/stores/plugins'
import TabItem from './TabItem.vue'
import TabContextMenu from './TabContextMenu.vue'
import SystemSettings from '../SystemSettings.vue'
import PluginManager from '../PluginManager.vue'
import { useSettingsStore } from '@/stores/settings'

const tabManagerStore = useTabManagerStore()
const pluginStore = usePluginStore()

const settingsStore = useSettingsStore()
const showSettings = ref(false)
const showPluginManager = ref(false)
const showNewTabDialog = ref(false)
const showTabManageDialog = ref(false)

// 计算属性
const pinnedTabs = computed(() => tabManagerStore.pinnedTabs)
const activeTabs = computed(() => tabManagerStore.activeTabs)
const tabCount = computed(() => tabManagerStore.tabCount)
const canCreateNewTab = computed(() => tabManagerStore.canCreateNewTab)
const availablePlugins = computed(() => pluginStore.plugins.filter(p => !p.disabled))

// 右键菜单相关
const contextMenuVisible = ref(false)
const contextMenuTab = ref<Tab | null>(null)
const contextMenuPosition = ref({ x: 0, y: 0 })

// DOM引用
const normalTabsContainer = ref<HTMLElement>()

// 处理标签页点击
const handleTabClick = async (tabId: string) => {
  try {
    await tabManagerStore.switchToTab(tabId)
  } catch (error) {
    console.error('切换标签页失败:', error)
    ElMessage.error('切换标签页失败')
  }
}

// 处理标签页关闭
const handleTabClose = async (tabId: string) => {
  try {
    await tabManagerStore.closeTab(tabId)
    ElMessage.success('标签页已关闭')
  } catch (error) {
    console.error('关闭标签页失败:', error)
    ElMessage.error('关闭标签页失败')
  }
}

// 处理创建标签页
const handleCreateTab = async (pluginId: string) => {
  try {
    await tabManagerStore.createNewTab(pluginId)
    ElMessage.success('新标签页已创建')
    showNewTabDialog.value = false
  } catch (error) {
    console.error('创建标签页失败:', error)
    ElMessage.error('创建标签页失败')
  }
}

// 处理新建标签页弹窗关闭
const handleNewTabDialogClose = () => {
  showNewTabDialog.value = false
}

// 处理打开插件管理
const handleOpenPluginManager = () => {
  showNewTabDialog.value = false
  handlePluginManager()
}

// 处理标签页管理操作
const handleManageAction = async (command: string) => {
  try {
    switch (command) {
      case 'close-all':
        await ElMessageBox.confirm('确定要关闭所有标签页吗？', '确认', {
          type: 'warning'
        })
        await tabManagerStore.closeAllTabs()
        ElMessage.success('所有标签页已关闭')
        showTabManageDialog.value = false
        break

      case 'close-others':
        if (tabManagerStore.activeTab) {
          await tabManagerStore.closeOtherTabs(tabManagerStore.activeTab.id)
          ElMessage.success('其他标签页已关闭')
          showTabManageDialog.value = false
        }
        break

      case 'close-unpinned':
        const unpinnedTabIds = activeTabs.value.map(t => t.id)
        for (const tabId of unpinnedTabIds) {
          await tabManagerStore.closeTab(tabId)
        }
        ElMessage.success('未固定标签页已关闭')
        showTabManageDialog.value = false
        break

      case 'restore-session':
        // TODO: 实现会话恢复功能
        ElMessage.info('会话恢复功能开发中')
        showTabManageDialog.value = false
        break
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('执行标签页管理命令失败:', error)
      ElMessage.error('操作失败')
    }
  }
}

// 处理标签页管理弹窗关闭
const handleTabManageDialogClose = () => {
  showTabManageDialog.value = false
}

// 处理系统设置
const handleSettings = () => {
  showSettings.value = true
}

// 处理插件管理
const handlePluginManager = () => {
  showPluginManager.value = true
}

const handleTogglePanel = () => {
  settingsStore.toggleLeftPanel()
}

// 处理标签页右键菜单
const handleTabContextMenu = (tab: Tab, event: MouseEvent) => {
  event.preventDefault()
  contextMenuTab.value = tab
  contextMenuPosition.value = { x: event.clientX, y: event.clientY }
  contextMenuVisible.value = true
}

// 处理标签页重命名
const handleTabRename = async (tab: Tab, newTitle: string) => {
  if (newTitle.trim()) {
    tabManagerStore.renameTab(tab.id, newTitle.trim())
    ElMessage.success('标签页已重命名')
  }
}

// 处理标签页固定
const handleTabPin = (tab: Tab) => {
  tabManagerStore.toggleTabPin(tab.id)
  const action = tab.isPinned ? '取消固定' : '固定'
  ElMessage.success(`标签页已${action}`)
}

// 处理关闭其他标签页
const handleCloseOthers = async (tab: Tab) => {
  try {
    await tabManagerStore.closeOtherTabs(tab.id)
    ElMessage.success('其他标签页已关闭')
  } catch (error) {
    console.error('关闭其他标签页失败:', error)
    ElMessage.error('操作失败')
  }
}

// 处理关闭右侧标签页
const handleCloseToRight = async (tab: Tab) => {
  try {
    await tabManagerStore.closeTabsToRight(tab.id)
    ElMessage.success('右侧标签页已关闭')
  } catch (error) {
    console.error('关闭右侧标签页失败:', error)
    ElMessage.error('操作失败')
  }
}

// 暴露方法给父组件调用
const openNewTabDialog = () => {
  showNewTabDialog.value = true
}

// 暴露给模板引用
defineExpose({
  openNewTabDialog
})
</script>

<style scoped>
.tab-bar {
  display: flex;
  align-items: center;
  background-color: #f5f5f5;
  border-bottom: 1px solid #e4e7ed;
  padding: 0 8px;
  min-height: 40px;
  overflow: hidden;
}

.pinned-tabs {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.tab-divider {
  width: 1px;
  height: 20px;
  background-color: #dcdfe6;
  margin: 0 8px;
}

.normal-tabs {
  display: flex;
  align-items: center;
  flex: 1;
  overflow-x: auto;
  overflow-y: hidden;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.normal-tabs::-webkit-scrollbar {
  display: none;
}

.tab-actions {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-shrink: 0;
  margin-left: 8px;
}

.new-tab-btn,
.manage-tab-btn {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  transition: all 0.2s;
}

.new-tab-btn:hover,
.manage-tab-btn:hover {
  background-color: #e4e7ed;
}

.plugin-icon {
  margin-right: 8px;
}

/* 弹窗样式 */
.new-tab-dialog {
  padding: 8px 0;
}

.dialog-description {
  margin-bottom: 16px;
}

.dialog-description p {
  margin: 0;
  color: #606266;
  font-size: 14px;
}

.plugin-list {
  max-height: 300px;
  overflow-y: auto;
}

.plugin-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  margin-bottom: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.plugin-item:hover:not(.disabled) {
  border-color: #409eff;
  background-color: #f0f9ff;
}

.plugin-item.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: #f5f7fa;
}

.plugin-item .plugin-icon {
  margin-right: 12px;
  font-size: 20px;
  color: #409eff;
}

.plugin-info {
  flex: 1;
}

.plugin-name {
  font-weight: 600;
  font-size: 14px;
  color: #303133;
  margin-bottom: 4px;
}

.plugin-description {
  font-size: 12px;
  color: #909399;
  line-height: 1.4;
}

.tab-manage-dialog {
  padding: 8px 0;
}

.manage-actions {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.manage-actions .el-button {
  justify-content: flex-start;
  text-align: left;
}

.manage-actions .el-button .el-icon {
  margin-right: 8px;
}
/* 响应式设计 */
@media (max-width: 768px) {
  .tab-bar {
    padding: 0 4px;
  }

  .normal-tabs {
    margin: 0 4px;
  }

  .tab-actions {
    margin-left: 4px;
  }
}
</style>
