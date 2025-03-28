<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'
import WeatherToolbar from './components/WeatherToolbar.vue'
import WeatherWindow from './components/WeatherWindow.vue'

const greetMsg = ref('')
const name = ref('')

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<template>
  <WeatherWindow>
    <WeatherToolbar />
    <div class="container">
      <h1 v-if="greetMsg">
        {{ greetMsg }}
      </h1>
      <input v-model="name">
      <button type="button" @click="greet">
        Greet
      </button>
    </div>
  </WeatherWindow>
</template>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
}
</style>
