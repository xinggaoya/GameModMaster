<template>
  <div class="download-progress-wrapper">
    <n-card v-if="activeDownload" size="small" class="download-card">
      <template #header>
        <div class="card-header">
          <span class="title">{{ activeDownload.trainer_id }}</span>
          <n-space>
            <n-tag type="info" size="small">{{ formatStatus }}</n-tag>
            <n-button v-if="isCancellable" size="tiny" quaternary @click="handleCancel">
              取消
            </n-button>
          </n-space>
        </div>
      </template>
      <div class="progress-container">
        <n-progress
          type="line"
          :percentage="progress"
          :processing="isProcessing"
          :status="progressStatus"
        />
        <div class="progress-details">
          <span>{{ formatProgress }}</span>
          <span v-if="activeDownload.speed">{{ formatSpeed }}</span>
        </div>
      </div>
    </n-card>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, onMounted, onBeforeUnmount } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { NCard, NProgress, NTag, NButton, NSpace } from 'naive-ui'

// 组件属性
const props = defineProps<{
  trainerId: string
}>()

// 组件事件
const emit = defineEmits<{
  (event: 'completed', trainerId: string): void
  (event: 'error', trainerId: string): void
  (event: 'canceled', trainerId: string): void
}>()

// 下载状态
const activeDownload = ref<{
  trainer_id: string
  progress: number
  downloaded_bytes: number
  total_bytes?: number
  status: string
  speed?: number
} | null>(null)

// 取消处理器
let unlistenFunc: (() => void) | null = null

// 计算进度百分比
const progress = computed(() => {
  if (!activeDownload.value) return 0
  return Math.min(Math.round(activeDownload.value.progress), 100)
})

// 计算进度状态
const progressStatus = computed(() => {
  if (!activeDownload.value) return 'default'

  switch (activeDownload.value.status) {
    case 'completed':
      return 'success'
    case 'error':
      return 'error'
    default:
      return 'default'
  }
})

// 处理中状态
const isProcessing = computed(() => {
  if (!activeDownload.value) return false
  return ['downloading', 'processing', 'extracting'].includes(activeDownload.value.status)
})

// 是否可取消
const isCancellable = computed(() => {
  if (!activeDownload.value) return false
  return activeDownload.value.status === 'downloading'
})

// 格式化状态
const formatStatus = computed(() => {
  if (!activeDownload.value) return '等待中'

  switch (activeDownload.value.status) {
    case 'downloading':
      return '下载中'
    case 'processing':
      return '处理中'
    case 'extracting':
      return '解压中'
    case 'completed':
      return '已完成'
    case 'error':
      return '错误'
    default:
      return activeDownload.value.status
  }
})

// 格式化进度
const formatProgress = computed(() => {
  if (!activeDownload.value) return ''

  const downloaded = formatBytes(activeDownload.value.downloaded_bytes)

  if (activeDownload.value.total_bytes) {
    const total = formatBytes(activeDownload.value.total_bytes)
    return `${downloaded} / ${total}`
  }

  return downloaded
})

// 格式化下载速度
const formatSpeed = computed(() => {
  if (!activeDownload.value || !activeDownload.value.speed) return ''

  return `${formatBytes(activeDownload.value.speed * 1024)}/s`
})

// 初始化下载监听
onMounted(async () => {
  // 检查是否有活动下载
  try {
    const downloads = await invoke<
      Array<{
        trainer_id: string
        progress: number
        downloaded_bytes: number
        total_bytes?: number
        status: string
        speed?: number
      }>
    >('get_active_downloads')
    const current = downloads.find((d) => d.trainer_id === props.trainerId)
    if (current) {
      activeDownload.value = current
    } else {
      activeDownload.value = {
        trainer_id: props.trainerId,
        progress: 0,
        downloaded_bytes: 0,
        status: 'waiting',
      }
    }
  } catch (err) {
    console.error('获取下载状态失败:', err)
  }

  // 监听下载进度事件
  unlistenFunc = await listen<{
    trainer_id: string
    progress: number
    downloaded_bytes: number
    total_bytes?: number
    status: string
    speed?: number
  }>('download-progress', (event) => {
    const { payload } = event

    if (payload && payload.trainer_id === props.trainerId) {
      activeDownload.value = payload

      // 处理完成事件
      if (payload.status === 'completed') {
        emit('completed', props.trainerId)
      }

      // 处理错误事件
      if (payload.status === 'error') {
        emit('error', props.trainerId)
      }
    }
  })
})

// 清理监听事件
onBeforeUnmount(() => {
  if (unlistenFunc) {
    unlistenFunc()
  }
})

// 取消下载
const handleCancel = async () => {
  try {
    await invoke('cancel_download', { fileId: props.trainerId })
    emit('canceled', props.trainerId)
  } catch (err) {
    console.error('取消下载失败:', err)
  }
}

// 格式化文件大小
function formatBytes(bytes: number, decimals = 2) {
  if (bytes === 0) return '0 Bytes'

  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB']

  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i]
}
</script>

<style scoped>
.download-progress-wrapper {
  margin: 10px 0;
}

.download-card {
  width: 100%;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.title {
  font-weight: bold;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.progress-container {
  margin-top: 8px;
}

.progress-details {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  margin-top: 4px;
  color: rgba(0, 0, 0, 0.6);
}
</style>
