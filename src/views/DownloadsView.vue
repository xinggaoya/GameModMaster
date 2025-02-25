<template>
  <div class="downloads-container">
    <NCard title="已下载的修改器">
      <StateDisplay
        :loading="store.isLoading"
        :empty="!store.downloadedTrainers.length"
        empty-text="暂无已下载的修改器"
      >
        <!-- 加载中状态的自定义显示 -->
        <template #loading>
          <TrainerSkeleton type="list" :count="3" />
        </template>

        <!-- 正常数据展示 -->
        <div class="downloads-list">
          <TrainerCard
            v-for="trainer in store.downloadedTrainers"
            :key="trainer.id"
            :trainer="trainer"
            compact
            class="trainer-list-item"
          >
            <!-- 额外信息插槽 -->
            <template #extra-info>
              <div class="launch-info" v-if="getInstallInfo(trainer.id)?.last_launch_time">
                <NText depth="3">
                  上次启动: {{ formatDate(getInstallInfo(trainer.id)?.last_launch_time) }}
                </NText>
              </div>
            </template>

            <!-- 操作按钮插槽 -->
            <template #actions>
              <NSpace align="center" :size="8">
                <ActionButton
                  size="small"
                  quaternary
                  circle
                  type="info"
                  :icon="InformationCircleOutline"
                  @click.stop="router.push({ name: 'detail', params: { id: trainer.id } })"
                />
                <ActionButton
                  size="small"
                  quaternary
                  circle
                  type="success"
                  :icon="PlayOutline"
                  @click.stop="handleLaunch(trainer)"
                />
                <ActionButton
                  size="small"
                  quaternary
                  circle
                  type="error"
                  :icon="TrashBinOutline"
                  @click.stop="handleDelete(trainer)"
                />
              </NSpace>
            </template>
          </TrainerCard>
        </div>
      </StateDisplay>
    </NCard>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTrainerStore } from '../stores/trainer'
import type { Trainer } from '../types'
import { NCard, NSpace, NText } from 'naive-ui'
import { InformationCircleOutline, PlayOutline, TrashBinOutline } from '@vicons/ionicons5'

// 导入自定义组件
import TrainerCard from '@/components/common/TrainerCard.vue'
import TrainerSkeleton from '@/components/common/TrainerSkeleton.vue'
import ActionButton from '@/components/common/ActionButton.vue'
import StateDisplay from '@/components/common/StateDisplay.vue'

const router = useRouter()
const store = useTrainerStore()

onMounted(() => {
  store.initialize()
})

const handleLaunch = async (trainer: Trainer) => {
  try {
    await store.launchTrainer(trainer.id)
    window.$message?.success('启动成功')
  } catch (error) {
    // 错误处理在store中已完成
  }
}

const handleDelete = async (trainer: Trainer) => {
  try {
    await store.deleteTrainer(trainer.id)
    window.$message?.success('删除成功')
  } catch (error) {
    // 错误处理在store中已完成
  }
}

const getInstallInfo = (trainerId: string) => {
  return store.installedTrainers.find((t) => t.id === trainerId)
}

const formatDate = (dateString: string | null | undefined) => {
  if (!dateString) return '未知'
  return new Date(dateString).toLocaleString()
}
</script>

<style scoped>
.downloads-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-small);
}

.trainer-list-item {
  border: 1px solid var(--divider-color);
  transition: all var(--transition-duration) var(--transition-timing);
}

.trainer-list-item:hover {
  transform: translateX(2px);
  box-shadow: 0 2px 8px var(--shadow-color);
}

.launch-info {
  font-size: var(--font-size-mini);
}

/* 响应式布局 */
@media (max-width: 640px) {
  .downloads-container {
    padding: var(--spacing-small);
  }
}
</style>
