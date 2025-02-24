import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import {
  create,
  NButton,
  NCard,
  NDescriptions,
  NDescriptionsItem,
  NDialog,
  NEmpty,
  NGrid,
  NGridItem,
  NH2,
  NH3,
  NIcon,
  NImage,
  NInput,
  NInputGroup,
  NLayout,
  NLayoutContent,
  NLayoutFooter,
  NLayoutHeader,
  NList,
  NListItem,
  NLoadingBarProvider,
  NMenu,
  NMessageProvider,
  NNotificationProvider,
  NPagination,
  NSpace,
  NSpin,
  NText,
  NThing,
} from 'naive-ui'

import App from './App.vue'
import router from './router'

const naive = create({
  components: [
    NButton,
    NCard,
    NDescriptions,
    NDescriptionsItem,
    NDialog,
    NEmpty,
    NGrid,
    NGridItem,
    NH2,
    NH3,
    NIcon,
    NImage,
    NInput,
    NInputGroup,
    NLayout,
    NLayoutContent,
    NLayoutFooter,
    NLayoutHeader,
    NList,
    NListItem,
    NLoadingBarProvider,
    NMenu,
    NMessageProvider,
    NNotificationProvider,
    NPagination,
    NSpace,
    NSpin,
    NText,
    NThing,
  ],
})

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(naive)

app.mount('#app')
