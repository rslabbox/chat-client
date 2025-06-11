<template>
  <div class="config-content">
    <div class="config-header">
      <div class="connection-status">
        <el-tag :type="isConnected ? 'success' : 'danger'" size="small">
          {{ isConnected ? '已连接' : '未连接' }}
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
