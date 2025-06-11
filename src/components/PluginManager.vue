<template>
  <el-dialog v-model="visible" width="800px" :before-close="handleClose" destroy-on-close>
    <template #header>
      <div class="dialog-header">
        <span class="dialog-title">插件管理</span>
        <div class="repo-status">
          <el-icon v-if="refreshingRepo" class="is-loading">
            <Loading />
          </el-icon>
          <el-icon v-else-if="repoConnected" class="repo-icon connected">
            <Connection />
          </el-icon>
          <el-icon v-else class="repo-icon disconnected">
            <WarningFilled />
          </el-icon>
          <span class="repo-text">
            {{ refreshingRepo ? '正在更新仓库...' : (repoConnected ? '仓库已连接' : '仓库连接失败') }}
          </span>
        </div>
      </div>
    </template>
    <div class="plugin-manager">
      <!-- 加载状态 -->
      <div v-if="loading" class="loading-container">
        <el-loading-spinner />
        <span>正在加载插件列表...</span>
      </div>

      <!-- 插件列表 -->
      <div v-else class="plugin-list">
        <div v-if="availablePlugins.length === 0" class="empty-state">
          <el-empty description="暂无可用插件" />
        </div>

        <div v-else class="plugin-grid">
          <div v-for="plugin in availablePlugins" :key="plugin.id" class="plugin-card">
            <!-- 插件头像 -->
            <div class="plugin-avatar">
              <el-avatar :size="40" :src="plugin.avatar" :alt="plugin.name">
                <el-icon>
                  <Box />
                </el-icon>
              </el-avatar>
            </div>

            <!-- 插件信息 -->
            <div class="plugin-info">
              <div class="plugin-header">
                <h3 class="plugin-name">{{ plugin.name }}</h3>
                <div class="plugin-version-info">
                  <el-tag size="small" type="info" style="font-size: 11px; padding: 1px 4px;">v{{ plugin.version
                    }}</el-tag>
                  <el-tag v-if="getPluginStatus(plugin) === 'installed'" size="small" type="success"
                    style="font-size: 10px; padding: 1px 3px; margin-left: 4px;">已安装</el-tag>
                  <el-tag v-else-if="getPluginStatus(plugin) === 'upgrade-available'" size="small" type="warning"
                    style="font-size: 10px; padding: 1px 3px; margin-left: 4px;">可升级</el-tag>
                </div>
              </div>

              <div class="plugin-author">
                <el-text type="info" size="small" style="font-size: 11px;">作者：{{ plugin.author }}</el-text>
              </div>

              <div class="plugin-description">
                <el-text style="font-size: 12px; color: #606266;">{{ plugin.description }}</el-text>
              </div>

              <!-- 插件操作按钮 -->
              <div class="plugin-actions">
                <!-- 下载/升级按钮 -->
                <el-button v-if="getPluginStatus(plugin) === 'not-installed'" type="primary" size="small"
                  :loading="downloadingPlugins.has(plugin.id)" @click="handleDownload(plugin)"
                  style="font-size: 12px; padding: 4px 8px;">
                  下载
                </el-button>

                <el-button v-else-if="getPluginStatus(plugin) === 'upgrade-available'" type="warning" size="small"
                  :loading="downloadingPlugins.has(plugin.id)" @click="handleDownload(plugin)"
                  style="font-size: 12px; padding: 4px 8px;">
                  升级
                </el-button>

                <el-button v-else type="success" size="small" disabled style="font-size: 12px; padding: 4px 8px;">
                  已安装
                </el-button>

                <el-button type="default" size="small" @click="handleHomepage(plugin)" :disabled="!plugin.homepage"
                  style="font-size: 12px; padding: 4px 8px;">
                  主页
                </el-button>

                <el-button type="default" size="small" @click="handleRepository(plugin)" :disabled="!plugin.repository"
                  style="font-size: 12px; padding: 4px 8px;">
                  仓库
                </el-button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleRefresh" :loading="loading || refreshingRepo">
          {{ (loading || refreshingRepo) ? '正在刷新...' : '刷新列表' }}
        </el-button>
        <el-button @click="handleClose">关闭</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { Box, Loading, Connection, WarningFilled } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { scanAvailablePlugins, downloadPlugin } from '@/api'
