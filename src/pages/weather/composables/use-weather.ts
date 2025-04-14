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
    query: async () => {
      const response = await invoke<WeatherData>('fetch_weather', {
        city: location.value,
      })

      if (response.name) {
        location.value = response.name
      }

      return response
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
