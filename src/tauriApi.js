import { appApi } from './api/appApi'
import { audioApi } from './api/audioApi'
import { floatingButtonApi } from './api/floatingButtonApi'
import { logApi } from './api/logApi'
import { pickCountApi } from './api/pickCountApi'
import { pickResultApi } from './api/pickResultApi'

export { appApi }

export function installTauriCompatApis() {
  window.floatingButtonApi = floatingButtonApi
  window.pickCountApi = pickCountApi
  window.pickResultApi = pickResultApi
  window.audioApi = audioApi
  window.logApi = logApi
}
