<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import {
  FolderOpenOutline,
  MoonOutline,
  SunnyOutline,
  LanguageOutline,
  InformationCircleOutline,
  RefreshOutline,
  LogoGithub,
  CheckmarkCircleOutline,
} from '@vicons/ionicons5'

interface AppSettings {
  download_path: string
  theme: string
  auto_extract: boolean
  auto_open_folder: boolean
  language: string
}

const message = useMessage()

// 设置状态
const settings = ref<AppSettings>({
  download_path: '',
  theme: 'system',
  auto_extract: true,
  auto_open_folder: false,
  language: 'zh-CN',
})

const isLoading = ref(false)
const isSaving = ref(false)
const appVersion = ref('')

// 主题选项
const themeOptions = [
  { label: '跟随系统', value: 'system' },
  { label: '浅色模式', value: 'light' },
  { label: '深色模式', value: 'dark' },
]

// 语言选项
const languageOptions = [
  { label: '简体中文', value: 'zh-CN' },
  { label: 'English', value: 'en-US' },
]

// 加载设置
const loadSettings = async () => {
  try {
    isLoading.value = true
    const result = await invoke<AppSettings>('get_settings')
    settings.value = result
    appVersion.value = await invoke<string>('get_app_version')
  } catch (error) {
    console.error('加载设置失败:', error)
    message.error('加载设置失败')
  } finally {
    isLoading.value = false
  }
}

// 保存设置
const saveSettings = async () => {
  try {
    isSaving.value = true
    await invoke('save_settings', { settings: settings.value })
    message.success('设置已保存')
  } catch (error) {
    console.error('保存设置失败:', error)
    message.error('保存设置失败')
  } finally {
    isSaving.value = false
  }
}

// 选择下载文件夹
const selectDownloadFolder = async () => {
  try {
    const result = await invoke<string | null>('select_download_folder')
    if (result) {
      settings.value.download_path = result
      message.success('下载路径已更新')
    }
  } catch (error) {
    console.error('选择文件夹失败:', error)
    message.error('选择文件夹失败')
  }
}

// 打开下载目录
const openDownloadFolder = async () => {
  try {
    await invoke('open_download_folder')
  } catch (error) {
    console.error('打开文件夹失败:', error)
    message.error('打开文件夹失败')
  }
}

// 打开 GitHub
const openGitHub = () => {
  window.open('https://github.com/xinggaoya/GameModMaster', '_blank')
}

onMounted(loadSettings)
</script>

