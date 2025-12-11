<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  ArrowBackOutline,
  DownloadOutline,
  PlayOutline,
  TrashOutline,
  TimeOutline,
  GameControllerOutline,
  CodeOutline,
} from '@vicons/ionicons5'
import { useTrainerStore } from '@/stores/trainer'
import { useMessage, useDialog } from 'naive-ui'
import type { Trainer } from '@/types'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()
const store = useTrainerStore()
const message = useMessage()
const dialog = useDialog()

const loading = ref(true)
const trainer = ref<Trainer | null>(null)
const isDownloading = ref(false)
const downloadProgress = ref(0)

const trainerId = computed(() => route.params.id as string)
const isDownloaded = computed(() =>
  store.downloadedTrainers.some((t) => t.id === trainer.value?.id),
)

const fetchDetail = async () => {
  try {
    loading.value = true
    trainer.value = await store.getTrainerDetail(trainerId.value)
  } catch (error) {
    console.error(error)
    message.error(t('detail.messages.fetchFailed'))
  } finally {
    loading.value = false
  }
}

const formatNumber = (num: number): string => {
  if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M'
  if (num >= 1000) return (num / 1000).toFixed(1) + 'K'
  return num.toString()
}

const handleDownload = async () => {
  if (!trainer.value) return
  try {
    isDownloading.value = true
    downloadProgress.value = 0

    const interval = setInterval(() => {
      if (downloadProgress.value < 90) {
        downloadProgress.value += Math.random() * 15
      }
    }, 300)

    await store.downloadTrainer(trainer.value)
    clearInterval(interval)
    downloadProgress.value = 100
    message.success(t('detail.messages.downloadSuccess'))
    await fetchDetail()
  } catch (error) {
    message.error(error instanceof Error ? error.message : t('detail.messages.downloadFailed'))
  } finally {
    isDownloading.value = false
  }
}

const handleLaunch = async () => {
  if (!trainer.value) return
  try {
    await store.launchTrainer(trainer.value.id)
    message.success(t('detail.messages.launchSuccess'))
  } catch (error) {
    message.error(error instanceof Error ? error.message : t('detail.messages.launchFailed'))
  }
}

const handleDelete = async () => {
  if (!trainer.value) return
  dialog.warning({
    title: t('detail.deleteConfirm.title'),
    content: t('detail.deleteConfirm.content', { name: trainer.value.name }),
    positiveText: t('detail.deleteConfirm.positive'),
    negativeText: t('detail.deleteConfirm.negative'),
    onPositiveClick: async () => {
      try {
        await store.deleteTrainer(trainer.value!.id)
        message.success(t('detail.messages.deleteSuccess'))
        await fetchDetail()
      } catch (error) {
        console.error(error)
        message.error(t('detail.messages.deleteFailed'))
      }
    },
  })
}

onMounted(fetchDetail)
</script>

