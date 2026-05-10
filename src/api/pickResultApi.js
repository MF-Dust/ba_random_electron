import { invoke, listenCompat } from './tauriCore'

export const pickResultApi = {
  getResults: () => invoke('get_pick_results'),
  getConfig: () => invoke('get_pick_result_config'),
  close: () => invoke('close_pick_result'),
  onOpen: (callback) => listenCompat('pick-result-open', callback),
  onReset: (callback) => listenCompat('pick-result-reset', callback)
}
