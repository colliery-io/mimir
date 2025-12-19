<template>
  <div v-if="show" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content">
      <h2>Create New Module</h2>
      <div class="form-group">
        <label for="module-name">Module Name:</label>
        <input
          id="module-name"
          v-model="moduleName"
          type="text"
          placeholder="Enter module name"
          @keyup.enter="handleCreate"
        />
      </div>
      <div class="form-group">
        <label for="module-type">Module Type:</label>
        <select id="module-type" v-model="moduleType">
          <option value="standard">Standard Adventure</option>
          <option value="mystery">Mystery</option>
          <option value="dungeon">Dungeon Crawl</option>
          <option value="heist">Heist</option>
          <option value="horror">Horror</option>
          <option value="political">Political Intrigue</option>
        </select>
      </div>
      <div class="form-group">
        <label for="module-sessions">Expected Sessions:</label>
        <input
          id="module-sessions"
          v-model.number="moduleSessions"
          type="number"
          min="1"
          placeholder="4"
          @keyup.enter="handleCreate"
        />
      </div>
      <div class="modal-actions">
        <button @click="$emit('close')" class="btn btn-secondary">
          Cancel
        </button>
        <button @click="handleCreate" class="btn btn-primary">
          Create Module
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  close: []
  create: [data: { name: string; type: string; sessions: number }]
}>()

const moduleName = ref('')
const moduleType = ref('standard')
const moduleSessions = ref(4)

// Reset form when modal closes
watch(() => props.show, (newShow) => {
  if (!newShow) {
    moduleName.value = ''
    moduleType.value = 'standard'
    moduleSessions.value = 4
  }
})

const handleCreate = () => {
  if (!moduleName.value.trim()) {
    return
  }

  emit('create', {
    name: moduleName.value.trim(),
    type: moduleType.value,
    sessions: moduleSessions.value
  })
}
</script>
