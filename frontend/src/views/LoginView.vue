<template>
  <div class="auth-page">
    <div class="auth-box">
      <div class="auth-header">
        <h1 class="auth-brand">Shikaku</h1>
        <p class="auth-subtitle">資格管理・目標設定システム</p>
      </div>

      <form class="auth-form" @submit.prevent="handleLogin" novalidate>
        <div v-if="error" class="alert alert-error">{{ error }}</div>

        <div class="form-group">
          <label class="form-label" for="email">メールアドレス</label>
          <input
            id="email"
            v-model="email"
            class="form-input"
            type="email"
            autocomplete="email"
            placeholder="you@example.com"
            required
          />
        </div>

        <div class="form-group">
          <label class="form-label" for="password">パスワード</label>
          <input
            id="password"
            v-model="password"
            class="form-input"
            type="password"
            autocomplete="current-password"
            placeholder="••••••••"
            required
          />
        </div>

        <button class="btn btn-primary auth-submit" type="submit" :disabled="loading">
          {{ loading ? 'ログイン中...' : 'ログイン' }}
        </button>
      </form>

      <p class="auth-footer">
        アカウントをお持ちでない方は
        <RouterLink to="/signup">新規登録</RouterLink>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { login } from '@/api/auth'
import { useAuthStore } from '@/stores/auth'
import { ApiError } from '@/api/client'

const router = useRouter()
const auth = useAuthStore()

const email = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

async function handleLogin() {
  error.value = ''
  if (!email.value || !password.value) {
    error.value = 'メールアドレスとパスワードを入力してください'
    return
  }
  loading.value = true
  try {
    const user = await login(email.value, password.value)
    auth.setUser(user)
    router.push({ name: 'dashboard' })
  } catch (e) {
    if (e instanceof ApiError) {
      error.value = e.message
    } else {
      error.value = 'ログインに失敗しました。しばらくしてから再試行してください。'
    }
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.auth-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  background: var(--c-bg);
}

.auth-box {
  width: 100%;
  max-width: 400px;
  background: var(--c-surface);
  border: 1px solid var(--c-border);
  border-radius: var(--radius-lg);
  padding: 40px 36px;
  box-shadow: var(--shadow-md);
}

.auth-header { text-align: center; margin-bottom: 32px; }

.auth-brand {
  font-size: 1.75rem;
  font-weight: 800;
  color: var(--c-brand);
  letter-spacing: -0.03em;
}

.auth-subtitle {
  font-size: 0.875rem;
  color: var(--c-text-secondary);
  margin-top: 6px;
}

.auth-form { display: flex; flex-direction: column; gap: 16px; }

.auth-submit { width: 100%; justify-content: center; padding: 11px; margin-top: 4px; }

.auth-footer {
  text-align: center;
  margin-top: 20px;
  font-size: 0.875rem;
  color: var(--c-text-secondary);
}

@media (max-width: 480px) {
  .auth-box { padding: 28px 20px; }
}
</style>
