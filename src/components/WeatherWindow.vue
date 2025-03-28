<template>
  <div class="window">
    <slot />
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="scss">
.window {
    display: flex;
    flex-direction: column;
    height: 100vh;
}
</style>
