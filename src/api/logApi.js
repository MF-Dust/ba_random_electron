import { invoke } from './tauriCore'

export const logApi = {
  send: (level, text) => {
    const safeText = String(text || '').slice(0, 800)
    if (!safeText) return
    invoke('renderer_log', { level, text: safeText }).catch(() => {})
  }
}
