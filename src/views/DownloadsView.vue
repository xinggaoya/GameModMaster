<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { DownloadOutline, SearchOutline, FolderOpenOutline, GameControllerOutline } from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import GameCard from '@/components/common/GameCard.vue'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const store = useTrainerStore()
const message = useMessage()
const { t } = useI18n()

const searchQuery = ref('')

const filteredTrainers = computed(() => {
  let result = [...store.downloadedTrainers]

  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(
      (t) =>
        t.name.toLowerCase().includes(query) ||
        t.game_version.toLowerCase().includes(query),
    )
  }

  return result
})

const openDownloadFolder = async () => {
  try {
    await invoke('open_download_folder')
  } catch (error) {
    console.error(error)
    message.error(t('downloads.errors.openFolder'))
  }
}

onMounted(() => {
  if (store.downloadedTrainers.length === 0) {
    store.initialize()
  }
})
</script>

<template>
  <div class="downloads-view">
    <header class="page-header">
      <div class="header-text">
        <h1 class="page-title">{{ t('downloads.title') }}</h1>
        <p class="page-subtitle">{{ t('downloads.subtitle') }}</p>
      </div>

      <div class="header-actions">
        <NButton @click="openDownloadFolder" quaternary>
          <template #icon>
            <NIcon><FolderOpenOutline /></NIcon>
          </template>
          {{ t('common.openFolder') }}
        </NButton>
      </div>
    </header>

    <div class="stats-bar">
      <div class="stat-item">
        <span class="stat-value">{{ store.downloadedTrainers.length }}</span>
        <span class="stat-label">{{ t('downloads.stats.downloaded') }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{{ store.installedTrainers.length }}</span>
        <span class="stat-label">{{ t('downloads.stats.installed') }}</span>
      </div>
    </div>

    <div class="search-bar" v-if="store.downloadedTrainers.length > 0">
      <NIcon class="search-icon" size="18">
        <SearchOutline />
      </NIcon>
      <input
        v-model="searchQuery"
        type="text"
        class="search-input"
        :placeholder="t('downloads.searchPlaceholder')"
      />
    </div>

    <div v-if="store.isLoading" class="loading-state">
      <NSpin size="large" />
    </div>

    <div v-else-if="store.downloadedTrainers.length === 0" class="empty-state">
      <NIcon size="64" color="#94a3b8">
        <DownloadOutline />
      </NIcon>
      <h3>{{ t('downloads.empty.title') }}</h3>
      <p>{{ t('downloads.empty.subtitle') }}</p>
      <NButton @click="router.push('/')" type="primary" size="large">
        <template #icon>
          <NIcon><GameControllerOutline /></NIcon>
        </template>
        {{ t('common.startExploring') }}
      </NButton>
    </div>

    <div v-else-if="filteredTrainers.length === 0" class="empty-state">
      <NIcon size="48" color="#94a3b8">
        <SearchOutline />
      </NIcon>
      <h3>{{ t('downloads.empty.searchTitle') }}</h3>
      <p>{{ t('downloads.empty.searchSubtitle', { query: searchQuery }) }}</p>
      <NButton @click="searchQuery = ''" size="large">
        {{ t('common.clear') }}
      </NButton>
    </div>

    <div v-else class="trainers-grid">
      <GameCard
        v-for="trainer in filteredTrainers"
        :key="trainer.id"
        :trainer="trainer"
        showButtons="downloaded"
      />
    </div>
  </div>
</template>

<style scoped>
.downloads-view {
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
}

.page-title {
  font-size: 2rem;
  font-weight: 800;
  margin: 0 0 6px 0;
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

.stats-bar {
  display: flex;
  gap: 24px;
  margin-bottom: 20px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 12px;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 800;
  color: #7c3aed;
}

.stat-label {
  font-size: 0.875rem;
  color: #64748b;
  font-weight: 600;
}

.search-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 0 16px;
  height: 48px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 14px;
  margin-bottom: 24px;
}

.search-icon {
  color: #94a3b8;
}

.search-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 0.938rem;
  outline: none;
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  text-align: center;
  gap: 16px;
}

.empty-state h3 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 700;
  color: #1f2937;
}

.empty-state p {
  margin: 0;
  color: #64748b;
}

.trainers-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}
</style>
