<template>
  <div class="toolbar" data-tauri-drag-region>
    <div class="toolbar-tabs">
      <button
        class="toolbar-button"
        :class="{ active: activeRoute === RouteName.Weather }"
        @click="openRoute(RouteName.Weather)"
      >
        <WeatherIcon class="toolbar-tabs__icon" />
      </button>

      <button
        class="toolbar-button"
        :class="{ active: activeRoute === RouteName.Settings }"
        @click="openRoute(RouteName.Settings)"
      >
        <SettingsIcon class="toolbar-tabs__icon" />
      </button>

      <button
        class="toolbar-button"
        :class="{ active: activeRoute === RouteName.About }"
        @click="openRoute(RouteName.About)"
      >
        <QuestionMarkIcon class="toolbar-tabs__icon" />
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
  background: #F0F0F0;
  -webkit-app-region: drag;

  &-tabs {
    display: flex;
    height: 100%;

    &__icon {
      height: 14px;
      width: 14px;
      color: #696969;
    }
  }

  &-controls {
    margin-left: auto;
    display: flex;
    height: 100%;

    &__icon {
      height: 18px;
      width: 18px;
      color: #696969;
    }
  }

  &-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 33px;
    background: none;
    border: none;
    color: white;
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
