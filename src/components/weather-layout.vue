<template>
  <div class="window-layout">
    <slot />
    <div class="window-layout__content">
      <RouterView />
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<style scoped lang="scss">
.window-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;

  &__content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    gap: 16px;
    height: 100%;
  }
}
</style>
