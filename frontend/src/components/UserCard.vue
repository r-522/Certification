<template>
  <div class="user-card" :class="{ 'is-favorite': user.is_favorite, 'is-self': isSelf }">
    <div class="card-header">
      <div class="avatar" :style="{ background: avatarColor }">
        {{ initial }}
      </div>
      <div class="user-info">
        <p class="user-name">
          {{ user.usenm }}
          <span v-if="isSelf" class="badge badge-neutral self-badge">自分</span>
        </p>
        <p class="user-meta">資格 {{ user.cert_count }}件</p>
      </div>
      <button
        v-if="!isSelf"
        class="fav-btn"
        :class="{ active: user.is_favorite }"
        @click="$emit('toggle-favorite', user.useid)"
        :aria-label="user.is_favorite ? 'お気に入り解除' : 'お気に入り登録'"
        :title="user.is_favorite ? 'お気に入り解除' : 'お気に入り登録'"
      >
        {{ user.is_favorite ? '★' : '☆' }}
      </button>
    </div>

    <div class="card-body" v-if="user.cert_count > 0 || user.achieved_count > 0">
      <div class="stats">
        <div class="stat" v-if="user.cert_count > 0">
          <span class="stat-value">{{ user.cert_count }}</span>
          <span class="stat-label">保有</span>
        </div>
        <div class="stat achieved" v-if="user.achieved_count > 0">
          <span class="stat-value">{{ user.achieved_count }}</span>
          <span class="stat-label">達成</span>
        </div>
      </div>
    </div>

    <!-- グッドマーク: 達成実績のあるユーザー -->
    <div v-if="user.has_achievement" class="achievement-badge" title="目標達成実績あり">
      <span class="good-icon">✓</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { UserSummary } from '@/types'
import { useAuthStore } from '@/stores/auth'

const props = defineProps<{ user: UserSummary }>()
defineEmits<{ 'toggle-favorite': [userId: string] }>()

const auth = useAuthStore()
const isSelf = computed(() => auth.user?.useid === props.user.useid)

const initial = computed(() => props.user.usenm.charAt(0).toUpperCase())

// ユーザー名からアバター背景色を一貫して生成
const AVATAR_COLORS = [
  '#0369a1','#0891b2','#0d9488','#059669',
  '#65a30d','#ca8a04','#d97706','#ea580c',
  '#dc2626','#db2777','#7c3aed','#4f46e5',
]
const avatarColor = computed(() => {
  let n = 0
  for (const c of props.user.usenm) n += c.charCodeAt(0)
  return AVATAR_COLORS[n % AVATAR_COLORS.length]
})
</script>

<style scoped>
.user-card {
  background: var(--c-surface);
  border: 1px solid var(--c-border);
  border-radius: var(--radius-md);
  padding: 16px;
  position: relative;
  transition: box-shadow 0.15s, border-color 0.15s;
}
.user-card:hover { box-shadow: var(--shadow-md); }

.user-card.is-favorite {
  border-color: #fbbf24;
  background: #fffbeb;
}
.user-card.is-self {
  border-style: dashed;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  font-weight: 700;
  color: #fff;
  flex-shrink: 0;
}

.user-info { flex: 1; min-width: 0; }

.user-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--c-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: flex;
  align-items: center;
  gap: 6px;
}
.self-badge { font-size: 0.6875rem; }

.user-meta {
  font-size: 0.8125rem;
  color: var(--c-text-secondary);
  margin-top: 2px;
}

.fav-btn {
  background: none;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  padding: 4px;
  line-height: 1;
  color: var(--c-text-muted);
  transition: color 0.15s, transform 0.15s;
  flex-shrink: 0;
}
.fav-btn:hover   { color: #f59e0b; transform: scale(1.15); }
.fav-btn.active  { color: #f59e0b; }

.card-body { margin-top: 12px; }

.stats {
  display: flex;
  gap: 16px;
}
.stat {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}
.stat-value {
  font-size: 1.125rem;
  font-weight: 700;
  color: var(--c-text-primary);
}
.stat-label {
  font-size: 0.75rem;
  color: var(--c-text-secondary);
}
.stat.achieved .stat-value { color: var(--c-success); }

/* グッドマーク */
.achievement-badge {
  position: absolute;
  top: -6px;
  right: -6px;
  width: 22px;
  height: 22px;
  background: var(--c-success);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 2px solid var(--c-surface);
}
.good-icon {
  color: #fff;
  font-size: 0.6875rem;
  font-weight: 700;
}
</style>
