<template>
  <div class="message-input">
    <div class="input-header">
      <h3>发送消息</h3>
    </div>

    <div class="input-content">
      <el-input v-model="inputText" type="textarea" :rows="6" placeholder="请输入消息内容..." resize="none"
        @keydown.ctrl.enter="handleSend" />
    </div>

    <div class="input-footer">
      <div class="input-tips">
        <el-text size="small" type="info">
          Ctrl + Enter 快速发送
        </el-text>
      </div>

      <div class="button-group">
        <el-button @click="handleClear" :icon="Delete" plain>
          清空
        </el-button>

        <el-button type="primary" @click="handleSend" :disabled="!inputText.trim()" :icon="Promotion">
          发送
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Delete, Promotion } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useHistoryStore } from '@/stores/history'
import { usePageManagerStore } from '@/stores/pageManager'
import { usePluginStore } from '@/stores/plugins'

const historyStore = useHistoryStore()
const pageManagerStore = usePageManagerStore()
const pluginStore = usePluginStore()
const inputText = ref('')

const handleSend = async () => {
  const content = inputText.value.trim()
  if (!content) {
    ElMessage.warning('请输入消息内容')
    return
  }

  try {
    let currentSessionId = pageManagerStore.currentSessionId
    const currentPluginId = pageManagerStore.currentPluginId
    const currentInstantId = pageManagerStore.currentInstanceId
    if (!currentSessionId) {
      if (!currentPluginId) {
        throw new Error('当前没有活跃的插件')
      }

      currentSessionId = historyStore.createNewSession(pageManagerStore.currentPluginId)
      if (pageManagerStore.currentPage) {
        pageManagerStore.currentPage.sessionId = currentSessionId
      }
    }

    historyStore.addMessageToSession(currentSessionId, content, historyStore.generateMessageId(), 'user')
    pluginStore.sendMessage(content, currentPluginId, currentInstantId)
    inputText.value = ''
  } catch (error) {
    // 错误已在 store 中处理，这里不需要额外处理
  }
}

const handleClear = () => {
  inputText.value = ''
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