import { downloadGithubRepo } from '@/api/download'
import type { AvailablePluginInfo } from '@/api/types'
import { usePluginStore } from '@/stores/plugins'

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
const loading = ref(false)
const refreshingRepo = ref(false)
const repoConnected = ref(true) // 仓库连接状态
const availablePlugins = ref<AvailablePluginInfo[]>([])
const downloadingPlugins = ref(new Set<string>())

// 使用插件存储
const pluginStore = usePluginStore()

// 监听 modelValue 变化
watch(() => props.modelValue, (newValue) => {
  visible.value = newValue
  if (newValue) {
    loadAvailablePlugins()
    // 后台静默更新仓库
    updateRepositoryInBackground()
  }
})

// 监听 visible 变化
watch(visible, (newValue) => {
  emit('update:modelValue', newValue)
})

// 组件挂载时加载插件列表
onMounted(() => {
  if (visible.value) {
    loadAvailablePlugins()
    // 后台静默更新仓库
    updateRepositoryInBackground()
  }
})

// 加载可用插件列表
const loadAvailablePlugins = async () => {
  try {
    loading.value = true
    availablePlugins.value = await scanAvailablePlugins()
  } catch (error) {
    console.error('加载插件列表失败:', error)
    ElMessage.error('加载插件列表失败')
  } finally {
    loading.value = false
  }
}

// 后台静默更新仓库
const updateRepositoryInBackground = async () => {
  try {
    refreshingRepo.value = true
    const repoResult = await downloadGithubRepo()

    if (repoResult.success) {
      repoConnected.value = true
      // 静默更新成功后，重新加载插件列表
      await loadAvailablePlugins()
    } else {
      repoConnected.value = false
      console.warn('后台更新插件仓库失败:', repoResult.message)
    }
  } catch (error) {
    repoConnected.value = false
    console.error('后台更新插件仓库失败:', error)
  } finally {
    refreshingRepo.value = false
  }
}

// 检查插件是否已安装
const isPluginInstalled = (pluginId: string): boolean => {
  return pluginStore.plugins.some(p => p.id === pluginId)
}

// 获取已安装插件的版本
const getInstalledPluginVersion = (pluginId: string): string | null => {
  const installedPlugin = pluginStore.plugins.find(p => p.id === pluginId)
  return installedPlugin ? installedPlugin.version : null
}

// 比较版本号
const compareVersions = (version1: string, version2: string): number => {
  const v1Parts = version1.split('.').map(Number)
  const v2Parts = version2.split('.').map(Number)

  const maxLength = Math.max(v1Parts.length, v2Parts.length)

  for (let i = 0; i < maxLength; i++) {
    const v1Part = v1Parts[i] || 0
    const v2Part = v2Parts[i] || 0

    if (v1Part > v2Part) return 1
    if (v1Part < v2Part) return -1
  }

  return 0
}

// 检查是否需要升级
const needsUpgrade = (plugin: AvailablePluginInfo): boolean => {
  const installedVersion = getInstalledPluginVersion(plugin.id)
  if (!installedVersion) return false

  return compareVersions(plugin.version, installedVersion) > 0
}

// 获取插件状态
const getPluginStatus = (plugin: AvailablePluginInfo): 'not-installed' | 'installed' | 'upgrade-available' => {
  if (!isPluginInstalled(plugin.id)) {
    return 'not-installed'
  }

  if (needsUpgrade(plugin)) {
    return 'upgrade-available'
  }

  return 'installed'
}

