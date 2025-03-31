import { createApp } from 'vue'
import App from './app.vue'
import { router } from './router'
import './main.css'

const app = createApp(App)

app.use(router)
app.mount('#app')
