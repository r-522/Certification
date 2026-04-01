<template>
  <div class="autocomplete" ref="root">
    <input
      class="form-input"
      type="text"
      :value="modelValue"
      :placeholder="placeholder"
      autocomplete="off"
      @input="onInput"
      @keydown.down.prevent="moveDown"
      @keydown.up.prevent="moveUp"
      @keydown.enter.prevent="selectHighlighted"
      @keydown.escape="close"
      @blur="onBlur"
      @focus="onFocus"
    />
    <ul v-if="open && suggestions.length" class="dropdown" role="listbox">
      <li
        v-for="(item, i) in suggestions"
        :key="item.catid"
        class="dropdown-item"
        :class="{ highlighted: i === highlightedIndex }"
        @mousedown.prevent="select(item.catnm)"
        role="option"
      >
        {{ item.catnm }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { searchCatalog } from '@/api/certifications'

const props = defineProps<{
  modelValue: string
  placeholder?: string
}>()
const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const suggestions = ref<{ catid: string; catnm: string }[]>([])
const open = ref(false)
const highlightedIndex = ref(-1)
let debounceTimer: ReturnType<typeof setTimeout>

async function onInput(e: Event) {
  const val = (e.target as HTMLInputElement).value
  emit('update:modelValue', val)

  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(async () => {
    if (val.trim().length === 0) {
      suggestions.value = []
      return
    }
    try {
      suggestions.value = await searchCatalog(val)
      open.value = true
      highlightedIndex.value = -1
    } catch {
      suggestions.value = []
    }
  }, 200)
}

function onFocus() {
  if (suggestions.value.length) open.value = true
}

function onBlur() {
  setTimeout(() => { open.value = false }, 150)
}

function close() { open.value = false }

function moveDown() {
  if (!open.value) return
  highlightedIndex.value = Math.min(highlightedIndex.value + 1, suggestions.value.length - 1)
}
function moveUp() {
  if (!open.value) return
  highlightedIndex.value = Math.max(highlightedIndex.value - 1, 0)
}
function selectHighlighted() {
  if (highlightedIndex.value >= 0 && suggestions.value[highlightedIndex.value]) {
    select(suggestions.value[highlightedIndex.value].catnm)
  }
}
function select(name: string) {
  emit('update:modelValue', name)
  suggestions.value = []
  open.value = false
}
</script>

<style scoped>
.autocomplete { position: relative; }

.dropdown {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  background: var(--c-surface);
  border: 1px solid var(--c-border);
  border-radius: var(--radius-sm);
  box-shadow: var(--shadow-md);
  list-style: none;
  max-height: 240px;
  overflow-y: auto;
  z-index: 200;
}

.dropdown-item {
  padding: 9px 12px;
  font-size: 0.9375rem;
  cursor: pointer;
  color: var(--c-text-primary);
  transition: background 0.1s;
}
.dropdown-item:hover,
.dropdown-item.highlighted {
  background: var(--c-action-light);
  color: var(--c-action);
}
</style>
