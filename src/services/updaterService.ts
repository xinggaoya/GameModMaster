import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'
import { getVersion } from '@tauri-apps/api/app'

// 更新信息类型
export interface UpdateInfo {
  latest_version: string
  download_url: string
  has_update: boolean
  release_notes: string
}

// 更新进度类型
export interface UpdateProgress {
  status: 'downloading' | 'completed' | 'error'
  progress: number
  message: string
}

// 当前更新进度
export const updateProgress = ref<UpdateProgress>({
  status: 'downloading',
  progress: 0,
  message: '',
})

// 更新信息
export const updateInfo = ref<UpdateInfo | null>(null)

// 是否有更新可用
export const hasUpdate = ref(false)

// 是否正在检查更新
export const isCheckingUpdate = ref(false)

// 是否正在下载更新
export const isDownloading = ref(false)

// 是否初始化了监听器
let listenerInitialized = false

// 初始化更新进度监听器
export async function initUpdateListener() {
  if (listenerInitialized) return

  try {
    // 监听更新进度事件
    await listen<UpdateProgress>('update-progress', (event) => {
      updateProgress.value = event.payload

      // 如果下载完成或出错，重置下载状态
      if (event.payload.status === 'completed' || event.payload.status === 'error') {
        setTimeout(() => {
          isDownloading.value = false
        }, 1000) // 稍微延迟以便用户看到完成状态
      }
    })

    listenerInitialized = true
    console.log('更新进度监听器初始化成功')
  } catch (error) {
    console.error('初始化更新进度监听器失败:', error)
  }
}

// 获取当前版本
export async function getAppVersion(): Promise<string> {
  try {
    // 使用 Tauri 前端 API 直接获取版本号
    return await getVersion()
  } catch (error) {
    console.error('获取应用版本失败:', error)
    return '未知'
  }
}

// 检查更新
export async function checkForUpdates(): Promise<UpdateInfo | null> {
  try {
    isCheckingUpdate.value = true

    // 获取当前版本
    const currentVersion = await getAppVersion()

    // 检查更新
    const result = await invoke<UpdateInfo>('check_update', { currentVersion: currentVersion })

    updateInfo.value = result
    hasUpdate.value = result.has_update

    return result
  } catch (error) {
    console.error('检查更新失败:', error)
    return null
  } finally {
    isCheckingUpdate.value = false
  }
}

// 下载并安装更新
export async function downloadAndInstallUpdate(downloadUrl: string): Promise<void> {
  if (isDownloading.value) return

  try {
    // 初始化监听器（如果尚未初始化）
    await initUpdateListener()

    isDownloading.value = true
    updateProgress.value = {
      status: 'downloading',
      progress: 0,
      message: '准备下载...',
    }

    // 开始下载和安装
    await invoke('download_and_install_update', { download_url: downloadUrl })
  } catch (error) {
    console.error('下载安装更新失败:', error)
    updateProgress.value = {
      status: 'error',
      progress: 0,
      message: `下载失败: ${error}`,
    }
    isDownloading.value = false
  }
}
