<template>
  <div class="toolbar" data-tauri-drag-region>
    <div class="toolbar-tabs">
      <button
        v-for="tab of TOOLBAR_TABS"
        :key="tab.name"
        class="toolbar-button"
        :class="{ active: activeRoute === tab.name }"
        @click="openRoute(tab.name)"
      >
        <component
          :is="tab.icon"
          class="toolbar-tabs__icon"
          :class="{ active: activeRoute === tab.name }"
        />
      </button>
    </div>

    <div class="toolbar-controls">
      <button class="toolbar-button" @click="minimize">
        <MinimizeIcon class="toolbar-controls__icon" />
      </button>
      <button class="toolbar-button close" @click="close">
        <CloseIcon class="toolbar-controls__icon" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouteName } from '@/router'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import QuestionMarkIcon from '~icons/custom/question-mark'
import SettingsIcon from '~icons/custom/settings'
import WeatherIcon from '~icons/custom/weather'
import CloseIcon from '~icons/custom/window-close'
import MinimizeIcon from '~icons/custom/window-minimize'
import type { RouteNameType } from '@/router'

const router = useRouter()
const appWindow = getCurrentWindow()
const activeRoute = computed(() => router.currentRoute.value.name)

const TOOLBAR_TABS = [
  { name: RouteName.Weather, icon: WeatherIcon },
  { name: RouteName.Settings, icon: SettingsIcon },
  { name: RouteName.About, icon: QuestionMarkIcon },
]

function minimize() {
  appWindow.minimize()
}

function close() {
  appWindow.close()
}

function openRoute(name: RouteNameType) {
  router.push({ name })
}
</script>

<style scoped lang="scss">
.toolbar {
  display: flex;
  align-items: center;
  height: 26px;
  flex-shrink: 0;
  background: #F0F0F0;
  -webkit-app-region: drag;

  &-tabs {
    display: flex;
    height: 100%;

    &__icon {
      height: 14px;
      width: 14px;
      color: var(--secondary-color);

      &.active {
        color: var(--primary-color);
      }
    }
  }

  &-controls {
    margin-left: auto;
    display: flex;
    height: 100%;

    &__icon {
      height: 18px;
      width: 18px;
      color: var(--secondary-color);
    }
  }

  &-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 33px;
    background: none;
    border: none;
    font-size: 18px;
    cursor: pointer;
    -webkit-app-region: no-drag;

    &.close:hover {
      background: rgba(255, 0, 0, 0.8);

      .toolbar-controls__icon {
        color: #fff;
      }
    }

    &.active, &:hover {
      background: #e3e3e3;
    }
  }
}
</style>
