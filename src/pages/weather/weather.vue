<template>
  <h1 v-if="greetMsg">
    {{ greetMsg }}
  </h1>
  <input v-model="name" autofocus>
  <button type="button" @click="greet">
    Greet
  </button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

const greetMsg = ref('')
const name = ref('')

async function greet() {
  if (!name.value) return
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke('greet', { name: name.value })
}
</script>
