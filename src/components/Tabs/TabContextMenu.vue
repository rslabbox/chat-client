<template>
  <teleport to="body">
    <div
      v-if="visible"
      class="tab-context-menu"
      :style="menuStyle"
      @click.stop
      @contextmenu.prevent
    >
      <div class="context-menu-content">
        <!-- 重命名 -->
        <div class="menu-item" @click="handleRename">
          <el-icon><Edit /></el-icon>
          <span>重命名</span>
        </div>

        <!-- 固定/取消固定 -->
        <div class="menu-item" @click="handlePin">
          <el-icon>
            <Lock v-if="!tab?.isPinned" />
            <Unlock v-else />
          </el-icon>
          <span>{{ tab?.isPinned ? '取消固定' : '固定标签页' }}</span>
        </div>

        <div class="menu-divider"></div>

        <!-- 复制标签页 -->
        <div class="menu-item" @click="handleDuplicate">
          <el-icon><CopyDocument /></el-icon>
          <span>复制标签页</span>
        </div>

        <!-- 在新窗口打开 -->
        <div class="menu-item" @click="handleOpenInNewWindow">
          <el-icon><Monitor /></el-icon>
          <span>在新窗口打开</span>
        </div>

        <div class="menu-divider"></div>

        <!-- 关闭标签页 -->
        <div 
          class="menu-item" 
          :class="{ disabled: tab?.isPinned }"
          @click="handleClose"
        >
          <el-icon><Close /></el-icon>
          <span>关闭标签页</span>
        </div>

        <!-- 关闭其他标签页 -->
        <div class="menu-item" @click="handleCloseOthers">
          <el-icon><CloseBold /></el-icon>
          <span>关闭其他标签页</span>
        </div>

        <!-- 关闭右侧标签页 -->
        <div class="menu-item" @click="handleCloseToRight">
          <el-icon><Right /></el-icon>
          <span>关闭右侧标签页</span>
        </div>

        <div class="menu-divider"></div>

        <!-- 会话信息 -->
        <div class="menu-item info-item">
          <div class="session-info">
            <div class="info-row">
              <span class="info-label">插件:</span>
              <span class="info-value">{{ pluginName }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">会话:</span>
              <span class="info-value">{{ sessionInfo }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">创建:</span>
              <span class="info-value">{{ formatTime(tab?.createdAt) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 重命名对话框 -->
    <el-dialog
      v-model="renameDialogVisible"
      title="重命名标签页"
      width="400px"
      :before-close="handleRenameCancel"
    >
      <el-form @submit.prevent="handleRenameConfirm">
        <el-form-item label="标签页名称">
          <el-input
            v-model="newTabTitle"
            placeholder="请输入新的标签页名称"
            maxlength="50"
            show-word-limit
            ref="renameInputRef"
            @keyup.enter="handleRenameConfirm"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="handleRenameCancel">取消</el-button>
          <el-button type="primary" @click="handleRenameConfirm">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { 
  Edit, Lock, Unlock, CopyDocument, Monitor, Close, CloseBold, Right 
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { usePluginStore } from '@/stores/plugins'
import type { Tab } from '@/stores/tabManager'

interface Props {
  visible: boolean
  tab: Tab | null
  position: { x: number; y: number }
}

interface Emits {
  (e: 'update:visible', visible: boolean): void
  (e: 'rename', tab: Tab, newTitle: string): void
  (e: 'pin', tab: Tab): void
  (e: 'close', tab: Tab): void
  (e: 'close-others', tab: Tab): void
  (e: 'close-to-right', tab: Tab): void
  (e: 'duplicate', tab: Tab): void
  (e: 'open-in-new-window', tab: Tab): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const pluginStore = usePluginStore()

// 响应式数据
const renameDialogVisible = ref(false)
const newTabTitle = ref('')
const renameInputRef = ref()

// 计算属性
const menuStyle = computed(() => ({
  left: `${props.position.x}px`,
  top: `${props.position.y}px`,
  zIndex: 9999
}))

const pluginName = computed(() => {
  if (!props.tab) return ''
  const plugin = pluginStore.plugins.find(p => p.id === props.tab!.pluginId)
  return plugin?.name || '未知插件'
})

const sessionInfo = computed(() => {
  if (!props.tab) return ''
  return `${props.tab.sessionId.substring(0, 8)}...`
})

// 监听菜单显示状态
watch(() => props.visible, (visible) => {
  if (visible) {
    // 点击其他地方关闭菜单
    nextTick(() => {
      document.addEventListener('click', handleClickOutside)
      document.addEventListener('contextmenu', handleClickOutside)
    })
  } else {
    document.removeEventListener('click', handleClickOutside)
    document.removeEventListener('contextmenu', handleClickOutside)
  }
})

// 点击外部关闭菜单
const handleClickOutside = () => {
  emit('update:visible', false)
}

// 格式化时间
const formatTime = (date?: Date) => {
  if (!date) return ''
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 菜单项处理函数
const handleRename = () => {
  if (!props.tab) return
  newTabTitle.value = props.tab.title
  renameDialogVisible.value = true
  emit('update:visible', false)
  
  nextTick(() => {
    renameInputRef.value?.focus()
    renameInputRef.value?.select()
  })
}

const handlePin = () => {
  if (!props.tab) return
  emit('pin', props.tab)
  emit('update:visible', false)
}

const handleClose = () => {
  if (!props.tab || props.tab.isPinned) return
  emit('close', props.tab)
  emit('update:visible', false)
}

const handleCloseOthers = () => {
  if (!props.tab) return
  emit('close-others', props.tab)
  emit('update:visible', false)
}

const handleCloseToRight = () => {
  if (!props.tab) return
  emit('close-to-right', props.tab)
  emit('update:visible', false)
}

const handleDuplicate = () => {
  if (!props.tab) return
  emit('duplicate', props.tab)
  emit('update:visible', false)
  ElMessage.info('复制标签页功能开发中')
}

const handleOpenInNewWindow = () => {
  if (!props.tab) return
  emit('open-in-new-window', props.tab)
  emit('update:visible', false)
  ElMessage.info('新窗口打开功能开发中')
}

// 重命名对话框处理
const handleRenameConfirm = () => {
  if (!props.tab || !newTabTitle.value.trim()) return
  
  emit('rename', props.tab, newTabTitle.value.trim())
  renameDialogVisible.value = false
  newTabTitle.value = ''
}

const handleRenameCancel = () => {
  renameDialogVisible.value = false
  newTabTitle.value = ''
}
</script>

<style scoped>
.tab-context-menu {
  position: fixed;
  background-color: #ffffff;
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  padding: 4px 0;
  min-width: 180px;
  z-index: 9999;
}

.context-menu-content {
  display: flex;
  flex-direction: column;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  cursor: pointer;
  transition: background-color 0.2s;
  font-size: 13px;
  color: #606266;
}

.menu-item:hover {
  background-color: #f5f7fa;
  color: #409eff;
}

.menu-item.disabled {
  color: #c0c4cc;
  cursor: not-allowed;
}

.menu-item.disabled:hover {
  background-color: transparent;
  color: #c0c4cc;
}

.menu-item .el-icon {
  margin-right: 8px;
  font-size: 14px;
}

.menu-divider {
  height: 1px;
  background-color: #e4e7ed;
  margin: 4px 0;
}

.info-item {
  cursor: default;
  padding: 8px 12px;
}

.info-item:hover {
  background-color: transparent;
  color: #606266;
}

.session-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
}

.info-label {
  color: #909399;
  font-weight: 500;
}

.info-value {
  color: #606266;
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 对话框样式调整 */
.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

/* 动画效果 */
.tab-context-menu {
  animation: menu-appear 0.15s ease-out;
}

@keyframes menu-appear {
  from {
    opacity: 0;
    transform: scale(0.95) translateY(-5px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}
</style>
