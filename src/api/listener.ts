import { useHistoryStore } from "@/stores/history"
import { usePageManagerStore } from "@/stores/pageManager"
import { listen, UnlistenFn } from "@tauri-apps/api/event"
import { ref } from "vue"
// 事件监听器
const eventListeners = ref<UnlistenFn[]>([])

// 设置事件监听器
const setupEventListeners = async () => {
    const historyManager = useHistoryStore()
    const pageManagerStore = usePageManagerStore()
    try {
        // 监听新的插件消息事件
        const unlistenPluginMessage = await listen('plugin-message', (event) => {
            console.log('Plugin message event:', event.payload)
            try {
                const data = JSON.parse(event.payload as string)
                const pluginId = data.plugin_id;
                let currentSessionId = pageManagerStore.currentSessionId;
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
                // const instantID = data.instance_id;
                let currentSessionId = pageManagerStore.currentSessionId;
                if (!currentSessionId) {
                    currentSessionId = historyManager.createNewSession(pluginId)
                    if (pageManagerStore.currentPage) {
                        pageManagerStore.currentPage.sessionId = currentSessionId
                    }
                }
                const streamID = data.data.stream_id;
                console.log(data)
                switch (data.type) {
                    case 'stream_start':
                        historyManager.addMessageToSession(currentSessionId, '', streamID, 'plugin', 'streaming', 'active')
                        break
                    case 'stream_data':
                        const isFinal = data.data.is_final;
                        historyManager.updateMessage(streamID, data.data.chunk, isFinal ? 'completed' : 'active')
                        break
                    case 'stream_end':
                        const isSuccess = data.data.success;
                        historyManager.updateMessage(streamID, '', isSuccess ? 'completed' : 'error')
                        break;
                    case 'stream_pause': 
                        historyManager.updateMessage(streamID, '', 'paused') 
                        break;
                    case 'stream_resume':
                        historyManager.updateMessage(streamID, '', 'active')
                        break
                    case 'stream_cancel':
                        historyManager.updateMessage(streamID, '[已取消]', 'cancelled')
                        break
                    default:
                        console.warn('Unknown stream message type:', data.type)
                }
            } catch (e) {
                console.error('Failed to parse plugin-stream event:', e)
            }
        })
        eventListeners.value.push(unlistenPluginStream)
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
