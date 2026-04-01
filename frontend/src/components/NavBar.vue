<template>
  <header class="navbar">
    <div class="navbar-inner">
      <RouterLink to="/" class="navbar-brand">Shikaku</RouterLink>

      <nav class="navbar-nav" role="navigation">
        <RouterLink to="/" class="nav-link" :class="{ active: route.name === 'dashboard' }">
          ダッシュボード
        </RouterLink>
        <RouterLink to="/certs" class="nav-link" :class="{ active: route.name === 'certs' }">
          保有資格
        </RouterLink>
        <RouterLink to="/goals" class="nav-link" :class="{ active: route.name === 'goals' }">
          目標
        </RouterLink>
        <RouterLink to="/users" class="nav-link" :class="{ active: route.name === 'users' }">
          ユーザー
        </RouterLink>
      </nav>

      <div class="navbar-user">
        <span class="user-name">{{ auth.user?.usenm }}</span>
        <button class="btn btn-ghost btn-sm" @click="handleLogout" :disabled="loggingOut">
          ログアウト
        </button>
      </div>

      <!-- mobile menu toggle -->
      <button class="menu-toggle" @click="menuOpen = !menuOpen" aria-label="メニュー">
        <span class="toggle-bar" />
        <span class="toggle-bar" />
        <span class="toggle-bar" />
      </button>
    </div>

    <!-- mobile nav -->
    <div v-if="menuOpen" class="mobile-nav">
      <RouterLink to="/" class="mobile-nav-link" @click="menuOpen = false">ダッシュボード</RouterLink>
      <RouterLink to="/certs" class="mobile-nav-link" @click="menuOpen = false">保有資格</RouterLink>
      <RouterLink to="/goals" class="mobile-nav-link" @click="menuOpen = false">目標</RouterLink>
      <RouterLink to="/users" class="mobile-nav-link" @click="menuOpen = false">ユーザー</RouterLink>
      <button class="mobile-nav-link mobile-logout" @click="handleLogout">ログアウト</button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
const route = useRoute()
const router = useRouter()
const loggingOut = ref(false)
const menuOpen = ref(false)

async function handleLogout() {
  loggingOut.value = true
  try {
    await auth.logout()
    router.push({ name: 'login' })
  } finally {
    loggingOut.value = false
    menuOpen.value = false
  }
}
</script>

<style scoped>
.navbar {
  background: var(--c-brand);
  color: #fff;
  position: sticky;
  top: 0;
  z-index: 100;
  box-shadow: 0 1px 0 rgba(255,255,255,0.06);
}

.navbar-inner {
  display: flex;
  align-items: center;
  gap: 32px;
  max-width: 1120px;
  margin: 0 auto;
  padding: 0 24px;
  height: 56px;
}

.navbar-brand {
  font-size: 1.125rem;
  font-weight: 700;
  color: #fff;
  letter-spacing: -0.01em;
  flex-shrink: 0;
  text-decoration: none;
}

.navbar-nav {
  display: flex;
  gap: 4px;
  flex: 1;
}

.nav-link {
  padding: 6px 12px;
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  color: rgba(255,255,255,0.7);
  text-decoration: none;
  transition: background 0.15s, color 0.15s;
}
.nav-link:hover,
.nav-link.active {
  background: rgba(255,255,255,0.1);
  color: #fff;
}

.navbar-user {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-left: auto;
}
.user-name {
  font-size: 0.875rem;
  color: rgba(255,255,255,0.8);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 120px;
}

.menu-toggle {
  display: none;
  flex-direction: column;
  gap: 4px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 6px;
  margin-left: auto;
}
.toggle-bar {
  display: block;
  width: 22px;
  height: 2px;
  background: rgba(255,255,255,0.8);
  border-radius: 2px;
}

.mobile-nav {
  display: none;
  flex-direction: column;
  background: var(--c-brand-light);
  padding: 8px 0 12px;
}
.mobile-nav-link {
  padding: 10px 24px;
  font-size: 0.9375rem;
  color: rgba(255,255,255,0.85);
  text-decoration: none;
  background: none;
  border: none;
  text-align: left;
  cursor: pointer;
  font-family: inherit;
}
.mobile-nav-link:hover { background: rgba(255,255,255,0.07); }
.mobile-logout { color: rgba(255,255,255,0.6); }

@media (max-width: 768px) {
  .navbar-nav, .navbar-user { display: none; }
  .menu-toggle { display: flex; }
  .mobile-nav  { display: flex; }
}
</style>
