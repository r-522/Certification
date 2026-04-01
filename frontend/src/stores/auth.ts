import { defineStore } from 'pinia'
import { ref } from 'vue'
import { me, logout as apiLogout } from '@/api/auth'
import type { AuthUser } from '@/types'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<AuthUser | null>(null)
  const loading = ref(false)

  async function fetchMe() {
    loading.value = true
    try {
      user.value = await me()
    } catch {
      user.value = null
    } finally {
      loading.value = false
    }
  }

  async function logout() {
    try {
      await apiLogout()
    } finally {
      user.value = null
    }
  }

  function setUser(u: AuthUser) {
    user.value = u
  }

  const isLoggedIn = () => user.value !== null

  return { user, loading, fetchMe, logout, setUser, isLoggedIn }
})
