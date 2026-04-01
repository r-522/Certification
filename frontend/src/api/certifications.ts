import type { Cert } from '@/types'
import { del, get, post, put } from './client'

export const listCerts = () => get<Cert[]>('/certifications')

export const createCert = (payload: { cernm: string; cerdt?: string | null; cerno?: string | null }) =>
  post<Cert>('/certifications', payload)

export const updateCert = (id: string, payload: { cernm?: string; cerdt?: string | null; cerno?: string | null }) =>
  put<Cert>(`/certifications/${id}`, payload)

export const deleteCert = (id: string) => del<void>(`/certifications/${id}`)

export const searchCatalog = (q: string) =>
  get<{ catid: string; catnm: string }[]>(`/catalog?q=${encodeURIComponent(q)}`)
