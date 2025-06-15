import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { MessageApi } from 'naive-ui'
import type { Trainer, InstalledTrainer } from '../types'
import { handleError } from '../utils/errorHandler'

// 缓存配置
const CACHE_CONFIG = {
  // 缓存过期时间（毫秒）
  expirationTime: 1000 * 60 * 15, // 15分钟
  // 最大重试次数
  maxRetries: 3,
  // 重试延迟（毫秒）
  retryDelay: 1000,
}

// 缓存项接口
interface CacheItem<T> {
  data: T
  timestamp: number
  expiration: number
}

// 本地存储键名
const STORAGE_KEY = {
  installed: 'installedTrainers',
  downloaded: 'downloadedTrainers',
  trainerList: 'trainerList',
  searchResults: 'searchResults',
}

// 保存数据到本地存储
const saveToStorage = {
  installed: (data: InstalledTrainer[]) => {
    localStorage.setItem(STORAGE_KEY.installed, JSON.stringify(data))
  },
  downloaded: (data: Trainer[]) => {
    localStorage.setItem(STORAGE_KEY.downloaded, JSON.stringify(data))
  },
  // 新增：缓存修改器列表
  trainerList: (data: Trainer[], page: number, timestamp: number = Date.now()) => {
    const cacheItem: CacheItem<Trainer[]> = {
      data,
      timestamp,
      expiration: timestamp + CACHE_CONFIG.expirationTime,
    }
    localStorage.setItem(`${STORAGE_KEY.trainerList}_${page}`, JSON.stringify(cacheItem))
  },
  // 新增：缓存搜索结果
  searchResults: (data: Trainer[], query: string, page: number, timestamp: number = Date.now()) => {
    const cacheItem: CacheItem<Trainer[]> = {
      data,
      timestamp,
      expiration: timestamp + CACHE_CONFIG.expirationTime,
    }
    localStorage.setItem(`${STORAGE_KEY.searchResults}_${query}_${page}`, JSON.stringify(cacheItem))
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
  // 新增：加载修改器列表缓存
  trainerList: (page: number): CacheItem<Trainer[]> | null => {
    const data = localStorage.getItem(`${STORAGE_KEY.trainerList}_${page}`)
    if (!data) return null

    const cacheItem: CacheItem<Trainer[]> = JSON.parse(data)
    // 检查是否过期
    if (cacheItem.expiration < Date.now()) {
      return null // 缓存已过期
    }

    return cacheItem
  },
  // 新增：加载搜索结果缓存
  searchResults: (query: string, page: number): CacheItem<Trainer[]> | null => {
    const data = localStorage.getItem(`${STORAGE_KEY.searchResults}_${query}_${page}`)
    if (!data) return null

    const cacheItem: CacheItem<Trainer[]> = JSON.parse(data)
    // 检查是否过期
    if (cacheItem.expiration < Date.now()) {
      return null // 缓存已过期
    }

    return cacheItem
  },
}

// 清除过期缓存
const cleanExpiredCache = () => {
  const now = Date.now()

  for (let i = 0; i < localStorage.length; i++) {
    const key = localStorage.key(i)
    if (!key) continue

    // 只清理我们的缓存项
    if (key.startsWith(STORAGE_KEY.trainerList) || key.startsWith(STORAGE_KEY.searchResults)) {
      try {
        const data = localStorage.getItem(key)
        if (data) {
          const cacheItem: CacheItem<Trainer[]> = JSON.parse(data)
          if (cacheItem.expiration < now) {
            localStorage.removeItem(key)
          }
        }
      } catch (e) {
        console.error('无效的缓存项:', key, e)
        // 删除无效的缓存项
        localStorage.removeItem(key)
      }
    }
  }
}

declare global {
  interface Window {
    $message: MessageApi
  }
}

// 异步操作重试包装器
const withRetry = async <T>(
  operation: () => Promise<T>,
  maxRetries: number = CACHE_CONFIG.maxRetries,
  delay: number = CACHE_CONFIG.retryDelay,
): Promise<T> => {
  let lastError: Error | unknown = null

  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await operation()
    } catch (err) {
      lastError = err
      console.warn(`操作失败 (尝试 ${attempt}/${maxRetries})`, err)

      if (attempt < maxRetries) {
        // 等待一段时间后重试
        await new Promise((resolve) => setTimeout(resolve, delay * attempt))
      }
    }
  }

  throw lastError
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
  const totalPages = ref(1)
  const lastUpdated = ref(Date.now()) // 上次更新时间

  // 计算属性
  const recentlyInstalledTrainers = computed(() => {
    return [...installedTrainers.value]
      .sort((a, b) => new Date(b.install_time).getTime() - new Date(a.install_time).getTime())
      .slice(0, 5)
  })

  const recentlyLaunchedTrainers = computed(() => {
    return [...installedTrainers.value]
      .filter((t) => t.last_launch_time)
      .sort((a, b) => {
        const timeA = a.last_launch_time ? new Date(a.last_launch_time).getTime() : 0
        const timeB = b.last_launch_time ? new Date(b.last_launch_time).getTime() : 0
        return timeB - timeA
      })
      .slice(0, 5)
  })

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

      // 清理过期缓存
      cleanExpiredCache()

      // 初始加载修改器列表
      await fetchTrainers(1)
      console.log('Store: 初始化完成', {
        trainersCount: trainers.value.length,
        installedCount: installedTrainers.value.length,
      })

      // 更新最后刷新时间
      lastUpdated.value = Date.now()
    } catch (err) {
      console.error('Store: 初始化失败:', err)
      error.value = err instanceof Error ? err.message : '加载数据失败'
      handleError(err, window.$message)
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

  // 获取修改器列表（优化版本）
  async function fetchTrainers(page: number) {
    try {
      isLoading.value = true
      error.value = null
      currentPage.value = page

      // 检查缓存是否有效
      const cachedData = loadFromStorage.trainerList(page)
      if (cachedData) {
        console.log('Store: 使用缓存的修改器列表，页码:', page)
        trainers.value = cachedData.data
        // 估算总页数（假设每页20条记录）
        totalPages.value = Math.ceil(100 / 20) // 使用一个默认值
        return
      }

      // 发起API请求
      const response = await withRetry(() =>
        invoke<{
          trainers: Trainer[]
          total: number
        }>('fetch_trainers', { page }),
      )

      trainers.value = response.trainers
      // 估算总页数（假设每页20条记录）
      totalPages.value = Math.ceil(response.total / 20)

      // 缓存结果
      saveToStorage.trainerList(response.trainers, page)

      console.log('Store: 已获取修改器列表，页码:', page, '总数:', response.trainers.length)
    } catch (err) {
      console.error('Store: 获取修改器列表失败:', err)
      error.value = err instanceof Error ? err.message : '获取修改器列表失败'
      handleError(err, window.$message)
    } finally {
      isLoading.value = false
    }
  }

  // 搜索修改器（优化版本）
  async function searchTrainers(query: string, page = 1) {
    try {
      if (!query.trim()) {
        return await fetchTrainers(page)
      }

      isLoading.value = true
      error.value = null
      searchQuery.value = query
      currentPage.value = page

      // 检查缓存是否有效
      const cachedData = loadFromStorage.searchResults(query, page)
      if (cachedData) {
        console.log('Store: 使用缓存的搜索结果，查询:', query, '页码:', page)
        trainers.value = cachedData.data
        totalPages.value = Math.ceil(cachedData.data.length / 20)
        return
      }

      // 发起API请求
      const response = await withRetry(() =>
        invoke<{
          trainers: Trainer[]
          total: number
        }>('search_trainers', { query, page }),
      )

      trainers.value = response.trainers
      totalPages.value = Math.ceil(response.total / 20)

      // 缓存结果
      saveToStorage.searchResults(response.trainers, query, page)

      console.log(
        'Store: 搜索完成，查询:',
        query,
        '页码:',
        page,
        '结果数:',
        response.trainers.length,
      )
    } catch (err) {
      console.error('Store: 搜索失败:', err)
      error.value = err instanceof Error ? err.message : '搜索失败'
      handleError(err, window.$message)
    } finally {
      isLoading.value = false
    }
  }

  // 获取修改器详情
  async function getTrainerDetail(id: string) {
    try {
      const result = await invoke<Trainer>('get_trainer_detail', { id })
      return result
    } catch (err) {
      handleError(err, window.$message)
      throw err // 允许调用者处理错误
    }
  }

  // 下载修改器
  async function downloadTrainer(trainer: Trainer) {
    try {
      const result = await invoke<string>('download_trainer', { trainer })

      // 添加到下载记录
      const exists = downloadedTrainers.value.some((t) => t.id === trainer.id)
      if (!exists) {
        downloadedTrainers.value.push(trainer)
        saveToStorage.downloaded(downloadedTrainers.value)
      }

      return result
    } catch (err) {
      handleError(err, window.$message)
      throw err
    }
  }

  // 删除修改器
  async function deleteTrainer(trainerId: string) {
    try {
      await invoke('delete_trainer', { trainerId })

      // 从下载记录中删除
      downloadedTrainers.value = downloadedTrainers.value.filter((t) => t.id !== trainerId)
      saveToStorage.downloaded(downloadedTrainers.value)

      return true
    } catch (err) {
      handleError(err, window.$message)
      throw err
    }
  }

  // 启动修改器
  async function launchTrainer(trainerId: string) {
    try {
      await invoke('launch_trainer', { trainerId })
      return true
    } catch (err) {
      handleError(err, window.$message)
      throw err
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
    totalPages,
    lastUpdated,

    // 计算属性
    recentlyInstalledTrainers,
    recentlyLaunchedTrainers,

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

    // 缓存管理
    cleanCache: cleanExpiredCache,
    refreshData: () => {
      // 强制刷新数据
      lastUpdated.value = Date.now()
      return fetchTrainers(currentPage.value)
    },
  }
})