// 处理插件下载
const handleDownload = async (plugin: AvailablePluginInfo) => {
  try {
    const status = getPluginStatus(plugin)
    const isUpgrade = status === 'upgrade-available'
    const installedVersion = getInstalledPluginVersion(plugin.id)

    let confirmMessage = `确定要下载并安装插件 "${plugin.name}" 吗？`
    let confirmTitle = '确认下载'

    if (isUpgrade) {
      confirmMessage = `确定要将插件 "${plugin.name}" 从 v${installedVersion} 升级到 v${plugin.version} 吗？`
      confirmTitle = '确认升级'
    }

    await ElMessageBox.confirm(
      confirmMessage,
      confirmTitle,
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'info',
      }
    )

    downloadingPlugins.value.add(plugin.id)

    const result = await downloadPlugin(plugin.id)

    if (result.success) {
      const successMessage = isUpgrade
        ? `插件 "${plugin.name}" 升级成功`
        : `插件 "${plugin.name}" 下载成功`
      ElMessage.success(successMessage)
      // 重新扫描插件列表
      await pluginStore.refreshPlugins()
    } else {
      const errorMessage = isUpgrade
        ? `升级失败: ${result.message || '未知错误'}`
        : `下载失败: ${result.message || '未知错误'}`
      ElMessage.error(errorMessage)
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('下载插件失败:', error)
      ElMessage.error('下载插件失败')
    }
  } finally {
    downloadingPlugins.value.delete(plugin.id)
  }
}

// 处理主页链接
const handleHomepage = (plugin: AvailablePluginInfo) => {
  if (plugin.homepage) {
    window.open(plugin.homepage, '_blank')
  }
}

// 处理仓库链接
const handleRepository = (plugin: AvailablePluginInfo) => {
  if (plugin.repository) {
    window.open(plugin.repository, '_blank')
  }
}

// 刷新插件列表（用户主动刷新）
const handleRefresh = async () => {
  try {
    refreshingRepo.value = true

    // 先尝试下载GitHub仓库更新插件列表
    try {
      const repoResult = await downloadGithubRepo()
      if (repoResult.success) {
        repoConnected.value = true
        ElMessage.success('插件仓库更新成功')
      } else {
        // 仓库下载失败，但不阻止后续操作
        repoConnected.value = false
        console.warn('插件仓库下载失败:', repoResult.message)
        ElMessage.warning('插件仓库连接失败，将显示本地已有插件')
      }
    } catch (error) {
      // 网络连接失败或其他错误
      repoConnected.value = false
      console.error('插件仓库连接失败:', error)
      ElMessage.warning('插件仓库连接失败，将显示本地已有插件')
    }

    // 无论仓库下载是否成功，都要扫描本地插件
    await loadAvailablePlugins()
    await pluginStore.refreshPlugins()

  } catch (error) {
    console.error('刷新插件列表失败:', error)
    ElMessage.error('刷新插件列表失败')
  } finally {
    refreshingRepo.value = false
  }
}

// 关闭对话框
const handleClose = () => {
  visible.value = false
}
</script>

<style scoped>
.plugin-manager {
  min-height: 400px;
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  gap: 16px;
}

.plugin-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 12px;
  padding: 12px 0;
}

.plugin-card {
  display: flex;
  gap: 10px;
  padding: 12px;
  border: 1px solid #e4e7ed;
  border-radius: 6px;
  background: #fff;
  transition: all 0.3s ease;
}

.plugin-card:hover {
  border-color: #409eff;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.plugin-avatar {
  flex-shrink: 0;
}

.plugin-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.plugin-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 6px;
}

.plugin-version-info {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 2px;
}

.plugin-name {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.plugin-author {
  margin: 1px 0;
}

.plugin-description {
  flex: 1;
  margin: 2px 0;
  line-height: 1.3;
  font-size: 12px;
  color: #606266;
}

.plugin-actions {
  display: flex;
  gap: 4px;
  margin-top: 6px;
}

.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.dialog-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.repo-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #606266;
}

.repo-icon.connected {
  color: #67c23a;
}

.repo-icon.disconnected {
  color: #f56c6c;
}

.repo-text {
  font-size: 12px;
}
.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>
