import { PiniaColada } from '@pinia/colada'
import { createPinia } from 'pinia'
import { createApp } from 'vue'
import App from './app.vue'
import { router } from './router'
import './main.css'

const app = createApp(App)
const pinia = createPinia()

app.use(router)
app.use(pinia)
app.use(PiniaColada, {
  pinia,
  queryOptions: {
    refetchOnReconnect: true,
    refetchOnWindowFocus: false,
  },
})

app.mount('#app')
