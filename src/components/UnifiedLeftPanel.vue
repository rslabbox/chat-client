<template>
  <div class="unified-left-panel">
    <div class="panel-header">
      <div class="tab-switcher">
        <div :class="['tab-item', { 'active': activeTab === 'config' }]" @click="activeTab = 'config'">
          插件配置
        </div>
        <div :class="['tab-item', { 'active': activeTab === 'history' }]" @click="activeTab = 'history'">
          历史对话
        </div>
        <div :class="['tab-item', { 'active': activeTab === 'shortcuts' }]" @click="activeTab = 'shortcuts'">
          快捷短语
        </div>
      </div>
    </div>

    <!-- 插件配置内容 -->
    <div v-show="activeTab === 'config'" class="panel-content">
      <ConfigPanel />
    </div>

    <!-- 历史对话内容 -->
    <div v-show="activeTab === 'history'" class="panel-content">
      <HistoryPanel />
    </div>

    <!-- 快捷短语内容 -->
    <div v-show="activeTab === 'shortcuts'" class="panel-content">
      <ShortcutsPanel />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ConfigPanel, HistoryPanel, ShortcutsPanel } from './panels'

// 标签页状态 - 默认打开插件配置
const activeTab = ref<'config' | 'history' | 'shortcuts'>('config')
</script>

<style scoped>
.unified-left-panel {
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

.panel-content {
  flex: 1 1 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 0;
}
</style>
