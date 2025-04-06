<script setup lang="ts">
import { NCard, NIcon, NStatistic, NSkeleton } from 'naive-ui'
import type { Component } from 'vue'

const props = defineProps<{
  title: string
  value: string | number
  prefix?: string
  suffix?: string
  icon?: Component
  color?: string
  loading?: boolean
}>()
</script>

<template>
  <NCard class="status-card" :class="{ 'is-loading': loading }">
    <NSkeleton v-if="loading" text style="width: 100%" />
    <template v-else>
      <div class="card-icon" :style="{ backgroundColor: color + '15' }">
        <NIcon :color="color" size="24">
          <component :is="icon" />
        </NIcon>
      </div>

      <div class="card-content">
        <NStatistic :value="value" :prefix="prefix" :suffix="suffix" />
        <div class="card-title">{{ title }}</div>
      </div>
    </template>
  </NCard>
</template>

<style scoped>
.status-card {
  transition:
    transform var(--transition-fast),
    box-shadow var(--transition-fast);
  background-color: white;
}

:deep(.dark) .status-card,
.dark .status-card {
  background-color: #1e1e1e;
}

.status-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-md);
}

.status-card.is-loading {
  cursor: default;
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

.card-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  margin-bottom: 12px;
}

.card-title {
  font-size: 0.85rem;
  color: var(--text-tertiary);
  margin-top: 4px;
}

:deep(.dark) .card-title,
.dark .card-title {
  color: #aaaaaa;
}

:deep(.n-statistic-value) {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary);
  font-family: var(--font-gaming);
}

:deep(.dark) :deep(.n-statistic-value),
.dark :deep(.n-statistic-value) {
  color: #e1e1e1;
}
</style>
