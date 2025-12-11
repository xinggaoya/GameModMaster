<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { SearchOutline, RefreshOutline, GameControllerOutline } from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import GameCard from '@/components/common/GameCard.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const store = useTrainerStore()

const searchQuery = ref('')
const isSearchFocused = ref(false)

const isLoading = computed(() => store.isLoading)
const totalPages = computed(() => store.totalPages)
const trainers = computed(() => store.trainers)
const errorMessage = computed(() => store.error || '')

const stats = computed(() => [
  { label: t('home.stats.available'), value: trainers.value.length, color: '#7c3aed' },
  { label: t('home.stats.downloaded'), value: store.downloadedTrainers.length, color: '#0891b2' },
  { label: t('home.stats.installed'), value: store.installedTrainers.length, color: '#059669' },
])

const handleSearch = () => {
  if (searchQuery.value.trim()) {
    store.currentPage = 1
    store.searchTrainers(searchQuery.value)
  } else {
    store.fetchTrainers(1)
  }
}

const clearSearch = () => {
  searchQuery.value = ''
  store.fetchTrainers(1)
}

const handleRefresh = async () => {
  if (searchQuery.value.trim()) {
    await store.searchTrainers(searchQuery.value, store.currentPage)
  } else {
    await store.fetchTrainers(store.currentPage)
  }
}

const handlePageChange = async (page: number) => {
  store.currentPage = page
  if (searchQuery.value.trim()) {
    await store.searchTrainers(searchQuery.value, page)
  } else {
    await store.fetchTrainers(page)
  }
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

onMounted(async () => {
  if (trainers.value.length === 0) {
    await store.fetchTrainers(1)
  }
})
</script>

<template>
  <div class="home-view">
    <header class="page-header">
      <div class="header-content">
        <div class="header-text">
          <h1 class="page-title">{{ t('home.title') }}</h1>
          <p class="page-subtitle">{{ t('home.subtitle') }}</p>
        </div>

        <div class="stats-row">
          <div
            v-for="stat in stats"
            :key="stat.label"
            class="stat-badge"
            :style="{ '--stat-color': stat.color }"
          >
            <span class="stat-value">{{ stat.value }}</span>
            <span class="stat-label">{{ stat.label }}</span>
          </div>
        </div>
      </div>
    </header>

    <section class="search-section">
      <div class="search-wrapper" :class="{ focused: isSearchFocused }">
        <NIcon class="search-icon" size="20">
          <SearchOutline />
        </NIcon>
        <input
          v-model="searchQuery"
          type="text"
          class="search-input"
          :placeholder="t('home.searchPlaceholder')"
          @focus="isSearchFocused = true"
          @blur="isSearchFocused = false"
          @keydown.enter="handleSearch"
        />
        <button
          v-if="searchQuery"
          class="clear-btn"
          @click="clearSearch"
        >
          {{ t('common.clear') }}
        </button>
        <button
          class="search-btn"
          @click="handleSearch"
          :disabled="isLoading"
        >
          {{ t('common.search') }}
        </button>
      </div>

      <button class="refresh-btn" @click="handleRefresh" :disabled="isLoading">
        <NIcon size="18" :class="{ spinning: isLoading }">
          <RefreshOutline />
        </NIcon>
      </button>
    </section>

    <div class="results-info" v-if="!isLoading && trainers.length > 0">
      <span class="results-count">
        {{
          searchQuery
            ? t('home.results.query', { query: searchQuery })
            : t('home.results.all')
        }}
        - {{ t('home.results.count', { count: trainers.length }) }}
      </span>
    </div>

    <div v-if="isLoading" class="loading-state">
      <NSpin size="large" />
      <p class="loading-text">{{ t('common.loading') }}</p>
    </div>

    <div v-else-if="errorMessage" class="error-state">
      <NIcon size="48" color="#ef4444">
        <GameControllerOutline />
      </NIcon>
      <h3>{{ t('common.errorTitle') }}</h3>
      <p>{{ errorMessage }}</p>
      <NButton @click="handleRefresh" type="primary">
        {{ t('common.retry') }}
      </NButton>
    </div>

    <div v-else-if="trainers.length === 0" class="empty-state">
      <NIcon size="64" color="#94a3b8">
        <SearchOutline />
      </NIcon>
      <h3>{{ t('home.empty.title') }}</h3>
      <p v-if="searchQuery">
        {{ t('home.empty.withQuery', { query: searchQuery }) }}
      </p>
      <p v-else>{{ t('home.empty.withoutQuery') }}</p>
      <NButton v-if="searchQuery" @click="clearSearch" type="primary">
        {{ t('common.clear') }}
      </NButton>
    </div>

    <section v-else class="trainers-grid">
      <GameCard
        v-for="trainer in trainers"
        :key="trainer.id"
        :trainer="trainer"
      />
    </section>

    <div class="pagination" v-if="totalPages > 1 && !isLoading">
      <NPagination
        v-model:page="store.currentPage"
        :page-count="totalPages"
        :page-sizes="[12, 24, 36]"
        show-size-picker
        @update:page="handlePageChange"
      />
    </div>
  </div>
</template>

<style scoped>
.home-view {
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 24px;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 24px;
  flex-wrap: wrap;
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

.stats-row {
  display: flex;
  gap: 12px;
}

.stat-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.stat-value {
  font-size: 1.25rem;
  font-weight: 800;
  color: var(--stat-color);
}

.stat-label {
  font-size: 0.813rem;
  font-weight: 600;
  color: #64748b;
}

.search-section {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
}

.search-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 0 16px;
  height: 52px;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(20px);
  border: 2px solid transparent;
  border-radius: 16px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);
  transition: all 0.2s ease;
}

