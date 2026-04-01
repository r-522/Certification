<template>
  <div class="auth-page">
    <div class="auth-box">
      <div class="auth-header">
        <h1 class="auth-brand">Shikaku</h1>
        <p class="auth-subtitle">新規アカウント登録</p>
      </div>

      <form class="auth-form" @submit.prevent="handleSignup" novalidate>
        <div v-if="error" class="alert alert-error">{{ error }}</div>

        <div class="form-group">
          <label class="form-label" for="username">ユーザー名</label>
          <input
            id="username"
            v-model="username"
            class="form-input"
            type="text"
            autocomplete="username"
            placeholder="山田 太郎"
            required
          />
        </div>

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
          <label class="form-label" for="password">
            パスワード
            <span class="form-hint">8文字以上</span>
          </label>
          <input
            id="password"
            v-model="password"
            class="form-input"
            type="password"
            autocomplete="new-password"
            placeholder="••••••••"
            required
            minlength="8"
          />
        </div>

        <button class="btn btn-primary auth-submit" type="submit" :disabled="loading">
          {{ loading ? '登録中...' : 'アカウントを作成' }}
        </button>
      </form>

      <p class="auth-footer">
        すでにアカウントをお持ちの方は
        <RouterLink to="/login">ログイン</RouterLink>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { signup } from '@/api/auth'
import { useAuthStore } from '@/stores/auth'
import { ApiError } from '@/api/client'

const router = useRouter()
const auth = useAuthStore()

const username = ref('')
const email = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

async function handleSignup() {
  error.value = ''
  if (!username.value.trim() || !email.value.trim() || !password.value) {
    error.value = '全ての項目を入力してください'
    return
  }
  if (password.value.length < 8) {
    error.value = 'パスワードは8文字以上で入力してください'
    return
  }
  loading.value = true
  try {
    const user = await signup(username.value.trim(), email.value.trim(), password.value)
    auth.setUser(user)
    router.push({ name: 'dashboard' })
  } catch (e) {
    if (e instanceof ApiError) {
      error.value = e.message
    } else {
      error.value = '登録に失敗しました。しばらくしてから再試行してください。'
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

.form-hint {
  font-weight: 400;
  color: var(--c-text-muted);
  font-size: 0.75rem;
  margin-left: 6px;
}

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
