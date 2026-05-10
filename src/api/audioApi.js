import { invoke } from './tauriCore'

export const audioApi = {
  playClickSound: () => invoke('play_click_sound'),
  playBgm: () => invoke('play_bgm'),
  stopBgm: () => invoke('stop_bgm'),
  playGachaSound: (volume) => invoke('play_gacha_sound', { volume }),
  stopGachaSound: () => invoke('stop_gacha_sound')
}
