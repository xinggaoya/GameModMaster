<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import {
  FolderOpenOutline,
  MoonOutline,
  InformationCircleOutline,
  LogoGithub,
  CheckmarkCircleOutline,
  GameControllerOutline,
  RefreshOutline,
  ArrowUpCircleOutline,
} from '@vicons/ionicons5'
import { useI18n } from 'vue-i18n'
import { setLocale, supportedLanguages, type Locale } from '@/i18n'
import UpdateDialog from '@/components/update/UpdateDialog.vue'
import {
  checkForUpdates,
  getAppVersion,
  hasUpdate,
  initUpdateListener,
  isCheckingUpdate,
  isDownloading,
  updateInfo,
  updateProgress,
} from '@/services/updaterService'

type ThemeSetting = 'light' | 'dark' | 'system'

interface AppSettings {
  download_path: string
  theme: ThemeSetting
  auto_extract: boolean
  auto_open_folder: boolean
  language: Locale
}

const { t } = useI18n()
const message = useMessage()
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

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
const isInitialized = ref(false)
const showUpdateDialog = ref(false)

const themeOptions = computed(() => [
  { label: t('settings.themeOptions.system'), value: 'system' },
  { label: t('settings.themeOptions.light'), value: 'light' },
  { label: t('settings.themeOptions.dark'), value: 'dark' },
])

const languageOptions = computed(() =>
  supportedLanguages.map((lang) => ({
    label: t(`languages.${lang.value}`),
    value: lang.value,
  })),
)

const resolveTheme = (theme: ThemeSetting) => {
  if (theme === 'system') {
    return mediaQuery.matches ? 'dark' : 'light'
  }
  return theme
}

const syncDomTheme = (resolvedTheme: 'light' | 'dark') => {
  const root = document.documentElement
  const body = document.body
  const isDark = resolvedTheme === 'dark'
  root.classList.toggle('dark-theme', isDark)
  root.classList.toggle('light-theme', !isDark)
  root.classList.toggle('dark', isDark)
  body.classList.toggle('dark-theme', isDark)
  body.classList.toggle('dark', isDark)
  root.dataset.theme = resolvedTheme
}

const applyTheme = (theme: ThemeSetting) => {
  const resolvedTheme = resolveTheme(theme)
  localStorage.setItem('theme', theme)
  syncDomTheme(resolvedTheme)
  window.dispatchEvent(new CustomEvent('theme-change', { detail: theme }))
}

watch(() => settings.value.theme, async (newTheme: ThemeSetting) => {
  applyTheme(newTheme)
  if (!isInitialized.value || isLoading.value) return
  try {
    await invoke('save_settings', { settings: settings.value })
  } catch (error) {
    console.error('auto save theme failed:', error)
    message.error(t('settings.messages.autoSaveThemeFailed'))
  }
})

watch(() => settings.value.auto_open_folder, async () => {
  if (!isInitialized.value || isLoading.value) return
  try {
    await invoke('save_settings', { settings: settings.value })
  } catch (error) {
    console.error('auto save folder setting failed:', error)
  }
})

watch(() => settings.value.language, async (newLang: Locale) => {
  if (!isInitialized.value || isLoading.value) return
  setLocale(newLang)
  try {
    await invoke('save_settings', { settings: settings.value })
  } catch (error) {
    console.error('auto save language failed:', error)
  }
})

const loadSettings = async () => {
  try {
    isLoading.value = true
    await initUpdateListener()
    const result = await invoke<AppSettings>('get_settings')
    settings.value = {
      ...settings.value,
      ...result,
    }
    appVersion.value = await getAppVersion()

    applyTheme((settings.value.theme || 'system') as ThemeSetting)
    if (settings.value.language) {
      setLocale(settings.value.language)
    }
    isInitialized.value = true
  } catch (error) {
    console.error('load settings failed:', error)
    message.error(t('settings.messages.loadFailed'))
  } finally {
    if (!isInitialized.value) {
      isInitialized.value = true
    }
    isLoading.value = false
  }
}

const updateStatusText = computed(() => {
  if (isDownloading.value) {
    return updateProgress.value.message || t('update.downloadingTitle')
  }
  if (isCheckingUpdate.value) {
    return t('update.checking')
  }
  if (hasUpdate.value && updateInfo.value) {
    return `${t('update.foundTitle')} · v${updateInfo.value.latest_version}`
  }
  if (updateInfo.value && !updateInfo.value.has_update) {
    return t('update.upToDate')
  }
  return t('update.check')
})

const updateStatusType = computed(() => {
  if (isDownloading.value) return 'warning'
  if (hasUpdate.value) return 'success'
  if (isCheckingUpdate.value) return 'info'
  if (updateInfo.value && !updateInfo.value.has_update) return 'default'
  return 'info'
})

