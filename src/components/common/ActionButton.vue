<template>
  <NButton :size="size" :quaternary="quaternary" :circle="circle" :type="type" v-bind="$attrs">
    <template #icon v-if="icon">
      <NIcon>
        <component :is="icon" />
      </NIcon>
    </template>
    <slot></slot>
  </NButton>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'
import { NButton, NIcon } from 'naive-ui'
import type { Component } from 'vue'

// 引入Naive UI的类型
type ButtonSize = 'tiny' | 'small' | 'medium' | 'large'
type ButtonType = 'default' | 'tertiary' | 'primary' | 'info' | 'success' | 'warning' | 'error'

defineProps({
  // 按钮尺寸
  size: {
    type: String as () => ButtonSize,
    default: 'medium',
  },
  // 是否为四级按钮（最低强调）
  quaternary: {
    type: Boolean,
    default: false,
  },
  // 是否为圆形按钮
  circle: {
    type: Boolean,
    default: false,
  },
  // 按钮类型
  type: {
    type: String as () => ButtonType,
    default: 'default',
  },
  // 图标组件
  icon: {
    type: Object as () => Component,
    default: null,
  },
})
</script>

<style scoped>
:deep(.n-button) {
  transition: all var(--transition-duration) var(--transition-timing);
}

:deep(.n-button:hover) {
  transform: translateY(-2px);
}

:deep(.n-button:active) {
  transform: translateY(0);
}
</style>
