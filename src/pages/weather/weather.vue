<template>
  <div v-if="weather.response" class="weather">
    <form @submit.prevent="blurInput">
      <input
        ref="inputRef"
        v-model="weather.location"
        class="weather-input"
      >
    </form>
    <h2 class="weather-subtitle">
      Обновлено в 19:29
    </h2>

    <div class="weather-container">
      <div :style="{ position: 'relative' }">
        <component
          :is="WEATHER_ICONS[weather.response.weather[0].icon]"
          class="weather-icon"
        />
        <p class="weather-clouds">
          {{ weather.response.weather[0].description }}
        </p>
      </div>

      <div>
        <p class="weather-temp">
          {{ formatNumber(weather.response.main.temp) }}°C
        </p>
      </div>
    </div>

    <div class="weather-container">
      <div>
        <p class="weather-info">
          Облачность:
          {{ weather.response.clouds.all }}%
        </p>
        <p class="weather-info">
          Влажность:
          {{ weather.response.main.humidity }}%
        </p>
      </div>

      <div>
        <p class="weather-info">
          Ветер:
          {{ weather.response.wind.speed }}м/c
        </p>
        <p class="weather-info">
          Давление:
          {{ Math.round(weather.response.main.pressure * 0.75) }}mmHg
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatNumber } from '@/utils/format-number'
import { useMagicKeys } from '@vueuse/core'
import { useTemplateRef, watch } from 'vue'
import { useWeather } from './composables/use-weather'
import { WEATHER_ICONS } from './constants/weather-icons'

const weather = useWeather()

const { escape } = useMagicKeys()
const inputRef = useTemplateRef('inputRef')

watch(escape, () => {
  if (!escape.value) return
  if (document.activeElement === inputRef.value) {
    blurInput()
  } else {
    focusInput()
  }
})

function blurInput() {
  inputRef.value?.blur()
  weather.fetchWeather()
}

function focusInput() {
  inputRef.value?.focus()
  inputRef.value?.select()
}

// fetch('https://ipwhois.app/json/?objects=success,city&lang=ru')
</script>

<style scoped lang="scss">
.weather {
  height: inherit;
  display: flex;
  flex-direction: column;
  justify-content: space-between;

  &-input {
    width: 100%;
    border: none;
    outline: none;
    font-weight: 600;
    font-size: 38px;
    line-height: 38px;
    text-align: center;
  }

  &-icon {
    height: 120px;
    width: 120px;
  }

  &-title {
    font-size: 38px;
    line-height: 38px;
    text-align: center;
  }

  &-subtitle {
    font-size: 11px;
    font-weight: 600;
    text-align: center;
  }

  &-container {
    display: flex;
    justify-content: space-around;
    align-items: center;
  }

  &-clouds {
    text-transform: capitalize;
    text-align: center;
    font-size: 11px;
    font-weight: 600;
    position: absolute;
    bottom: 12px;
    width: 100%;
  }

  &-temp {
    font-size: 48px;
    font-weight: 600;
  }

  &-info {
    text-align: center;
    font-size: 14px;
    font-weight: 600;
  }
}
</style>
