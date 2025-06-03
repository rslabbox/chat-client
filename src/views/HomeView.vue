<template>
  <div class="app-container">
    <!-- 顶部工具栏 -->
    <PluginToolbar />

    <!-- 主要内容区域 -->
    <div class="main-content">
      <!-- 左侧配置面板 -->
      <div
        ref="leftPanel"
        class="left-panel"
        :class="{ 'panel-hidden': !settingsStore.leftPanelVisible }"
        :style="{ width: settingsStore.leftPanelVisible ? leftPanelWidth + 'px' : '0px' }"
      >
        <div class="panel-content" v-show="settingsStore.leftPanelVisible">
          <ConfigPanel />
        </div>
      </div>

      <!-- 垂直分割线 -->
      <div
        class="vertical-divider"
        v-show="settingsStore.leftPanelVisible"
        @mousedown="startVerticalResize"
      ></div>

      <!-- 中间消息区域 -->
      <div class="center-panel">
        <!-- 上半部分：消息显示区 -->
        <div ref="messageDisplay" class="message-display" :style="{ height: messageDisplayHeight + 'px' }">
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
        class="vertical-divider"
        v-show="settingsStore.rightPanelVisible"
        @mousedown="startRightVerticalResize"
      ></div>

      <!-- 右侧历史记录面板 -->
      <div
        ref="rightPanel"
        class="right-panel"
        :class="{ 'panel-hidden': !settingsStore.rightPanelVisible }"
        :style="{ width: settingsStore.rightPanelVisible ? rightPanelWidth + 'px' : '0px' }"
      >
        <div class="panel-content" v-show="settingsStore.rightPanelVisible">
          <HistoryPanel />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import PluginToolbar from '../components/PluginToolbar.vue'
import ConfigPanel from '../components/ConfigPanel.vue'
import MessageDisplay from '../components/MessageDisplay.vue'
import MessageInput from '../components/MessageInput.vue'
import HistoryPanel from '../components/HistoryPanel.vue'

const settingsStore = useSettingsStore()

// 响应式数据
const leftPanelWidth = ref(300) // 左侧面板宽度，默认占1/4
const rightPanelWidth = ref(300) // 右侧面板宽度，默认占1/4
const messageDisplayHeight = ref(200) // 消息显示区高度

// DOM引用
const leftPanel = ref<HTMLElement>()
const rightPanel = ref<HTMLElement>()
const messageDisplay = ref<HTMLElement>()

// 调整大小相关变量
let isResizingVertical = false
let isResizingRightVertical = false
let isResizingHorizontal = false
let startX = 0
let startY = 0
let startWidth = 0
let startHeight = 0

// 垂直调整大小
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

  const deltaX = startX - e.clientX // 注意这里是反向的
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
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;  /* 使用视口高度 */
  width: 100vw;   /* 使用视口宽度 */
  background-color: #f5f5f5;
  overflow: hidden;
  /* 确保容器占满整个视口 */
  position: fixed;
  top: 0;
  left: 0;
}

.top-nav {
  height: 50px;
  background-color: #ffffff;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  padding: 0 20px;
  flex-shrink: 0;
}

.main-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.left-panel {
  background-color: #ffffff;
  border-right: 1px solid #e4e7ed;
  min-width: 200px;
  max-width: 50vw;
  transition: width 0.3s ease;
  overflow: hidden;
}

.left-panel.panel-hidden {
  min-width: 0;
  border-right: none;
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

.right-panel {
  background-color: #ffffff;
  border-left: 1px solid #e4e7ed;
  min-width: 200px;
  max-width: 50vw;
  transition: width 0.3s ease;
  overflow: hidden;
}

.right-panel.panel-hidden {
  min-width: 0;
  border-left: none;
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
</style>