<template>
  <div class="detail-view">
    <button class="back-btn" @click="router.back()">
      <NIcon size="20"><ArrowBackOutline /></NIcon>
      <span>{{ t('common.back') }}</span>
    </button>

    <div v-if="loading" class="loading-state">
      <NSpin size="large" />
    </div>

    <template v-else-if="trainer">
      <div class="detail-layout">
        <div class="cover-section">
          <div class="cover-card">
            <img
              :src="trainer.thumbnail || '/placeholder.png'"
              :alt="trainer.name"
              class="cover-image"
            />
            <div v-if="isDownloaded" class="status-badge">{{ t('detail.status.downloaded') }}</div>
          </div>

          <div class="action-area">
            <template v-if="isDownloading">
              <NProgress
                :percentage="downloadProgress"
                type="line"
                :show-indicator="false"
                processing
              />
              <span class="progress-text">
                {{ t('detail.progress.downloading', { progress: Math.round(downloadProgress) }) }}
              </span>
            </template>
            <template v-else-if="isDownloaded">
              <NButton type="success" size="large" block @click="handleLaunch">
                <template #icon><NIcon><PlayOutline /></NIcon></template>
                {{ t('detail.actions.launch') }}
              </NButton>
              <NButton quaternary size="large" block @click="handleDelete">
                <template #icon><NIcon><TrashOutline /></NIcon></template>
                {{ t('detail.actions.delete') }}
              </NButton>
            </template>
            <template v-else>
              <NButton type="primary" size="large" block @click="handleDownload">
                <template #icon><NIcon><DownloadOutline /></NIcon></template>
                {{ t('detail.actions.download') }}
              </NButton>
            </template>
          </div>
        </div>

        <div class="info-section">
          <h1 class="trainer-name">{{ trainer.name }}</h1>

          <div class="meta-grid">
            <div class="meta-card">
              <NIcon size="20" color="#7c3aed"><CodeOutline /></NIcon>
              <div class="meta-content">
                <span class="meta-label">{{ t('detail.meta.version') }}</span>
                <span class="meta-value">{{ trainer.version }}</span>
              </div>
            </div>
            <div class="meta-card">
              <NIcon size="20" color="#0891b2"><GameControllerOutline /></NIcon>
              <div class="meta-content">
                <span class="meta-label">{{ t('detail.meta.gameVersion') }}</span>
                <span class="meta-value">{{ trainer.game_version }}</span>
              </div>
            </div>
            <div class="meta-card">
              <NIcon size="20" color="#059669"><DownloadOutline /></NIcon>
              <div class="meta-content">
                <span class="meta-label">{{ t('detail.meta.downloads') }}</span>
                <span class="meta-value">{{ formatNumber(trainer.download_count) }}</span>
              </div>
            </div>
            <div class="meta-card">
              <NIcon size="20" color="#d97706"><TimeOutline /></NIcon>
              <div class="meta-content">
                <span class="meta-label">{{ t('detail.meta.lastUpdate') }}</span>
                <span class="meta-value">{{ trainer.last_update }}</span>
              </div>
            </div>
          </div>

          <div class="description-card">
            <h3>{{ t('detail.description.title') }}</h3>
            <pre class="description-text">{{ trainer.description || t('detail.description.empty') }}</pre>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.detail-view {
  max-width: 1200px;
  margin: 0 auto;
}

.back-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border: none;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  font-size: 0.875rem;
  font-weight: 600;
  color: #64748b;
  cursor: pointer;
  margin-bottom: 24px;
  transition: all 0.2s ease;
}

.back-btn:hover {
  color: #7c3aed;
  transform: translateX(-4px);
}

.loading-state {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}

.detail-layout {
  display: grid;
  grid-template-columns: 360px 1fr;
  gap: 32px;
}

.cover-section {
  position: sticky;
  top: 24px;
}

.cover-card {
  position: relative;
  border-radius: 20px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  margin-bottom: 20px;
}

.cover-image {
  width: 100%;
  aspect-ratio: 16/10;
  object-fit: cover;
}

.status-badge {
  position: absolute;
  top: 12px;
  right: 12px;
  padding: 6px 12px;
  background: #059669;
  color: white;
  border-radius: 8px;
  font-size: 0.75rem;
  font-weight: 700;
}

.action-area {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 20px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 16px;
}

.progress-text {
  text-align: center;
  font-size: 0.875rem;
  color: #64748b;
}

.trainer-name {
  font-size: 1.75rem;
  font-weight: 800;
  color: #1f2937;
  margin: 0 0 24px 0;
  line-height: 1.3;
}

.meta-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-bottom: 24px;
}

.meta-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 14px;
}

.meta-content {
  display: flex;
  flex-direction: column;
}

.meta-label {
  font-size: 0.75rem;
  color: #64748b;
  margin-bottom: 2px;
}

.meta-value {
  font-size: 0.938rem;
  font-weight: 700;
  color: #1f2937;
}

.description-card {
  padding: 20px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 16px;
}

.description-card h3 {
  margin: 0 0 12px 0;
  font-size: 1rem;
  font-weight: 700;
  color: #1f2937;
}

.description-text {
  margin: 0;
  font-size: 0.875rem;
  color: #475569;
  line-height: 1.7;
  white-space: pre-wrap;
  font-family: inherit;
}

@media (max-width: 900px) {
  .detail-layout {
    grid-template-columns: 1fr;
  }

  .cover-section {
    position: static;
  }

  .meta-grid {
    grid-template-columns: 1fr;
  }
}
</style>
