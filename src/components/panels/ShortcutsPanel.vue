<template>
  <div class="shortcuts-content">
    <div class="shortcuts-list">
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
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { Delete, FolderAdd } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useHistoryStore } from '@/stores/history'
import { usePageManagerStore } from '@/stores/pageManager'

const historyStore = useHistoryStore()
const pageManagerStore = usePageManagerStore()

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

// 快捷短语相关方法
const generateShortcutId = () => {
  return `shortcut_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
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
    title: newShortcut.title.trim() || '',
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
    const currentSessionId = pageManagerStore.currentSessionId
    if (!currentSessionId) {
      throw new Error('当前没有活跃的会话')
    }
    historyStore.addMessageToSession(currentSessionId, content, historyStore.generateMessageId(), 'user')
    ElMessage.success('快捷短语已发送')
  } catch (error) {
    ElMessage.error('发送失败，请检查插件连接状态')
    console.error('发送快捷短语失败:', error)
  }
}

// 组件挂载时初始化
onMounted(() => {
  loadShortcuts()
})
</script>

<style scoped>
.shortcuts-content {
  flex: 1;
  display: grid;
  grid-template-rows: 1fr auto;
  overflow: hidden;
  height: 100%;
  min-height: 0;
}

.shortcuts-list {
  overflow-y: auto;
  overflow-x: hidden;
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  /* 确保滚动条正确显示，避免内容被截断 */
  box-sizing: border-box;
  /* 添加底部间距，确保最后一个项目完全可见 */
  padding-bottom: 20px;
  /* Grid布局中的第一行，占用所有可用空间 */
  grid-row: 1;
  min-height: 0;
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

.add-shortcut-bottom {
  padding: 16px;
  border-top: 1px solid #e4e7ed;
  background-color: #ffffff;
  /* Grid布局中的第二行，固定高度 */
  grid-row: 2;
  /* 确保底部按钮区域不会被滚动内容遮挡 */
  position: relative;
  z-index: 1;
}
</style>
