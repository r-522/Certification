<template>
  <div>
    <div class="dashboard-header">
      <h1 class="page-title">おはようございます、{{ auth.user?.usenm }} さん</h1>
      <p class="header-sub">今日も資格取得に向けて一歩ずつ進めましょう。</p>
    </div>

    <div v-if="loading" class="loading-row">読み込み中...</div>

    <div v-else class="stat-grid">
      <RouterLink to="/certs" class="stat-card">
        <span class="stat-label">保有資格</span>
        <span class="stat-num">{{ certCount }}</span>
        <span class="stat-unit">件</span>
      </RouterLink>

      <RouterLink to="/goals" class="stat-card">
        <span class="stat-label">目標中</span>
        <span class="stat-num active-num">{{ activeGoalCount }}</span>
        <span class="stat-unit">件</span>
      </RouterLink>

      <RouterLink to="/goals" class="stat-card achieved-card">
        <span class="stat-label">達成済み目標</span>
        <span class="stat-num achieved-num">{{ achievedGoalCount }}</span>
        <span class="stat-unit">件</span>
      </RouterLink>

      <RouterLink to="/goals" class="stat-card warn-card" v-if="nearDeadlineCount > 0">
        <span class="stat-label">期限が近い目標</span>
        <span class="stat-num warn-num">{{ nearDeadlineCount }}</span>
        <span class="stat-unit">件</span>
      </RouterLink>
    </div>

    <div class="section-row" v-if="!loading">
      <div class="quick-links">
        <h2 class="section-title">クイックアクション</h2>
        <div class="quick-grid">
          <RouterLink to="/certs" class="quick-btn">
            <span class="quick-icon">＋</span> 資格を追加
          </RouterLink>
          <RouterLink to="/goals" class="quick-btn">
            <span class="quick-icon">◎</span> 目標を設定
          </RouterLink>
          <RouterLink to="/users" class="quick-btn">
            <span class="quick-icon">◈</span> 他のユーザーを見る
          </RouterLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { listCerts } from '@/api/certifications'
import { listGoals } from '@/api/goals'
import type { Cert, Goal } from '@/types'

const auth = useAuthStore()
const certs = ref<Cert[]>([])
const goals = ref<Goal[]>([])
const loading = ref(true)

const certCount = computed(() => certs.value.length)
const activeGoalCount = computed(() => goals.value.filter(g => g.golst === 'active').length)
const achievedGoalCount = computed(() => goals.value.filter(g => g.golst === 'achieved').length)
const nearDeadlineCount = computed(() => {
  const soon = new Date()
  soon.setDate(soon.getDate() + 14)
  return goals.value.filter(g => {
    if (g.golst !== 'active' || !g.goldt) return false
    return new Date(g.goldt) <= soon
  }).length
})

onMounted(async () => {
  try {
    ;[certs.value, goals.value] = await Promise.all([listCerts(), listGoals()])
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.dashboard-header { margin-bottom: 28px; }

.header-sub {
  margin-top: 4px;
  color: var(--c-text-secondary);
  font-size: 0.9375rem;
}

.loading-row {
  color: var(--c-text-muted);
  font-size: 0.9375rem;
  padding: 24px 0;
}

.stat-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 16px;
  margin-bottom: 36px;
}

.stat-card {
  background: var(--c-surface);
  border: 1px solid var(--c-border);
  border-radius: var(--radius-md);
  padding: 20px 20px 16px;
  text-decoration: none;
  display: flex;
  flex-direction: column;
  gap: 4px;
  transition: box-shadow 0.15s, border-color 0.15s;
}
.stat-card:hover { box-shadow: var(--shadow-md); border-color: var(--c-action); }

.stat-label {
  font-size: 0.8125rem;
  color: var(--c-text-secondary);
  font-weight: 500;
}

.stat-num {
  font-size: 2.25rem;
  font-weight: 800;
  color: var(--c-text-primary);
  line-height: 1;
  letter-spacing: -0.03em;
}

.stat-unit {
  font-size: 0.875rem;
  color: var(--c-text-muted);
}

.active-num { color: var(--c-action); }
.achieved-card .achieved-num { color: var(--c-success); }
.warn-card { border-color: #fbbf24; background: #fffbeb; }
.warn-num { color: var(--c-warn); }

.section-row { margin-top: 8px; }

.quick-grid {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.quick-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: var(--c-surface);
  border: 1px solid var(--c-border);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--c-text-primary);
  text-decoration: none;
  transition: background 0.15s, border-color 0.15s;
}
.quick-btn:hover {
  background: var(--c-action-light);
  border-color: var(--c-action);
  color: var(--c-action);
}

.quick-icon {
  font-size: 1rem;
  color: var(--c-action);
}
</style>
