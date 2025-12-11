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
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const currentVersion = ref('')
const showUpdateDialog = ref(false)

const getCurrentVersion = async () => {
  try {
    currentVersion.value = await getAppVersion()
  } catch (error) {
    console.error('获取应用版本失败:', error)
  }
}

const buttonText = computed(() => {
  if (isCheckingUpdate.value) {
    return t('update.checking')
  }
  if (hasUpdate.value) {
    return t('update.foundTitle')
  }
  return t('update.check')
})

const checkUpdate = async () => {
  if (isCheckingUpdate.value) return

  const updateResult = await checkForUpdates()

  if (updateResult?.has_update) {
    showUpdateDialog.value = true
  } else if (updateResult) {
    window.$message?.success(t('update.upToDate'))
  }
}

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
