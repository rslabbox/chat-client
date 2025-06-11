<template>
  <div class="tab-manager">
    <!-- 标签页栏 -->
    <TabBar />

    <!-- 标签页内容区域 -->
    <div class="tab-content-area">
      <!-- 当前活跃标签页的内容 -->
      <TabContent v-if="activeTab" :key="activeTab.id" :tab="activeTab" class="active-tab-content" />

      <!-- 无标签页时的空状态 -->
      <div v-else class="empty-state">
        <el-empty description="暂无打开的标签页" :image-size="120">
          <template #description>
            <p>暂无打开的标签页</p>
            <p class="empty-hint">选择一个插件开始新的对话</p>
          </template>
          <el-dropdown @command="handleCreateTab" trigger="click">
            <el-button type="primary" :icon="Plus">
              新建标签页
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item v-for="plugin in availablePlugins" :key="plugin.id" :command="plugin.id"
                  :disabled="plugin.disabled">
                  <el-icon v-if="plugin.icon" class="plugin-icon">
                    <component :is="plugin.icon" />
                  </el-icon>
                  {{ plugin.name }}
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </el-empty>
      </div>
    </div>

    <!-- 标签页加载状态 -->
    <div v-if="isLoading" class="tab-loading-overlay" v-loading="isLoading" element-loading-text="加载中...">
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { Plus } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useTabManagerStore } from '@/stores/tabManager'
import { usePluginStore } from '@/stores/plugins'
import TabBar from './TabBar.vue'
import TabContent from './TabContent.vue'

const tabManagerStore = useTabManagerStore()
const pluginStore = usePluginStore()

// 计算属性
const activeTab = computed(() => tabManagerStore.activeTab)
const isLoading = computed(() => tabManagerStore.isLoading)
const availablePlugins = computed(() => pluginStore.plugins.filter(p => !p.disabled))

// 创建新标签页
const handleCreateTab = async (pluginId: string) => {
  try {
    await tabManagerStore.createNewTab(pluginId)
    ElMessage.success('新标签页已创建')
  } catch (error) {
    console.error('创建标签页失败:', error)
    ElMessage.error('创建标签页失败')
  }
}

// 键盘快捷键处理
const handleKeydown = (event: KeyboardEvent) => {
  // Ctrl/Cmd + T: 新建标签页
  if ((event.ctrlKey || event.metaKey) && event.key === 't') {
    event.preventDefault()
    // 显示插件选择菜单或创建默认标签页
    if (availablePlugins.value.length === 1) {
      handleCreateTab(availablePlugins.value[0].id)
    }
    // 如果有多个插件，这里可以显示快速选择界面
  }

  // Ctrl/Cmd + W: 关闭当前标签页
  if ((event.ctrlKey || event.metaKey) && event.key === 'w') {
    event.preventDefault()
    if (activeTab.value && !activeTab.value.isPinned) {
      tabManagerStore.closeTab(activeTab.value.id)
    }
  }

  // Ctrl/Cmd + Shift + T: 恢复最近关闭的标签页
  if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.key === 'T') {
    event.preventDefault()
    // TODO: 实现恢复最近关闭的标签页
    ElMessage.info('恢复标签页功能开发中')
  }

  // Ctrl/Cmd + 数字键: 切换到对应标签页
  if ((event.ctrlKey || event.metaKey) && /^[1-9]$/.test(event.key)) {
    event.preventDefault()
    const index = parseInt(event.key) - 1
    const tabs = tabManagerStore.tabs
    if (index < tabs.length) {
      tabManagerStore.switchToTab(tabs[index].id)
    }
  }

  // Ctrl/Cmd + Tab: 切换到下一个标签页
  if ((event.ctrlKey || event.metaKey) && event.key === 'Tab') {
    event.preventDefault()
    const tabs = tabManagerStore.tabs
    if (tabs.length > 1 && activeTab.value) {
      const currentIndex = tabs.findIndex(t => t.id === activeTab.value!.id)
      const nextIndex = (currentIndex + 1) % tabs.length
      tabManagerStore.switchToTab(tabs[nextIndex].id)
    }
  }

  // Ctrl/Cmd + Shift + Tab: 切换到上一个标签页
  if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.key === 'Tab') {
    event.preventDefault()
    const tabs = tabManagerStore.tabs
    if (tabs.length > 1 && activeTab.value) {
      const currentIndex = tabs.findIndex(t => t.id === activeTab.value!.id)
      const prevIndex = currentIndex === 0 ? tabs.length - 1 : currentIndex - 1
      tabManagerStore.switchToTab(tabs[prevIndex].id)
    }
  }
}

// 组件挂载时设置事件监听
onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
  
  // 如果没有标签页且有可用插件，可以考虑自动创建一个默认标签页
  if (tabManagerStore.tabs.length === 0 && availablePlugins.value.length > 0) {
    // 这里可以根据用户偏好决定是否自动创建标签页
    console.log('没有打开的标签页，等待用户手动创建')
  }
})

// 组件卸载时清理事件监听
onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.tab-manager {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: #f5f5f5;
  position: relative;
}

.tab-content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: #ffffff;
}

.active-tab-content {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #fafafa;
}

.empty-hint {
  color: #909399;
  font-size: 14px;
  margin-top: 8px;
}

.plugin-icon {
  margin-right: 8px;
}

.tab-loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .empty-state {
    padding: 20px;
  }
  
  .empty-hint {
    font-size: 13px;
  }
}

/* 动画效果 */
.active-tab-content {
  animation: tab-content-appear 0.2s ease-out;
}

@keyframes tab-content-appear {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 确保标签页内容区域正确显示 */
.tab-content-area > * {
  width: 100%;
  height: 100%;
}
</style>
