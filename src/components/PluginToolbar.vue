<template>
  <div class="plugin-toolbar">
    <div class="toolbar-content">
      <!-- 左侧：面板切换、插件选择器和刷新按钮 -->
      <div class="left-section">
        <div class="panel-toggle">
          <el-button
            type="default"
            @click="handleTogglePanel"
            :icon="settingsStore.leftPanelVisible ? ArrowLeft : ArrowRight"
            size="small"
            circle
            :title="settingsStore.leftPanelVisible ? '收起配置面板' : '展开配置面板'"
          />
        </div>

        <div class="plugin-selector">
          <el-select
            v-model="selectedPluginId"
            placeholder="选择插件"
            style="width: 200px;"
            @change="handlePluginChange"
            :loading="pluginStore.isLoading"
            size="small"
          >
            <el-option
              v-for="plugin in pluginStore.plugins"
              :key="plugin.id"
              :label="plugin.name"
              :value="plugin.id"
              :disabled="plugin.disabled"
            >
            </el-option>
          </el-select>
        </div>

        <div class="toolbar-actions">
          <el-button
            type="primary"
            @click="handleRefresh"
            :icon="Refresh"
            size="small"
          >
            刷新插件
          </el-button>
        </div>
      </div>

      <!-- 右侧：历史记录面板切换和系统设置按钮 -->
      <div class="right-section">
        <el-button
          type="default"
          @click="handleToggleRightPanel"
          :icon="settingsStore.rightPanelVisible ? ArrowRight : ArrowLeft"
          size="small"
          circle
          :title="settingsStore.rightPanelVisible ? '收起历史记录' : '展开历史记录'"
        />
        <el-button
          type="default"
          @click="handleSettings"
          :icon="Setting"
          size="small"
          circle
          title="系统设置"
        />
      </div>
    </div>
  </div>

  <!-- 系统设置对话框 -->
  <SystemSettings v-model="showSettings" />
</template>

<script setup lang="ts">
import { Refresh, Setting, ArrowLeft, ArrowRight } from '@element-plus/icons-vue'
import { ref, onMounted, watch } from 'vue'
import { usePluginStore } from '@/stores/plugins'
import { useSettingsStore } from '@/stores/settings'
import SystemSettings from './SystemSettings.vue'


const pluginStore = usePluginStore()
const settingsStore = useSettingsStore()
const selectedPluginId = ref<string>('')
const showSettings = ref(false)

// 监听当前插件变化，同步选择框
watch(() => pluginStore.currentPlugin?.id, (newId) => {
  if (newId) {
    selectedPluginId.value = newId
  }
}, { immediate: true })

// 处理插件切换
const handlePluginChange = async (pluginId: string) => {
  if (pluginId && pluginId !== pluginStore.currentPlugin?.id) {
    const instanceId = crypto.randomUUID()
    await pluginStore.switchToPlugin(pluginId, instanceId)
  }
}

// 处理刷新插件
const handleRefresh = async () => {
  await pluginStore.loadPlugins()
}

// 处理系统设置
const handleSettings = () => {
  showSettings.value = true
}

// 处理左侧面板切换
const handleTogglePanel = () => {
  settingsStore.toggleLeftPanel()
}

// 处理右侧面板切换
const handleToggleRightPanel = () => {
  settingsStore.toggleRightPanel()
}

// 组件挂载时加载插件列表
onMounted(() => {
  pluginStore.loadPlugins()
})
</script>

<style scoped>
.plugin-toolbar {
  background-color: #ffffff;
  border-bottom: 1px solid #e4e7ed;
  padding: 12px 20px;
  flex-shrink: 0;
}

.toolbar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 1200px;
  margin: 0 auto;
}

.left-section {
  display: flex;
  align-items: center;
  gap: 12px;
}

.right-section {
  display: flex;
  align-items: center;
  gap: 8px;
}

.plugin-selector {
  display: flex;
  align-items: center;
}

.toolbar-actions {
  display: flex;
  align-items: center;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .toolbar-content {
    flex-direction: column;
    gap: 12px;
    align-items: stretch;
  }

  .left-section {
    flex-direction: column;
    gap: 8px;
    align-items: stretch;
  }

  .right-section {
    justify-content: center;
  }

  .plugin-selector .el-select {
    width: 100% !important;
  }

  .toolbar-actions {
    justify-content: center;
  }
}
</style>
