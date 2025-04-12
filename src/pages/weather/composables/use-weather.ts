import { useQuery, useQueryCache } from '@pinia/colada'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { acceptHMRUpdate, defineStore } from 'pinia'
import { ref } from 'vue'

export const WEATHER_QUERY_KEY = 'weather'

interface WeatherData {
  name: string
  clouds: {
    all: number
  }
  main: {
    temp: number
    humidity: number
    pressure: number
  }
  weather: {
    description: string
    icon: string
  }[]
  wind: {
    speed: number
  }
}

export const useWeather = defineStore('weather/use-weather', () => {
  const location = ref('Moscow')
  const queryCache = useQueryCache()

  const {
    isLoading,
    data: response,
    refetch: fetchWeather,
  } = useQuery({
    key: [WEATHER_QUERY_KEY],
    query: () => {
      return invoke<WeatherData>('fetch_weather', {
        city: location.value,
        lang: 'ru',
        apiKey: '4b7f29a8e15af3ec8d463f83ce5dd419',
      })
    },
  })

  listen<WeatherData>('weather_update', ({ payload }) => {
    queryCache.setQueryData([WEATHER_QUERY_KEY], payload)
  })

  return {
    isLoading,
    response,
    location,
    fetchWeather,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useWeather, import.meta.hot))
}
