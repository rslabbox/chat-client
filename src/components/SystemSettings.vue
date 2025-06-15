<template>
  <el-dialog v-model="visible" title="系统设置" width="600px" :before-close="handleClose" destroy-on-close>
    <el-tabs v-model="activeTab" type="border-card">
      <!-- 通用设置 -->
      <el-tab-pane label="通用" name="general">
        <div class="settings-section">
          <div class="setting-item">
            <div class="setting-label">
              <span>主题</span>
              <el-text type="info" size="small">选择应用程序的外观主题</el-text>
            </div>
            <div class="setting-control">
              <el-select v-model="settings.theme" placeholder="选择主题" style="width: 200px;">
                <el-option label="浅色" value="light" />
                <el-option label="深色" value="dark" />
                <el-option label="跟随系统" value="auto" />
              </el-select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>语言</span>
              <el-text type="info" size="small">选择界面显示语言</el-text>
            </div>
            <div class="setting-control">
              <el-select v-model="settings.language" placeholder="选择语言" style="width: 200px;">
                <el-option label="简体中文" value="zh-CN" />
                <el-option label="English" value="en-US" />
              </el-select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>启动时自动连接插件</span>
              <el-text type="info" size="small">应用启动时自动连接上次使用的插件</el-text>
            </div>
            <div class="setting-control">
              <el-switch v-model="settings.autoConnect" />
            </div>
          </div>
        </div>
      </el-tab-pane>

      <!-- 插件设置 -->
      <el-tab-pane label="插件" name="plugins">
        <div class="settings-section">
          <div class="setting-item">
            <div class="setting-label">
              <span>插件目录</span>
              <el-text type="info" size="small">插件文件存放的目录路径</el-text>
            </div>
            <div class="setting-control">
              <el-input v-model="settings.pluginDirectory" placeholder="插件目录路径" style="width: 300px;" readonly>
                <template #append>
                  <el-button @click="selectPluginDirectory" :icon="FolderOpened">
                    浏览
                  </el-button>
                </template>
              </el-input>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>插件热重载</span>
              <el-text type="info" size="small">开发模式下自动重载插件文件变化</el-text>
            </div>
            <div class="setting-control">
              <el-switch v-model="settings.pluginHotReload" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>插件日志级别</span>
              <el-text type="info" size="small">设置插件系统的日志详细程度</el-text>
            </div>
            <div class="setting-control">
              <el-select v-model="settings.pluginLogLevel" style="width: 200px;">
                <el-option label="错误" value="error" />
                <el-option label="警告" value="warn" />
                <el-option label="信息" value="info" />
                <el-option label="调试" value="debug" />
              </el-select>
            </div>
          </div>
        </div>
      </el-tab-pane>

      <!-- 消息设置 -->
      <el-tab-pane label="消息" name="messages">
        <div class="settings-section">
          <div class="setting-item">
            <div class="setting-label">
              <span>消息历史保存天数</span>
              <el-text type="info" size="small">超过指定天数的消息将被自动清理</el-text>
            </div>
            <div class="setting-control">
              <el-input-number v-model="settings.messageRetentionDays" :min="1" :max="365" style="width: 200px;" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>最大消息显示数量</span>
              <el-text type="info" size="small">消息显示区域最多显示的消息条数</el-text>
            </div>
            <div class="setting-control">
              <el-input-number v-model="settings.maxDisplayMessages" :min="50" :max="1000" :step="50"
                style="width: 200px;" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>自动滚动到最新消息</span>
              <el-text type="info" size="small">收到新消息时自动滚动到底部</el-text>
            </div>
            <div class="setting-control">
              <el-switch v-model="settings.autoScrollToLatest" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>启用 Markdown 渲染</span>
              <el-text type="info" size="small">将消息内容按 Markdown 格式渲染显示</el-text>
            </div>
            <div class="setting-control">
              <el-switch v-model="settings.enableMarkdown" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>清空消息栏</span>
              <el-text type="info" size="small">发送消息后清空输入框</el-text>
            </div>
            <div class="setting-control">
              <el-switch v-model="settings.clearMessageInputOnSend" />
            </div>
          </div>
        </div>
      </el-tab-pane>

      <!-- 高级设置 -->
      <el-tab-pane label="高级" name="advanced">
        <div class="settings-section">
          <div class="setting-item">
            <div class="setting-label">
              <span>开发者模式</span>
              <el-text type="info" size="small">启用开发者工具和调试功能</el-text>
            </div>
            <div class="setting-control">
              <el-switch v-model="settings.developerMode" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>硬件加速</span>
              <el-text type="info" size="small">使用GPU加速渲染（需要重启应用）</el-text>
            </div>
            <div class="setting-control">
              <el-switch v-model="settings.hardwareAcceleration" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-label">
              <span>重置所有设置</span>
              <el-text type="info" size="small">将所有设置恢复为默认值</el-text>
            </div>
            <div class="setting-control">
              <el-button type="danger" @click="resetSettings" plain>
                重置设置
              </el-button>
            </div>
          </div>
        </div>
      </el-tab-pane>
    </el-tabs>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button type="primary" @click="saveSettings">保存</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { FolderOpened } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useSettingsStore } from '@/stores/settings'

