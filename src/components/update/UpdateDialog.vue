<template>
  <NModal
    :show="show"
    preset="dialog"
    :title="title"
    :closable="!isDownloading"
    @update:show="updateShow"
  >
    <div class="update-dialog">
      <!-- 更新信息 -->
      <div v-if="updateInfo && !isDownloading" class="update-info">
        <div class="version-info">
          <NText>当前版本: {{ currentVersion }}</NText>
          <NText type="success">最新版本: {{ updateInfo.latest_version }}</NText>
        </div>

        <NDivider>更新内容</NDivider>

        <div class="release-notes">
          <NScrollbar style="max-height: 200px">
            <div class="markdown-content">{{ updateInfo.release_notes }}</div>
          </NScrollbar>
        </div>
      </div>

      <!-- 下载进度 -->
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
        <NButton v-if="!isDownloading" @click="onCancel" :disabled="isDownloading"> 取消 </NButton>
        <NButton
          v-if="!isDownloading && updateInfo"
          type="primary"
          @click="startUpdate"
          :disabled="isDownloading"
        >
          立即更新
        </NButton>
      </NSpace>
    </template>
  </NModal>
</template>

<script setup lang="ts">
import { ref, computed, defineProps, defineEmits } from 'vue'
import { NModal, NButton, NSpace, NText, NDivider, NProgress, NScrollbar } from 'naive-ui'
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

// 对话框标题
const title = computed(() => {
  if (isDownloading.value) {
    return '正在下载更新'
  }
  return '发现新版本'
})

// 更新show值
const updateShow = (value: boolean) => {
  emit('update:show', value)
}

// 开始更新
const startUpdate = async () => {
  if (!updateInfo.value) return

  await downloadAndInstallUpdate(updateInfo.value.download_url)
}

// 取消更新
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
