<template>
  <div class="shortcuts-content">
    <div class="shortcuts-list">
      <div v-for="shortcut in shortcuts" :key="shortcut.id"
        :class="['shortcut-item', { 'disabled': !isPluginConnected }]"
        @click="isPluginConnected ? handleShortcutClick(shortcut.content) : null">
        <div class="shortcut-content">
          <div v-if="shortcut.title" class="shortcut-title">{{ shortcut.title }}</div>
          <div class="shortcut-text" :class="{ 'no-title': !shortcut.title }">{{ shortcut.content }}</div>
        </div>
        <div class="shortcut-actions">
          <el-button type="primary" size="small" :icon="EditPen" circle plain
            @click.stop="handleEditShortcut(shortcut)" />
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

    <!-- 编辑快捷短语对话框 -->
    <el-dialog v-model="showEditShortcut" title="编辑快捷短语" width="400px">
      <el-form :model="editShortcut" label-width="80px">
        <el-form-item label="标题">
          <el-input v-model="editShortcut.title" placeholder="标题（可选）" />
        </el-form-item>
        <el-form-item label="内容" required>
          <el-input v-model="editShortcut.content" type="textarea" :rows="4" placeholder="请输入快捷短语内容" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showEditShortcut = false">取消</el-button>
        <el-button type="primary" @click="handleUpdateShortcut">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed, watch } from 'vue'
import { Delete, FolderAdd, EditPen } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useHistoryStore } from '@/stores/history'
import { usePageManagerStore } from '@/stores/pageManager'
import { usePluginStore } from '@/stores/plugins'
import { useShortcutsStore, type Shortcut } from '@/stores/shortcuts'

const historyStore = useHistoryStore()
const pageManagerStore = usePageManagerStore()
const pluginStore = usePluginStore()
const shortcutsStore = useShortcutsStore()

// 插件连接状态
const isPluginConnected = computed(() => {
  const instanceId = pageManagerStore.currentInstanceId
  if (!instanceId) return false
  return pluginStore.getInstanceState(instanceId)?.isConnected || false
})

// 当前插件ID
const currentPluginId = computed(() => pageManagerStore.currentPluginId)
const currentInstanceId = computed(() => pageManagerStore.currentInstanceId)

// 快捷短语状态
const shortcuts = ref<Shortcut[]>([])
const showAddShortcut = ref(false)
const showEditShortcut = ref(false)
const newShortcut = reactive({
  title: '',
  content: ''
})
const editShortcut = reactive({
  id: '',
  title: '',
  content: ''
})
// 编辑快捷短语弹窗
const handleEditShortcut = (shortcut: Shortcut) => {
  editShortcut.id = shortcut.id
  editShortcut.title = shortcut.title || ''
  editShortcut.content = shortcut.content
  showEditShortcut.value = true
}

const handleUpdateShortcut = () => {
  if (!editShortcut.content.trim()) {
    ElMessage.warning('请填写内容')
    return
  }
  if (!currentPluginId.value) {
    ElMessage.error('当前没有活跃的插件')
    return
  }
  const success = shortcutsStore.updateShortcut(
    editShortcut.id,
    currentPluginId.value,
    editShortcut.title.trim(),
    editShortcut.content.trim()
  )
  if (!success) {
    ElMessage.error('编辑失败，可能已存在相同内容的快捷短语')
    return
  }
  try {
    loadShortcuts()
    showEditShortcut.value = false
    editShortcut.id = ''
    editShortcut.title = ''
    editShortcut.content = ''
    ElMessage.success('快捷短语已更新')
  } catch (error) {
    ElMessage.error('编辑快捷短语失败')
    console.error('编辑快捷短语失败:', error)
  }
}

// 加载当前插件的快捷短语
const loadShortcuts = () => {
  if (!currentPluginId.value) {
    shortcuts.value = []
    return
  }
  shortcuts.value = shortcutsStore.getShortcutsByPluginId(currentPluginId.value)
}

// 监听插件ID变化，重新加载快捷短语
watch(currentPluginId, () => {
  loadShortcuts()
}, { immediate: true })

const handleAddShortcut = () => {
  if (!newShortcut.content.trim()) {
    ElMessage.warning('请填写内容')
    return
  }

  if (!currentPluginId.value) {
    ElMessage.error('当前没有活跃的插件')
    return
  }

  const success = shortcutsStore.addShortcut(currentPluginId.value, newShortcut.title.trim(), newShortcut.content.trim())
  if (!success) {
    ElMessage.error('添加快捷短语失败，可能已存在相同内容的快捷短语')
    return
  }

  try {
    // 重新加载快捷短语列表
    loadShortcuts()

    // 重置表单
    newShortcut.title = ''
    newShortcut.content = ''
    showAddShortcut.value = false

    ElMessage.success('快捷短语已添加')
  } catch (error) {
    ElMessage.error('添加快捷短语失败')
    console.error('添加快捷短语失败:', error)
  }
}

const handleDeleteShortcut = async (shortcutId: string) => {
  if (!currentPluginId.value) {
    ElMessage.error('当前没有活跃的插件')
    return
  }

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

    const success = shortcutsStore.deleteShortcut(shortcutId, currentPluginId.value)
    if (success) {
      // 重新加载快捷短语列表
      loadShortcuts()
      ElMessage.success('快捷短语已删除')
    } else {
      ElMessage.error('删除失败，快捷短语不存在')
    }
  } catch {
    // 用户取消删除
  }
}

const handleShortcutClick = async (content: string) => {
  // 检查插件连接状态
  if (!isPluginConnected.value) {
    ElMessage.error('插件未连接，无法发送快捷短语')
    return
  }

  try {
    // 直接发送快捷短语内容
    const currentSessionId = pageManagerStore.currentSessionId
    if (!currentSessionId) {
      throw new Error('当前没有活跃的会话')
    }
    historyStore.addMessageToSession(currentSessionId, content, historyStore.generateMessageId(), 'user')
    pluginStore.sendMessage(content, currentPluginId.value, currentInstanceId.value)
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

.shortcut-item:hover:not(.disabled) {
  background-color: #f5f7fa;
  border-color: #c0c4cc;
}

.shortcut-item.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: #f5f7fa;
  border-color: #e4e7ed;
}

.shortcut-item.disabled .shortcut-title,
.shortcut-item.disabled .shortcut-text {
  color: #c0c4cc;
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
