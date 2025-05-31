<template>
  <div class="serial-debugger">
    <!-- 标题栏 -->
    <div class="header">
      <h1>串口调试助手</h1>
      <div class="status-indicator">
        <span class="status-dot" :class="{ connected: isConnected }"></span>
        <span class="status-text">{{ isConnected ? '已连接' : '未连接' }}</span>
      </div>
    </div>

    <div class="main-container">
      <!-- 左侧控制面板 -->
      <div class="control-panel">
        <div class="panel-section">
          <h3>串口配置</h3>
          <div class="form-group">
            <label>端口号:</label>
            <select v-model="serialConfig.port" class="form-control">
              <option value="">请选择端口</option>
              <option value="COM1">COM1</option>
              <option value="COM2">COM2</option>
              <option value="COM3">COM3</option>
              <option value="COM4">COM4</option>
              <option value="COM5">COM5</option>
            </select>
          </div>
          
          <div class="form-group">
            <label>波特率:</label>
            <select v-model="serialConfig.baudRate" class="form-control">
              <option value="9600">9600</option>
              <option value="115200">115200</option>
              <option value="38400">38400</option>
              <option value="19200">19200</option>
              <option value="57600">57600</option>
            </select>
          </div>
          
          <div class="form-group">
            <label>数据位:</label>
            <select v-model="serialConfig.dataBits" class="form-control">
              <option value="8">8</option>
              <option value="7">7</option>
              <option value="6">6</option>
              <option value="5">5</option>
            </select>
          </div>
          
          <div class="form-group">
            <label>停止位:</label>
            <select v-model="serialConfig.stopBits" class="form-control">
              <option value="1">1</option>
              <option value="1.5">1.5</option>
              <option value="2">2</option>
            </select>
          </div>
          
          <div class="form-group">
            <label>校验位:</label>
            <select v-model="serialConfig.parity" class="form-control">
              <option value="none">无校验</option>
              <option value="odd">奇校验</option>
              <option value="even">偶校验</option>
            </select>
          </div>
          
          <div class="button-group">
            <button 
              class="btn btn-primary" 
              :class="{ 'btn-danger': isConnected }"
              @click="toggleConnection"
            >
              {{ isConnected ? '断开连接' : '连接' }}
            </button>
            <button class="btn btn-secondary" @click="refreshPorts">
              刷新端口
            </button>
          </div>
        </div>

        <!-- 发送设置 -->
        <div class="panel-section">
          <h3>发送设置</h3>
          <div class="checkbox-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="sendSettings.hexMode">
              十六进制发送
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="sendSettings.autoNewline">
              自动换行
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="sendSettings.autoRepeat">
              自动重复发送
            </label>
          </div>
          
          <div class="form-group" v-if="sendSettings.autoRepeat">
            <label>重复间隔 (ms):</label>
            <input 
              type="number" 
              v-model="sendSettings.repeatInterval" 
              class="form-control"
              min="10"
              max="10000"
            >
          </div>
        </div>

        <!-- 接收设置 -->
        <div class="panel-section">
          <h3>接收设置</h3>
          <div class="checkbox-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="receiveSettings.hexDisplay">
              十六进制显示
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="receiveSettings.timestamp">
              显示时间戳
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="receiveSettings.autoScroll">
              自动滚动
            </label>
          </div>
        </div>
      </div>

      <!-- 右侧数据区域 -->
      <div class="data-panel">
        <!-- 接收区域 -->
        <div class="receive-area">
          <div class="area-header">
            <h3>接收数据</h3>
            <div class="header-controls">
              <span class="data-count">{{ receivedData.length }} 条记录</span>
              <button class="btn btn-small" @click="clearReceiveData">清空</button>
              <button class="btn btn-small" @click="saveReceiveData">保存</button>
            </div>
          </div>
          <div class="data-display" ref="receiveDisplay">
            <div 
              v-for="(item, index) in receivedData" 
              :key="index" 
              class="data-item"
              :class="item.type"
            >
              <span v-if="receiveSettings.timestamp" class="timestamp">
                {{ item.timestamp }}
              </span>
              <span class="data-content">{{ item.content }}</span>
            </div>
            <div v-if="receivedData.length === 0" class="empty-message">
              暂无接收数据
            </div>
          </div>
        </div>

        <!-- 发送区域 -->
        <div class="send-area">
          <div class="area-header">
            <h3>发送数据</h3>
            <div class="header-controls">
              <button class="btn btn-small" @click="clearSendData">清空</button>
            </div>
          </div>
          <div class="send-input-group">
            <textarea 
              v-model="sendData" 
              class="send-input"
              placeholder="请输入要发送的数据..."
              :disabled="!isConnected"
            ></textarea>
            <button 
              class="btn btn-primary send-btn" 
              @click="sendSerialData"
              :disabled="!isConnected || !sendData.trim()"
            >
              发送
            </button>
          </div>
          
          <!-- 快捷发送 -->
          <div class="quick-send">
            <h4>快捷发送</h4>
            <div class="quick-buttons">
              <button 
                v-for="(cmd, index) in quickCommands" 
                :key="index"
                class="btn btn-small quick-btn"
                @click="sendQuickCommand(cmd)"
                :disabled="!isConnected"
              >
                {{ cmd.name }}
              </button>
              <button class="btn btn-small btn-add" @click="addQuickCommand">
                + 添加
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 状态栏 -->
    <div class="status-bar">
      <div class="status-item">
        发送: {{ statistics.sentBytes }} 字节
      </div>
      <div class="status-item">
        接收: {{ statistics.receivedBytes }} 字节
      </div>
      <div class="status-item">
        错误: {{ statistics.errors }} 次
      </div>
      <div class="status-item">
        连接时间: {{ connectionTime }}
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'SerialDebugger',
  data() {
    return {
      isConnected: false,
      
      // 串口配置
      serialConfig: {
        port: '',
        baudRate: '115200',
        dataBits: '8',
        stopBits: '1',
        parity: 'none'
      },
      
      // 发送设置
      sendSettings: {
        hexMode: false,
        autoNewline: true,
        autoRepeat: false,
        repeatInterval: 1000
      },
      
      // 接收设置
      receiveSettings: {
        hexDisplay: false,
        timestamp: true,
        autoScroll: true
      },
      
      // 数据
      sendData: '',
      receivedData: [
        {
          timestamp: '2024-01-01 12:00:01',
          content: 'Hello World!',
          type: 'received'
        },
        {
          timestamp: '2024-01-01 12:00:02',
          content: 'AT+OK',
          type: 'sent'
        }
      ],
      
      // 快捷命令
      quickCommands: [
        { name: 'AT', command: 'AT' },
        { name: 'Reset', command: 'AT+RST' },
        { name: 'Version', command: 'AT+GMR' },
        { name: 'Help', command: 'AT+HELP' }
      ],
      
      // 统计信息
      statistics: {
        sentBytes: 1024,
        receivedBytes: 2048,
        errors: 0
      },
      
      connectionTime: '00:00:00'
    }
  },
  
  methods: {
    toggleConnection() {
      this.isConnected = !this.isConnected;
    },
    
    refreshPorts() {
      // 模拟刷新端口
      console.log('刷新端口');
    },
    
    sendSerialData() {
      if (!this.sendData.trim()) return;
      
      // 模拟发送数据
      const newItem = {
        timestamp: new Date().toLocaleString(),
        content: this.sendData,
        type: 'sent'
      };
      this.receivedData.push(newItem);
      this.sendData = '';
      
      this.$nextTick(() => {
        if (this.receiveSettings.autoScroll) {
          this.scrollToBottom();
        }
      });
    },
    
    sendQuickCommand(cmd) {
      this.sendData = cmd.command;
      this.sendSerialData();
    },
    
    addQuickCommand() {
      // 模拟添加快捷命令
      const name = prompt('请输入命令名称:');
      const command = prompt('请输入命令内容:');
      if (name && command) {
        this.quickCommands.push({ name, command });
      }
    },
    
    clearReceiveData() {
      this.receivedData = [];
    },
    
    clearSendData() {
      this.sendData = '';
    },
    
    saveReceiveData() {
      // 模拟保存数据
      console.log('保存接收数据');
    },
    
    scrollToBottom() {
      const display = this.$refs.receiveDisplay;
      if (display) {
        display.scrollTop = display.scrollHeight;
      }
    }
  }
}
</script>

