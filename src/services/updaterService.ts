import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'
import { getVersion } from '@tauri-apps/api/app'
import i18n from '@/i18n'

export interface UpdateInfo {
  latest_version: string
  download_url: string
  has_update: boolean
  release_notes: string
}

export interface UpdateProgress {
  status: 'downloading' | 'completed' | 'error'
  progress: number
  message: string
}

export const updateProgress = ref<UpdateProgress>({
  status: 'downloading',
  progress: 0,
  message: '',
})

export const updateInfo = ref<UpdateInfo | null>(null)

export const hasUpdate = ref(false)

export const isCheckingUpdate = ref(false)

export const isDownloading = ref(false)

let listenerInitialized = false

export async function initUpdateListener() {
  if (listenerInitialized) return

  try {
    await listen<UpdateProgress>('update-progress', (event) => {
      updateProgress.value = event.payload

      if (event.payload.status === 'completed' || event.payload.status === 'error') {
        setTimeout(() => {
          isDownloading.value = false
        }, 1000)
      }
    })

    listenerInitialized = true
    console.log('update progress listener initialized')
  } catch (error) {
    console.error('failed to init update progress listener:', error)
  }
}

export async function getAppVersion(): Promise<string> {
  try {
    return await getVersion()
  } catch (error) {
    console.error('get app version failed:', error)
    return '未知'
  }
}

export async function checkForUpdates(): Promise<UpdateInfo | null> {
  try {
    isCheckingUpdate.value = true
    const currentVersion = await getAppVersion()
    const result = await invoke<UpdateInfo>('check_update', { currentVersion })

    updateInfo.value = result
    hasUpdate.value = result.has_update

    return result
  } catch (error) {
    console.error('check update failed:', error)
    return null
  } finally {
    isCheckingUpdate.value = false
  }
}

export async function downloadAndInstallUpdate(downloadUrl: string): Promise<void> {
  if (isDownloading.value) return

  try {
    await initUpdateListener()

    isDownloading.value = true
    updateProgress.value = {
      status: 'downloading',
      progress: 0,
      message: i18n.global.t('update.progress.preparing'),
    }

    await invoke('download_and_install_update', { downloadUrl })
  } catch (error) {
    console.error('download update failed:', error)
    updateProgress.value = {
      status: 'error',
      progress: 0,
      message: i18n.global.t('update.progress.failed', { error }),
    }
    isDownloading.value = false
  }
}
