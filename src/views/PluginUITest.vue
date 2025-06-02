<template>
  <div class="plugin-ui-test">
    <el-card class="test-card">
      <template #header>
        <div class="card-header">
          <span>插件UI测试</span>
          <el-button type="primary" @click="loadExamplePlugin">
            加载示例插件
          </el-button>
        </div>
      </template>
      
      <div class="test-content">
        <el-alert
          v-if="message"
          :title="message"
          :type="messageType"
          :closable="false"
          style="margin-bottom: 20px;"
        />
        
        <div class="plugin-selection">
          <el-select
            v-model="selectedPluginId"
            placeholder="选择插件"
            style="width: 100%; margin-bottom: 20px;"
            @change="handlePluginChange"
          >
            <el-option
              v-for="plugin in plugins"
              :key="plugin.id"
              :label="plugin.name"
              :value="plugin.id"
            />
          </el-select>
        </div>
        
        <div class="ui-display">
          <h3>插件UI:</h3>
          <PluginUI :plugin-id="selectedPluginId" />
        </div>
        
        <div class="raw-data" v-if="rawUIData">
          <h3>原始UI数据:</h3>
          <el-input
            v-model="rawUIData"
            type="textarea"
            :rows="10"
            readonly
            style="font-family: monospace;"
          />
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { scanPlugins, mountPlugin, getPluginUi } from '@/api'
import type { PluginMetadata } from '@/api/types'
import PluginUI from '@/components/PluginUI.vue'

// 响应式数据
const plugins = ref<PluginMetadata[]>([])
const selectedPluginId = ref<string>('')
const rawUIData = ref<string>('')
const message = ref<string>('')
const messageType = ref<'success' | 'warning' | 'error' | 'info'>('info')

// 加载插件列表
const loadPlugins = async () => {
  try {
    plugins.value = await scanPlugins()
    message.value = `成功加载 ${plugins.value.length} 个插件`
    messageType.value = 'success'
  } catch (error) {
    message.value = `加载插件失败: ${error}`
    messageType.value = 'error'
  }
}

// 加载示例插件
const loadExamplePlugin = async () => {
  try {
    message.value = '正在挂载示例插件...'
    messageType.value = 'info'
    
    const result = await mountPlugin('example_plugin')
    message.value = result
    messageType.value = 'success'
    
    // 自动选择示例插件
    selectedPluginId.value = 'example_plugin'
    await handlePluginChange('example_plugin')
  } catch (error) {
    message.value = `挂载插件失败: ${error}`
    messageType.value = 'error'
  }
}

// 处理插件选择变化
const handlePluginChange = async (pluginId: string) => {
  if (!pluginId) {
    rawUIData.value = ''
    return
  }
  
  try {
    message.value = '正在加载插件UI...'
    messageType.value = 'info'
    
    const ui = await getPluginUi(pluginId)
    rawUIData.value = JSON.stringify(ui, null, 2)
    
    message.value = '插件UI加载成功'
    messageType.value = 'success'
  } catch (error) {
    message.value = `加载插件UI失败: ${error}`
    messageType.value = 'error'
    rawUIData.value = ''
  }
}

// 组件挂载时加载插件列表
onMounted(() => {
  loadPlugins()
})
</script>

<style scoped>
.plugin-ui-test {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.test-card {
  width: 100%;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.test-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.ui-display {
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 20px;
  background-color: #fafafa;
}

.raw-data {
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 20px;
  background-color: #f9f9f9;
}

.raw-data h3 {
  margin-top: 0;
  margin-bottom: 15px;
  color: #606266;
}

.ui-display h3 {
  margin-top: 0;
  margin-bottom: 15px;
  color: #606266;
}
</style>
