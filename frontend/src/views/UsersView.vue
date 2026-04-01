<template>
  <div>
    <div class="page-header">
      <div>
        <h1 class="page-title">ユーザー一覧</h1>
        <p class="page-sub">★ でお気に入り登録すると一覧上部に表示されます</p>
      </div>
      <div class="search-wrap">
        <input
          v-model="searchQuery"
          class="form-input search-input"
          type="text"
          placeholder="名前で絞り込む"
        />
      </div>
    </div>

    <div v-if="loading" class="loading-row">読み込み中...</div>

    <div v-else-if="filteredUsers.length === 0" class="empty-state">
      <p>{{ searchQuery ? '該当するユーザーがいません。' : 'ユーザーがまだ登録されていません。' }}</p>
    </div>

    <template v-else>
      <!-- お気に入りセクション -->
      <div v-if="favoriteUsers.length" class="section-block">
        <h2 class="section-title section-fav">
          <span class="fav-icon">★</span> お気に入り
        </h2>
        <div class="user-grid fav-grid">
          <UserCard
            v-for="user in favoriteUsers"
            :key="user.useid"
            :user="user"
            @toggle-favorite="handleFavorite"
          />
        </div>
        <div class="divider" />
      </div>

      <!-- 全ユーザー（崩しを入れた不揃いグリッド） -->
      <div class="section-block">
        <h2 class="section-title" v-if="favoriteUsers.length">その他のユーザー</h2>
        <div class="user-grid mosaic-grid">
          <UserCard
            v-for="(user, idx) in otherUsers"
            :key="user.useid"
            :user="user"
            :class="mosaicClass(idx)"
            @toggle-favorite="handleFavorite"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { UserSummary } from '@/types'
import { listUsers, toggleFavorite } from '@/api/users'
import UserCard from '@/components/UserCard.vue'

const users = ref<UserSummary[]>([])
const loading = ref(true)
const searchQuery = ref('')

const filteredUsers = computed(() => {
  const q = searchQuery.value.trim().toLowerCase()
  if (!q) return users.value
  return users.value.filter(u => u.usenm.toLowerCase().includes(q))
})

const favoriteUsers = computed(() => filteredUsers.value.filter(u => u.is_favorite))
const otherUsers = computed(() => filteredUsers.value.filter(u => !u.is_favorite))

// 不揃いグリッド: 5件ごとに1件だけ wide カードにする
function mosaicClass(idx: number): string {
  if (idx % 7 === 2) return 'wide'
  if (idx % 7 === 5) return 'wide'
  return ''
}

async function handleFavorite(userId: string) {
  const result = await toggleFavorite(userId)
  const target = users.value.find(u => u.useid === userId)
  if (target) target.is_favorite = result.is_favorite
}

onMounted(async () => {
  try { users.value = await listUsers() } finally { loading.value = false }
})
</script>

<style scoped>
.page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 28px;
  flex-wrap: wrap;
}

.page-sub {
  font-size: 0.8125rem;
  color: var(--c-text-secondary);
  margin-top: 4px;
}

.search-wrap { flex-shrink: 0; }
.search-input { width: 220px; }

.loading-row { color: var(--c-text-muted); padding: 24px 0; }

.section-block { margin-bottom: 12px; }

.section-fav {
  display: flex;
  align-items: center;
  gap: 6px;
}
.fav-icon { color: #f59e0b; }

/* お気に入りグリッド: 横並び、等サイズ */
.fav-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

/* 崩しグリッド: 一部カードを横長に */
.mosaic-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 12px;
}

/* wide カードは 2 列分を占有 */
.mosaic-grid .wide {
  grid-column: span 2;
}

/* 列数が足りなければ span を自動解除 */
@media (max-width: 500px) {
  .mosaic-grid .wide { grid-column: span 1; }
  .search-input { width: 160px; }
}
</style>