const handleCheckUpdate = async () => {
  try {
    const result = await checkForUpdates()
    if (result?.has_update) {
      showUpdateDialog.value = true
    } else if (result) {
      message.success(t('update.upToDate'))
    }
  } catch (error) {
    console.error('check update failed:', error)
    message.error(t('settings.messages.checkUpdateFailed'))
  }
}

const saveSettings = async () => {
  try {
    isSaving.value = true
    await invoke('save_settings', { settings: settings.value })
    applyTheme(settings.value.theme)
    setLocale(settings.value.language)
    message.success(t('settings.messages.saveSuccess'))
  } catch (error) {
    console.error('save settings failed:', error)
    message.error(t('settings.messages.saveFailed'))
  } finally {
    isSaving.value = false
  }
}

const selectDownloadFolder = async () => {
  try {
    const result = await invoke<string | null>('select_download_folder')
    if (result) {
      settings.value.download_path = result
      message.success(t('settings.messages.saveSuccess'))
    }
  } catch (error) {
    console.error('select folder failed:', error)
    message.error(t('settings.messages.selectFolderFailed'))
  }
}

const openDownloadFolder = async () => {
  try {
    await invoke('open_download_folder')
  } catch (error) {
    console.error('open folder failed:', error)
    message.error(t('settings.messages.openFolderFailed'))
  }
}

const openGitHub = () => {
  window.open('https://github.com/xinggaoya/GameModMaster', '_blank')
}

onMounted(loadSettings)
</script>

<template>
  <div class="settings-view">
    <header class="page-header">
      <h1 class="page-title">{{ t('settings.title') }}</h1>
      <p class="page-subtitle">{{ t('settings.subtitle') }}</p>
    </header>

    <NSpin :show="isLoading">
      <div class="settings-grid">
        <section class="settings-section">
          <div class="section-header">
            <NIcon size="20" class="section-icon">
              <FolderOpenOutline />
            </NIcon>
            <h2 class="section-title">{{ t('settings.sections.download') }}</h2>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">{{ t('settings.fields.downloadPath.label') }}</div>
              <div class="setting-description">{{ t('settings.fields.downloadPath.desc') }}</div>
            </div>
            <div class="setting-control path-control">
              <NInput
                v-model:value="settings.download_path"
                :placeholder="t('settings.fields.downloadPath.label')"
                readonly
                class="path-input"
              />
              <NButton @click="selectDownloadFolder" type="primary">
                {{ t('settings.buttons.browse') }}
              </NButton>
              <NButton @click="openDownloadFolder" quaternary>
                {{ t('settings.buttons.open') }}
              </NButton>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">{{ t('settings.fields.autoOpen.label') }}</div>
              <div class="setting-description">{{ t('settings.fields.autoOpen.desc') }}</div>
            </div>
            <div class="setting-control">
              <NSwitch v-model:value="settings.auto_open_folder" />
            </div>
          </div>
        </section>

        <section class="settings-section">
          <div class="section-header">
            <NIcon size="20" class="section-icon">
              <MoonOutline />
            </NIcon>
            <h2 class="section-title">{{ t('settings.sections.appearance') }}</h2>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-label">{{ t('settings.fields.theme.label') }}</div>
              <div class="setting-description">{{ t('settings.fields.theme.desc') }}</div>
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
              <div class="setting-label">{{ t('settings.fields.language.label') }}</div>
              <div class="setting-description">{{ t('settings.fields.language.desc') }}</div>
            </div>
            <div class="setting-control">
              <NSelect
                v-model:value="settings.language"
                :options="languageOptions"
                class="theme-select"
              />
            </div>
          </div>
        </section>

        <section class="settings-section">
          <div class="section-header">
            <NIcon size="20" class="section-icon">
              <ArrowUpCircleOutline />
            </NIcon>
            <h2 class="section-title">{{ t('settings.sections.update') }}</h2>
          </div>

          <div class="update-panel">
            <div class="update-info">
              <div class="meta-row">
                <div class="meta-label">{{ t('update.currentVersion') }}</div>
                <div class="meta-value">v{{ appVersion || '0.0.0' }}</div>
              </div>
              <div class="meta-row" v-if="updateInfo && updateInfo.latest_version">
                <div class="meta-label">{{ t('update.latestVersion') }}</div>
                <div class="meta-value highlight">v{{ updateInfo.latest_version }}</div>
              </div>
              <div class="status-chip" :data-type="updateStatusType">
                {{ updateStatusText }}
              </div>
            </div>
            <div class="update-actions">
              <NButton
                type="primary"
                size="large"
                :loading="isCheckingUpdate"
                @click="handleCheckUpdate"
              >
                <template #icon>
                  <NIcon><RefreshOutline /></NIcon>
                </template>
                {{ hasUpdate ? t('update.start') : t('update.check') }}
              </NButton>
              <NButton
                secondary
                strong
                size="large"
                :disabled="!hasUpdate"
                @click="showUpdateDialog = true"
              >
                <template #icon>
                  <NIcon><ArrowUpCircleOutline /></NIcon>
                </template>
                {{ t('update.foundTitle') }}
              </NButton>
            </div>
          </div>
        </section>

        <section class="settings-section">
          <div class="section-header">
            <NIcon size="20" class="section-icon">
              <InformationCircleOutline />
            </NIcon>
            <h2 class="section-title">{{ t('settings.sections.about') }}</h2>
          </div>

          <div class="about-card">
            <div class="app-info">
              <div class="app-logo">
                <NIcon size="32" color="#7c3aed">
                  <GameControllerOutline />
                </NIcon>
              </div>
              <div class="app-details">
                <h3 class="app-name">{{ t('common.appName') }}</h3>
                <p class="app-version">{{ t('settings.about.version') }} {{ appVersion || '1.0.0' }}</p>
                <p class="app-author">{{ t('settings.about.author') }}: xinggaoya</p>
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
          {{ t('settings.buttons.save') }}
        </NButton>
      </div>
    </NSpin>

    <UpdateDialog v-model:show="showUpdateDialog" :current-version="appVersion || '0.0.0'" />
  </div>
