<template>
  <h1 v-if="weatherResponse">
    {{ weatherResponse.name }}
    <sup>{{ weatherResponse.main.temp }} °C</sup>
  </h1>
  <input v-model="city" autofocus>
  <button type="button" @click="fetchWeather">
    Get weather
  </button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { onMounted, ref } from 'vue'

const city = ref('Irkutsk')
const weatherResponse = ref()

async function fetchWeather() {
  if (!city.value) return
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  weatherResponse.value = await invoke('fetch_weather', {
    city: city.value,
    lang: 'ru',
    apiKey: '4b7f29a8e15af3ec8d463f83ce5dd419',
  })
}

listen('weather_update', ({ payload }) => {
  console.log(payload)
  weatherResponse.value = payload
})

onMounted(fetchWeather)
</script>
