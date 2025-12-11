<template>
  <NModal
    :show="show"
    preset="dialog"
    :title="title"
    :closable="!isDownloading"
    @update:show="updateShow"
  >
    <div class="update-dialog">
      <div v-if="updateInfo && !isDownloading" class="update-info">
        <div class="version-info">
          <NText>{{ t('update.currentVersion') }}: {{ currentVersion }}</NText>
          <NText type="success">{{ t('update.latestVersion') }}: {{ updateInfo.latest_version }}</NText>
        </div>

        <NDivider>{{ t('update.contentTitle') }}</NDivider>

        <div class="release-notes">
          <NScrollbar style="max-height: 200px">
            <div class="markdown-content">{{ updateInfo.release_notes }}</div>
          </NScrollbar>
        </div>
      </div>

      <div v-if="isDownloading" class="download-progress">
        <NProgress
          type="line"
          :percentage="updateProgress.progress"
          :processing="updateProgress.status === 'downloading'"
          :status="updateProgress.status === 'error' ? 'error' : 'success'"
        />
        <p class="progress-message">{{ updateProgress.message }}</p>
      </div>
    </div>

    <template #action>
      <NSpace justify="end">
        <NButton v-if="!isDownloading" @click="onCancel" :disabled="isDownloading">
          {{ t('update.cancel') }}
        </NButton>
        <NButton
          v-if="!isDownloading && updateInfo"
          type="primary"
          @click="startUpdate"
          :disabled="isDownloading"
        >
          {{ t('update.start') }}
        </NButton>
      </NSpace>
    </template>
  </NModal>
</template>

<script setup lang="ts">
import { computed, defineProps, defineEmits } from 'vue'
import { NModal, NButton, NSpace, NText, NDivider, NProgress, NScrollbar } from 'naive-ui'
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
.update-dialog {
  min-width: 400px;
  min-height: 200px;
}

.version-info {
  display: flex;
  justify-content: space-between;
  margin-bottom: var(--spacing-base);
}

.release-notes {
  margin-top: var(--spacing-base);
  max-height: 200px;
  overflow-y: auto;
}

.markdown-content {
  white-space: pre-line;
  line-height: 1.5;
}

.download-progress {
  padding: var(--spacing-base) 0;
}

.progress-message {
  margin-top: var(--spacing-small);
  text-align: center;
  color: var(--text-color-secondary);
}
</style>
