<template>
  <NModal
    class="update-modal"
    :show="show"
    :mask-closable="!isDownloading"
    :close-on-esc="!isDownloading"
    transform-origin="center"
    @update:show="updateShow"
  >
    <div class="update-card">
      <div class="card-header">
        <div class="title-block">
          <p class="label">{{ t('update.check') }}</p>
          <h3 class="title">{{ title }}</h3>
          <p class="subtitle" v-if="updateInfo && updateInfo.latest_version">
            v{{ currentVersion }} → v{{ updateInfo.latest_version }}
          </p>
        </div>
        <div class="status-chip" :data-type="statusType">
          {{ statusText }}
        </div>
      </div>

      <div class="card-body">
        <div v-if="updateInfo && !isDownloading" class="update-info">
          <div class="version-line">
            <span class="meta">{{ t('update.currentVersion') }}</span>
            <span class="value">v{{ currentVersion }}</span>
          </div>
          <div class="version-line">
            <span class="meta">{{ t('update.latestVersion') }}</span>
            <span class="value highlight">v{{ updateInfo.latest_version }}</span>
          </div>

          <div class="release-notes">
            <div class="notes-header">{{ t('update.contentTitle') }}</div>
            <NScrollbar style="max-height: 220px">
              <div class="markdown-content">{{ updateInfo.release_notes }}</div>
            </NScrollbar>
          </div>
        </div>

        <div v-else class="download-progress">
          <NProgress
            type="line"
            :percentage="updateProgress.progress"
            :processing="updateProgress.status === 'downloading'"
            :status="updateProgress.status === 'error' ? 'error' : 'success'"
          />
          <p class="progress-message">{{ updateProgress.message }}</p>
        </div>
      </div>

      <div class="card-footer">
        <NButton quaternary @click="onCancel" :disabled="isDownloading">
          {{ t('update.cancel') }}
        </NButton>
        <NButton
          v-if="updateInfo && !isDownloading"
          type="primary"
          size="medium"
          @click="startUpdate"
          :disabled="isDownloading"
        >
          {{ t('update.start') }}
        </NButton>
      </div>
    </div>
  </NModal>
</template>

<script setup lang="ts">
import { computed, defineProps, defineEmits } from 'vue'
import { NModal, NButton, NProgress, NScrollbar } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import {
  updateInfo,
  updateProgress,
  isDownloading,
  downloadAndInstallUpdate,
} from '../../services/updaterService'

const props = defineProps({
  show: {
    type: Boolean,
    default: false,
  },
  currentVersion: {
    type: String,
    required: true,
  },
})

const emit = defineEmits(['update:show', 'cancel'])
const { t } = useI18n()

const title = computed(() => {
  if (isDownloading.value) {
    return t('update.downloadingTitle')
  }
  return t('update.foundTitle')
})

const statusText = computed(() => {
  if (isDownloading.value) return updateProgress.value.message || t('update.downloadingTitle')
  if (updateInfo.value?.has_update) return t('update.foundTitle')
  if (updateInfo.value && !updateInfo.value.has_update) return t('update.upToDate')
  return t('update.check')
})

const statusType = computed(() => {
  if (isDownloading.value) return 'warning'
  if (updateInfo.value?.has_update) return 'success'
  if (updateInfo.value && !updateInfo.value.has_update) return 'default'
  return 'info'
})

const updateShow = (value: boolean) => {
  emit('update:show', value)
}

const startUpdate = async () => {
  if (!updateInfo.value) return
  await downloadAndInstallUpdate(updateInfo.value.download_url)
}

const onCancel = () => {
  emit('update:show', false)
  emit('cancel')
}
</script>

<style scoped>
.update-modal :global(.n-modal-mask) {
  backdrop-filter: blur(6px);
}

.update-card {
  min-width: 420px;
  background: var(--card-color, #ffffff);
  border: 1px solid rgba(124, 58, 237, 0.15);
  border-radius: 18px;
  box-shadow: 0 14px 45px rgba(0, 0, 0, 0.16);
  padding: 18px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

:global(.dark-theme) .update-card {
  background: rgba(15, 23, 42, 0.96);
  border-color: rgba(255, 255, 255, 0.08);
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.4);
}

.card-header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: flex-start;
}

.title-block {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.label {
  margin: 0;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-secondary, #475569);
  letter-spacing: 0.02em;
  text-transform: uppercase;
}

.title {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 800;
  color: var(--text-primary, #0f172a);
}

.subtitle {
  margin: 0;
  font-size: 0.95rem;
  color: var(--text-secondary, #475569);
}

.status-chip {
  display: inline-flex;
  align-items: center;
  padding: 6px 12px;
  border-radius: 999px;
  font-size: 0.85rem;
  font-weight: 700;
  color: #0f172a;
  background: #e2e8f0;
  border: 1px solid rgba(15, 23, 42, 0.08);
}

.status-chip[data-type='success'] {
  background: rgba(34, 197, 94, 0.18);
  color: #14532d;
  border-color: rgba(34, 197, 94, 0.35);
}

.status-chip[data-type='warning'] {
  background: rgba(249, 115, 22, 0.18);
  color: #9a3412;
  border-color: rgba(249, 115, 22, 0.35);
}

.status-chip[data-type='info'] {
  background: rgba(8, 145, 178, 0.16);
  color: #0f172a;
  border-color: rgba(8, 145, 178, 0.3);
}

.card-body {
  padding: 12px;
  border-radius: 14px;
  background: rgba(255, 255, 255, 0.96);
  border: 1px solid rgba(15, 23, 42, 0.05);
}

:global(.dark-theme) .card-body {
  background: rgba(15, 23, 42, 0.92);
  border-color: rgba(255, 255, 255, 0.08);
}

.update-info {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.version-line {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.meta {
  font-size: 0.9rem;
  color: var(--text-secondary, #475569);
}

.value {
  font-size: 1rem;
  font-weight: 700;
  color: var(--text-primary, #0f172a);
}

.value.highlight {
  color: #7c3aed;
}

:global(.dark-theme) .value.highlight {
  color: #a78bfa;
}

.release-notes {
  margin-top: 4px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.96);
  border: 1px solid rgba(15, 23, 42, 0.05);
}

:global(.dark-theme) .release-notes {
  background: rgba(30, 41, 59, 0.92);
  border-color: rgba(255, 255, 255, 0.08);
}

.notes-header {
  padding: 10px 12px;
  font-size: 0.95rem;
  font-weight: 700;
  color: var(--text-primary, #0f172a);
  border-bottom: 1px solid rgba(15, 23, 42, 0.05);
}

.markdown-content {
  padding: 12px;
  white-space: pre-line;
  line-height: 1.6;
  color: var(--text-primary, #0f172a);
}

.download-progress {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.progress-message {
  margin: 0;
  text-align: center;
  color: var(--text-secondary, #475569);
  font-size: 0.9rem;
}

.card-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

@media (max-width: 520px) {
  .update-card {
    min-width: 0;
  }
}
</style>