</template>

<style scoped>
.settings-view {
  max-width: 800px;
  margin: 0 auto;
  color: var(--text-primary);
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
  color: var(--text-secondary);
}

.settings-grid {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings-section {
  background: var(--card-color);
  color: var(--text-primary);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid var(--divider-color);
  border-radius: 20px;
  padding: 24px;
  box-shadow: var(--shadow-lg);
  transition: background 0.3s ease, border-color 0.3s ease;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--divider-color);
}

.section-icon {
  color: #7c3aed;
}

.section-title {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 700;
  color: var(--text-primary);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  border-bottom: 1px solid var(--divider-color);
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
  color: var(--text-primary);
  margin-bottom: 4px;
}

.setting-description {
  font-size: 0.813rem;
  color: var(--text-secondary);
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

.theme-select {
  min-width: 140px;
}

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
  width: 56px;
  height: 56px;
  border-radius: 14px;
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
  font-size: 1rem;
  font-weight: 700;
  color: var(--text-primary);
}

.app-version {
  margin: 2px 0 0 0;
  font-size: 0.813rem;
  color: var(--text-secondary);
}

.app-author {
  margin: 0;
  font-size: 0.75rem;
  color: var(--text-tertiary);
}

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

:global(.dark-theme) .settings-section {
  background: rgba(30, 41, 59, 0.9);
  border-color: rgba(255, 255, 255, 0.08);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.4);
}

:global(.dark-theme) .section-icon {
  color: #a78bfa;
}

:global(.dark-theme) .save-button {
  box-shadow: 0 10px 25px rgba(124, 58, 237, 0.35);
}

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

.update-panel {
  display: flex;
  gap: 20px;
  align-items: center;
  justify-content: space-between;
  background: linear-gradient(135deg, rgba(124, 58, 237, 0.1), rgba(8, 145, 178, 0.08));
  border: 1px solid var(--divider-color);
  border-radius: 18px;
  padding: 18px 20px;
}

.update-info {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.meta-row {
  display: flex;
  gap: 12px;
  align-items: center;
}

.meta-label {
  font-size: 0.875rem;
  color: var(--text-secondary);
  min-width: 96px;
}

.meta-value {
  font-size: 1rem;
  font-weight: 700;
  color: var(--text-primary);
}

.meta-value.highlight {
  color: #7c3aed;
}

.status-chip {
  display: inline-flex;
  align-items: center;
  padding: 6px 12px;
  border-radius: 999px;
  font-size: 0.875rem;
  font-weight: 600;
  color: #0f172a;
  background: #e2e8f0;
  border: 1px solid rgba(15, 23, 42, 0.08);
}

.status-chip[data-type='success'] {
  background: rgba(34, 197, 94, 0.15);
  color: #166534;
  border-color: rgba(34, 197, 94, 0.3);
}

.status-chip[data-type='warning'] {
  background: rgba(249, 115, 22, 0.15);
  color: #9a3412;
  border-color: rgba(249, 115, 22, 0.3);
}

.status-chip[data-type='info'] {
  background: rgba(8, 145, 178, 0.12);
  color: #0f172a;
  border-color: rgba(8, 145, 178, 0.3);
}

.update-actions {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-shrink: 0;
}

@media (max-width: 720px) {
  .update-panel {
    flex-direction: column;
    align-items: flex-start;
  }

  .update-actions {
    width: 100%;
    flex-wrap: wrap;
  }

  .update-actions .n-button {
    flex: 1;
  }
}
</style>