<style scoped>
.serial-debugger {
  height: 100vh;
  display: flex;
  flex-direction: column;
  font-family: 'Microsoft YaHei', Arial, sans-serif;
  background-color: #f5f5f5;
}

.header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 15px 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.header h1 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 12px;
  height: 12px;
  background-color: #ff4757;
  border-radius: 50%;
  transition: background-color 0.3s;
}

.status-dot.connected {
  background-color: #2ed573;
}

.status-text {
  font-size: 14px;
  font-weight: 500;
}

.main-container {
  flex: 1;
  display: flex;
  gap: 20px;
  padding: 20px;
  overflow: hidden;
}

.control-panel {
  width: 300px;
  background: white;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  overflow-y: auto;
}

.panel-section {
  margin-bottom: 25px;
  padding-bottom: 20px;
  border-bottom: 1px solid #eee;
}

.panel-section:last-child {
  border-bottom: none;
  margin-bottom: 0;
}

.panel-section h3 {
  margin: 0 0 15px 0;
  color: #333;
  font-size: 16px;
  font-weight: 600;
}

.panel-section h4 {
  margin: 10px 0;
  color: #666;
  font-size: 14px;
  font-weight: 500;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  color: #555;
  font-size: 14px;
  font-weight: 500;
}

.form-control {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  transition: border-color 0.3s;
}