<template>
  <div class="settings-view">
    <!-- 页面标题 -->
    <header class="page-header">
      <h1 class="page-title">设置</h1>
      <p class="page-subtitle">自定义您的 GameMod Master 体验</p>
    </header>

    <NSpin :show="isLoading">
      <div class="settings-grid">
        <!-- 下载设置 -->
        <section class="settings-section">
          <div class="section-header">
            <NIcon size="20" class="section-icon">
              <FolderOpenOutline />
            </NIcon>
            <h2 class="section-title">下载设置</h2>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">下载路径</div>
              <div class="setting-description">修改器将保存到此目录</div>
            </div>
            <div class="setting-control path-control">
              <NInput
                v-model:value="settings.download_path"
                placeholder="选择下载路径"
                readonly
                class="path-input"
              />
              <NButton @click="selectDownloadFolder" type="primary">
                浏览
              </NButton>
              <NButton @click="openDownloadFolder" quaternary>
                打开
              </NButton>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">自动解压</div>
              <div class="setting-description">下载完成后自动解压修改器</div>
            </div>
            <div class="setting-control">
              <NSwitch v-model:value="settings.auto_extract" />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">下载完成后打开文件夹</div>
              <div class="setting-description">下载完成后自动打开所在目录</div>
            </div>
            <div class="setting-control">
              <NSwitch v-model:value="settings.auto_open_folder" />
            </div>
          </div>
        </section>

        <!-- 外观设置 -->
        <section class="settings-section">
          <div class="section-header">
            <NIcon size="20" class="section-icon">
              <MoonOutline />
            </NIcon>
            <h2 class="section-title">外观</h2>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">主题</div>
              <div class="setting-description">选择应用的外观主题</div>
            </div>
            <div class="setting-control">
              <NSelect
                v-model:value="settings.theme"
                :options="themeOptions"
                class="theme-select"
              />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">语言</div>
              <div class="setting-description">界面显示语言</div>
            </div>
            <div class="setting-control">
              <NSelect
                v-model:value="settings.language"
                :options="languageOptions"
                class="language-select"
              />
            </div>
          </div>
        </section>

        <!-- 关于 -->
        <section class="settings-section">
          <div class="section-header">
            <NIcon size="20" class="section-icon">
              <InformationCircleOutline />
            </NIcon>
            <h2 class="section-title">关于</h2>
          </div>

          <div class="about-card">
            <div class="app-info">
              <div class="app-logo">
                <NIcon size="48" color="#7c3aed">
                  <CheckmarkCircleOutline />
                </NIcon>
              </div>
              <div class="app-details">
                <h3 class="app-name">GameMod Master</h3>
                <p class="app-version">版本 {{ appVersion || '1.0.0' }}</p>
                <p class="app-author">by xinggaoya</p>
              </div>
            </div>
            <div class="about-actions">
              <NButton @click="openGitHub" quaternary>
                <template #icon>
                  <NIcon><LogoGithub /></NIcon>
                </template>
                GitHub
              </NButton>
            </div>
          </div>
        </section>
      </div>

      <!-- 保存按钮 -->
      <div class="save-area">
        <NButton
          type="primary"
          size="large"
          @click="saveSettings"
          :loading="isSaving"
          class="save-button"
        >
          <template #icon>
            <NIcon><CheckmarkCircleOutline /></NIcon>
          </template>
          保存设置
        </NButton>
      </div>
    </NSpin>
  </div>
</template>

<style scoped>
.settings-view {
  max-width: 800px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 32px;
}

.page-title {
  font-size: 2rem;
  font-weight: 800;
  margin: 0 0 8px 0;
  background: linear-gradient(135deg, #7c3aed 0%, #0891b2 100%);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.page-subtitle {
  margin: 0;
  font-size: 1rem;
  color: #64748b;
}

.settings-grid {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings-section {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 20px;
  padding: 24px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.06);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
}

.section-icon {
  color: #7c3aed;
}

.section-title {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 700;
  color: #1f2937;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  border-bottom: 1px solid rgba(0, 0, 0, 0.04);
}

.setting-item:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.setting-info {
  flex: 1;
}

.setting-label {
  font-size: 0.938rem;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 4px;
}

.setting-description {
  font-size: 0.813rem;
  color: #64748b;
}

.setting-control {
  flex-shrink: 0;
}

.path-control {
  display: flex;
  gap: 8px;
  flex: 1;
  max-width: 400px;
}

.path-input {
  flex: 1;
}

.theme-select,
.language-select {
  min-width: 140px;
}

/* 关于卡片 */
.about-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.app-info {
  display: flex;
  align-items: center;
  gap: 16px;
}

.app-logo {
  width: 64px;
  height: 64px;
  border-radius: 16px;
  background: linear-gradient(135deg, rgba(124, 58, 237, 0.1) 0%, rgba(8, 145, 178, 0.1) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-details {
  display: flex;
  flex-direction: column;
}

.app-name {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 700;
  color: #1f2937;
}

.app-version {
  margin: 4px 0 0 0;
  font-size: 0.875rem;
  color: #64748b;
}

.app-author {
  margin: 2px 0 0 0;
  font-size: 0.813rem;
  color: #94a3b8;
}

/* 保存区域 */
.save-area {
  margin-top: 32px;
  display: flex;
  justify-content: center;
}

.save-button {
  min-width: 160px;
  height: 48px;
  border-radius: 14px;
  font-weight: 600;
  font-size: 1rem;
}

/* 响应式 */
@media (max-width: 640px) {
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .path-control {
    max-width: none;
    width: 100%;
    flex-wrap: wrap;
  }

  .about-card {
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;
  }
}
</style>
