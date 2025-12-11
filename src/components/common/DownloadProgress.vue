<template>
  <div class="download-progress-wrapper">
    <n-card v-if="activeDownload" size="small" class="download-card">
      <template #header>
        <div class="card-header">
          <span class="title">{{ activeDownload.trainer_id }}</span>
          <n-space>
            <n-tag type="info" size="small">{{ formatStatus }}</n-tag>
            <n-button v-if="isCancellable" size="tiny" quaternary @click="handleCancel">
              {{ t('progress.cancel') }}
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
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  trainerId: string
}>()

const { t } = useI18n()

const emit = defineEmits<{
  (event: 'completed', trainerId: string): void
  (event: 'error', trainerId: string): void
  (event: 'canceled', trainerId: string): void
}>()

const activeDownload = ref<{
  trainer_id: string
  progress: number
  downloaded_bytes: number
  total_bytes?: number
  status: string
  speed?: number
} | null>(null)

let unlistenFunc: (() => void) | null = null

const progress = computed(() => {
  if (!activeDownload.value) return 0
  return Math.min(Math.round(activeDownload.value.progress), 100)
})

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

const isProcessing = computed(() => {
  if (!activeDownload.value) return false
  return ['downloading', 'processing', 'extracting'].includes(activeDownload.value.status)
})

const isCancellable = computed(() => {
  if (!activeDownload.value) return false
  return activeDownload.value.status === 'downloading'
})

const formatStatus = computed(() => {
  if (!activeDownload.value) return t('progress.status.waiting')

  switch (activeDownload.value.status) {
    case 'downloading':
      return t('progress.status.downloading')
    case 'processing':
      return t('progress.status.processing')
    case 'extracting':
      return t('progress.status.extracting')
    case 'completed':
      return t('progress.status.completed')
    case 'error':
      return t('progress.status.error')
    default:
      return activeDownload.value.status
  }
})

const formatProgress = computed(() => {
  if (!activeDownload.value) return ''

  const downloaded = formatBytes(activeDownload.value.downloaded_bytes)

  if (activeDownload.value.total_bytes) {
    const total = formatBytes(activeDownload.value.total_bytes)
    return `${downloaded} / ${total}`
  }

  return downloaded
})

const formatSpeed = computed(() => {
  if (!activeDownload.value || !activeDownload.value.speed) return ''

  return `${formatBytes(activeDownload.value.speed * 1024)}/s`
})

onMounted(async () => {
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
    console.error('无法获取下载状态', err)
  }

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

      if (payload.status === 'completed') {
        emit('completed', props.trainerId)
      }

      if (payload.status === 'error') {
        emit('error', props.trainerId)
      }
    }
  })
})

onBeforeUnmount(() => {
  if (unlistenFunc) {
    unlistenFunc()
  }
})

const handleCancel = async () => {
  try {
    await invoke('cancel_download', { fileId: props.trainerId })
    emit('canceled', props.trainerId)
  } catch (err) {
    console.error('取消下载失败:', err)
  }
}

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
