import type { UserSummary } from '@/types'
import { get, post } from './client'

export const listUsers = () => get<UserSummary[]>('/users')

export const toggleFavorite = (userId: string) =>
  post<{ is_favorite: boolean }>(`/favorites/${userId}`)
