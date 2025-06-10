<template>
  <div class="tab-bar">
    <!-- 固定标签页 -->
    <div v-if="pinnedTabs.length > 0" class="pinned-tabs">
      <TabItem
        v-for="tab in pinnedTabs"
        :key="tab.id"
        :tab="tab"
        :is-pinned="true"
        @click="handleTabClick(tab.id)"
        @close="() => handleTabClose(tab.id)"
        @context-menu="handleTabContextMenu(tab, $event)"
      />
      <div class="tab-divider"></div>
    </div>

    <!-- 普通标签页 -->
    <div class="normal-tabs" ref="normalTabsContainer">
      <TabItem
        v-for="tab in activeTabs"
        :key="tab.id"
        :tab="tab"
        :is-pinned="false"
        @click="handleTabClick(tab.id)"
        @close="() => handleTabClose(tab.id)"
        @context-menu="handleTabContextMenu(tab, $event)"
      />
    </div>

    <!-- 新建标签页按钮 -->
    <div class="tab-actions">
      <el-dropdown @command="handleNewTabCommand" trigger="click">
        <el-button
          type="text"
          :icon="Plus"
          size="small"
          class="new-tab-btn"
          :disabled="!canCreateNewTab"
          title="新建标签页"
        />
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item
              v-for="plugin in availablePlugins"
              :key="plugin.id"
              :command="plugin.id"
              :disabled="plugin.disabled"
            >
              <el-icon v-if="plugin.icon" class="plugin-icon">
                <component :is="plugin.icon" />
              </el-icon>
              {{ plugin.name }}
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>

      <!-- 标签页管理按钮 -->
      <el-dropdown @command="handleTabManageCommand" trigger="click">
        <el-button
          type="text"
          :icon="More"
          size="small"
          class="manage-tab-btn"
          title="标签页管理"
        />
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="close-all" :disabled="tabCount === 0">
              关闭所有标签页
            </el-dropdown-item>
            <el-dropdown-item command="close-others" :disabled="tabCount <= 1">
              关闭其他标签页
            </el-dropdown-item>
            <el-dropdown-item command="close-unpinned" :disabled="activeTabs.length === 0">
              关闭未固定标签页
            </el-dropdown-item>
            <el-dropdown-divider />
            <el-dropdown-item command="restore-session">
              恢复会话
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>

    <!-- 标签页右键菜单 -->
    <TabContextMenu
      v-model:visible="contextMenuVisible"
      :tab="contextMenuTab"
      :position="contextMenuPosition"
      @rename="handleTabRename"
      @pin="handleTabPin"
      @close="() => handleTabClose('')"
      @close-others="handleCloseOthers"
      @close-to-right="handleCloseToRight"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Plus, More } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useTabManagerStore, type Tab } from '@/stores/tabManager'
import { usePluginStore } from '@/stores/plugins'
import TabItem from './TabItem.vue'
import TabContextMenu from './TabContextMenu.vue'

const tabManagerStore = useTabManagerStore()
const pluginStore = usePluginStore()

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

// 处理新建标签页命令
const handleNewTabCommand = async (pluginId: string) => {
  try {
    await tabManagerStore.createNewTab(pluginId)
    ElMessage.success('新标签页已创建')
  } catch (error) {
    console.error('创建标签页失败:', error)
    ElMessage.error('创建标签页失败')
  }
}

// 处理标签页管理命令
const handleTabManageCommand = async (command: string) => {
  try {
    switch (command) {
      case 'close-all':
        await ElMessageBox.confirm('确定要关闭所有标签页吗？', '确认', {
          type: 'warning'
        })
        await tabManagerStore.closeAllTabs()
        ElMessage.success('所有标签页已关闭')
        break
      
      case 'close-others':
        if (tabManagerStore.activeTab) {
          await tabManagerStore.closeOtherTabs(tabManagerStore.activeTab.id)
          ElMessage.success('其他标签页已关闭')
        }
        break
      
      case 'close-unpinned':
        const unpinnedTabIds = activeTabs.value.map(t => t.id)
        for (const tabId of unpinnedTabIds) {
          await tabManagerStore.closeTab(tabId)
        }
        ElMessage.success('未固定标签页已关闭')
        break
      
      case 'restore-session':
        // TODO: 实现会话恢复功能
        ElMessage.info('会话恢复功能开发中')
        break
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('执行标签页管理命令失败:', error)
      ElMessage.error('操作失败')
    }
  }
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
