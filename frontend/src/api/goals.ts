import type { Goal, GoalStatus } from '@/types'
import { del, get, post, put } from './client'

export const listGoals = () => get<Goal[]>('/goals')

export const createGoal = (payload: { golnm: string; goldt?: string | null; golno?: string | null }) =>
  post<Goal>('/goals', payload)

export const updateGoal = (
  id: string,
  payload: { golnm?: string; goldt?: string | null; golst?: GoalStatus; golno?: string | null },
) => put<Goal>(`/goals/${id}`, payload)

export const deleteGoal = (id: string) => del<void>(`/goals/${id}`)
