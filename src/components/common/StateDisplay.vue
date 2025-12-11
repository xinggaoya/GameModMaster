<template>
  <div class="state-display">
    <div v-if="loading" class="loading-state">
      <slot name="loading">
        <div class="default-loading">
          <NSpin :size="spinSize" />
          <p v-if="displayLoadingText" class="loading-text">{{ displayLoadingText }}</p>
        </div>
      </slot>
    </div>

    <div v-else-if="error" class="error-state">
      <slot name="error">
        <NResult status="error" :title="displayErrorTitle" :description="error">
          <template #footer>
            <NButton @click="retryHandler" v-if="hasRetry">{{ t('common.retry') }}</NButton>
          </template>
        </NResult>
      </slot>
    </div>

    <div v-else-if="empty" class="empty-state">
      <slot name="empty">
        <NEmpty :description="displayEmptyText" />
      </slot>
    </div>

    <slot v-else></slot>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, computed, type PropType } from 'vue'
import { NSpin, NEmpty, NResult, NButton } from 'naive-ui'
import { useI18n } from 'vue-i18n'

const props = defineProps({
  loading: {
    type: Boolean,
    default: false,
  },
  loadingText: {
    type: String,
    default: '',
  },
  spinSize: {
    type: Number,
    default: 48,
  },
  error: {
    type: [String, null] as PropType<string | null>,
    default: '',
  },
  errorTitle: {
    type: String,
    default: '',
  },
  hasRetry: {
    type: Boolean,
    default: true,
  },
  empty: {
    type: Boolean,
    default: false,
  },
  emptyText: {
    type: String,
    default: '',
  },
})

const emit = defineEmits(['retry'])
const { t } = useI18n()

const displayLoadingText = computed(() => props.loadingText || t('common.loading'))
const displayErrorTitle = computed(() => props.errorTitle || t('common.errorTitle'))
const displayEmptyText = computed(() => props.emptyText || t('common.noData'))

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
