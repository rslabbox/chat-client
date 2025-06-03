<template>
  <div class="history-panel">
    <div class="panel-header">
      <div class="tab-switcher">
        <div :class="['tab-item', { 'active': activeTab === 'history' }]" @click="activeTab = 'history'">
          历史对话
        </div>
        <div :class="['tab-item', { 'active': activeTab === 'shortcuts' }]" @click="activeTab = 'shortcuts'">
          快捷短语
        </div>
      </div>
    </div>

    <!-- 历史对话内容 -->
    <div v-show="activeTab === 'history'" class="session-list">
      <div v-for="session in currentPluginSessions" :key="session.id"
        :class="['session-item', { 'active': session.id === currentSessionId }]"
        @click="handleSessionClick(session.id)">
        <div class="session-content">
          <div class="session-title">{{ session.title }}</div>
          <div class="session-meta">
            <span class="session-time">{{ formatTime(session.updatedAt) }}</span>
            <span class="session-count">{{ session.messageCount }} 条消息</span>
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

    <!-- 快捷短语内容 -->
    <div v-show="activeTab === 'shortcuts'" class="shortcuts-list">
      <div class="shortcuts-content">
        <div v-for="shortcut in shortcuts" :key="shortcut.id" class="shortcut-item"
          @click="handleShortcutClick(shortcut.content)">
          <div class="shortcut-content">
            <div v-if="shortcut.title" class="shortcut-title">{{ shortcut.title }}</div>
            <div class="shortcut-text" :class="{ 'no-title': !shortcut.title }">{{ shortcut.content }}</div>
          </div>
          <div class="shortcut-actions">
            <el-button type="danger" size="small" :icon="Delete" circle plain
              @click.stop="handleDeleteShortcut(shortcut.id)" />
          </div>
        </div>

        <div v-if="shortcuts.length === 0" class="empty-shortcuts">
          <el-empty description="暂无快捷短语" :image-size="80" />
        </div>
      </div>

      <!-- 添加按钮固定在底部 -->
      <div class="add-shortcut-bottom">
        <el-button type="primary" @click="showAddShortcut = true" :icon="FolderAdd"
          style="width: 100%; margin-bottom: 10px;">
          添加快捷短语
        </el-button>
      </div>
    </div>
  </div>

  <!-- 添加快捷短语对话框 -->
  <el-dialog v-model="showAddShortcut" title="添加快捷短语" width="400px">
    <el-form :model="newShortcut" label-width="80px">
      <el-form-item label="标题">
        <el-input v-model="newShortcut.title" placeholder="标题（可选）" />
      </el-form-item>
      <el-form-item label="内容" required>
        <el-input v-model="newShortcut.content" type="textarea" :rows="4" placeholder="请输入快捷短语内容" />
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="showAddShortcut = false">取消</el-button>
      <el-button type="primary" @click="handleAddShortcut">确定</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { storeToRefs } from 'pinia'
import { Delete } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useMessageStore } from '@/stores/messages'
import { FolderAdd, AddLocation } from '@element-plus/icons-vue'

const messageStore = useMessageStore()
const { currentPluginSessions, currentSessionId } = storeToRefs(messageStore)

// 标签页状态
const activeTab = ref<'history' | 'shortcuts'>('history')

// 快捷短语接口
interface Shortcut {
  id: string
  title: string
  content: string
  createdAt: Date
}

// 快捷短语状态
const shortcuts = ref<Shortcut[]>([])
const showAddShortcut = ref(false)
const newShortcut = reactive({
  title: '',
  content: ''
})

// 快捷短语存储键
const SHORTCUTS_STORAGE_KEY = 'chat-client-shortcuts'

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

const handleSessionClick = (sessionId: string) => {
  if (messageStore.switchToSession(sessionId)) {
    ElMessage.success('已切换到该对话')
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

    if (messageStore.deleteSession(sessionId)) {
      ElMessage.success('对话已删除')
    } else {
      ElMessage.error('删除失败')
    }
  } catch {
    // 用户取消删除
  }
}

