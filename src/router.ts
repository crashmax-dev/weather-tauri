import {
  createMemoryHistory,
  createRouter,
} from 'vue-router'

export const RouteName = {
  Weather: 'weather',
  Settings: 'settings',
  About: 'about',
} as const

export type RouteNameType = typeof RouteName[keyof typeof RouteName]

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      name: RouteName.Weather,
      path: '/',
      component: () => import('@/pages/weather/weather.vue'),
    },
    {
      name: RouteName.Settings,
      path: '/settings',
      component: () => import('@/pages/settings/settings.vue'),
    },
    {
      name: RouteName.About,
      path: '/about',
      component: () => import('@/pages/about/about.vue'),
    },
  ],
})
