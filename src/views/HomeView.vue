<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Search } from '@vicons/ionicons5'
import { useTrainerStore } from '../stores/trainer'
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

// 导入自定义组件
import TrainerCard from '@/components/common/TrainerCard.vue'
import TrainerSkeleton from '@/components/common/TrainerSkeleton.vue'
import StateDisplay from '@/components/common/StateDisplay.vue'

const router = useRouter()
const store = useTrainerStore()

const searchQuery = ref('')
const totalPages = computed(() => store.totalPages)

// 计算错误信息，确保返回字符串
const errorMessage = computed(() => store.error || '')

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
    // 错误处理由store处理
  }
}

const handleRetry = async () => {
  await store.fetchTrainers(store.currentPage)
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
      <StateDisplay
        :loading="store.isLoading"
        :error="errorMessage"
        :empty="!store.trainers.length"
        empty-text="暂无修改器数据"
        @retry="handleRetry"
      >
        <!-- 加载中状态的自定义显示 -->
        <template #loading>
          <NGrid :cols="3" :x-gap="16" :y-gap="16" responsive="screen">
            <NGridItem v-for="i in 9" :key="i">
              <TrainerSkeleton type="card" />
            </NGridItem>
          </NGrid>
        </template>

        <!-- 正常数据展示 -->
        <NGrid :cols="3" :x-gap="16" :y-gap="16" responsive="screen">
          <NGridItem v-for="trainer in store.trainers" :key="trainer.id">
            <TrainerCard
              :trainer="trainer"
              @click="router.push({ name: 'detail', params: { id: trainer.id } })"
            />
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
      </StateDisplay>
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
