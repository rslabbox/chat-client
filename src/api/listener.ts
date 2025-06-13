import { useHistoryStore } from "@/stores/history"
import { usePageManagerStore } from "@/stores/pageManager"
import { useTabManagerStore } from "@/stores/tabManager"
import { useStreamStore } from "@/stores/stream"
import { usePluginStore } from "@/stores/plugins"
import { listen, UnlistenFn } from "@tauri-apps/api/event"
import { ref } from "vue"
// 事件监听器
const eventListeners = ref<UnlistenFn[]>([])

// 设置事件监听器
const setupEventListeners = async () => {
    const historyManager = useHistoryStore()
    const pageManagerStore = usePageManagerStore()
    const tabManagerStore = useTabManagerStore()
    const streamStore = useStreamStore()
    const pluginStore = usePluginStore()
    try {
        // 监听新的插件消息事件
        const unlistenPluginMessage = await listen('plugin-message', (event) => {
            console.log('Plugin message event:', event.payload)
            try {
                const data = JSON.parse(event.payload as string)
                const pluginId = data.plugin_id;
                const instanceId = data.instance_id;

                // 根据instanceId从tabManager获取正确的sessionId
                let currentSessionId = tabManagerStore.getSessionIdByInstanceId(instanceId);

                // 如果找不到对应的sessionId，回退到pageManager的currentSessionId
                if (!currentSessionId) {
                    currentSessionId = pageManagerStore.currentSessionId;
                    console.warn(`未找到instanceId ${instanceId} 对应的sessionId，使用当前活跃页面的sessionId: ${currentSessionId}`);
                }

                if (!currentSessionId) {
                    currentSessionId = historyManager.createNewSession(pluginId)
                    if (pageManagerStore.currentPage) {
                        pageManagerStore.currentPage.sessionId = currentSessionId
                    }
                }
                historyManager.addMessageToSession(currentSessionId, data.content, data.message_id, 'plugin')
            } catch (e) {
                console.error('Failed to parse plugin-message event:', e)
            }
        })
        eventListeners.value.push(unlistenPluginMessage)

        // 监听流式消息事件
        const unlistenPluginStream = await listen('plugin-stream', (event) => {
            try {
                const data = JSON.parse(event.payload as string)
                const pluginId = data.plugin_id;
                const instanceId = data.instance_id;

                // 根据instanceId从tabManager获取正确的sessionId
                let currentSessionId = tabManagerStore.getSessionIdByInstanceId(instanceId);

                // 如果找不到对应的sessionId，回退到pageManager的currentSessionId
                if (!currentSessionId) {
                    currentSessionId = pageManagerStore.currentSessionId;
                    console.warn(`未找到instanceId ${instanceId} 对应的sessionId，使用当前活跃页面的sessionId: ${currentSessionId}`);
                }

                if (!currentSessionId) {
                    currentSessionId = historyManager.createNewSession(pluginId)
                    if (pageManagerStore.currentPage) {
                        pageManagerStore.currentPage.sessionId = currentSessionId
                    }
                }
                const streamID = data.data.stream_id;
                console.log('流式消息事件:', data)
                console.log('当前页面实例ID:', pageManagerStore.currentInstanceId)
                console.log('消息中的实例ID:', data.instance_id)
                switch (data.type) {
                    case 'stream_start':
                        historyManager.addMessageToSession(currentSessionId, '', streamID, 'plugin', 'streaming', 'active')
                        // 在流式消息状态管理器中记录新的流
                        console.log('记录流式消息:', { streamID, pluginId, instanceId: data.instance_id })
                        streamStore.startStream(streamID, pluginId, data.instance_id, streamID)
                        break
                    case 'stream_data':
                        const isFinal = data.data.is_final;
                        historyManager.updateMessage(streamID, data.data.chunk, isFinal ? 'completed' : 'active')
                        // 如果是最后一块数据，更新流状态
                        if (isFinal) {
                            streamStore.updateStreamStatus(streamID, 'completed')
                        }
                        break
                    case 'stream_end':
                        const isSuccess = data.data.success;
                        historyManager.updateMessage(streamID, '', isSuccess ? 'completed' : 'error')
                        // 结束流式消息
                        streamStore.endStream(streamID, isSuccess ? 'completed' : 'error')
                        break;
                    case 'stream_pause':
                        historyManager.updateMessage(streamID, '', 'paused')
                        streamStore.pauseStream(streamID)
                        break;
                    case 'stream_resume':
                        historyManager.updateMessage(streamID, '', 'active')
                        streamStore.resumeStream(streamID)
                        break
                    case 'stream_cancel':
                        historyManager.updateMessage(streamID, '', 'cancelled')
                        streamStore.cancelStream(streamID)
                        break
                    default:
                        console.warn('Unknown stream message type:', data.type)
                }
            } catch (e) {
                console.error('Failed to parse plugin-stream event:', e)
            }
        })
        eventListeners.value.push(unlistenPluginStream)

        // 监听插件断开连接请求事件
        const unlistenPluginDisconnectRequest = await listen('plugin-disconnect-request', async (event) => {
            console.log('Plugin disconnect request event:', event.payload)
            try {
                const data = JSON.parse(event.payload as string)
                const { instance_id } = data

                await pluginStore.disconnectPluginInstance(instance_id)

                console.log(`Plugin instance ${instance_id} disconnected by plugin request`)
            } catch (e) {
                console.error('Failed to handle plugin-disconnect-request event:', e)
            }
        })
        eventListeners.value.push(unlistenPluginDisconnectRequest)
    } catch (error) {
        console.error('Failed to setup event listeners:', error)
    }
}

// 清理事件监听器
const cleanupEventListeners = () => {
    eventListeners.value.forEach(unlisten => {
        try {
            unlisten()
        } catch (error) {
            console.error('Failed to unlisten event:', error)
        }
    })
    eventListeners.value = []
}

export { setupEventListeners, cleanupEventListeners }
