<template>
  <div v-if="weather.response" class="weather">
    <h1 class="weather-title">
      {{ weather.response.name }}
    </h1>
    <h2 class="weather-subtitle">
      Обновлено в 19:29
    </h2>

    <div class="weather-container">
      <div>
        <img
          :src="`http://openweathermap.org/img/wn/${weather.response.weather[0].icon}@4x.png`"
          alt="Weather icon"
          :style="{ height: '150px', width: '150px' }"
        >
        <p class="weather-clouds">
          {{ weather.response.weather[0].description }}
        </p>
      </div>

      <div>
        <p class="weather-temp">
          {{ weather.response.main.temp }}°C
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
import { useWeather } from './composables/use-weather'

const weather = useWeather()
</script>

<style scoped lang="scss">
.weather {
  height: inherit;
  display: flex;
  flex-direction: column;
  justify-content: space-between;

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
    justify-content: space-between;
    align-items: center;
  }

  &-clouds {
    text-transform: capitalize;
    text-align: center;
    font-size: 11px;
    font-weight: 600;
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
