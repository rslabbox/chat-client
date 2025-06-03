<template>
  <div class="config-panel">
    <div class="config-header">
      <h3>插件配置</h3>

      <div class="connection-status">
        <el-tag :type="isConnected ? 'success' : 'danger'" size="small">
          {{ isConnected ? '已连接' : '未连接' }}
        </el-tag>
      </div>
    </div>

    <div class="config-content">
      <!-- 插件UI组件 -->
      <PluginUI :plugin-id="pluginStore.currentPluginId || undefined" />
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
import { Link, Close } from '@element-plus/icons-vue'
import { computed } from 'vue'
import { usePluginStore } from '@/stores/plugins'
import PluginUI from './PluginUI.vue'

const pluginStore = usePluginStore()

// 计算属性
const isConnected = computed(() => pluginStore.isCurrentPluginConnected)

// 处理连接
const handleConnect = async () => {
  if (pluginStore.currentPluginId) {
    await pluginStore.connectPluginById(pluginStore.currentPluginId)
  }
}

// 处理断开连接
const handleDisconnect = async () => {
  if (pluginStore.currentPluginId) {
    await pluginStore.disconnectPluginById(pluginStore.currentPluginId)
  }
}
</script>

<style scoped>
.config-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px;
}

.config-header {
  margin-bottom: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.config-header h3 {
  margin: 0;
  color: #303133;
  font-size: 16px;
  font-weight: 600;
}

.config-content {
  flex: 1;
  display: block;
  align-items: center;
  justify-content: center;
  margin-top: auto;
}

.config-footer {
  margin-top: auto;
}

.message-stats {
  margin-bottom: 15px;
}

.stats-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.stat-label {
  font-size: 13px;
  color: #606266;
}
</style>