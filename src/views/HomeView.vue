<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Search } from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
import type { Trainer } from '../types'
import {
  NCard,
  NInput,
  NInputGroup,
  NButton,
  NIcon,
  NGrid,
  NGridItem,
  NSpace,
  NText,
  NImage,
  NEmpty,
  NEllipsis,
  NSpin,
  NPagination,
  NTag,
  NTooltip,
} from 'naive-ui'

const router = useRouter()
const store = useTrainerStore()

const searchQuery = ref('')
const totalPages = computed(() => store.totalPages)

const handleSearch = () => {
  store.currentPage = 1 // 搜索时重置页码
  store.searchTrainers(searchQuery.value)
}

const handlePageChange = async (page: number) => {
  try {
    store.currentPage = page
    await store.fetchTrainers(page)
    window.scrollTo({ top: 0, behavior: 'smooth' }) // 滚动到顶部
  } catch (error) {
    window.$message?.error('获取数据失败')
  }
}

onMounted(async () => {
  await store.initialize()
})
</script>

<template>
  <div class="home-container">
    <!-- 搜索区域 -->
    <div class="search-section">
      <NCard content-style="padding: 0;">
        <NInputGroup>
          <NInput
            v-model:value="searchQuery"
            placeholder="搜索游戏修改器..."
            @keydown.enter="handleSearch"
            clearable
          >
            <template #prefix>
              <NIcon><Search /></NIcon>
            </template>
          </NInput>
          <n-button type="primary" @click="handleSearch" :loading="store.isLoading" ghost>
            搜索
          </n-button>
        </NInputGroup>
      </NCard>
    </div>

    <!-- 内容区域 -->
    <div class="content-section">
      <NSpin :show="store.isLoading">
        <div v-if="store.error" class="error-message">
          {{ store.error }}
        </div>
        <NEmpty
          v-else-if="!store.trainers.length"
          description="暂无修改器数据"
          class="empty-state"
        />
        <template v-else>
          <NGrid :cols="3" :x-gap="16" :y-gap="16" responsive="screen">
            <NGridItem v-for="trainer in store.trainers" :key="trainer.id">
              <NCard
                class="trainer-card"
                hoverable
                @click="router.push({ name: 'detail', params: { id: trainer.id } })"
              >
                <template #cover>
                  <div class="image-container">
                    <NImage
                      :src="trainer.thumbnail"
                      :alt="trainer.name"
                      class="trainer-image"
                      :preview-disabled="true"
                      fallback-src="/placeholder.png"
                      object-fit="cover"
                    />
                    <div class="image-overlay">
                      <NButton quaternary circle class="detail-button">
                        <template #icon>
                          <NIcon><Search /></NIcon>
                        </template>
                      </NButton>
                    </div>
                  </div>
                </template>

                <div class="card-content">
                  <div class="trainer-header">
                    <NEllipsis :line-clamp="1" class="trainer-name">
                      {{ trainer.name }}
                    </NEllipsis>
                    <NTag size="small" :bordered="false" type="success">
                      {{ trainer.version }}
                    </NTag>
                  </div>

                  <div class="trainer-info">
                    <NTooltip trigger="hover">
                      <template #trigger>
                        <NText depth="3" class="game-version">
                          支持游戏版本: {{ trainer.game_version }}
                        </NText>
                      </template>
                      {{ trainer.game_version }}
                    </NTooltip>
                    <NText depth="3" class="update-time">
                      更新于: {{ new Date(trainer.last_update).toLocaleDateString() }}
                    </NText>
                  </div>
                </div>
              </NCard>
            </NGridItem>
          </NGrid>

          <!-- 分页 -->
          <div class="pagination-container">
            <NPagination
              v-model:page="store.currentPage"
              :page-count="totalPages"
              :on-update:page="handlePageChange"
            />
          </div>
        </template>
      </NSpin>
    </div>
  </div>
</template>

<style scoped>
.home-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.search-section {
  position: sticky;
  top: 0;
  z-index: 1;
  backdrop-filter: blur(10px);
}

.content-section {
  flex: 1;
}

.trainer-card {
  height: 100%;
  transition: all 0.3s ease;
  cursor: pointer;
}

.trainer-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.image-container {
  position: relative;
  width: 100%;
  padding-top: 56.25%; /* 16:9 比例 */
  overflow: hidden;
}

.trainer-image {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  transition: transform 0.3s ease;
}

.image-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.trainer-card:hover .image-overlay {
  opacity: 1;
}

.trainer-card:hover .trainer-image {
  transform: scale(1.05);
}

.detail-button {
  color: #fff !important;
  background: rgba(255, 255, 255, 0.2) !important;
  backdrop-filter: blur(4px);
}

.detail-button:hover {
  background: rgba(255, 255, 255, 0.3) !important;
}

.card-content {
  padding: 16px;
}

.trainer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  margin-bottom: 12px;
}

.trainer-name {
  font-size: 1.1em;
  font-weight: 600;
  flex: 1;
  min-width: 0;
}

.trainer-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.game-version,
.update-time {
  font-size: 0.9em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.pagination-container {
  margin-top: 24px;
  display: flex;
  justify-content: center;
}

.error-message {
  text-align: center;
  color: var(--error-color);
  padding: 20px;
}

.empty-state {
  padding: 40px;
  text-align: center;
}

/* 响应式布局 */
@media (max-width: 1200px) {
  :deep(.n-grid) {
    --cols: 2 !important;
  }
}

@media (max-width: 768px) {
  :deep(.n-grid) {
    --cols: 1 !important;
  }

  .home-container {
    gap: 16px;
  }

  .search-section {
    padding: 8px 0;
  }

  .card-content {
    padding: 12px;
  }
}
</style>
