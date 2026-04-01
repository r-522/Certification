<template>
  <div>
    <div class="page-header">
      <h1 class="page-title">保有資格</h1>
      <button class="btn btn-primary" @click="openAddForm">＋ 追加</button>
    </div>

    <!-- 追加/編集フォーム -->
    <div v-if="formOpen" class="card form-card">
      <h2 class="section-title">{{ editTarget ? '資格を編集' : '資格を追加' }}</h2>
      <div v-if="formError" class="alert alert-error" style="margin-bottom:12px">{{ formError }}</div>

      <form class="form-fields" @submit.prevent="submitForm" novalidate>
        <div class="form-group">
          <label class="form-label">資格名 <span class="required">*</span></label>
          <CertAutocomplete v-model="form.cernm" placeholder="例: 基本情報技術者試験" />
        </div>
        <div class="form-row">
          <div class="form-group">
            <label class="form-label">取得日</label>
            <input v-model="form.cerdt" class="form-input" type="date" />
          </div>
        </div>
        <div class="form-group">
          <label class="form-label">メモ</label>
          <textarea v-model="form.cerno" class="form-textarea" placeholder="合格スコアや受験回数など" />
        </div>
        <div class="form-actions">
          <button type="button" class="btn btn-secondary" @click="closeForm">キャンセル</button>
          <button type="submit" class="btn btn-primary" :disabled="submitting">
            {{ submitting ? '保存中...' : '保存' }}
          </button>
        </div>
      </form>
    </div>

    <!-- 一覧 -->
    <div v-if="loading" class="loading-row">読み込み中...</div>

    <div v-else-if="certs.length === 0 && !formOpen" class="empty-state">
      <p>保有資格がまだ登録されていません。</p>
      <button class="btn btn-primary" style="margin-top:16px" @click="openAddForm">最初の資格を追加</button>
    </div>

    <div v-else class="cert-list">
      <div v-for="cert in certs" :key="cert.cerid" class="cert-item">
        <div class="cert-main">
          <p class="cert-name">{{ cert.cernm }}</p>
          <p class="cert-meta">
            <span v-if="cert.cerdt" class="badge badge-success">{{ formatDate(cert.cerdt) }} 取得</span>
            <span v-if="cert.cerno" class="cert-note">{{ cert.cerno }}</span>
          </p>
        </div>
        <div class="cert-actions">
          <button class="btn btn-ghost btn-sm" @click="openEditForm(cert)">編集</button>
          <button class="btn btn-danger btn-sm" @click="confirmDelete(cert)">削除</button>
        </div>
      </div>
    </div>

    <!-- 削除確認 -->
    <div v-if="deleteTarget" class="dialog-overlay" @click.self="deleteTarget = null">
      <div class="dialog-box">
        <p class="dialog-msg">「{{ deleteTarget.cernm }}」を削除しますか？</p>
        <div class="dialog-actions">
          <button class="btn btn-secondary" @click="deleteTarget = null">キャンセル</button>
          <button class="btn btn-primary" style="background:var(--c-danger);border-color:var(--c-danger)" @click="doDelete" :disabled="submitting">
            削除する
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { Cert } from '@/types'
import { listCerts, createCert, updateCert, deleteCert } from '@/api/certifications'
import { ApiError } from '@/api/client'
import CertAutocomplete from '@/components/CertAutocomplete.vue'

const certs = ref<Cert[]>([])
const loading = ref(true)
const formOpen = ref(false)
const submitting = ref(false)
const formError = ref('')
const editTarget = ref<Cert | null>(null)
const deleteTarget = ref<Cert | null>(null)

const form = ref({ cernm: '', cerdt: '', cerno: '' })

function openAddForm() {
  editTarget.value = null
  form.value = { cernm: '', cerdt: '', cerno: '' }
  formError.value = ''
  formOpen.value = true
}

function openEditForm(cert: Cert) {
  editTarget.value = cert
  form.value = { cernm: cert.cernm, cerdt: cert.cerdt ?? '', cerno: cert.cerno ?? '' }
  formError.value = ''
  formOpen.value = true
}

function closeForm() {
  formOpen.value = false
  editTarget.value = null
}

function confirmDelete(cert: Cert) {
  deleteTarget.value = cert
}

function formatDate(d: string) {
  return d.replace(/-/g, '/')
}

async function submitForm() {
  if (!form.value.cernm.trim()) {
    formError.value = '資格名を入力してください'
    return
  }
  submitting.value = true
  formError.value = ''
  try {
    const payload = {
      cernm: form.value.cernm.trim(),
      cerdt: form.value.cerdt || null,
      cerno: form.value.cerno.trim() || null,
    }
    if (editTarget.value) {
      const updated = await updateCert(editTarget.value.cerid, payload)
      const idx = certs.value.findIndex(c => c.cerid === updated.cerid)
      if (idx !== -1) certs.value[idx] = updated
    } else {
      const created = await createCert(payload)
      certs.value.unshift(created)
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
    await deleteCert(deleteTarget.value.cerid)
    certs.value = certs.value.filter(c => c.cerid !== deleteTarget.value!.cerid)
    deleteTarget.value = null
  } catch {
    // silent
  } finally {
    submitting.value = false
  }
}

onMounted(async () => {
  try {
    certs.value = await listCerts()
  } finally {
    loading.value = false
  }
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

.form-card { margin-bottom: 24px; }

.form-fields { display: flex; flex-direction: column; gap: 14px; }

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.form-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  padding-top: 4px;
}

.required { color: var(--c-danger); }

.cert-list { display: flex; flex-direction: column; gap: 1px; }

.cert-item {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 14px 16px;
  background: var(--c-surface);
  border: 1px solid var(--c-border);
  border-radius: var(--radius-sm);
  transition: box-shadow 0.12s;
}
.cert-item:hover { box-shadow: var(--shadow-sm); }
.cert-item + .cert-item { margin-top: 8px; }

.cert-main { flex: 1; min-width: 0; }

.cert-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--c-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.cert-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
  flex-wrap: wrap;
}

.cert-note {
  font-size: 0.8125rem;
  color: var(--c-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 300px;
}

.cert-actions { display: flex; gap: 6px; flex-shrink: 0; }

/* 削除確認ダイアログ */
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

.dialog-msg {
  font-size: 0.9375rem;
  color: var(--c-text-primary);
  margin-bottom: 20px;
  line-height: 1.5;
}

.dialog-actions { display: flex; gap: 10px; justify-content: flex-end; }

@media (max-width: 480px) {
  .form-row { grid-template-columns: 1fr; }
}
</style>
