<template>
  <div class="tab-content">
    <!-- 标签页头部信息 -->
    <div v-if="showTabHeader" class="tab-header">
      <div class="tab-info">
        <el-icon v-if="pluginIcon" :color="tab.color">
          <component :is="pluginIcon" />
        </el-icon>
        <span class="tab-title">{{ tab.title }}</span>
        <el-tag v-if="tab.isPinned" size="small" type="info">固定</el-tag>
      </div>
      <div class="tab-stats">
        <span class="message-count">{{ messageCount }} 条消息</span>
        <span class="session-time">{{ formatTime(tab.updatedAt) }}</span>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="main-content">
      <!-- 左侧配置面板 -->
      <div
        v-if="settingsStore.leftPanelVisible"
        class="left-panel"
        :style="{ width: leftPanelWidth + 'px' }"
      >
        <div class="panel-content">
          <ConfigPanel />
        </div>
      </div>

      <!-- 垂直分割线 -->
      <div
        v-if="settingsStore.leftPanelVisible"
        class="vertical-divider"
        @mousedown="startVerticalResize"
      ></div>

      <!-- 中间消息区域 -->
      <div class="center-panel">
        <!-- 上半部分：消息显示区 -->
        <div 
          class="message-display" 
          :style="{ height: messageDisplayHeight + 'px' }"
        >
          <MessageDisplay />
        </div>

        <!-- 水平分割线 -->
        <div class="horizontal-divider" @mousedown="startHorizontalResize"></div>

        <!-- 下半部分：消息发送区 -->
        <div class="message-input">
          <MessageInput />
        </div>
      </div>

      <!-- 垂直分割线 -->
      <div
        v-if="settingsStore.rightPanelVisible"
        class="vertical-divider"
        @mousedown="startRightVerticalResize"
      ></div>

      <!-- 右侧历史记录面板 -->
      <div
        v-if="settingsStore.rightPanelVisible"
        class="right-panel"
        :style="{ width: rightPanelWidth + 'px' }"
      >
        <div class="panel-content">
          <HistoryPanel />
        </div>
      </div>
    </div>

    <!-- 标签页状态指示器 -->
    <div v-if="isLoading" class="loading-indicator">
      <el-loading-service />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import { usePluginStore } from '@/stores/plugins'
import { useHistoryStore } from '@/stores/history'
import ConfigPanel from '../ConfigPanel.vue'
import MessageDisplay from '../MessageDisplay.vue'
import MessageInput from '../MessageInput.vue'
import HistoryPanel from '../HistoryPanel.vue'
import type { Tab } from '@/stores/tabManager'

interface Props {
  tab: Tab
  showTabHeader?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  showTabHeader: false
})

const settingsStore = useSettingsStore()
const pluginStore = usePluginStore()
const historyStore = useHistoryStore()

// 响应式数据
const leftPanelWidth = ref(300)
const rightPanelWidth = ref(300)
const messageDisplayHeight = ref(400)
const isLoading = ref(false)

// 计算属性
const pluginIcon = computed(() => {
  const plugin = pluginStore.plugins.find(p => p.id === props.tab.pluginId)
  return plugin?.icon
})

const messageCount = computed(() => {
  const session = historyStore.getSessionById(props.tab.sessionId)
  return session?.messages.length || 0
})

// 调整大小相关变量
let isResizingVertical = false
let isResizingRightVertical = false
let isResizingHorizontal = false
let startX = 0
let startY = 0
let startWidth = 0
let startHeight = 0

