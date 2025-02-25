<template>
  <div class="check-update">
    <ActionButton
      :type="hasUpdate ? 'success' : 'default'"
      :loading="isCheckingUpdate"
      @click="checkUpdate"
    >
      {{ buttonText }}
      <template #icon v-if="hasUpdate">
        <NIcon><ArrowUpCircleOutline /></NIcon>
      </template>
    </ActionButton>

    <UpdateDialog v-model:show="showUpdateDialog" :current-version="currentVersion" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { NIcon } from 'naive-ui'
import { ArrowUpCircleOutline } from '@vicons/ionicons5'
import ActionButton from '../common/ActionButton.vue'
import UpdateDialog from './UpdateDialog.vue'
import {
  checkForUpdates,
  hasUpdate,
  isCheckingUpdate,
  getAppVersion,
  initUpdateListener,
} from '../../services/updaterService'

// 当前版本
const currentVersion = ref('')
// 是否显示更新对话框
const showUpdateDialog = ref(false)

// 获取当前版本
const getCurrentVersion = async () => {
  try {
    currentVersion.value = await getAppVersion()
  } catch (error) {
    console.error('获取应用版本失败:', error)
  }
}

// 按钮文本
const buttonText = computed(() => {
  if (isCheckingUpdate.value) {
    return '检查中...'
  }
  if (hasUpdate.value) {
    return '有新版本可用'
  }
  return '检查更新'
})

// 检查更新
const checkUpdate = async () => {
  if (isCheckingUpdate.value) return

  const updateResult = await checkForUpdates()

  if (updateResult?.has_update) {
    showUpdateDialog.value = true
  } else if (updateResult) {
    // 显示已是最新版本的消息
    window.$message?.success('当前已是最新版本')
  }
}

// 在组件挂载时获取当前版本并初始化更新监听器
onMounted(async () => {
  await getCurrentVersion()
  await initUpdateListener()
})
</script>

<style scoped>
.check-update {
  display: inline-block;
}
</style>
