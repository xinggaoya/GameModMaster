import { invoke } from '@tauri-apps/api/core'
import type { Trainer, InstalledTrainer } from '../types'

// 缓存配置
const CACHE_CONFIG = {
  expirationTime: 1000 * 60 * 15, // 15分钟
  maxRetries: 3,
  retryDelay: 1000,
}

// 本地存储键名（用于迁移）
const STORAGE_KEY = {
  installed: 'installedTrainers',
  downloaded: 'downloadedTrainers',
  trainerList: 'trainerList',
  searchResults: 'searchResults',
}

// 缓存项接口
interface CacheItem<T> {
  data: T
  timestamp: number
  expiration: number
}

// 新的存储服务类
export class StorageService {
  // 获取已安装的修改器
  static async getInstalledTrainers(): Promise<InstalledTrainer[]> {
    try {
      return await invoke<InstalledTrainer[]>('get_installed_trainers')
    } catch (error) {
      console.error('Failed to get installed trainers:', error)
      return []
    }
  }

  // 保存已安装的修改器
  static async saveInstalledTrainers(trainers: InstalledTrainer[]): Promise<void> {
    try {
      await invoke('save_installed_trainers', { trainers })
    } catch (error) {
      console.error('Failed to save installed trainers:', error)
      throw error
    }
  }

  // 获取已下载的修改器
  static async getDownloadedTrainers(): Promise<Trainer[]> {
    try {
      return await invoke<Trainer[]>('get_downloaded_trainers')
    } catch (error) {
      console.error('Failed to get downloaded trainers:', error)
      return []
    }
  }

  // 保存已下载的修改器
  static async saveDownloadedTrainers(trainers: Trainer[]): Promise<void> {
    try {
      await invoke('save_downloaded_trainers', { trainers })
    } catch (error) {
      console.error('Failed to save downloaded trainers:', error)
      throw error
    }
  }

  // 缓存修改器列表
  static async cacheTrainerList(page: number, trainers: Trainer[]): Promise<void> {
    try {
      await invoke('cache_trainer_list', { page, trainers })
    } catch (error) {
      console.error('Failed to cache trainer list:', error)
    }
  }

  // 获取缓存的修改器列表
  static async getCachedTrainerList(page: number): Promise<Trainer[] | null> {
    try {
      return await invoke<Trainer[]>('get_cached_trainer_list', { page })
    } catch (error) {
      console.error('Failed to get cached trainer list:', error)
      return null
    }
  }

  // 缓存搜索结果
  static async cacheSearchResults(query: string, page: number, trainers: Trainer[]): Promise<void> {
    try {
      await invoke('cache_search_results', { query, page, trainers })
    } catch (error) {
      console.error('Failed to cache search results:', error)
    }
  }

  // 获取缓存的搜索结果
  static async getCachedSearchResults(query: string, page: number): Promise<Trainer[] | null> {
    try {
      return await invoke<Trainer[]>('get_cached_search_results', { query, page })
    } catch (error) {
      console.error('Failed to get cached search results:', error)
      return null
    }
  }

  // 清理过期缓存
  static async cleanExpiredCache(): Promise<void> {
    try {
      await invoke('clean_expired_cache')
    } catch (error) {
      console.error('Failed to clean expired cache:', error)
    }
  }

  // 从 localStorage 迁移数据
  static async migrateFromLocalStorage(): Promise<void> {
    try {
      // 检查是否已经迁移过
      const migrationFlag = localStorage.getItem('storage_migrated')
      if (migrationFlag === 'true') {
        console.log('Storage already migrated')
        return
      }

      // 收集 localStorage 中的数据
      const migrateData: Record<string, any> = {}

      // 收集已安装的修改器
      const installed = localStorage.getItem(STORAGE_KEY.installed)
      if (installed) {
        try {
          migrateData[STORAGE_KEY.installed] = JSON.parse(installed)
        } catch (e) {
          console.error('Failed to parse installed trainers:', e)
        }
      }

      // 收集已下载的修改器
      const downloaded = localStorage.getItem(STORAGE_KEY.downloaded)
      if (downloaded) {
        try {
          migrateData[STORAGE_KEY.downloaded] = JSON.parse(downloaded)
        } catch (e) {
          console.error('Failed to parse downloaded trainers:', e)
        }
      }

      // 收集缓存数据（可选，因为缓存可能会过期）
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i)
        if (key && (key.startsWith(STORAGE_KEY.trainerList) || key.startsWith(STORAGE_KEY.searchResults))) {
          try {
            const value = localStorage.getItem(key)
            if (value) {
              const parsed = JSON.parse(value)
              // 检查是否过期
              if (parsed.expiration && parsed.expiration > Date.now()) {
                migrateData[key] = parsed
              }
            }
          } catch (e) {
            console.error('Failed to parse cache item:', key, e)
          }
        }
      }

      // 发送数据到后端
      if (Object.keys(migrateData).length > 0) {
        await invoke('migrate_from_local_storage', { data: migrateData })
        console.log('Storage migration completed')
      }

      // 标记迁移完成
      localStorage.setItem('storage_migrated', 'true')

      // 清理 localStorage 中的数据（可选）
      // 如果想保留数据作为备份，可以注释掉下面的代码
      Object.keys(migrateData).forEach(key => {
        localStorage.removeItem(key)
      })

    } catch (error) {
      console.error('Failed to migrate from localStorage:', error)
      throw error
    }
  }

  // 获取所有存储键（调试用）
  static async getStorageKeys(): Promise<string[]> {
    try {
      return await invoke<string[]>('get_storage_keys')
    } catch (error) {
      console.error('Failed to get storage keys:', error)
      return []
    }
  }

  // 清空所有存储（危险操作）
  static async clearAllStorage(): Promise<void> {
    try {
      await invoke('clear_all_storage')
      localStorage.removeItem('storage_migrated') // 重置迁移标志
    } catch (error) {
      console.error('Failed to clear all storage:', error)
      throw error
    }
  }

  // 检查是否已迁移
  static isMigrated(): boolean {
    return localStorage.getItem('storage_migrated') === 'true'
  }

  // 重置迁移状态（用于测试）
  static resetMigration(): void {
    localStorage.removeItem('storage_migrated')
  }
}

// 异步操作重试包装器（从原来的 store 复制）
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
        await new Promise((resolve) => setTimeout(resolve, delay * attempt))
      }
    }
  }

  throw lastError
}

export { withRetry }