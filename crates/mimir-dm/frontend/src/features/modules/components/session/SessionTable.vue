<template>
  <div class="session-table-container">
    <div class="session-header flex justify-between items-center mb-4">
      <h3 class="text-xl font-bold">Sessions</h3>
      <button v-if="!props.readonly" @click="$emit('create')" class="btn btn-primary btn-sm">
        Add Session
      </button>
    </div>
    
    <div v-if="sessions.length === 0" class="empty-state text-center py-8">
      <p class="text-gray-500">No sessions yet. Start by creating your first session!</p>
    </div>
    
    <div v-else class="overflow-x-auto">
      <table class="table table-zebra w-full">
        <thead>
          <tr>
            <th>Session</th>
            <th>Date</th>
            <th>Session #</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="session in sessions" :key="session.id">
            <td>Session #{{ session.session_number || session.id }}</td>
            <td>{{ formatDate(session.scheduled_date || session.actual_date || '') }}</td>
            <td>{{ session.session_number || '-' }}</td>
            <td>
              <select 
                :value="session.status"
                @change="$emit('transition', session.id, ($event.target as HTMLSelectElement).value)"
                class="select select-sm"
                :class="getStatusClass(session.status)"
                :disabled="props.readonly"
              >
                <option value="next_week">Next Week</option>
                <option value="prep_needed">Prep Needed</option>
                <option value="in_prep">In Prep</option>
                <option value="ready">Ready</option>
                <option value="complete">Complete</option>
              </select>
            </td>
            <td>
              <div class="flex gap-2">
                <button 
                  @click="$emit('open-document', session, 'outline')"
                  class="btn btn-ghost btn-xs"
                  title="Session Outline"
                >
                  📝 Outline
                </button>
                <button 
                  @click="$emit('open-document', session, 'notes')"
                  class="btn btn-ghost btn-xs"
                  title="Session Notes"
                >
                  📔 Notes
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Session } from '@/types'

interface Props {
  sessions: Session[]
  readonly?: boolean
}

const props = defineProps<Props>()

defineEmits<{
  create: []
  'open-document': [session: Session, docType: 'outline' | 'notes']
  transition: [id: number | string, status: string]
  delete: [id: number | string]
}>()

function formatDate(date: string): string {
  if (!date) return 'Not set'
  try {
    return new Date(date).toLocaleDateString()
  } catch {
    return date
  }
}

function getStatusClass(status: string): string {
  const classes: Record<string, string> = {
    planned: 'badge-info',
    active: 'badge-success',
    completed: 'badge-neutral'
  }
  return classes[status] || 'badge-ghost'
}
</script>