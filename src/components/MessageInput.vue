<template>
  <div class="message-input">
    <div class="input-header">
      <h3>发送消息</h3>
    </div>

    <div class="input-content">
      <el-input v-model="inputText" type="textarea" :rows="6"
        :placeholder="isPluginConnected ? '请输入消息内容...' : '插件未连接，无法发送消息'" :disabled="!isPluginConnected" resize="none"
        @keydown.ctrl.enter="handleSend" />
    </div>

    <div class="input-footer">
      <div class="input-tips">
        <el-text size="small" :type="isPluginConnected ? 'info' : 'warning'">
          {{ isPluginConnected ? 'Ctrl + Enter 快速发送' : '请先连接插件' }}
        </el-text>
      </div>

      <div class="button-group">
        <el-button @click="handleClear" :icon="Delete" :disabled="!isPluginConnected" plain>
          清空
        </el-button>

        <el-button type="primary" @click="() => {
          console.log('按钮点击 - hasActiveStream:', hasActiveStream);
          console.log('按钮点击 - 将调用:', hasActiveStream ? 'handleStop' : 'handleSend');
          hasActiveStream ? handleStop() : handleSend();
        }" :disabled="hasActiveStream ? !canStop : !canSend" :icon="buttonIcon">
          {{ buttonText }}
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Delete, Promotion, VideoPause } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useHistoryStore } from '@/stores/history'
import { usePageManagerStore } from '@/stores/pageManager'
import { usePluginStore } from '@/stores/plugins'
import { useStreamStore } from '@/stores/stream'
import { useTabManagerStore } from '@/stores/tabManager'
import { cancelStreamMessage } from '@/api'

const historyStore = useHistoryStore()
const pageManagerStore = usePageManagerStore()
const pluginStore = usePluginStore()
const streamStore = useStreamStore()
const tabManagerStore = useTabManagerStore()
const inputText = ref('')

// 获取当前实例ID（优先使用 tabManager，回退到 pageManager）
const currentInstanceId = computed(() => {
  return tabManagerStore.activeTab?.instanceId || pageManagerStore.currentInstanceId
})

// 获取当前插件ID
const currentPluginId = computed(() => {
  return tabManagerStore.activeTab?.pluginId || pageManagerStore.currentPluginId
})

// 获取当前会话ID
const currentSessionId = computed(() => {
  return tabManagerStore.activeTab?.sessionId || pageManagerStore.currentSessionId
})

// 插件连接状态
const isPluginConnected = computed(() => {
  const instanceId = currentInstanceId.value
  if (!instanceId) return false
  return pluginStore.getInstanceState(instanceId)?.isConnected || false
})

// 检查当前实例是否有活跃的流式消息
const hasActiveStream = computed(() => {
  const instanceId = currentInstanceId.value
  const result = instanceId ? streamStore.hasActiveStream(instanceId) : false
  console.log('hasActiveStream 计算:', { instanceId, result, allStreams: streamStore.activeStreams })
  return result
})

// 获取当前活跃的流式消息
const activeStream = computed(() => {
  const instanceId = currentInstanceId.value
  const result = instanceId ? streamStore.getActiveStreamByInstance(instanceId) : null
  console.log('activeStream 计算:', { instanceId, result })
  return result
})

// 按钮文本
const buttonText = computed(() => {
  const result = hasActiveStream.value ? '停止' : '发送'
  console.log('buttonText 计算:', { hasActiveStream: hasActiveStream.value, result })
  return result
})

// 按钮图标
const buttonIcon = computed(() => {
  return hasActiveStream.value ? VideoPause : Promotion
})

// 发送按钮是否可用
const canSend = computed(() => {
  return inputText.value.trim() && isPluginConnected.value
})

// 停止按钮是否可用
const canStop = computed(() => {
  return hasActiveStream.value && isPluginConnected.value
})

const handleSend = async () => {
  const content = inputText.value.trim()
  if (!content) {
    ElMessage.warning('请输入消息内容')
    return
  }

  // 检查插件连接状态
  if (!isPluginConnected.value) {
    ElMessage.error('插件未连接，无法发送消息')
    return
  }

  try {
    let sessionId = currentSessionId.value
    const pluginId = currentPluginId.value
    const instanceId = currentInstanceId.value

    if (!sessionId) {
      if (!pluginId) {
        throw new Error('当前没有活跃的插件')
      }

      sessionId = historyStore.createNewSession(pluginId)

      // 更新对应的管理器中的会话ID
      if (tabManagerStore.activeTab) {
        tabManagerStore.activeTab.sessionId = sessionId
      } else if (pageManagerStore.currentPage) {
        pageManagerStore.currentPage.sessionId = sessionId
      }
    }

    historyStore.addMessageToSession(sessionId, content, historyStore.generateMessageId(), 'user')
    pluginStore.sendMessage(content, pluginId, instanceId)
    inputText.value = ''
  } catch (error) {
    // 错误已在 store 中处理，这里不需要额外处理
  }
}

const handleClear = () => {
  inputText.value = ''
}

// 停止流式消息
const handleStop = async () => {
  const instanceId = currentInstanceId.value
  const currentStream = activeStream.value

  console.log('停止流式消息 - 当前实例ID:', instanceId)
  console.log('停止流式消息 - 当前流:', currentStream)
  console.log('停止流式消息 - 所有活跃流:', streamStore.activeStreams)

  if (!instanceId || !currentStream) {
    ElMessage.error('没有找到活跃的流式消息')
    return
  }

  try {
    console.log('调用取消API:', { instanceId, streamId: currentStream.streamId })
    await cancelStreamMessage(instanceId, currentStream.streamId)
    ElMessage.success('流式消息已停止')
  } catch (error) {
    console.error('停止流式消息失败:', error)
    ElMessage.error('停止流式消息失败')
  }
}
</script>

<style scoped>
.message-input {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px;
}

.input-header {
  margin-bottom: 15px;
  border-bottom: 1px solid #e4e7ed;
  padding-bottom: 10px;
}

.input-header h3 {
  margin: 0;
  color: #303133;
  font-size: 16px;
  font-weight: 600;
}

.input-content {
  flex: 1;
  margin-bottom: 15px;
}

.input-content :deep(.el-textarea__inner) {
  resize: none !important;
  height: 100% !important;
  min-height: 120px !important;
}

.input-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.input-tips {
  flex: 1;
}

.button-group {
  display: flex;
  gap: 10px;
}
</style>