.search-wrapper.focused {
  border-color: #7c3aed;
  box-shadow: 0 4px 20px rgba(124, 58, 237, 0.15);
}

.search-icon {
  color: #94a3b8;
  flex-shrink: 0;
}

.search-wrapper.focused .search-icon {
  color: #7c3aed;
}

.search-input {
  flex: 1;
  height: 100%;
  border: none;
  background: transparent;
  font-size: 0.938rem;
  color: #1f2937;
  outline: none;
}

.search-input::placeholder {
  color: #94a3b8;
}

.clear-btn {
  padding: 6px 12px;
  border: none;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 8px;
  font-size: 0.813rem;
  font-weight: 600;
  color: #64748b;
  cursor: pointer;
  transition: all 0.15s ease;
}

.clear-btn:hover {
  background: rgba(0, 0, 0, 0.1);
}

.search-btn {
  padding: 8px 20px;
  border: none;
  background: linear-gradient(135deg, #7c3aed 0%, #6d28d9 100%);
  border-radius: 10px;
  font-size: 0.875rem;
  font-weight: 600;
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
}

.search-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(124, 58, 237, 0.3);
}

.search-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.refresh-btn {
  width: 52px;
  height: 52px;
  border: none;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);
  color: #64748b;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.refresh-btn:hover:not(:disabled) {
  color: #7c3aed;
  transform: translateY(-1px);
}

.refresh-btn:disabled {
  cursor: not-allowed;
}

.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.results-info {
  margin-bottom: 20px;
}

.results-count {
  font-size: 0.875rem;
  color: #64748b;
  font-weight: 500;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  text-align: center;
  gap: 16px;
}

.loading-text {
  color: #64748b;
  font-size: 1rem;
}

.error-state h3,
.empty-state h3 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 700;
  color: #1f2937;
}

.error-state p,
.empty-state p {
  margin: 0;
  color: #64748b;
}

.trainers-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.pagination {
  display: flex;
  justify-content: center;
  margin-top: 32px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 16px;
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: flex-start;
  }

  .stats-row {
    width: 100%;
    justify-content: space-between;
  }

  .stat-badge {
    padding: 6px 12px;
  }

  .stat-value {
    font-size: 1rem;
  }

  .trainers-grid {
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  }
}
</style>
