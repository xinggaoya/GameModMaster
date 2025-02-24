import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { MessageApi } from 'naive-ui'
import type { Trainer, InstalledTrainer } from '../types'

// 本地存储键名
const STORAGE_KEY = {
  installed: 'installedTrainers',
  downloaded: 'downloadedTrainers',
}

// 保存数据到本地存储
const saveToStorage = {
  installed: (data: InstalledTrainer[]) => {
    localStorage.setItem(STORAGE_KEY.installed, JSON.stringify(data))
  },
  downloaded: (data: Trainer[]) => {
    localStorage.setItem(STORAGE_KEY.downloaded, JSON.stringify(data))
  },
}

// 从本地存储加载数据
const loadFromStorage = {
  installed: (): InstalledTrainer[] => {
    const data = localStorage.getItem(STORAGE_KEY.installed)
    return data ? JSON.parse(data) : []
  },
  downloaded: (): Trainer[] => {
    const data = localStorage.getItem(STORAGE_KEY.downloaded)
    return data ? JSON.parse(data) : []
  },
}

declare global {
  interface Window {
    $message: MessageApi
  }
}

export const useTrainerStore = defineStore('trainer', () => {
  // 状态
  const trainers = ref<Trainer[]>([]) // 所有修改器列表
  const installedTrainers = ref<InstalledTrainer[]>([]) // 已安装的修改器
  const downloadedTrainers = ref<Trainer[]>([]) // 已下载的修改器
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const searchQuery = ref('')
  const currentPage = ref(1)

  // 初始化函数
  async function initialize() {
    console.log('Store: 开始初始化')
    try {
      isLoading.value = true
      error.value = null
      // 从本地存储加载数据
      installedTrainers.value = loadFromStorage.installed()
      downloadedTrainers.value = loadFromStorage.downloaded()
      console.log('Store: 已加载本地数据:', {
        installed: installedTrainers.value.length,
        downloaded: downloadedTrainers.value.length,
      })
      // 初始加载修改器列表
      await fetchTrainers(1)
      console.log('Store: 初始化完成', {
        trainersCount: trainers.value.length,
        installedCount: installedTrainers.value.length,
      })
    } catch (err) {
      console.error('Store: 初始化失败:', err)
      error.value = err instanceof Error ? err.message : '加载数据失败'
    } finally {
      isLoading.value = false
      console.log('Store: 加载状态已重置')
    }
  }

  // 添加修改器
  async function addTrainer(trainer: Trainer) {
    try {
      // 检查是否已经存在
      const exists = installedTrainers.value.some((t) => t.id === trainer.id)
      if (exists) {
        throw new Error('修改器已经安装')
      }

      // 添加安装时间
      const now = new Date().toISOString()
      const trainerWithMeta: InstalledTrainer = {
        ...trainer,
        installed_path: '', // 由于是本地存储，不需要实际路径
        install_time: now,
        last_launch_time: now,
      }

      // 添加到列表
      installedTrainers.value.push(trainerWithMeta)
      // 保存到本地存储
      saveToStorage.installed(installedTrainers.value)

      return true
    } catch (err) {
      error.value = err instanceof Error ? err.message : '添加修改器失败'
      console.error('Failed to add trainer:', err)
      return false
    }
  }

  // 移除修改器
  async function removeTrainer(trainerId: string) {
    try {
      // 从列表中移除
      installedTrainers.value = installedTrainers.value.filter((t) => t.id !== trainerId)
      // 更新本地存储
      saveToStorage.installed(installedTrainers.value)
      return true
    } catch (err) {
      error.value = err instanceof Error ? err.message : '移除修改器失败'
      console.error('Failed to remove trainer:', err)
      return false
    }
  }

  // 更新修改器信息
  async function updateTrainer(trainer: Trainer) {
    try {
      const index = installedTrainers.value.findIndex((t) => t.id === trainer.id)
      if (index === -1) {
        throw new Error('修改器不存在')
      }

      // 更新信息
      installedTrainers.value[index] = {
        ...installedTrainers.value[index],
        ...trainer,
      }

      // 保存到本地存储
      saveToStorage.installed(installedTrainers.value)
      return true
    } catch (err) {
      error.value = err instanceof Error ? err.message : '更新修改器失败'
      console.error('Failed to update trainer:', err)
      return false
    }
  }

  // 更新启动时间
  async function updateLaunchTime(trainerId: string) {
    try {
      const index = installedTrainers.value.findIndex((t) => t.id === trainerId)
      if (index === -1) {
        throw new Error('修改器不存在')
      }

      // 更新启动时间
      installedTrainers.value[index] = {
        ...installedTrainers.value[index],
        last_launch_time: new Date().toISOString(),
      }

      // 保存到本地存储
      saveToStorage.installed(installedTrainers.value)
      return true
    } catch (err) {
      error.value = err instanceof Error ? err.message : '更新启动时间失败'
      console.error('Failed to update launch time:', err)
      return false
    }
  }

  // 获取修改器信息
  function getTrainer(trainerId: string): Trainer | undefined {
    return installedTrainers.value.find((t: Trainer) => t.id === trainerId)
  }

  // 清除错误
  function clearError() {
    error.value = null
  }

  const fetchTrainers = async (page: number = 1) => {
    console.log('Store: 开始获取修改器列表', { page })
    try {
      const result = await invoke<Trainer[]>('fetch_trainers', { page })
      console.log('Store: 获取修改器列表成功', { count: result?.length })
      // 确保结果是数组
      if (Array.isArray(result)) {
        trainers.value = result
        currentPage.value = page
      } else {
        throw new Error('返回数据格式错误')
      }
    } catch (err) {
      console.error('Store: 获取修改器列表失败:', err)
      error.value = err instanceof Error ? err.message : '获取修改器列表失败'
      window.$message?.error('获取修改器列表失败')
      throw err
    }
  }

  const searchTrainers = async (query: string) => {
    isLoading.value = true
    try {
      searchQuery.value = query
      const result = await invoke<Trainer[]>('search_trainers', { query })
      trainers.value = result
    } catch (error) {
      console.error('Failed to search trainers:', error)
      window.$message?.error('搜索修改器失败')
    } finally {
      isLoading.value = false
    }
  }

  const getTrainerDetail = async (id: string): Promise<Trainer | null> => {
    try {
      return await invoke<Trainer>('get_trainer_detail', { id })
    } catch (error) {
      console.error('Failed to get trainer detail:', error)
      window.$message.error('获取修改器详情失败')
      return null
    }
  }

  const downloadTrainer = async (trainer: Trainer) => {
    try {
      console.log('开始下载修改器:', trainer)
      const downloadPath = await invoke<string>('download_trainer', { trainer })
      console.log('下载结果:', downloadPath)

      // 添加安装信息
      const trainerWithMeta: InstalledTrainer = {
        ...trainer,
        installed_path: downloadPath,
        install_time: new Date().toISOString(),
        last_launch_time: null,
      }

      // 添加到已下载列表
      const downloadExists = downloadedTrainers.value.some((t) => t.id === trainer.id)
      if (!downloadExists) {
        downloadedTrainers.value.push(trainer)
        // 保存到本地存储
        saveToStorage.downloaded(downloadedTrainers.value)
      }

      // 添加到已安装列表
      const installExists = installedTrainers.value.some((t) => t.id === trainer.id)
      if (!installExists) {
        installedTrainers.value.push(trainerWithMeta)
        // 保存到本地存储
        saveToStorage.installed(installedTrainers.value)
      }

      if (window.$message) {
        window.$message.success('下载成功')
      }
    } catch (error) {
      console.error('下载失败:', error)
      if (window.$message) {
        window.$message.error(`下载失败: ${error instanceof Error ? error.message : String(error)}`)
      }
      throw new Error(error instanceof Error ? error.message : String(error))
    }
  }

  const deleteTrainer = async (trainerId: string) => {
    try {
      await invoke('delete_trainer', { trainerId })
      // 从已下载列表中移除
      downloadedTrainers.value = downloadedTrainers.value.filter((t) => t.id !== trainerId)
      saveToStorage.downloaded(downloadedTrainers.value)

      // 从已安装列表中移除
      installedTrainers.value = installedTrainers.value.filter((t) => t.id !== trainerId)
      saveToStorage.installed(installedTrainers.value)

      window.$message.success('删除成功')
    } catch (error) {
      console.error('Failed to delete trainer:', error)
      window.$message.error('删除失败')
      throw error
    }
  }

  const launchTrainer = async (trainerId: string) => {
    try {
      await invoke('launch_trainer', { trainerId })
      // 更新最后启动时间
      const index = installedTrainers.value.findIndex((t) => t.id === trainerId)
      if (index !== -1) {
        installedTrainers.value[index] = {
          ...installedTrainers.value[index],
          last_launch_time: new Date().toISOString(),
        }
        // 更新本地存储
        saveToStorage.installed(installedTrainers.value)
      }
      window.$message.success('启动成功')
    } catch (error) {
      console.error('Failed to launch trainer:', error)
      window.$message.error('启动失败')
      throw error
    }
  }

  return {
    // 状态
    trainers,
    installedTrainers,
    downloadedTrainers,
    isLoading,
    error,
    searchQuery,
    currentPage,

    // 方法
    initialize,
    addTrainer,
    removeTrainer,
    updateTrainer,
    updateLaunchTime,
    getTrainer,
    clearError,
    fetchTrainers,
    searchTrainers,
    getTrainerDetail,
    downloadTrainer,
    deleteTrainer,
    launchTrainer,
  }
})
