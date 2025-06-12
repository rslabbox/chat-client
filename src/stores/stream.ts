import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

// 流式消息状态接口
export interface StreamState {
  streamId: string
  pluginId: string
  instanceId: string
  status: 'active' | 'paused' | 'completed' | 'cancelled' | 'error'
  startTime: Date
  messageId?: string // 关联的消息ID
}

// 流式消息管理器
export const useStreamStore = defineStore('stream', () => {
  // 状态
  const activeStreams = ref<Record<string, StreamState>>({})

  // 计算属性：获取指定实例的活跃流
  const getActiveStreamByInstance = (instanceId: string): StreamState | null => {
    const streams = Object.values(activeStreams.value)
    return streams.find(stream => 
      stream.instanceId === instanceId && 
      stream.status === 'active'
    ) || null
  }

  // 计算属性：检查指定实例是否有活跃流
  const hasActiveStream = (instanceId: string): boolean => {
    return getActiveStreamByInstance(instanceId) !== null
  }

  // 计算属性：获取所有活跃流
  const getAllActiveStreams = computed(() => {
    return Object.values(activeStreams.value).filter(stream => stream.status === 'active')
  })

  // 开始新的流式消息
  const startStream = (
    streamId: string,
    pluginId: string,
    instanceId: string,
    messageId?: string
  ): void => {
    activeStreams.value[streamId] = {
      streamId,
      pluginId,
      instanceId,
      status: 'active',
      startTime: new Date(),
      messageId
    }
    console.log(`Stream started: ${streamId} for instance: ${instanceId}`)
  }

  // 更新流状态
  const updateStreamStatus = (
    streamId: string,
    status: 'active' | 'paused' | 'completed' | 'cancelled' | 'error'
  ): boolean => {
    if (activeStreams.value[streamId]) {
      activeStreams.value[streamId].status = status
      console.log(`Stream ${streamId} status updated to: ${status}`)
      
      // 如果流结束，延迟清理以确保UI更新
      if (['completed', 'cancelled', 'error'].includes(status)) {
        setTimeout(() => {
          delete activeStreams.value[streamId]
          console.log(`Stream ${streamId} cleaned up`)
        }, 1000)
      }
      return true
    }
    return false
  }

  // 结束流式消息
  const endStream = (
    streamId: string,
    status: 'completed' | 'cancelled' | 'error' = 'completed'
  ): boolean => {
    return updateStreamStatus(streamId, status)
  }

  // 取消流式消息
  const cancelStream = (streamId: string): boolean => {
    return updateStreamStatus(streamId, 'cancelled')
  }

  // 暂停流式消息
  const pauseStream = (streamId: string): boolean => {
    return updateStreamStatus(streamId, 'paused')
  }

  // 恢复流式消息
  const resumeStream = (streamId: string): boolean => {
    return updateStreamStatus(streamId, 'active')
  }

  // 获取流状态
  const getStreamState = (streamId: string): StreamState | null => {
    return activeStreams.value[streamId] || null
  }

  // 清理所有流
  const clearAllStreams = (): void => {
    activeStreams.value = {}
    console.log('All streams cleared')
  }

  // 清理指定实例的所有流
  const clearInstanceStreams = (instanceId: string): void => {
    const streamsToRemove = Object.keys(activeStreams.value).filter(
      streamId => activeStreams.value[streamId].instanceId === instanceId
    )
    
    streamsToRemove.forEach(streamId => {
      delete activeStreams.value[streamId]
    })
    
    if (streamsToRemove.length > 0) {
      console.log(`Cleared ${streamsToRemove.length} streams for instance: ${instanceId}`)
    }
  }

  return {
    // 状态
    activeStreams,
    getAllActiveStreams,

    // 查询方法
    getActiveStreamByInstance,
    hasActiveStream,
    getStreamState,

    // 操作方法
    startStream,
    updateStreamStatus,
    endStream,
    cancelStream,
    pauseStream,
    resumeStream,
    clearAllStreams,
    clearInstanceStreams
  }
})
