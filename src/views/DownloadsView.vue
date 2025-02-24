<template>
  <div class="downloads-container">
    <NCard>
      <template #header>
        <div class="header-container">
          <h2>已下载的修改器</h2>
        </div>
      </template>

      <NEmpty v-if="!store.downloadedTrainers.length" description="暂无已下载的修改器" />

      <NList v-else>
        <NListItem v-for="trainer in store.downloadedTrainers" :key="trainer.id">
          <NThing>
            <template #avatar>
              <NImage
                :src="trainer.thumbnail"
                :alt="trainer.name"
                width="100"
                height="100"
                object-fit="cover"
              />
            </template>
            <template #header>
              {{ trainer.name }}
            </template>
            <template #description>
              <NSpace vertical>
                <NText>版本: {{ trainer.version }}</NText>
                <NText>游戏版本: {{ trainer.game_version }}</NText>
              </NSpace>
            </template>
            <template #footer>
              <NSpace>
                <NButton
                  type="primary"
                  @click="router.push({ name: 'detail', params: { id: trainer.id } })"
                >
                  详情
                </NButton>
                <NButton type="info" @click="handleLaunch(trainer)">启动</NButton>
                <NButton type="error" @click="handleDelete(trainer)">删除</NButton>
              </NSpace>
            </template>
          </NThing>
        </NListItem>
      </NList>
    </NCard>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTrainerStore } from '../stores/trainer'
import type { Trainer } from '../types'
import { NCard, NEmpty, NList, NListItem, NThing, NImage, NSpace, NText, NButton } from 'naive-ui'

const router = useRouter()
const store = useTrainerStore()

onMounted(() => {
  store.initialize()
})

const handleLaunch = async (trainer: Trainer) => {
  try {
    await store.launchTrainer(trainer.id)
  } catch (error) {
    window.$message?.error('启动失败')
  }
}

const handleDelete = async (trainer: Trainer) => {
  try {
    await store.deleteTrainer(trainer.id)
  } catch (error) {
    window.$message?.error('删除失败')
  }
}
</script>

<style scoped>
.downloads-container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.header-container {
  margin-bottom: 20px;
}
</style>
