<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useMessage, useDialog } from 'naive-ui'
import { DownloadOutline, PlayOutline, TrashOutline, TimeOutline } from '@vicons/ionicons5'
import type { Trainer } from '@/types'
import { useTrainerStore } from '@/stores/trainer'
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  trainer: Trainer
  showButtons?: 'default' | 'downloaded'
}>()

const { t, locale } = useI18n()
const showButtons = props.showButtons || 'default'

const router = useRouter()
const store = useTrainerStore()
const message = useMessage()
const dialog = useDialog()

const isDownloaded = computed(() => {
  return store.downloadedTrainers.some((t) => t.id === props.trainer.id)
})

const formatDate = (dateString: string) => {
  if (!dateString) return ''
  const date = new Date(dateString)
  return new Intl.DateTimeFormat(locale.value, { month: 'short', day: 'numeric' }).format(date)
}

const handleCardClick = () => {
  router.push(`/detail/${props.trainer.id}`)
}

const handleDownload = async (e: Event) => {
  e.stopPropagation()
  try {
    message.loading(t('common.loading'))
    const detail = await store.getTrainerDetail(props.trainer.id)
    await store.downloadTrainer(detail)
    message.success(t('gameCard.messages.downloadSuccess'))
  } catch (error) {
    message.error(error instanceof Error ? error.message : t('gameCard.messages.downloadFailed'))
  }
}

const handleLaunch = async (e: Event) => {
  e.stopPropagation()
  try {
    await store.launchTrainer(props.trainer.id)
    message.success(t('gameCard.messages.launchSuccess'))
  } catch (error) {
    message.error(error instanceof Error ? error.message : t('gameCard.messages.launchFailed'))
  }
}

const handleDelete = async (e: Event) => {
  e.stopPropagation()
  dialog.warning({
    title: t('gameCard.deleteConfirm.title'),
    content: t('gameCard.deleteConfirm.content', { name: props.trainer.name }),
    positiveText: t('gameCard.deleteConfirm.positive'),
    negativeText: t('gameCard.deleteConfirm.negative'),
    onPositiveClick: async () => {
      try {
        await store.deleteTrainer(props.trainer.id)
        message.success(t('gameCard.messages.deleteSuccess'))
      } catch (error) {
        message.error(t('gameCard.messages.deleteFailed'))
      }
    },
  })
}
</script>

<template>
  <div class="game-card" @click="handleCardClick">
    <div class="card-cover">
      <img
        :src="trainer.thumbnail || '/placeholder.png'"
        :alt="trainer.name"
        class="cover-image"
        loading="lazy"
      />
      <div class="cover-overlay">
        <span class="view-hint">{{ t('gameCard.viewDetail') }}</span>
      </div>

      <div v-if="isDownloaded" class="status-tag downloaded">
        {{ t('gameCard.downloaded') }}
      </div>
    </div>

    <div class="card-content">
      <h3 class="card-title" :title="trainer.name">
        {{ trainer.name }}
      </h3>

      <div class="card-meta">
        <span class="meta-item version">{{ trainer.version }}</span>
        <span class="meta-item" v-if="trainer.last_update">
          <NIcon size="12"><TimeOutline /></NIcon>
          {{ formatDate(trainer.last_update) }}
        </span>
      </div>

      <div class="card-actions">
        <template v-if="showButtons === 'downloaded'">
          <button class="action-btn danger" @click="handleDelete">
            <NIcon size="14"><TrashOutline /></NIcon>
          </button>
          <button class="action-btn primary" @click="handleLaunch">
            <NIcon size="14"><PlayOutline /></NIcon>
            {{ t('gameCard.actions.launch') }}
          </button>
        </template>
        <template v-else>
          <button
            class="action-btn primary"
            @click="handleDownload"
            :disabled="isDownloaded"
          >
            <NIcon size="14"><DownloadOutline /></NIcon>
            {{ isDownloaded ? t('gameCard.actions.downloaded') : t('gameCard.actions.download') }}
          </button>
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
.game-card {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
  cursor: pointer;
  transition: all 0.25s ease;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.game-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.1);
}

.card-cover {
  position: relative;
  aspect-ratio: 16/9;
  overflow: hidden;
}

.cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.game-card:hover .cover-image {
  transform: scale(1.05);
}

.cover-overlay {
  position: absolute;
  inset: 0;
  background: linear-gradient(to top, rgba(0,0,0,0.6) 0%, transparent 60%);
  display: flex;
  align-items: flex-end;
  justify-content: center;
  padding-bottom: 12px;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.game-card:hover .cover-overlay {
  opacity: 1;
}

.view-hint {
  color: white;
  font-size: 0.813rem;
  font-weight: 600;
}

.status-tag {
  position: absolute;
  top: 10px;
  right: 10px;
  padding: 4px 10px;
  border-radius: 8px;
  font-size: 0.688rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

.status-tag.downloaded {
  background: #059669;
  color: white;
}

.card-content {
  padding: 14px;
}

.card-title {
  margin: 0 0 10px 0;
  font-size: 0.938rem;
  font-weight: 700;
  color: #1f2937;
  line-height: 1.3;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.card-meta {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 0.75rem;
  color: #64748b;
}

.meta-item.version {
  padding: 3px 8px;
  background: rgba(124, 58, 237, 0.1);
  color: #7c3aed;
  border-radius: 6px;
  font-weight: 600;
}

.card-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 12px;
  border: none;
  border-radius: 10px;
  font-size: 0.813rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s ease;
}

.action-btn.primary {
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  color: white;
}

.action-btn.primary:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.3);
}

.action-btn.primary:disabled {
  background: #cbd5e1;
  cursor: not-allowed;
}

.action-btn.danger {
  flex: 0;
  padding: 10px;
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.action-btn.danger:hover {
  background: #ef4444;
  color: white;
}
</style>
