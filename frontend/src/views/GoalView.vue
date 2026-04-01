<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">目標管理</h1>
      <button class="btn btn-primary" @click="openAddForm">＋ 追加</button>
    </div>

    <!-- 追加/編集フォーム -->
    <div v-if="formOpen" class="card form-card">
      <h2 class="section-title">{{ editTarget ? '目標を編集' : '目標を追加' }}</h2>
      <div v-if="formError" class="alert alert-error" style="margin-bottom:12px">{{ formError }}</div>

      <form class="form-fields" @submit.prevent="submitForm" novalidate>
        <div class="form-group">
          <label class="form-label">目標資格 <span class="required">*</span></label>
          <CertAutocomplete v-model="form.golnm" placeholder="例: AWS 認定ソリューションアーキテクト" />
        </div>
        <div class="form-row">
          <div class="form-group">
            <label class="form-label">目標取得日</label>
            <input v-model="form.goldt" class="form-input" type="date" />
          </div>
          <div class="form-group" v-if="editTarget">
            <label class="form-label">ステータス</label>
            <select v-model="form.golst" class="form-select">
              <option value="active">目標中</option>
              <option value="achieved">達成</option>
              <option value="abandoned">中断</option>
            </select>
          </div>
        </div>
        <div class="form-group">
          <label class="form-label">メモ</label>
          <textarea v-model="form.golno" class="form-textarea" placeholder="学習計画、参考書など" />
        </div>
        <div class="form-actions">
          <button type="button" class="btn btn-secondary" @click="closeForm">キャンセル</button>
          <button type="submit" class="btn btn-primary" :disabled="submitting">
            {{ submitting ? '保存中...' : '保存' }}
          </button>
        </div>
      </form>
    </div>

    <!-- ステータスタブ -->
    <div class="tab-bar">
      <button
        v-for="tab in tabs"
        :key="tab.value"
        class="tab-btn"
        :class="{ active: activeTab === tab.value }"
        @click="activeTab = tab.value"
      >
        {{ tab.label }}
        <span class="tab-count">{{ tabCounts[tab.value] }}</span>
      </button>
    </div>

    <div v-if="loading" class="loading-row">読み込み中...</div>

    <div v-else-if="filteredGoals.length === 0" class="empty-state">
      <p>{{ activeTab === 'active' ? '目標中の資格はありません。' : '該当する目標はありません。' }}</p>
    </div>

    <div v-else class="goal-list">
      <div
        v-for="goal in filteredGoals"
        :key="goal.golid"
        class="goal-item"
        :class="`status-${goal.golst}`"
      >
        <div class="goal-left">
          <span class="status-dot" :class="`dot-${goal.golst}`" />
          <div class="goal-info">
            <p class="goal-name">{{ goal.golnm }}</p>
            <p class="goal-meta">
              <span v-if="goal.goldt" :class="deadlineBadgeClass(goal)">
                {{ formatDate(goal.goldt) }} まで
              </span>
              <span v-if="goal.golno" class="goal-note">{{ goal.golno }}</span>
            </p>
          </div>
        </div>
        <div class="goal-actions">
          <button
            v-if="goal.golst === 'active'"
            class="btn btn-sm"
            style="color:var(--c-success);border-color:var(--c-success)"
            @click="markAchieved(goal)"
            title="達成にする"
          >
            達成
          </button>
          <button class="btn btn-ghost btn-sm" @click="openEditForm(goal)">編集</button>
          <button class="btn btn-danger btn-sm" @click="confirmDelete(goal)">削除</button>
        </div>
      </div>
    </div>

    <!-- 削除確認 -->
    <div v-if="deleteTarget" class="dialog-overlay" @click.self="deleteTarget = null">
      <div class="dialog-box">
        <p class="dialog-msg">「{{ deleteTarget.golnm }}」を削除しますか？</p>
        <div class="dialog-actions">
          <button class="btn btn-secondary" @click="deleteTarget = null">キャンセル</button>
          <button
            class="btn btn-primary"
            style="background:var(--c-danger);border-color:var(--c-danger)"
            @click="doDelete"
            :disabled="submitting"
          >削除する</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { Goal, GoalStatus } from '@/types'
import { listGoals, createGoal, updateGoal, deleteGoal } from '@/api/goals'
import { ApiError } from '@/api/client'
import CertAutocomplete from '@/components/CertAutocomplete.vue'

const goals = ref<Goal[]>([])
const loading = ref(true)
const formOpen = ref(false)
const submitting = ref(false)
const formError = ref('')
const editTarget = ref<Goal | null>(null)
const deleteTarget = ref<Goal | null>(null)
const activeTab = ref<GoalStatus>('active')

const form = ref({ golnm: '', goldt: '', golst: 'active' as GoalStatus, golno: '' })

const tabs = [
  { value: 'active'    as GoalStatus, label: '目標中' },
  { value: 'achieved'  as GoalStatus, label: '達成済み' },
  { value: 'abandoned' as GoalStatus, label: '中断' },
]

const tabCounts = computed(() => ({
  active:    goals.value.filter(g => g.golst === 'active').length,
  achieved:  goals.value.filter(g => g.golst === 'achieved').length,
  abandoned: goals.value.filter(g => g.golst === 'abandoned').length,
}))

const filteredGoals = computed(() =>
  goals.value.filter(g => g.golst === activeTab.value)
)

