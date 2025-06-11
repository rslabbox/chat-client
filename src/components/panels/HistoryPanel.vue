<template>
  <div class="history-content">
    <div class="session-list">
      <div v-for="session in currentPluginSessions" :key="session.id"
        :class="['session-item', { 'active': session.id === pageManagerStore.currentSessionId }]"
        @click="handleSessionClick(session.id)">
        <div class="session-content">
          <div class="session-title">{{ session.title }}</div>
          <div class="session-meta">
            <span class="session-time">{{ formatTime(session.updatedAt) }}</span>
            <span class="session-count">{{ session.messages.length }} 条消息</span>
          </div>
        </div>
        <div class="session-actions">
          <el-button type="danger" size="small" :icon="Delete" circle plain
            @click.stop="handleDeleteSession(session.id)" />
        </div>
      </div>

      <div v-if="currentPluginSessions.length === 0" class="empty-sessions">
        <el-empty description="暂无历史对话" :image-size="80" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Delete } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useHistoryStore } from '@/stores/history'
import { usePageManagerStore } from '@/stores/pageManager'

const historyStore = useHistoryStore()
const pageManagerStore = usePageManagerStore()

// 获取当前插件的会话列表
const currentPluginSessions = computed(() => {
  if (!pageManagerStore.currentPluginId) return []
  return historyStore.getSessionsByPluginId(pageManagerStore.currentPluginId)
})

// 时间格式化
const formatTime = (date: Date) => {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))

  if (days === 0) {
    return date.toLocaleTimeString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit'
    })
  } else if (days === 1) {
    return '昨天'
  } else if (days < 7) {
    return `${days}天前`
  } else {
    return date.toLocaleDateString('zh-CN', {
      month: 'short',
      day: 'numeric'
    })
  }
}

// 历史对话相关方法
const handleSessionClick = async (sessionId: string) => {
  try {
    await pageManagerStore.switchToSession(sessionId)
    ElMessage.success('已切换到该对话')
  } catch (error) {
    console.error('切换会话失败:', error)
    ElMessage.error('切换会话失败')
  }
}

const handleDeleteSession = async (sessionId: string) => {
  try {
    await ElMessageBox.confirm(
      '确定要删除这个对话吗？删除后无法恢复。',
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )

    if (historyStore.deleteSession(sessionId)) {
      ElMessage.success('对话已删除')
      if (sessionId == pageManagerStore.currentSessionId && pageManagerStore.currentPage) {
        pageManagerStore.currentPage.sessionId = null
      }
    } else {
      ElMessage.error('删除失败')
    }
  } catch {
    // 用户取消删除
  }
}
</script>

<style scoped>
.history-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
}

.session-list {
  flex: 1 1 0;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 10px;
  /* 确保滚动条正确显示，避免内容被截断 */
  box-sizing: border-box;
  /* 添加底部间距，确保最后一个项目完全可见 */
  padding-bottom: 20px;
  /* 强制flex子项收缩 */
  min-height: 0;
}

.session-item {
  display: flex;
  align-items: center;
  padding: 12px;
  margin-bottom: 8px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid transparent;
}

.session-item:hover {
  background-color: #f5f7fa;
  border-color: #e4e7ed;
}

.session-item.active {
  background-color: #ecf5ff;
  border-color: #409eff;
}

.session-content {
  flex: 1;
  min-width: 0;
}

.session-title {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  color: #909399;
}

.session-time {
  flex-shrink: 0;
}

.session-count {
  flex-shrink: 0;
  margin-left: 8px;
}

.session-actions {
  margin-left: 8px;
  opacity: 0;
  transition: opacity 0.2s;
}

.session-item:hover .session-actions {
  opacity: 1;
}

.empty-sessions {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
}
</style>
