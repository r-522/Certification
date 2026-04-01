export interface AuthUser {
  useid: string
  usenm: string
}

export interface Cert {
  cerid: string
  cerus: string
  cernm: string
  cerdt: string | null   // YYYY-MM-DD
  cerno: string | null
  cercr: string
  cerup: string
}

export type GoalStatus = 'active' | 'achieved' | 'abandoned'

export interface Goal {
  golid: string
  golus: string
  golnm: string
  goldt: string | null   // YYYY-MM-DD
  golst: GoalStatus
  golno: string | null
  golcr: string
  golup: string
}

export interface Catalog {
  catid: string
  catnm: string
  catcr: string
}

export interface UserSummary {
  useid: string
  usenm: string
  cert_count: number
  achieved_count: number
  has_achievement: boolean
  is_favorite: boolean
}