// Props
interface Props {
  modelValue: boolean
}

const props = defineProps<Props>()

// Emits
const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

// 响应式数据
const visible = ref(props.modelValue)
const activeTab = ref('general')

// 使用设置存储
const settingsStore = useSettingsStore()
const { settings } = settingsStore

// 监听 modelValue 变化
watch(() => props.modelValue, (newValue) => {
  visible.value = newValue
  if (newValue) {
    loadSettings()
  }
})

// 监听 visible 变化
watch(visible, (newValue) => {
  emit('update:modelValue', newValue)
})

// 加载设置
const loadSettings = async () => {
  await settingsStore.loadSettings()
}

// 保存设置
const saveSettings = async () => {
  try {
    const success = await settingsStore.saveSettings()
    if (success) {
      ElMessage.success('设置已保存')
      visible.value = false

      // 应用主题变化
      settingsStore.applyTheme(settings.theme)
    } else {
      ElMessage.error('保存设置失败')
    }
  } catch (error) {
    console.error('保存设置失败:', error)
    ElMessage.error('保存设置失败')
  }
}

// 选择插件目录
const selectPluginDirectory = async () => {
  try {
    // TODO: 使用 Tauri 的文件选择器
    console.log('选择插件目录')
    ElMessage.info('文件选择器功能待实现')
  } catch (error) {
    console.error('选择目录失败:', error)
    ElMessage.error('选择目录失败')
  }
}

// 重置设置
const resetSettings = async () => {
  try {
    await ElMessageBox.confirm(
      '确定要重置所有设置吗？此操作不可撤销。',
      '重置设置',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )

    const success = await settingsStore.resetSettings()
    if (success) {
      ElMessage.success('设置已重置')
      // 应用默认主题
      settingsStore.applyTheme(settings.theme)
    } else {
      ElMessage.error('重置设置失败')
    }
  } catch {
    // 用户取消操作
  }
}

// 关闭对话框
const handleClose = () => {
  visible.value = false
}
</script>

<style scoped>
.settings-section {
  padding: 20px 0;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid #f0f0f0;
}

.setting-item:last-child {
  margin-bottom: 0;
  border-bottom: none;
}

.setting-label {
  flex: 1;
  margin-right: 20px;
}

.setting-label span {
  display: block;
  font-weight: 500;
  margin-bottom: 4px;
  color: #303133;
}

.setting-control {
  flex-shrink: 0;
}

.dialog-footer {
  text-align: right;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .setting-item {
    flex-direction: column;
    align-items: stretch;
  }

  .setting-label {
    margin-right: 0;
    margin-bottom: 12px;
  }

  .setting-control {
    width: 100%;
  }

  .setting-control .el-select,
  .setting-control .el-input {
    width: 100% !important;
  }
}
</style>