// 格式化时间
const formatTime = (date: Date) => {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / (1000 * 60))
  
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}小时前`
  
  const days = Math.floor(hours / 24)
  if (days < 7) return `${days}天前`
  
  return date.toLocaleDateString('zh-CN')
}

// 垂直调整大小（左侧面板）
const startVerticalResize = (e: MouseEvent) => {
  isResizingVertical = true
  startX = e.clientX
  startWidth = leftPanelWidth.value
  document.addEventListener('mousemove', doVerticalResize)
  document.addEventListener('mouseup', stopVerticalResize)
  e.preventDefault()
}

const doVerticalResize = (e: MouseEvent) => {
  if (!isResizingVertical) return
  
  const deltaX = e.clientX - startX
  const newWidth = startWidth + deltaX
  const minWidth = 200
  const maxWidth = window.innerWidth * 0.5
  
  leftPanelWidth.value = Math.max(minWidth, Math.min(newWidth, maxWidth))
}

const stopVerticalResize = () => {
  isResizingVertical = false
  document.removeEventListener('mousemove', doVerticalResize)
  document.removeEventListener('mouseup', stopVerticalResize)
}

// 右侧垂直调整大小
const startRightVerticalResize = (e: MouseEvent) => {
  isResizingRightVertical = true
  startX = e.clientX
  startWidth = rightPanelWidth.value
  document.addEventListener('mousemove', doRightVerticalResize)
  document.addEventListener('mouseup', stopRightVerticalResize)
  e.preventDefault()
}

const doRightVerticalResize = (e: MouseEvent) => {
  if (!isResizingRightVertical) return

  const deltaX = startX - e.clientX
  const newWidth = startWidth + deltaX
  const minWidth = 200
  const maxWidth = window.innerWidth * 0.5

  rightPanelWidth.value = Math.max(minWidth, Math.min(newWidth, maxWidth))
}

const stopRightVerticalResize = () => {
  isResizingRightVertical = false
  document.removeEventListener('mousemove', doRightVerticalResize)
  document.removeEventListener('mouseup', stopRightVerticalResize)
}

// 水平调整大小
const startHorizontalResize = (e: MouseEvent) => {
  isResizingHorizontal = true
  startY = e.clientY
  startHeight = messageDisplayHeight.value
  document.addEventListener('mousemove', doHorizontalResize)
  document.addEventListener('mouseup', stopHorizontalResize)
  e.preventDefault()
}

const doHorizontalResize = (e: MouseEvent) => {
  if (!isResizingHorizontal) return
  
  const deltaY = e.clientY - startY
  const newHeight = startHeight + deltaY
  const minHeight = 200
  const maxHeight = window.innerHeight - 300
  
  messageDisplayHeight.value = Math.max(minHeight, Math.min(newHeight, maxHeight))
}

const stopHorizontalResize = () => {
  isResizingHorizontal = false
  document.removeEventListener('mousemove', doHorizontalResize)
  document.removeEventListener('mouseup', stopHorizontalResize)
}

// 初始化
onMounted(() => {
  // 设置初始高度
  messageDisplayHeight.value = window.innerHeight - 320
  
  // 窗口大小变化时的处理
  const handleResize = () => {
    const maxWidth = window.innerWidth * 0.5
    if (leftPanelWidth.value > maxWidth) {
      leftPanelWidth.value = maxWidth
    }
    if (rightPanelWidth.value > maxWidth) {
      rightPanelWidth.value = maxWidth
    }

    messageDisplayHeight.value = window.innerHeight - 320
  }
  
  window.addEventListener('resize', handleResize)
  
  onUnmounted(() => {
    window.removeEventListener('resize', handleResize)
  })
})
</script>

<style scoped>
.tab-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: #f5f5f5;
  position: relative;
}

.tab-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background-color: #ffffff;
  border-bottom: 1px solid #e4e7ed;
  flex-shrink: 0;
}

.tab-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tab-title {
  font-weight: 500;
  color: #303133;
}

.tab-stats {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: #909399;
}

.main-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.left-panel,
.right-panel {
  background-color: #ffffff;
  border-right: 1px solid #e4e7ed;
  min-width: 200px;
  max-width: 50vw;
  overflow: hidden;
}

.right-panel {
  border-right: none;
  border-left: 1px solid #e4e7ed;
}

.panel-content {
  width: 100%;
  height: 100%;
}

.vertical-divider {
  width: 4px;
  background-color: #dcdfe6;
  cursor: col-resize;
  transition: background-color 0.2s;
}

.vertical-divider:hover {
  background-color: #409eff;
}

.center-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: #ffffff;
  min-width: 400px;
}

.message-display {
  border-bottom: 1px solid #e4e7ed;
  min-height: 200px;
}

.horizontal-divider {
  height: 4px;
  background-color: #dcdfe6;
  cursor: row-resize;
  transition: background-color 0.2s;
}

.horizontal-divider:hover {
  background-color: #409eff;
}

.message-input {
  flex: 1;
  min-height: 150px;
}

.loading-indicator {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .tab-header {
    padding: 6px 12px;
  }
  
  .tab-stats {
    flex-direction: column;
    gap: 4px;
    align-items: flex-end;
  }
  
  .center-panel {
    min-width: 300px;
  }
}
</style>
