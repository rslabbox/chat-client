<template>
  <div class="app-container">
    <!-- 左侧配置面板 -->
    <div 
      ref="leftPanel" 
      class="left-panel"
      :style="{ width: leftPanelWidth + 'px' }"
    >
      <ConfigPanel 
        @connect="handleConnect"
        @disconnect="handleDisconnect"
        :isConnected="isConnected"
      />
    </div>

    <!-- 垂直分割线 -->
    <div 
      class="vertical-divider" 
      @mousedown="startVerticalResize"
    ></div>

    <!-- 右侧消息区域 -->
    <div class="right-panel">
      <!-- 上半部分：消息显示区 -->
      <div 
        ref="messageDisplay"
        class="message-display"
        :style="{ height: messageDisplayHeight + 'px' }"
      >
        <MessageDisplay 
          :messages="messages"
          @clear="handleClearMessages"
        />
      </div>

      <!-- 水平分割线 -->
      <div 
        class="horizontal-divider" 
        @mousedown="startHorizontalResize"
      ></div>

      <!-- 下半部分：消息发送区 -->
      <div class="message-input">
        <MessageInput 
          @send="handleSendMessage"
          @clear="handleClearInput"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import ConfigPanel from './ConfigPanel.vue'
import MessageDisplay from './MessageDisplay.vue'
import MessageInput from './MessageInput.vue'

interface Message {
  id: string
  content: string
  timestamp: Date
  type: 'sent' | 'received'
}

// 响应式数据
const leftPanelWidth = ref(300) // 左侧面板宽度，默认占1/4
const messageDisplayHeight = ref(200) // 消息显示区高度
const messages = ref<Message[]>([])
const isConnected = ref(false)
console.log(messageDisplayHeight);

// DOM引用
const leftPanel = ref<HTMLElement>()
const messageDisplay = ref<HTMLElement>()

// 调整大小相关变量
let isResizingVertical = false
let isResizingHorizontal = false
let startX = 0
let startY = 0
let startWidth = 0
let startHeight = 0

// 事件处理函数
const handleConnect = () => {
  isConnected.value = true
  console.log('连接插件')
}

const handleDisconnect = () => {
  isConnected.value = false
  console.log('断开连接')
}

const handleSendMessage = (content: string) => {
  const message: Message = {
    id: Date.now().toString(),
    content,
    timestamp: new Date(),
    type: 'sent'
  }
  messages.value.push(message)
  console.log('发送消息:', content)
  
  // 模拟接收回复
  setTimeout(() => {
    const reply: Message = {
      id: (Date.now() + 1).toString(),
      content: `收到消息: ${content}`,
      timestamp: new Date(),
      type: 'received'
    }
    messages.value.push(reply)
  }, 1000)
}

const handleClearMessages = () => {
  messages.value = []
}

const handleClearInput = () => {
  console.log('清空输入')
}

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
  messageDisplayHeight.value = window.innerHeight - 300
  console.log('清空输入')
  
  // 窗口大小变化时的处理
  const handleResize = () => {
    console.log("rrrr", window.innerHeight);
    const maxWidth = window.innerWidth * 0.5
    if (leftPanelWidth.value > maxWidth) {
      leftPanelWidth.value = maxWidth
    }
    
    messageDisplayHeight.value = window.innerHeight - 300
    // if (messageDisplayHeight.value > maxHeight) {
    //   messageDisplayHeight.value = maxHeight
    // }
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
  height: 100vh;  /* 使用视口高度 */
  width: 100vw;   /* 使用视口宽度 */
  background-color: #f5f5f5;
  overflow: hidden;
  /* 确保容器占满整个视口 */
  position: fixed;
  top: 0;
  left: 0;
}

.left-panel {
  background-color: #ffffff;
  border-right: 1px solid #e4e7ed;
  min-width: 200px;
  max-width: 50vw;
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

.right-panel {
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
</style>