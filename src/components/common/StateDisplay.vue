<template>
  <div class="state-display">
    <!-- 加载中状态 -->
    <div v-if="loading" class="loading-state">
      <slot name="loading">
        <div class="default-loading">
          <NSpin :size="spinSize" />
          <p v-if="loadingText" class="loading-text">{{ loadingText }}</p>
        </div>
      </slot>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <slot name="error">
        <NResult status="error" :title="errorTitle" :description="error">
          <template #footer>
            <NButton @click="retryHandler" v-if="hasRetry">重试</NButton>
          </template>
        </NResult>
      </slot>
    </div>

    <!-- 空数据状态 -->
    <div v-else-if="empty" class="empty-state">
      <slot name="empty">
        <NEmpty :description="emptyText" />
      </slot>
    </div>

    <!-- 正常数据状态 -->
    <slot v-else></slot>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'
import { NSpin, NEmpty, NResult, NButton } from 'naive-ui'

const props = defineProps({
  // 是否加载中
  loading: {
    type: Boolean,
    default: false,
  },
  // 加载中提示文字
  loadingText: {
    type: String,
    default: '加载中...',
  },
  // 加载图标大小
  spinSize: {
    type: Number,
    default: 48,
  },
  // 错误信息 - 接受null或字符串
  error: {
    type: [String, null],
    default: '',
  },
  // 错误标题
  errorTitle: {
    type: String,
    default: '发生错误',
  },
  // 是否显示重试按钮
  hasRetry: {
    type: Boolean,
    default: true,
  },
  // 是否为空数据
  empty: {
    type: Boolean,
    default: false,
  },
  // 空数据提示文字
  emptyText: {
    type: String,
    default: '暂无数据',
  },
})

const emit = defineEmits(['retry'])

// 重试处理函数
const retryHandler = () => {
  emit('retry')
}
</script>

<style scoped>
.state-display {
  width: 100%;
}

.loading-state,
.error-state,
.empty-state {
  padding: var(--spacing-large);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  min-height: 200px;
}

.default-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-base);
}

.loading-text {
  color: var(--text-color-secondary);
  font-size: var(--font-size-base);
}
</style>
