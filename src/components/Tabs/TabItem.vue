<template>
  <div
    :class="[
      'tab-item',
      {
        'active': tab.isActive,
        'pinned': isPinned,
        'dragging': isDragging
      }
    ]"
    @click="handleClick"
    @mousedown="handleMouseDown"
    @contextmenu="handleContextMenu"
    draggable="true"
    @dragstart="handleDragStart"
    @dragover="handleDragOver"
    @drop="handleDrop"
    @dragend="handleDragEnd"
  >
    <!-- 标签页图标 -->
    <div class="tab-icon">
      <el-icon v-if="tab.icon" :color="tab.color">
        <component :is="tab.icon" />
      </el-icon>
      <el-icon v-else-if="pluginIcon" :color="tab.color">
        <component :is="pluginIcon" />
      </el-icon>
      <el-icon v-else>
        <Document />
      </el-icon>
    </div>

    <!-- 标签页标题 -->
    <div class="tab-title" :title="tab.title">
      {{ displayTitle }}
    </div>

    <!-- 固定图标 -->
    <div v-if="isPinned" class="pin-icon">
      <el-icon size="12">
        <Lock />
      </el-icon>
    </div>

    <!-- 关闭按钮 -->
    <div
      v-if="!isPinned"
      class="close-btn"
      @click.stop="handleClose"
      @mousedown.stop
    >
      <el-icon size="14">
        <Close />
      </el-icon>
    </div>

    <!-- 加载指示器 -->
    <div v-if="isLoading" class="loading-indicator">
      <el-icon class="is-loading">
        <Loading />
      </el-icon>
    </div>

    <!-- 未保存指示器 -->
    <div v-if="hasUnsavedChanges" class="unsaved-indicator">
      <div class="unsaved-dot"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { Document, Close, Lock, Loading } from '@element-plus/icons-vue'
import { usePluginStore } from '@/stores/plugins'
import type { Tab } from '@/stores/tabManager'

interface Props {
  tab: Tab
  isPinned: boolean
}

interface Emits {
  (e: 'click'): void
  (e: 'close'): void
  (e: 'context-menu', event: MouseEvent): void
  (e: 'drag-start', tab: Tab): void
  (e: 'drag-over', event: DragEvent): void
  (e: 'drop', tab: Tab): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const pluginStore = usePluginStore()

// 响应式数据
const isDragging = ref(false)
const isLoading = ref(false)
const hasUnsavedChanges = ref(false)

// 计算属性
const displayTitle = computed(() => {
  const maxLength = props.isPinned ? 8 : 20
  return props.tab.title.length > maxLength 
    ? props.tab.title.substring(0, maxLength) + '...'
    : props.tab.title
})

const pluginIcon = computed(() => {
  const plugin = pluginStore.plugins.find(p => p.id === props.tab.pluginId)
  return plugin?.icon
})

// 事件处理
const handleClick = () => {
  emit('click')
}

const handleClose = () => {
  emit('close')
}

const handleContextMenu = (event: MouseEvent) => {
  emit('context-menu', event)
}

const handleMouseDown = (event: MouseEvent) => {
  // 中键点击关闭标签页
  if (event.button === 1 && !props.isPinned) {
    event.preventDefault()
    handleClose()
  }
}

// 拖拽相关
const handleDragStart = (event: DragEvent) => {
  isDragging.value = true
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move'
    event.dataTransfer.setData('text/plain', props.tab.id)
  }
  emit('drag-start', props.tab)
}

const handleDragOver = (event: DragEvent) => {
  event.preventDefault()
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move'
  }
  emit('drag-over', event)
}

const handleDrop = (event: DragEvent) => {
  event.preventDefault()
  emit('drop', props.tab)
}

const handleDragEnd = () => {
  isDragging.value = false
}

// 设置加载状态
const setLoading = (loading: boolean) => {
  isLoading.value = loading
}

// 设置未保存状态
const setUnsavedChanges = (hasChanges: boolean) => {
  hasUnsavedChanges.value = hasChanges
}

// 暴露方法给父组件
defineExpose({
  setLoading,
  setUnsavedChanges
})
</script>

<style scoped>
.tab-item {
  display: flex;
  align-items: center;
  padding: 6px 8px;
  margin: 0 1px;
  background-color: #ffffff;
  border: 1px solid #e4e7ed;
  border-bottom: none;
  border-radius: 6px 6px 0 0;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  min-width: 120px;
  max-width: 200px;
  height: 32px;
  user-select: none;
}

.tab-item:hover {
  background-color: #f5f7fa;
  border-color: #c0c4cc;
}

.tab-item.active {
  background-color: #409eff;
  border-color: #409eff;
  color: #ffffff;
}

.tab-item.active:hover {
  background-color: #337ecc;
}

.tab-item.pinned {
  min-width: 80px;
  max-width: 120px;
  background-color: #f0f9ff;
  border-color: #b3d8ff;
}

.tab-item.pinned.active {
  background-color: #409eff;
  border-color: #409eff;
}

.tab-item.dragging {
  opacity: 0.5;
  transform: rotate(5deg);
}

.tab-icon {
  display: flex;
  align-items: center;
  margin-right: 6px;
  flex-shrink: 0;
}

.tab-title {
  flex: 1;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.2;
}

.pin-icon {
  display: flex;
  align-items: center;
  margin-left: 4px;
  opacity: 0.7;
  flex-shrink: 0;
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  margin-left: 4px;
  border-radius: 3px;
  opacity: 0;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.tab-item:hover .close-btn {
  opacity: 0.7;
}

.close-btn:hover {
  opacity: 1;
  background-color: rgba(0, 0, 0, 0.1);
}

.tab-item.active .close-btn:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

.loading-indicator {
  position: absolute;
  top: 2px;
  right: 2px;
  width: 12px;
  height: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.unsaved-indicator {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 8px;
  height: 8px;
}

.unsaved-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: #f56c6c;
}

/* 固定标签页样式调整 */
.tab-item.pinned .tab-title {
  font-size: 11px;
}

.tab-item.pinned .tab-icon {
  margin-right: 4px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .tab-item {
    min-width: 100px;
    max-width: 150px;
    padding: 4px 6px;
    height: 28px;
  }
  
  .tab-item.pinned {
    min-width: 60px;
    max-width: 80px;
  }
  
  .tab-title {
    font-size: 11px;
  }
  
  .tab-item.pinned .tab-title {
    font-size: 10px;
  }
}

/* 拖拽时的视觉效果 */
.tab-item[draggable="true"]:active {
  cursor: grabbing;
}

/* 动画效果 */
@keyframes tab-appear {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.tab-item {
  animation: tab-appear 0.2s ease-out;
}
</style>