function deadlineBadgeClass(goal: Goal): string {
  if (!goal.goldt || goal.golst !== 'active') return 'badge badge-neutral'
  const diff = (new Date(goal.goldt).getTime() - Date.now()) / 86400000
  if (diff < 0) return 'badge badge-danger'
  if (diff <= 14) return 'badge badge-warn'
  return 'badge badge-neutral'
}

function formatDate(d: string) { return d.replace(/-/g, '/') }

function openAddForm() {
  editTarget.value = null
  form.value = { golnm: '', goldt: '', golst: 'active', golno: '' }
  formError.value = ''
  formOpen.value = true
}

function openEditForm(goal: Goal) {
  editTarget.value = goal
  form.value = { golnm: goal.golnm, goldt: goal.goldt ?? '', golst: goal.golst as GoalStatus, golno: goal.golno ?? '' }
  formError.value = ''
  formOpen.value = true
}

function closeForm() { formOpen.value = false; editTarget.value = null }
function confirmDelete(goal: Goal) { deleteTarget.value = goal }

async function markAchieved(goal: Goal) {
  const updated = await updateGoal(goal.golid, { golst: 'achieved' })
  const idx = goals.value.findIndex(g => g.golid === updated.golid)
  if (idx !== -1) goals.value[idx] = updated
}

async function submitForm() {
  if (!form.value.golnm.trim()) { formError.value = '目標資格名を入力してください'; return }
  submitting.value = true
  formError.value = ''
  try {
    const payload = {
      golnm: form.value.golnm.trim(),
      goldt: form.value.goldt || null,
      golst: form.value.golst,
      golno: form.value.golno.trim() || null,
    }
    if (editTarget.value) {
      const updated = await updateGoal(editTarget.value.golid, payload)
      const idx = goals.value.findIndex(g => g.golid === updated.golid)
      if (idx !== -1) goals.value[idx] = updated
    } else {
      const created = await createGoal(payload)
      goals.value.unshift(created)
    }
    closeForm()
  } catch (e) {
    formError.value = e instanceof ApiError ? e.message : '保存に失敗しました'
  } finally {
    submitting.value = false
  }
}

async function doDelete() {
  if (!deleteTarget.value) return
  submitting.value = true
  try {
    await deleteGoal(deleteTarget.value.golid)
    goals.value = goals.value.filter(g => g.golid !== deleteTarget.value!.golid)
    deleteTarget.value = null
  } finally {
    submitting.value = false
  }
}

onMounted(async () => {
  try { goals.value = await listGoals() } finally { loading.value = false }
})
</script>

<style scoped>
.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
  gap: 16px;
}

.loading-row { color: var(--c-text-muted); padding: 24px 0; }
.required { color: var(--c-danger); }

.form-card { margin-bottom: 24px; }
.form-fields { display: flex; flex-direction: column; gap: 14px; }
.form-row { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
.form-actions { display: flex; gap: 10px; justify-content: flex-end; padding-top: 4px; }

/* Tabs */
.tab-bar {
  display: flex;
  gap: 4px;
  margin-bottom: 16px;
  border-bottom: 2px solid var(--c-border);
  padding-bottom: 0;
}

.tab-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  margin-bottom: -2px;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--c-text-secondary);
  cursor: pointer;
  transition: color 0.15s, border-color 0.15s;
}
.tab-btn:hover { color: var(--c-text-primary); }
.tab-btn.active { color: var(--c-action); border-bottom-color: var(--c-action); }

.tab-count {
  background: var(--c-bg);
  border: 1px solid var(--c-border);
  border-radius: 99px;
  font-size: 0.75rem;
  padding: 0 6px;
  min-width: 20px;
  text-align: center;
}

/* Goal list */
.goal-list { display: flex; flex-direction: column; gap: 8px; }

.goal-item {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 14px 16px;
  background: var(--c-surface);
  border: 1px solid var(--c-border);
  border-radius: var(--radius-sm);
}
.goal-item.status-achieved { opacity: 0.75; }
.goal-item.status-abandoned { opacity: 0.55; }

.goal-left { display: flex; align-items: flex-start; gap: 12px; flex: 1; min-width: 0; }

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
  margin-top: 6px;
}
.dot-active    { background: var(--c-action); }
.dot-achieved  { background: var(--c-success); }
.dot-abandoned { background: var(--c-text-muted); }

.goal-info { min-width: 0; }

.goal-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--c-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.goal-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
  flex-wrap: wrap;
}

.goal-note {
  font-size: 0.8125rem;
  color: var(--c-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 280px;
}

.goal-actions { display: flex; gap: 6px; flex-shrink: 0; align-items: center; }

/* Dialog */
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 300;
  padding: 24px;
}
.dialog-box {
  background: var(--c-surface);
  border-radius: var(--radius-md);
  padding: 28px 24px;
  max-width: 380px;
  width: 100%;
  box-shadow: var(--shadow-md);
}
.dialog-msg { font-size: 0.9375rem; margin-bottom: 20px; line-height: 1.5; }
.dialog-actions { display: flex; gap: 10px; justify-content: flex-end; }

@media (max-width: 480px) {
  .form-row { grid-template-columns: 1fr; }
  .goal-item { flex-direction: column; }
  .goal-actions { justify-content: flex-end; }
}
</style>
