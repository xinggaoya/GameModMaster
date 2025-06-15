<template>
  <div class="log-viewer-container">
    <n-card title="日志查看器" :bordered="false">
      <template #header-extra>
        <n-space>
          <n-button @click="refreshLogs" size="small">
            <template #icon>
              <n-icon><refresh-outline /></n-icon>
            </template>
            刷新
          </n-button>
          <n-button @click="exportLog" size="small">
            <template #icon>
              <n-icon><download-outline /></n-icon>
            </template>
            导出
          </n-button>
        </n-space>
      </template>

      <div class="log-content">
        <n-scrollbar>
          <pre v-if="logs.length > 0" class="log-text">{{ logs.join('\n') }}</pre>
          <div v-else class="empty-logs">没有可用的日志</div>
        </n-scrollbar>
      </div>
    </n-card>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open as openPath } from '@tauri-apps/api/shell'
import { readTextFile } from '@tauri-apps/api/fs'
import { NCard, NButton, NScrollbar, NSpace, NIcon } from 'naive-ui'
import { RefreshOutline, DownloadOutline } from '@vicons/ionicons5'

const logs = ref<string[]>([])
const logPath = ref('')

// 加载日志文件
async function loadLogs() {
  try {
    // 获取日志目录
    const logPathObj = await invoke<string>('export_latest_log').catch(() => null)

    if (!logPathObj) {
      logs.value = ['无法获取日志文件路径']
      return
    }

    logPath.value = logPathObj

    // 读取日志文件内容
    const content = await readTextFile(logPathObj)
    logs.value = content.split('\n').filter((line: string) => line.trim() !== '')
  } catch (error) {
    console.error('加载日志失败:', error)
    logs.value = [`加载日志失败: ${error instanceof Error ? error.message : String(error)}`]
  }
}

// 刷新日志
async function refreshLogs() {
  logs.value = ['加载中...']
  await loadLogs()
}

// 导出日志
async function exportLog() {
  try {
    const logDir = await invoke<string>('get_log_dir')
    if (logDir) {
      await openPath(logDir)
    }
  } catch (error) {
    console.error('打开日志目录失败:', error)
    window.$message?.error('打开日志目录失败')
  }
}

// 初始加载
onMounted(async () => {
  await loadLogs()
})
</script>

<style scoped>
.log-viewer-container {
  width: 100%;
  height: 100%;
}

.log-content {
  height: 400px;
  border: 1px solid #eee;
  border-radius: 4px;
  margin-top: 10px;
  overflow: hidden;
}

.log-text {
  padding: 10px;
  margin: 0;
  font-family: monospace;
  font-size: 12px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
}

.empty-logs {
  padding: 20px;
  text-align: center;
  color: rgba(0, 0, 0, 0.45);
}
</style>
