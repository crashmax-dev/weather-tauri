import { LazyStore } from '@tauri-apps/plugin-store'

export const store = new LazyStore('config.json', {
  autoSave: false,
})
