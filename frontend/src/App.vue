<template>
  <div class="app-root">
    <NavBar v-if="auth.isLoggedIn()" />
    <main class="main-content" :class="{ 'with-nav': auth.isLoggedIn() }">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import NavBar from '@/components/NavBar.vue'

const auth = useAuthStore()
</script>
