import type { AuthUser } from '@/types'
import { get, post } from './client'

export const signup = (usenm: string, useml: string, usepw: string) =>
  post<AuthUser>('/auth/signup', { usenm, useml, usepw })

export const login = (useml: string, usepw: string) =>
  post<AuthUser>('/auth/login', { useml, usepw })

export const logout = () => post<void>('/auth/logout')

export const me = () => get<AuthUser>('/auth/me')