.form-control:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 2px rgba(102, 126, 234, 0.2);
}

.button-group {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #555;
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  margin: 0;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
  outline: none;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background-color: #667eea;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #5a6fd8;
}

.btn-danger {
  background-color: #ff4757;
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background-color: #ff3742;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background-color: #5a6268;
}

.btn-small {
  padding: 6px 12px;
  font-size: 12px;
}

.btn-add {
  background-color: #2ed573;
  color: white;
}

.btn-add:hover:not(:disabled) {
  background-color: #26d467;
}

.data-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.receive-area {
  flex: 2;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  display: flex;
  flex-direction: column;
}

.send-area {
  flex: 1;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  display: flex;
  flex-direction: column;
}

.area-header {
  padding: 15px 20px;
  border-bottom: 1px solid #eee;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.area-header h3 {
  margin: 0;
  color: #333;
  font-size: 16px;
  font-weight: 600;
}

.header-controls {
  display: flex;
  align-items: center;
  gap: 10px;
}

.data-count {
  font-size: 12px;
  color: #666;
}

.data-display {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
}

.data-item {
  margin-bottom: 8px;
  padding: 6px 10px;
  border-radius: 4px;
  word-break: break-all;
}

.data-item.received {
  background-color: #f8f9fa;
  border-left: 3px solid #2ed573;
}

.data-item.sent {
  background-color: #e3f2fd;
  border-left: 3px solid #667eea;
}

.timestamp {
  color: #999;
  font-size: 11px;
  margin-right: 10px;
}

.data-content {
  color: #333;
}

.empty-message {
  text-align: center;
  color: #999;
  font-style: italic;
  margin-top: 50px;
}

.send-input-group {
  padding: 20px;
  display: flex;
  gap: 10px;
}

.send-input {
  flex: 1;
  min-height: 80px;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  resize: vertical;
  outline: none;
  transition: border-color 0.3s;
}

.send-input:focus {
  border-color: #667eea;
  box-shadow: 0 0 0 2px rgba(102, 126, 234, 0.2);
}

.send-btn {
  align-self: flex-start;
  height: fit-content;
}

.quick-send {
  padding: 0 20px 20px;
}

.quick-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.quick-btn {
  background-color: #f8f9fa;
  color: #333;
  border: 1px solid #ddd;
}

.quick-btn:hover:not(:disabled) {
  background-color: #e9ecef;
}

.status-bar {
  background: white;
  padding: 10px 20px;
  border-top: 1px solid #eee;
  display: flex;
  gap: 30px;
  font-size: 12px;
  color: #666;
}

.status-item {
  display: flex;
  align-items: center;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .main-container {
    flex-direction: column;
    gap: 15px;
    padding: 15px;
  }
  
  .control-panel {
    width: 100%;
  }
  
  .header {
    padding: 10px 15px;
  }
  
  .header h1 {
    font-size: 20px;
  }
  
  .button-group {
    flex-direction: column;
  }
  
  .btn {
    width: 100%;
  }
}
</style>