// 快捷短语相关方法
const generateShortcutId = () => {
  return `shortcut_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
}

const loadShortcuts = () => {
  try {
    const stored = localStorage.getItem(SHORTCUTS_STORAGE_KEY)
    if (stored) {
      const data = JSON.parse(stored)
      shortcuts.value = data.map((item: any) => ({
        ...item,
        createdAt: new Date(item.createdAt)
      }))
    }
  } catch (error) {
    console.error('加载快捷短语失败:', error)
  }
}

const saveShortcuts = () => {
  try {
    localStorage.setItem(SHORTCUTS_STORAGE_KEY, JSON.stringify(shortcuts.value))
  } catch (error) {
    console.error('保存快捷短语失败:', error)
  }
}

const handleAddShortcut = () => {
  if (!newShortcut.content.trim()) {
    ElMessage.warning('请填写内容')
    return
  }

  const shortcut: Shortcut = {
    id: generateShortcutId(),
    title: newShortcut.title.trim() || '', // 标题可以为空
    content: newShortcut.content.trim(),
    createdAt: new Date()
  }

  shortcuts.value.unshift(shortcut)
  saveShortcuts()

  // 重置表单
  newShortcut.title = ''
  newShortcut.content = ''
  showAddShortcut.value = false

  ElMessage.success('快捷短语已添加')
}

const handleDeleteShortcut = async (shortcutId: string) => {
  try {
    await ElMessageBox.confirm(
      '确定要删除这个快捷短语吗？',
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )

    const index = shortcuts.value.findIndex(s => s.id === shortcutId)
    if (index > -1) {
      shortcuts.value.splice(index, 1)
      saveShortcuts()
      ElMessage.success('快捷短语已删除')
    }
  } catch {
    // 用户取消删除
  }
}

const handleShortcutClick = async (content: string) => {
  try {
    // 直接发送快捷短语内容
    await messageStore.sendMessage(content)
    ElMessage.success('快捷短语已发送')
  } catch (error) {
    ElMessage.error('发送失败，请检查插件连接状态')
    console.error('发送快捷短语失败:', error)
  }
}

// 初始化加载快捷短语
loadShortcuts()
</script>

<style scoped>
.history-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: #ffffff;
}

.panel-header {
  border-bottom: 1px solid #e4e7ed;
  flex-shrink: 0;
}

.tab-switcher {
  display: flex;
  background-color: #f5f7fa;
}

.tab-item {
  flex: 1;
  padding: 12px 16px;
  text-align: center;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  color: #606266;
  background-color: #f5f7fa;
  border-bottom: 2px solid transparent;
  transition: all 0.2s;
}

.tab-item:hover {
  color: #409eff;
  background-color: #ecf5ff;
}

.tab-item.active {
  color: #409eff;
  background-color: #ffffff;
  border-bottom-color: #409eff;
}

.session-list {
  flex: 1;
  overflow-y: auto;
  padding: 10px;
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

/* 快捷短语样式 */
.shortcuts-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.shortcuts-content {
  flex: 1;
  overflow-y: auto;
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.shortcut-item {
  display: flex;
  align-items: center;
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid #e4e7ed;
  background-color: #ffffff;
}

.shortcut-item:hover {
  background-color: #f5f7fa;
  border-color: #c0c4cc;
}

.shortcut-content {
  flex: 1;
  min-width: 0;
}

.shortcut-title {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.shortcut-text {
  font-size: 12px;
  color: #909399;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.shortcut-text.no-title {
  font-size: 14px;
  color: #303133;
  font-weight: 500;
}

.shortcut-actions {
  margin-left: 8px;
  opacity: 0;
  transition: opacity 0.2s;
}

.shortcut-item:hover .shortcut-actions {
  opacity: 1;
}

.empty-shortcuts {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
}

/* 底部添加按钮样式 */
.add-shortcut-bottom {
  padding: 16px;
  border-top: 1px solid #e4e7ed;
  background-color: #ffffff;
  flex-shrink: 0;
}

.add-shortcut-btn {
  width: 100%;
  height: 30px;
  font-size: 16px;
  font-weight: 600;
}
</style>
