<template>
  <div class="config-content">
    <div class="config-header">
      <div class="connection-status">
        <div v-if="isConnected && currentPlugin" class="plugin-info">
          <span class="plugin-name">{{ currentPlugin.name }}</span>
          <div class="plugin-meta">
            <span v-if="currentPlugin.version" class="plugin-version">v{{ currentPlugin.version }}</span>
            <el-tag type="success" size="small">已连接</el-tag>
          </div>
        </div>
        <div v-else-if="currentPlugin" class="plugin-info">
          <span class="plugin-name">{{ currentPlugin.name }}</span>
          <div class="plugin-meta">
            <span v-if="currentPlugin.version" class="plugin-version">v{{ currentPlugin.version }}</span>
            <el-tag type="danger" size="small">未连接</el-tag>
          </div>
        </div>
        <el-tag v-else type="warning" size="small">
          无插件
        </el-tag>
      </div>
    </div>

    <div class="config-main">
      <!-- 插件UI组件 -->
      <PluginUI :instance-id="pageManagerStore.currentInstanceId || undefined" />
    </div>

    <div class="config-footer">
      <el-button v-if="!isConnected" type="primary" @click="handleConnect" :icon="Link"
        style="width: 100%; margin-bottom: 10px;">
        连接
      </el-button>

      <el-button v-else type="danger" @click="handleDisconnect" :icon="Close" style="width: 100%; margin-bottom: 10px;">
        断开连接
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Link, Close } from '@element-plus/icons-vue'
import { usePageManagerStore } from '@/stores/pageManager'
import { usePluginStore } from '@/stores/plugins'
import PluginUI from './PluginUI.vue'

const pageManagerStore = usePageManagerStore()
const pluginStore = usePluginStore()

// 插件连接状态
const isConnected = computed(() => pluginStore.getInstanceState(pageManagerStore.currentInstanceId ?? "")?.isConnected || false)

// 当前插件信息
const currentPlugin = computed(() => {
  const currentPluginId = pageManagerStore.currentPluginId
  if (!currentPluginId) return null
  return pluginStore.plugins.find(plugin => plugin.id === currentPluginId) || null
})

// 插件连接相关方法
const handleConnect = async () => {
  if (pageManagerStore.currentInstanceId) {
    await pluginStore.connectPluginInstance(pageManagerStore.currentInstanceId)
  }
}

const handleDisconnect = async () => {
  if (pageManagerStore.currentInstanceId) {
    await pluginStore.disconnectPluginInstance(pageManagerStore.currentInstanceId)
  }
}
</script>

<style scoped>
.config-content {
  flex: 1;
  display: grid;
  grid-template-rows: auto 1fr auto;
  overflow: hidden;
  height: 100%;
  min-height: 0;
}

.config-header {
  padding: 16px;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  justify-content: space-between;
  align-items: center;
  /* Grid布局中的第一行，固定高度 */
  grid-row: 1;
}

.connection-status {
  display: flex;
  align-items: center;
  width: 100%;
}

.plugin-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.plugin-name {
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  flex: 1;
  margin-right: 12px;
}

.plugin-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.plugin-version {
  font-size: 12px;
  color: #909399;
  background-color: #f5f7fa;
  padding: 2px 6px;
  border-radius: 4px;
  font-weight: 500;
}
.config-main {
  overflow-y: auto;
  overflow-x: hidden;
  padding: 16px;
  /* 确保滚动条正确显示，避免内容被截断 */
  box-sizing: border-box;
  /* Grid布局中的第二行，占用所有可用空间 */
  grid-row: 2;
  min-height: 0;
}

.config-footer {
  padding: 16px;
  border-top: 1px solid #e4e7ed;
  background-color: #ffffff;
  /* Grid布局中的第三行，固定高度 */
  grid-row: 3;
  /* 确保底部按钮区域不会被滚动内容遮挡 */
  position: relative;
  z-index: 1;
}
</style>
