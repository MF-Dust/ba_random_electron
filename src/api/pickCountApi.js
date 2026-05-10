import { invoke, listenCompat } from './tauriCore'

export const pickCountApi = {
  getConfig: () => invoke('get_pick_count_config'),
  cancel: () => invoke('cancel_pick_count'),
  confirm: (count, playMusic) => invoke('confirm_pick_count', { count, playMusic }),
  onOpen: (callback) => listenCompat('pick-count-open', callback),
  onStopBgm: (callback) => listenCompat('pick-count-stop-bgm', callback)
}
